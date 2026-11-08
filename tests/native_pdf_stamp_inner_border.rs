use oxidize_pdf::parser::{ContentOperation, ContentParser, PdfDocument};
use stamp_album_gen2::native_pdf::{
    album::NativeAlbum,
    albumeasy,
    colour::NamedColour,
    document::Bounds,
    stamp_inner_border::{InnerBorderStyle, StampInnerBorder},
};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/stamp-inner-borders-v1")
}

fn contents(path: &Path) -> Result<Vec<String>, String> {
    let pdf = PdfDocument::open(path).map_err(|e| e.to_string())?;
    (0..pdf.page_count().map_err(|e| e.to_string())?)
        .map(|index| {
            let page = pdf.get_page(index).map_err(|e| e.to_string())?;
            assert!(
                (page.media_box[2] - 612.0).abs() < 0.01
                    && (page.media_box[3] - 792.0).abs() < 0.01
            );
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
    coordinates: [f32; 4],
    width: f32,
    colour: [f32; 3],
    dash: Vec<f32>,
}

fn inner_strokes(content: &str) -> Result<Vec<Stroke>, String> {
    let mut colour = [0.0; 3];
    let mut width = 0.0;
    let mut dash = Vec::new();
    let mut stack = Vec::new();
    let mut start = None;
    let mut end = None;
    let mut strokes = Vec::new();
    for operation in ContentParser::parse(content.as_bytes()).map_err(|e| e.to_string())? {
        match operation {
            ContentOperation::SaveGraphicsState => stack.push((colour, width, dash.clone())),
            ContentOperation::RestoreGraphicsState => {
                (colour, width, dash) = stack.pop().ok_or("Unbalanced PDF graphics state")?;
            }
            ContentOperation::SetStrokingRGB(r, g, b) => colour = [r, g, b],
            ContentOperation::SetLineWidth(w) => width = w,
            ContentOperation::SetDashPattern(d, phase) => {
                assert_eq!(phase, 0.0);
                dash = d;
            }
            ContentOperation::MoveTo(x, y) => {
                start = Some((x, y));
                end = None;
            }
            ContentOperation::LineTo(x, y) => end = Some((x, y)),
            ContentOperation::Rectangle(..) => {
                start = None;
                end = None;
            }
            ContentOperation::Stroke => {
                if colour.iter().any(|c| *c > 0.0) {
                    if let (Some((x1, y1)), Some((x2, y2))) = (start, end) {
                        strokes.push(Stroke {
                            coordinates: [x1, y1, x2, y2],
                            width,
                            colour,
                            dash: dash.clone(),
                        });
                    }
                }
                start = None;
                end = None;
            }
            _ => {}
        }
    }
    assert!(stack.is_empty());
    Ok(strokes)
}

#[test]
fn measured_side_geometry_matches_reference_with_half_thickness_endpoints() -> Result<(), String> {
    let pt = 72.0 / 25.4;
    let border = StampInnerBorder::new(
        2.0,
        0.2,
        InnerBorderStyle::Solid,
        NamedColour::parse("darkblue")?,
    )?;
    let paths = border
        .layout(
            Bounds {
                left: 133.32283 / pt,
                right: 297.73228 / pt,
                bottom: 493.81105 / pt,
                top: 593.0236 / pt,
            },
            0.1,
        )?
        .ok_or("Missing inner paths")?;
    for (actual, expected) in paths.segments.iter().zip([
        (139.41733, 499.62205, 139.41733, 587.2126),
        (291.6378, 499.62205, 291.6378, 587.2126),
        (139.13387, 586.92916, 291.92127, 586.92916),
        (139.13387, 499.90553, 291.92127, 499.90553),
    ]) {
        for (actual, expected) in [actual.0, actual.1, actual.2, actual.3]
            .iter()
            .zip([expected.0, expected.1, expected.2, expected.3])
        {
            assert!((actual * pt - expected).abs() < 0.01);
        }
    }
    Ok(())
}

#[test]
fn three_inputs_capture_inner_styles_state_cancellation_and_preserve_images() -> Result<(), String>
{
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let mut outputs = Vec::new();
    let reference = contents(&fixture().join("album.pdf"))?;
    let positions =
        regex::Regex::new(r"([0-9.]+) 0\.00 0\.00 ([0-9.]+) ([-0-9.]+) ([-0-9.]+) cm\s+/\S+ Do")
            .map_err(|e| e.to_string())?;
    let segments = regex::Regex::new(
        r"[-0-9.]+ [-0-9.]+ m\s+[-0-9.]+ [-0-9.]+ l\s+[0-9.]+ [0-9.]+ [0-9.]+ RG\s+S",
    )
    .map_err(|e| e.to_string())?;
    let images =
        regex::Regex::new(r"([0-9.]+) 0\.00 0\.00 ([0-9.]+) [-0-9.]+ [-0-9.]+ cm\s+/\S+ Do")
            .map_err(|e| e.to_string())?;
    for input in ["album.txt", "album-raw.stampalbum", "album.stampalbum"] {
        let output = temp.path().join(format!("{input}.pdf"));
        if input == "album.stampalbum" {
            assert_eq!(
                NativeAlbum::load(&fixture().join(input))?
                    .write_pdf(&output)?
                    .len(),
                2
            );
        } else {
            assert_eq!(albumeasy::render_file(&fixture().join(input), &output)?, 2);
        }
        let pages = contents(&output)?;
        assert_eq!(pages.len(), 2);
        for (i, page) in pages.iter().enumerate() {
            assert_eq!(page.matches(" Do").count(), 6);
            assert_eq!(
                segments.find_iter(page).count(),
                if i == 0 { 20 } else { 16 }
            );
            assert!(page.contains(&format!("(Page {} of 2) Tj", i + 1)));
            assert!(page.contains("(INNER BORDER - REFERENCE) Tj"));
            let actual = inner_strokes(page)?;
            let expected = inner_strokes(&reference[i])?;
            assert_eq!(actual.len(), expected.len());
            for (a, e) in actual
                .as_chunks::<4>()
                .0
                .iter()
                .zip(expected.as_chunks::<4>().0)
            {
                // Native row/title placement differs; compare translated side geometry.
                let ax = a[0].coordinates[0];
                let ay = a[0].coordinates[1];
                let ex = e[0].coordinates[0];
                let ey = e[0].coordinates[1];
                for (a, e) in a.iter().zip(e) {
                    assert!((a.width - e.width).abs() < 0.01);
                    assert_eq!(a.dash, e.dash);
                    for (a, e) in a.colour.iter().zip(e.colour) {
                        assert!((a - e).abs() < 0.001);
                    }
                    let last_image = positions
                        .captures_iter(&pages[1])
                        .last()
                        .ok_or("Missing last image")?;
                    let image_centre = last_image[3].parse::<f32>().map_err(|e| e.to_string())?
                        + last_image[1].parse::<f32>().map_err(|e| e.to_string())? / 2.0;
                    let strokes = inner_strokes(&pages[1])?;
                    let last = &strokes[strokes.len() - 4..];
                    assert!(((last[0].coordinates[0]+last[1].coordinates[0])/2.0-image_centre).abs()<0.02,
                        "Outer BLANK's last stamp, not NO_FRAME's preceding stamp, retains the inner frame");
                    for (index, (a, e)) in a.coordinates.iter().zip(e.coordinates).enumerate() {
                        let difference = if index % 2 == 0 {
                            (a - ax) - (e - ex)
                        } else {
                            (a - ay) - (e - ey)
                        };
                        assert!(
                            difference.abs() < 0.02,
                            "{input} page {} relative side {index}: {difference}",
                            i + 1
                        );
                    }
                }
            }
        }
        assert!(pages[0].contains("[1.00] 0.00 d"));
        assert!(pages[0].contains("[3.00 2.00 3.00 2.00] 0.00 d"));
        assert!(pages[0].contains("[3.00 2.00 1.00 2.00] 0.00 d"));
        assert!(pages[1].contains("0.99 w"));
        assert!(pages[1].contains("0.000 0.000 0.545 RG"));
        let dimensions = images
            .captures_iter(&pages.join("\n"))
            .map(|c| {
                Ok((
                    c[1].parse::<f64>().map_err(|e| e.to_string())?,
                    c[2].parse::<f64>().map_err(|e| e.to_string())?,
                ))
            })
            .collect::<Result<Vec<_>, String>>()?;
        assert_eq!(dimensions.len(), 12);
        outputs.push((pages, dimensions));
    }
    assert_eq!(outputs[0], outputs[1], "TXT/raw rendered content equality");
    for (a, b) in outputs[0].1.iter().zip(&outputs[2].1) {
        assert!((a.0 - b.0).abs() < 0.02 && (a.1 - b.1).abs() < 0.02);
    }
    Ok(())
}

#[test]
fn invalid_and_oversized_inner_borders_preserve_output_and_line_errors() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("preserved.pdf");
    fs::write(&output, b"preserved").map_err(|e| e.to_string())?;
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    for (from, to) in [
        ("STAMP_INNER_BORDER (2 0.2)", "STAMP_INNER_BORDER (-1 0.2)"),
        ("STAMP_INNER_BORDER (2 0.2)", "STAMP_INNER_BORDER (2 -1)"),
        ("STAMP_INNER_BORDER (2 0.2)", "STAMP_INNER_BORDER (NaN 0.2)"),
        ("STAMP_INNER_BORDER (2 0.2)", "STAMP_INNER_BORDER (20 0.2)"),
        ("STAMP_INNER_BORDER (2 0.2)", "STAMP_INNER_BORDER (2)"),
        (
            "STAMP_INNER_BORDER_STYLE (SOLID)",
            "STAMP_INNER_BORDER_STYLE (CORNER)",
        ),
        (
            "STAMP_INNER_BORDER_STYLE (SOLID)",
            "STAMP_INNER_BORDER_STYLE (\"SOLID\")",
        ),
        (
            "COLOR_STAMP_INNER_BORDER (darkblue)",
            "COLOR_STAMP_INNER_BORDER (invalid)",
        ),
    ] {
        assert!(albumeasy::render(&source.replace(from, to), &fixture(), &output).is_err());
        assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    }
    let native =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    for values in [
        "-1 0.2 SOLID darkblue",
        "2 -1 SOLID darkblue",
        "2 0.2 SIDES darkblue",
        "2 0.2 SOLID",
        "NaN 0.2 SOLID darkblue",
    ] {
        let error = NativeAlbum::parse(&native.replace("2 0.2 SOLID darkblue", values), &fixture())
            .expect_err("Bad inner input");
        assert!(error.contains("Line ") && error.contains("INNER_BORDER"));
    }
    let oversized = NativeAlbum::parse(
        &native.replace("2 0.2 SOLID darkblue", "20 0.2 SOLID darkblue"),
        &fixture(),
    )?;
    assert!(oversized.write_pdf(&output).is_err());
    assert_eq!(fs::read(&output).map_err(|e| e.to_string())?, b"preserved");
    Ok(())
}

#[test]
#[ignore = "Regenerates native proofs without modifying the accepted AlbumEasy reference"]
fn regenerate_inner_border_proofs() -> Result<(), String> {
    for (input, output) in [
        ("album.txt", "album-txt-native.pdf"),
        ("album-raw.stampalbum", "album-raw-native.pdf"),
        ("album.stampalbum", "album-native.pdf"),
    ] {
        if input == "album.stampalbum" {
            NativeAlbum::load(&fixture().join(input))?.write_pdf(&fixture().join(output))?;
        } else {
            albumeasy::render_file(&fixture().join(input), &fixture().join(output))?;
        }
    }
    albumeasy::render_file(
        &fixture().join("zero-values.txt"),
        &fixture().join("zero-values-txt-native.pdf"),
    )?;
    Ok(())
}

#[test]
fn zero_padding_hairline_and_cancellation_match_reference() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("zero.pdf");
    assert_eq!(
        albumeasy::render_file(&fixture().join("zero-values.txt"), &output)?,
        1
    );
    let expected = inner_strokes(&contents(&fixture().join("zero-values.pdf"))?[0])?;
    let actual = inner_strokes(&contents(&output)?[0])?;
    assert_eq!(expected.len(), 12);
    assert_eq!(actual.len(), 12);
    assert!(actual[4..8].iter().all(|stroke| stroke.width == 0.0));
    for (a, e) in actual
        .as_chunks::<4>()
        .0
        .iter()
        .zip(expected.as_chunks::<4>().0)
    {
        for (a, e) in a.iter().zip(e) {
            assert!((a.width - e.width).abs() < 0.01);
            for index in 0..4 {
                let start_index = index % 2;
                assert!(
                    ((a.coordinates[index] - a.coordinates[start_index])
                        - (e.coordinates[index] - e.coordinates[start_index]))
                        .abs()
                        < 0.02
                );
            }
        }
    }
    let source =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    let native = NativeAlbum::parse(
        &source.replace("2 0.2 SOLID darkblue", "2 0 SOLID darkblue"),
        &fixture(),
    )?;
    native.write_pdf(&output)?;
    let mut all = Vec::new();
    for page in contents(&output)? {
        all.extend(inner_strokes(&page)?);
    }
    assert!(all.iter().any(|stroke| stroke.width == 0.0));
    Ok(())
}

