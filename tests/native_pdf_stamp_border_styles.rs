use oxidize_pdf::parser::{ContentOperation, ContentParser, PdfDocument};
use stamp_album_gen2::native_pdf::{
    album::NativeAlbum,
    albumeasy,
    colour::NamedColour,
    document::Bounds,
    stamp_inner_border::{BorderStyleState, InnerBorderStyle, StampInnerBorder},
};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/stamp-border-styles-v1")
}

fn contents(path: &Path) -> Result<Vec<String>, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    (0..pdf.page_count().map_err(|e| e.to_string())?)
        .map(|index| {
            let page = pdf.get_page(index).map_err(|e| e.to_string())?;
            assert!((page.media_box[2] - 612.0).abs() < 0.01);
            assert!((page.media_box[3] - 792.0).abs() < 0.01);
            Ok(pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?
                .iter()
                .map(|s| String::from_utf8_lossy(s))
                .collect::<Vec<_>>()
                .join("\n"))
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Stroke {
    points: Vec<(f32, f32)>,
    width: f32,
    colour: [f32; 3],
    dash: Vec<f32>,
}

#[derive(Clone)]
struct State {
    matrix: [f32; 6],
    width: f32,
    colour: [f32; 3],
    dash: Vec<f32>,
}

impl State {
    fn point(&self, x: f32, y: f32) -> (f32, f32) {
        let [a, b, c, d, e, f] = self.matrix;
        (a * x + c * y + e, b * x + d * y + f)
    }
}

fn geometry(content: &str) -> Result<(Vec<Stroke>, Vec<[f32; 4]>), String> {
    let mut state = State {
        matrix: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0],
        width: 0.0,
        colour: [0.0; 3],
        dash: Vec::new(),
    };
    let mut stack = Vec::new();
    let mut points = Vec::new();
    let mut strokes = Vec::new();
    let mut images = Vec::new();
    for operation in ContentParser::parse(content.as_bytes()).map_err(|e| e.to_string())? {
        match operation {
            ContentOperation::SaveGraphicsState => stack.push(state.clone()),
            ContentOperation::RestoreGraphicsState => {
                state = stack.pop().ok_or("Unbalanced state")?
            }
            ContentOperation::SetStrokingRGB(r, g, b) => state.colour = [r, g, b],
            ContentOperation::SetLineWidth(w) => state.width = w,
            ContentOperation::SetDashPattern(d, phase) => {
                assert_eq!(phase, 0.0);
                state.dash = d;
            }
            ContentOperation::SetTransformMatrix(a, b, c, d, e, f) => {
                let [aa, bb, cc, dd, ee, ff] = state.matrix;
                state.matrix = [
                    aa * a + cc * b,
                    bb * a + dd * b,
                    aa * c + cc * d,
                    bb * c + dd * d,
                    aa * e + cc * f + ee,
                    bb * e + dd * f + ff,
                ];
            }
            ContentOperation::MoveTo(x, y) => {
                points.clear();
                points.push(state.point(x, y));
            }
            ContentOperation::LineTo(x, y) => points.push(state.point(x, y)),
            ContentOperation::Rectangle(x, y, w, h) => {
                points = vec![
                    state.point(x, y),
                    state.point(x + w, y),
                    state.point(x + w, y + h),
                    state.point(x, y + h),
                    state.point(x, y),
                ];
            }
            ContentOperation::Stroke => {
                if !points.is_empty() {
                    strokes.push(Stroke {
                        points: points.clone(),
                        width: state.width,
                        colour: state.colour,
                        dash: state.dash.clone(),
                    });
                }
                points.clear();
            }
            ContentOperation::PaintXObject(_) => {
                let (x, y) = state.point(0.0, 0.0);
                let (r, t) = state.point(1.0, 1.0);
                images.push([x, y, r - x, t - y]);
            }
            _ => {}
        }
    }
    assert!(stack.is_empty());
    Ok((strokes, images))
}

fn grouped(strokes: Vec<Stroke>, images: &[[f32; 4]]) -> Vec<Vec<Stroke>> {
    let mut groups = vec![Vec::new(); images.len()];
    let mut page_borders = 0;
    for mut stroke in strokes {
        let xs = stroke.points.iter().map(|p| p.0);
        let ys = stroke.points.iter().map(|p| p.1);
        let width = xs.clone().fold(f32::NEG_INFINITY, f32::max) - xs.fold(f32::INFINITY, f32::min);
        let height =
            ys.clone().fold(f32::NEG_INFINITY, f32::max) - ys.fold(f32::INFINITY, f32::min);
        if width > 500.0 || height > 500.0 {
            page_borders += 1;
            continue;
        }
        let count = stroke.points.len() as f32;
        let x = stroke.points.iter().map(|p| p.0).sum::<f32>() / count;
        let y = stroke.points.iter().map(|p| p.1).sum::<f32>() / count;
        let index = images
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let distance = |image: &[f32; 4]| {
                    (x - image[0] - image[2] / 2.0).powi(2)
                        + (y - image[1] - image[3] / 2.0).powi(2)
                };
                distance(a).total_cmp(&distance(b))
            })
            .expect("Fixture has images")
            .0;
        let image = images[index];
        for point in &mut stroke.points {
            point.0 -= image[0] + image[2] / 2.0;
            point.1 -= image[1] + image[3] / 2.0;
        }
        groups[index].push(stroke);
    }
    assert_eq!(page_borders, 1);
    for group in &mut groups {
        group.sort_by(|a, b| {
            a.colour[0]
                .total_cmp(&b.colour[0])
                .then(a.colour[1].total_cmp(&b.colour[1]))
                .then(a.colour[2].total_cmp(&b.colour[2]))
                .then(a.points.len().cmp(&b.points.len()))
                .then(a.points[0].0.total_cmp(&b.points[0].0))
                .then(a.points[0].1.total_cmp(&b.points[0].1))
        });
    }
    groups
}

