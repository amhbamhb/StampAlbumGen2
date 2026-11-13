use oxidize_pdf::parser::{ContentOperation, ContentParser, PdfDocument};
use stamp_album_gen2::native_pdf::{album::NativeAlbum, albumeasy};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/stamp-headings-v1")
}

#[derive(Clone, Copy, Debug)]
struct Transform {
    matrix: [f32; 6],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        }
    }
}

impl Transform {
    fn apply(&self, x: f32, y: f32) -> (f32, f32) {
        let [a, b, c, d, e, f] = self.matrix;
        (a * x + c * y + e, b * x + d * y + f)
    }

    fn multiply(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let [aa, bb, cc, dd, ee, ff] = self.matrix;
        self.matrix = [
            aa * a + cc * b,
            bb * a + dd * b,
            aa * c + cc * d,
            bb * c + dd * d,
            aa * e + cc * f + ee,
            bb * e + dd * f + ff,
        ];
    }
}

#[derive(Clone, Debug)]
struct Stroke {
    points: Vec<(f32, f32)>,
    width: f32,
}

#[derive(Clone, Debug)]
struct HeadingGlyph {
    text: String,
    left: f32,
    baseline: f32,
    colour: [f32; 3],
}

#[derive(Clone, Debug)]
struct PageGeometry {
    strokes: Vec<Stroke>,
    headings: Vec<HeadingGlyph>,
    images: Vec<(f32, f32, f32, f32)>,
}

fn parse_page(content: &str) -> Result<PageGeometry, String> {
    let mut stack: Vec<(Transform, [f32; 3], [f32; 3], f32)> = Vec::new();
    let mut transform = Transform::default();
    let mut stroke_colour = [0.0f32; 3];
    let mut text_colour = [0.0f32; 3];
    let mut width = 0.0f32;
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut strokes = Vec::new();
    let mut headings = Vec::new();
    let mut images = Vec::new();
    let mut text_matrix = [1.0f32, 0.0, 0.0, 1.0, 0.0, 0.0];
    let mut in_text = false;
    for operation in ContentParser::parse(content.as_bytes()).map_err(|e| e.to_string())? {
        match operation {
            ContentOperation::SaveGraphicsState => {
                stack.push((transform, stroke_colour, text_colour, width))
            }
            ContentOperation::RestoreGraphicsState => {
                (transform, stroke_colour, text_colour, width) =
                    stack.pop().ok_or("Unbalanced graphics state")?
            }
            ContentOperation::SetTransformMatrix(a, b, c, d, e, f) => {
                transform.multiply(a, b, c, d, e, f)
            }
            ContentOperation::SetLineWidth(w) => width = w,
            ContentOperation::SetStrokingRGB(r, g, b) => stroke_colour = [r, g, b],
            ContentOperation::MoveTo(x, y) => {
                points.clear();
                points.push(transform.apply(x, y));
            }
            ContentOperation::LineTo(x, y) => points.push(transform.apply(x, y)),
            ContentOperation::Rectangle(x, y, w, h) => {
                points = vec![
                    transform.apply(x, y),
                    transform.apply(x + w, y),
                    transform.apply(x + w, y + h),
                    transform.apply(x, y + h),
                    transform.apply(x, y),
                ]
            }
            ContentOperation::Stroke => {
                if !points.is_empty() {
                    strokes.push(Stroke {
                        points: points.clone(),
                        width,
                    });
                }
                points.clear();
            }
            ContentOperation::PaintXObject(_) => {
                let (x, y) = transform.apply(0.0, 0.0);
                let (r, t) = transform.apply(1.0, 1.0);
                images.push((x, y, r - x, t - y));
            }
            ContentOperation::BeginText => {
                in_text = true;
                text_matrix = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];
            }
            ContentOperation::EndText => in_text = false,
            ContentOperation::SetNonStrokingRGB(r, g, b) => text_colour = [r, g, b],
            ContentOperation::SetTextMatrix(a, b, c, d, e, f) => text_matrix = [a, b, c, d, e, f],
            ContentOperation::MoveText(tx, ty) => {
                text_matrix[4] += text_matrix[0] * tx + text_matrix[2] * ty;
                text_matrix[5] += text_matrix[1] * tx + text_matrix[3] * ty;
            }
            ContentOperation::ShowText(bytes) if in_text => {
                if let Ok(text) = String::from_utf8(bytes) {
                    let text = text.trim().to_string();
                    if !text.is_empty() {
                        let x = text_matrix[4];
                        let y = text_matrix[5];
                        let (left, baseline) = transform.apply(x, y);
                        headings.push(HeadingGlyph {
                            text,
                            left,
                            baseline,
                            colour: text_colour,
                        });
                    }
                }
            }
            _ => {}
        }
    }
    Ok(PageGeometry {
        strokes: strokes
            .into_iter()
            .filter(|stroke| {
                stroke.points.len() >= 2
                    && (stroke
                        .points
                        .iter()
                        .map(|point| point.0)
                        .fold(f32::NEG_INFINITY, f32::max)
                        - stroke
                            .points
                            .iter()
                            .map(|point| point.0)
                            .fold(f32::INFINITY, f32::min))
                        < 500.0
            })
            .collect(),
        headings,
        images,
    })
}