#[test]
fn save_restore_captures_inner_style_without_changing_earlier_items() -> Result<(), String> {
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    let output = temp.path().join("style.pdf");
    let source = fs::read_to_string(fixture().join("album.txt")).map_err(|e| e.to_string())?;
    let changed = source.replace(
        "ALBUM_STAMP_SETTINGS_SAVE",
        "ALBUM_STAMP_SETTINGS_SAVE\nSTAMP_INNER_BORDER_STYLE (DASHED)",
    );
    albumeasy::render(&changed, &fixture(), &output)?;
    let pages = contents(&output)?;
    assert!(inner_strokes(&pages[0])?[..8]
        .iter()
        .all(|stroke| stroke.dash.is_empty()));
    let second = inner_strokes(&pages[1])?;
    assert!(second[..4]
        .iter()
        .all(|stroke| stroke.dash == vec![3.0, 2.0, 3.0, 2.0]));
    assert!(second[4..8].iter().all(|stroke| stroke.dash.is_empty()));
    let source =
        fs::read_to_string(fixture().join("album.stampalbum")).map_err(|e| e.to_string())?;
    NativeAlbum::parse(
        &source.replace("1 0.35 SOLID darkred", "1 0.35 DASHED darkred"),
        &fixture(),
    )?
    .write_pdf(&output)?;
    let second = inner_strokes(&contents(&output)?[1])?;
    assert!(second[..4]
        .iter()
        .all(|stroke| stroke.dash == vec![3.0, 2.0, 3.0, 2.0]));
    assert!(second[4..8].iter().all(|stroke| stroke.dash.is_empty()));
    Ok(())
}