fn compare(actual: &str, expected: &str, context: &str) -> Result<(), String> {
    let (astrokes, aimages) = geometry(actual)?;
    let (estrokes, eimages) = geometry(expected)?;
    assert_eq!(aimages.len(), eimages.len(), "{context}");
    for (a, e) in aimages.iter().zip(&eimages) {
        assert!(
            (a[2] - e[2]).abs() < 0.02 && (a[3] - e[3]).abs() < 0.02,
            "{context}: physical image sizes {a:?} / {e:?}"
        );
    }
    let agroups = grouped(astrokes, &aimages);
    let egroups = grouped(estrokes, &eimages);
    for (index, (ag, eg)) in agroups.iter().zip(&egroups).enumerate() {
        assert_eq!(ag.len(), eg.len(), "{context} stamp {index}");
        for (a, e) in ag.iter().zip(eg) {
            assert_eq!(a.points.len(), e.points.len(), "{context} stamp {index}");
            assert_eq!(a.dash, e.dash, "{context} stamp {index}");
            assert!((a.width - e.width).abs() < 0.01);
            for (a, e) in a.colour.iter().zip(e.colour) {
                assert!((a - e).abs() < 0.001);
            }
            for (a, e) in a.points.iter().zip(&e.points) {
                assert!(
                    (a.0 - e.0).abs() < 0.02 && (a.1 - e.1).abs() < 0.02,
                    "{context} stamp {index}: image-centred actual {a:?} / reference {e:?}"
                );
            }
        }
    }
    Ok(())
}

#[test]
fn three_inputs_match_actual_reference_styles_selectors_masks_and_saved_state() -> Result<(), String>
{
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    for stem in ["album", "inner-state"] {
        let reference = contents(&fixture().join(format!("{stem}.pdf")))?;
        let mut compat = None;
        for suffix in [".txt", "-raw.stampalbum", ".stampalbum"] {
            let input = fixture().join(format!("{stem}{suffix}"));
            let output = temp.path().join(format!("{stem}{suffix}.pdf"));
            if suffix == ".stampalbum" {
                NativeAlbum::load(&input)?.write_pdf(&output)?;
            } else {
                assert_eq!(albumeasy::render_file(&input, &output)?, 2);
            }
            let pages = contents(&output)?;
            assert_eq!(pages.len(), 2);
            for (i, (a, e)) in pages.iter().zip(&reference).enumerate() {
                assert_eq!(geometry(a)?.1.len(), 6);
                assert!(a.contains(&format!("(Page {} of 2) Tj", i + 1)));
                compare(a, e, &format!("{stem}{suffix} page {i}"))?;
            }
            if suffix == ".txt" {
                compat = Some(pages);
            } else if suffix == "-raw.stampalbum" {
                assert_eq!(compat.as_ref(), Some(&pages));
            }
        }
    }
    Ok(())
}