fn load_pdf(path: &Path) -> Result<Vec<PageGeometry>, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    (0..pdf.page_count().map_err(|e| e.to_string())?)
        .map(|index| {
            let page = pdf.get_page(index).map_err(|e| e.to_string())?;
            let content = pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?
                .iter()
                .map(|stream| String::from_utf8_lossy(stream))
                .collect::<Vec<_>>()
                .join("\n");
            parse_page(&content)
        })
        .collect()
}

fn frame_tops(page: &PageGeometry) -> Vec<f32> {
    page.strokes
        .iter()
        .filter_map(|stroke| {
            if stroke.points.len() == 5 && (stroke.width - 0.57).abs() < 0.03 {
                Some(
                    stroke
                        .points
                        .iter()
                        .map(|point| point.1)
                        .fold(f32::NEG_INFINITY, f32::max),
                )
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn heading_signature(
    pages: &[PageGeometry],
    interesting: &[&str],
) -> BTreeSet<(String, i32, i32, i32, i32, i32)> {
    let set = interesting
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    pages
        .iter()
        .flat_map(|page| page.headings.iter())
        .filter(|glyph| set.contains(glyph.text.as_str()))
        .map(|glyph| {
            (
                glyph.text.clone(),
                (glyph.left * 100.0).round() as i32,
                (glyph.baseline * 100.0).round() as i32,
                (glyph.colour[0] * 1000.0).round() as i32,
                (glyph.colour[1] * 1000.0).round() as i32,
                (glyph.colour[2] * 1000.0).round() as i32,
            )
        })
        .collect()
}

fn image_signature(pages: &[PageGeometry]) -> BTreeSet<(i32, i32, i32, i32)> {
    pages
        .iter()
        .flat_map(|page| page.images.iter())
        .map(|&(x, y, w, h)| {
            (
                (x * 100.0).round() as i32,
                (y * 100.0).round() as i32,
                (w * 100.0).round() as i32,
                (h * 100.0).round() as i32,
            )
        })
        .collect()
}

fn approx(value: f32, expected: f32, tolerance: f32) -> bool {
    (value - expected).abs() <= tolerance
}

#[test]
fn compatibility_and_raw_match_on_heading_fixtures() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let alignment_labels = [
        "1970 ISSUE",
        "Larger format",
        "Local heading",
        "Aligned heading",
    ];
    let padding_labels = [
        "1970 ISSUE",
        "Centred heading",
        "Shifted right",
        "Shifted left",
        "Centred again",
        "Baseline",
        "Increased gap",
        "Reduced gap",
        "Restored",
    ];
    for name in ["alignment", "padding-state"] {
        let txt = fixture().join(format!("{name}.txt"));
        let raw = fixture().join(format!("{name}-raw.stampalbum"));
        let compat = dir.path().join(format!("{name}-compat.pdf"));
        let raw_pdf = dir.path().join(format!("{name}-raw.pdf"));
        let input = fs::read_to_string(&txt).map_err(|e| e.to_string())?;
        let source = fixture();
        let pages = albumeasy::render(&input, &source, &compat)?;
        assert_eq!(pages, 2);
        let raw_input = fs::read_to_string(raw).map_err(|e| e.to_string())?;
        let raw_pages = albumeasy::render(&raw_input, &source, &raw_pdf)?;
        assert_eq!(raw_pages, 2);
        let compat_geom = load_pdf(&compat)?;
        let raw_geom = load_pdf(&raw_pdf)?;
        let labels = if name == "alignment" {
            &alignment_labels[..]
        } else {
            &padding_labels[..]
        };
        assert_eq!(image_signature(&compat_geom), image_signature(&raw_geom));
        assert_eq!(
            heading_signature(&compat_geom, labels),
            heading_signature(&raw_geom, labels)
        );
    }
    Ok(())
}

#[test]
fn native_alignment_and_offsets_follow_measured_layout() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("alignment-native.pdf");
    let source = fixture().join("alignment.stampalbum");
    NativeAlbum::parse(
        &fs::read_to_string(source).map_err(|e| e.to_string())?,
        &fixture(),
    )?
    .write_pdf(&output)?;
    let pages = load_pdf(&output)?;
    assert_eq!(pages.len(), 2);
    assert_eq!(
        pages.iter().map(|page| page.images.len()).sum::<usize>(),
        12
    );
    let headings = pages
        .iter()
        .flat_map(|page| page.headings.iter())
        .cloned()
        .collect::<Vec<_>>();
    let issue = headings
        .iter()
        .filter(|glyph| glyph.text == "1970 ISSUE")
        .map(|glyph| glyph.baseline)
        .collect::<Vec<_>>();
    assert_eq!(issue.len(), 12);
    assert!(approx(issue[0], 680.06, 0.25) && approx(issue[1], 680.06, 0.25));
    assert!(approx(issue[2], 451.69, 0.3) && approx(issue[3], 471.54, 0.3));
    assert!(approx(issue[4], 223.33, 0.3) && approx(issue[5], 263.01, 0.3));
    assert!(approx(issue[6], 680.06, 0.25) && approx(issue[7], 680.06, 0.25));
    assert!(approx(issue[8], 471.54, 0.3) && approx(issue[9], 471.54, 0.3));
    assert!(approx(issue[10], 263.01, 0.3) && approx(issue[11], 263.01, 0.3));
    assert_eq!(
        headings
            .iter()
            .filter(|glyph| glyph.text == "Larger format")
            .count(),
        2
    );
    assert_eq!(
        headings
            .iter()
            .filter(|glyph| glyph.text == "Local heading")
            .count(),
        4
    );
    assert_eq!(
        headings
            .iter()
            .filter(|glyph| glyph.text == "Aligned heading")
            .count(),
        4
    );
    let tops = frame_tops(&pages[0]);
    assert!(!tops.is_empty());
    Ok(())
}

#[test]
fn native_padding_and_colour_restore_are_persistent_and_scoped() -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = dir.path().join("padding-native.pdf");
    let source = fixture().join("padding-state.stampalbum");
    NativeAlbum::parse(
        &fs::read_to_string(source).map_err(|e| e.to_string())?,
        &fixture(),
    )?
    .write_pdf(&output)?;
    let pages = load_pdf(&output)?;
    assert_eq!(pages.len(), 2);
    assert_eq!(
        pages.iter().map(|page| page.images.len()).sum::<usize>(),
        12
    );
    let headings = pages
        .iter()
        .flat_map(|page| page.headings.iter())
        .cloned()
        .collect::<Vec<_>>();
    let issue = headings
        .iter()
        .filter(|glyph| glyph.text == "1970 ISSUE")
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(issue.len(), 12);
    assert!(approx(issue[2].left - issue[0].left, 8.5, 0.6));
    assert!(approx(issue[1].left - issue[3].left, 8.5, 0.6));
    let increased = headings
        .iter()
        .find(|glyph| glyph.text == "Increased gap")
        .ok_or("Missing increased-gap heading")?;
    let reduced = headings
        .iter()
        .find(|glyph| glyph.text == "Reduced gap")
        .ok_or("Missing reduced-gap heading")?;
    assert!(approx(increased.baseline, reduced.baseline, 0.25));
    assert!(
        approx(increased.colour[0], 0.0, 0.02)
            && approx(increased.colour[1], 0.392, 0.02)
            && approx(increased.colour[2], 0.0, 0.02)
    );
    let restored = headings
        .iter()
        .find(|glyph| glyph.text == "Restored")
        .ok_or("Missing restored heading")?;
    assert!(
        approx(restored.colour[0], 0.545, 0.02)
            && approx(restored.colour[1], 0.0, 0.02)
            && approx(restored.colour[2], 0.0, 0.02)
    );
    Ok(())
}