#[test]
fn denmark_zero_deferral_inputs_still_render_without_fixture_changes() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/denmark-margin-native-v5");
    let temp = tempfile::tempdir().map_err(|e| e.to_string())?;
    for name in [
        "Denmark-1989-1989-supported.txt",
        "Denmark-1989-1989-raw.stampalbum",
    ] {
        let input = root.join(name);
        let before = fs::read(&input).map_err(|e| e.to_string())?;
        assert!(!String::from_utf8_lossy(&before).contains("# NATIVE-TODO:"));
        let output = temp.path().join(format!("{name}.pdf"));
        assert_eq!(albumeasy::render_file(&input, &output)?, 16);
        let pdf = PdfDocument::open(&output).map_err(|e| e.to_string())?;
        let mut images = 0;
        for i in 0..16 {
            let page = pdf.get_page(i).map_err(|e| e.to_string())?;
            let content = pdf
                .get_page_content_streams(&page)
                .map_err(|e| e.to_string())?
                .iter()
                .map(|s| String::from_utf8_lossy(s))
                .collect::<Vec<_>>()
                .join("\n");
            assert!((page.media_box[2] - 216.0 * 72.0 / 25.4).abs() < 0.01);
            assert!(content.contains(&format!("(Page {} of 16) Tj", i + 1)));
            images += content.matches(" Do").count();
        }
        assert_eq!(images, 41);
        assert_eq!(fs::read(input).map_err(|e| e.to_string())?, before);
    }
    Ok(())
}