#[test]
fn unequal_thickness_and_inner_hairline_corners_match_reference() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("probe.pdf");
    assert_eq!(
        albumeasy::render_file(&fixture().join("corner-geometry.txt"), &output)?,
        1
    );
    compare(
        &contents(&output)?[0],
        &contents(&fixture().join("corner-geometry.pdf"))?[0],
        "corner thickness probe",
    )
}

#[test]
fn duplicate_selectors_are_idempotent_like_albumeasy() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("duplicates.pdf");
    assert_eq!(
        albumeasy::render_file(&fixture().join("selector-duplicates.txt"), &output)?,
        1
    );
    let actual = contents(&output)?;
    compare(
        &actual[0],
        &contents(&fixture().join("selector-duplicates.pdf"))?[0],
        "duplicates",
    )?;
    assert_eq!(geometry(&actual[0])?.0.len(), 5);
    let source =
        fs::read_to_string(fixture().join("inner-state.stampalbum")).map_err(|e| e.to_string())?;
    NativeAlbum::parse(
        &source
            .replace("SIDES L R T B", "SIDES L L R R T T B B")
            .replace("CORNER TL BR", "CORNER TL TL BR BR"),
        &fixture(),
    )?
    .write_pdf(&output)?;
    let repeated = contents(&output)?;
    NativeAlbum::parse(&source, &fixture())?.write_pdf(&output)?;
    assert_eq!(repeated, contents(&output)?);
    Ok(())
}

#[test]
fn native_sides_inherit_existing_inner_style_and_dimension_updates_retain_mask()
-> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/stamp-inner-borders-v1");
    let source = fs::read_to_string(root.join("album.stampalbum")).map_err(|e| e.to_string())?;
    let end = source
        .lines()
        .position(|line| line.starts_with("STAMP_IMAGE"))
        .ok_or("Missing image")?;
    let source = source.lines().take(end + 1).collect::<Vec<_>>().join("\n");
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("inherited.pdf");
    for reset in [false, true] {
        let commands = if reset {
            "NATIVE_STAMP_INNER_BORDER (2 0.2 DASHED darkblue)\nNATIVE_STAMP_INNER_BORDER_STYLE (SIDES T B)\nNATIVE_STAMP_INNER_BORDER (1 0.3 SOLID darkblue)"
        } else {
            "NATIVE_STAMP_INNER_BORDER (2 0.2 DASHED darkblue)\nNATIVE_STAMP_INNER_BORDER_STYLE (SIDES T B)"
        };
        NativeAlbum::parse(
            &source.replace("NATIVE_STAMP_INNER_BORDER (2 0.2 SOLID darkblue)", commands),
            &root,
        )?
        .write_pdf(&output)?;
        let (strokes, images) = geometry(&contents(&output)?[0])?;
        let groups = grouped(strokes, &images);
        let inner = groups[0]
            .iter()
            .filter(|stroke| stroke.colour[2] > 0.5)
            .collect::<Vec<_>>();
        assert_eq!(inner.len(), 2);
        for stroke in inner {
            assert_eq!(
                stroke.dash,
                if reset {
                    vec![]
                } else {
                    vec![3.0, 2.0, 3.0, 2.0]
                }
            );
            assert!(
                (stroke.points[0].1 - stroke.points[1].1).abs() < 0.01,
                "Only top/bottom sides"
            );
        }
    }
    Ok(())
}

#[test]
fn all_side_and_corner_combinations_are_independent_and_blank_is_not_cancellation()
-> Result<(), String> {
    let bounds = Bounds {
        left: 10.0,
        right: 42.0,
        bottom: 20.0,
        top: 55.0,
    };
    for mask in 0u8..16 {
        let mut state = BorderStyleState::new(InnerBorderStyle::Solid);
        let mut sides = vec!["SIDES"];
        let mut corners = vec!["CORNER"];
        for index in 0..4 {
            if mask & (1 << index) != 0 {
                sides.push(["L", "R", "T", "B"][index]);
                corners.push(["TL", "TR", "BL", "BR"][index]);
            }
        }
        state.apply(&sides)?;
        state.apply(&["DASHED"])?;
        assert_eq!(
            state
                .outer_paths(bounds, 0.2, NamedColour::parse("black")?)?
                .stroke_paths()
                .len(),
            mask.count_ones() as usize
        );
        if mask == 0 {
            corners = vec!["CORNER"];
        }
        state.apply(&corners)?;
        let paths = state.outer_paths(bounds, 0.2, NamedColour::parse("black")?)?;
        assert_eq!(
            paths.stroke_paths().len(),
            if mask == 0 {
                4
            } else {
                mask.count_ones() as usize
            }
        );
        assert!(paths.stroke_paths().iter().all(|p| p.len() == 3));
        assert!(paths.style.dash_points().is_empty());
        state.apply(&["SOLID"])?;
        assert_eq!(
            state
                .outer_paths(bounds, 0.2, NamedColour::parse("black")?)?
                .stroke_paths()
                .len(),
            mask.count_ones() as usize
        );
        state.apply(&["BLANK"])?;
        assert!(
            state
                .outer_paths(bounds, 0.2, NamedColour::parse("black")?)?
                .stroke_paths()
                .is_empty()
        );
    }
    let inner = StampInnerBorder::new(
        2.0,
        0.2,
        InnerBorderStyle::Corner(15),
        NamedColour::parse("darkred")?,
    )?;
    assert_eq!(
        inner
            .layout(bounds, 0.2)?
            .ok_or("Missing corners")?
            .stroke_paths()
            .len(),
        4
    );
    Ok(())
}

#[test]
fn invalid_selectors_fail_with_source_lines_before_replacing_existing_output() -> Result<(), String>
{
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("preserved.pdf");
    fs::write(&output, b"preserved").map_err(|e| e.to_string())?;
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let native =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    for command in ["STAMP_BORDER_STYLE", "STAMP_INNER_BORDER_STYLE"] {
        for values in [
            "",
            "UNKNOWN",
            "SOLID L",
            "CORNER L",
            "SIDES TL",
            "\"CORNER\" TL",
            "SIDES \"B\"",
        ] {
            let error = albumeasy::render(
                &format!("{source}\n{command} ({values})"),
                &fixture(),
                &output,
            )
            .expect_err("Invalid selector must fail");
            assert!(
                error.contains("Line ") && error.contains(command),
                "{error}"
            );
            let result = NativeAlbum::parse(
                &format!("{native}\nNATIVE_{command} ({values})"),
                &fixture(),
            );
            assert!(
                result.is_err(),
                "Unexpectedly accepted NATIVE_{command} ({values})"
            );
            let error = result.err().ok_or("Missing native error")?;
            assert!(error.contains("Line "), "{error}");
            assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
        }
    }
    Ok(())
}

#[test]
#[ignore = "Regenerates native proofs only; accepted AlbumEasy references remain immutable"]
fn regenerate_border_style_proofs() -> Result<(), String> {
    for stem in ["album", "inner-state"] {
        for (suffix, out) in [
            (".txt", "-txt-native.pdf"),
            ("-raw.stampalbum", "-raw-native.pdf"),
            (".stampalbum", "-native.pdf"),
        ] {
            let input = fixture().join(format!("{stem}{suffix}"));
            let output = fixture().join(format!("{stem}{out}"));
            if suffix == ".stampalbum" {
                NativeAlbum::load(&input)?.write_pdf(&output)?;
            } else {
                albumeasy::render_file(&input, &output)?;
            }
        }
    }
    albumeasy::render_file(
        &fixture().join("corner-geometry.txt"),
        &fixture().join("corner-geometry-txt-native.pdf"),
    )?;
    albumeasy::render_file(
        &fixture().join("selector-duplicates.txt"),
        &fixture().join("selector-duplicates-txt-native.pdf"),
    )?;
    Ok(())
}
