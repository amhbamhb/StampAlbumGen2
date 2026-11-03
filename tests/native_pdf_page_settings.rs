use oxidize_pdf::parser::PdfDocument;
use serde_json::Value;
use stamp_album_gen2::native_pdf::{
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder, album_pages_margins::Margins,
        album_pages_marginse::EvenMargins, album_pages_size::PageSize,
        album_pages_spacing::Spacing,
    },
    document::PageSettings,
    units::{mm_to_points, points_to_mm},
};
#[path = "support/native_pdf_proof.rs"]
mod proof;
use proof::write_page_geometry_proof;
use std::{error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn fixture(family: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy")
        .join(family)
}

fn args(source: &str, name: &str) -> Result<Vec<f64>> {
    let line = source
        .lines()
        .find(|line| line.split_whitespace().next() == Some(name))
        .ok_or_else(|| format!("missing fixture command {name}"))?;
    let (_, values) = line.split_once('(').ok_or("missing opening parenthesis")?;
    let (values, _) = values
        .split_once(')')
        .ok_or("missing closing parenthesis")?;
    Ok(values
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?)
}

fn settings(source: &str) -> Result<PageSettings> {
    let mut settings = PageSettings::new(
        PageSize::parse(&args(source, "ALBUM_PAGES_SIZE")?)?,
        Margins::parse(&args(source, "ALBUM_PAGES_MARGINS")?)?,
        Spacing::parse(&args(source, "ALBUM_PAGES_SPACING")?)?,
        PageBorder::parse(&[0.2, 0.0, 0.0, 0.0])?,
        AlbumBorderColour::parse(&["black"])?,
    );
    if source
        .lines()
        .any(|line| line.starts_with("ALBUM_PAGES_MARGINSE "))
    {
        settings.set_even_margins(EvenMargins::parse(&args(source, "ALBUM_PAGES_MARGINSE")?)?);
    }
    Ok(settings)
}

fn near(actual: f64, expected: f64) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= 0.01,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn four_page_sizes_render_and_read_back() -> Result<()> {
    let root = fixture("page-size-v1");
    let manifest: Value = serde_json::from_slice(&fs::read(root.join("expected.json"))?)?;
    let temporary = tempfile::tempdir()?;
    for case in manifest["cases"].as_array().ok_or("cases")? {
        if case["pdf_created"] != true {
            continue;
        }
        let input = case["input"].as_str().ok_or("input")?;
        let source = fs::read_to_string(root.join(input))?;
        let output = temporary.path().join(input).with_extension("pdf");
        write_page_geometry_proof(&settings(&source)?, 1, &output)?;
        let native = PdfDocument::open(&output)?;
        let reference =
            PdfDocument::open(root.join(case["reference"].as_str().ok_or("reference")?))?;
        assert_eq!(native.page_count()?, 1);
        for document in [&native, &reference] {
            let page = document.get_page(0)?;
            near(
                points_to_mm(page.media_box[2] - page.media_box[0]),
                case["page_size_mm"][0].as_f64().ok_or("width")?,
            );
            near(
                points_to_mm(page.media_box[3] - page.media_box[1]),
                case["page_size_mm"][1].as_f64().ok_or("height")?,
            );
        }
    }
    Ok(())
}

#[test]
fn margin_and_even_page_bounds_match_frozen_expectations() -> Result<()> {
    let root = fixture("margins-v1");
    let manifest: Value = serde_json::from_slice(&fs::read(root.join("expected.json"))?)?;
    for case in manifest["cases"].as_array().ok_or("cases")? {
        if case["pdf_created"] != true {
            continue;
        }
        let source = fs::read_to_string(root.join(case["input"].as_str().ok_or("input")?))?;
        let settings = settings(&source)?;
        for (index, expected) in case["border_by_page_mm"]
            .as_array()
            .ok_or("borders")?
            .iter()
            .enumerate()
        {
            let layout = settings.resolve(index + 1)?;
            let border = layout.margin_bounds.inset(0.1, 0.1)?;
            for (actual, wanted) in [border.left, border.bottom, border.right, border.top]
                .iter()
                .zip(expected.as_array().ok_or("border")?)
            {
                near(*actual, wanted.as_f64().ok_or("coordinate")?);
            }
        }
    }
    Ok(())
}

#[test]
fn spacing_state_matches_reference_content_anchors_and_gaps() -> Result<()> {
    let root = fixture("spacing-v1");
    let manifest: Value = serde_json::from_slice(&fs::read(root.join("expected.json"))?)?;
    for case in manifest["cases"].as_array().ok_or("cases")? {
        if case["pdf_created"] != true {
            continue;
        }
        let source = fs::read_to_string(root.join(case["input"].as_str().ok_or("input")?))?;
        let settings = settings(&source)?;
        let layout = settings.resolve(1)?;
        let frames = case["frames_mm"].as_array().ok_or("frames")?;
        near(
            layout.content_bounds.left,
            frames[0][0].as_f64().ok_or("left")?,
        );
        near(
            layout.content_bounds.right,
            frames[1][0].as_f64().ok_or("right")? + 30.0,
        );
        let first = &case["text_anchors_mm"][0];
        let second = &case["text_anchors_mm"][1];
        near(
            layout.content_bounds.left,
            first[0].as_f64().ok_or("anchor")?,
        );
        near(
            first[1].as_f64().ok_or("y")? - second[1].as_f64().ok_or("y")?,
            points_to_mm(10.0) + settings.spacing().vertical_mm(),
        );
        near(
            settings
                .spacing()
                .next_top(frames[1][1].as_f64().ok_or("bottom")?)?,
            frames[3][1].as_f64().ok_or("bottom")? + 25.0,
        );
    }
    Ok(())
}

#[test]
fn invalid_commands_geometry_and_output_fail_explicitly() -> Result<()> {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0] {
        assert!(PageSize::parse(&[invalid, 297.0]).is_err());
        assert!(Margins::parse(&[0.0, invalid, 0.0, 0.0]).is_err());
        assert!(EvenMargins::parse(&[0.0, 0.0, invalid, 0.0]).is_err());
        assert!(Spacing::parse(&[0.0, invalid]).is_err());
    }
    assert!(PageSize::parse(&[0.0, 297.0]).is_err());
    assert!(PageSize::parse(&[210.0, 0.0]).is_err());
    assert!(PageSize::parse(&[f64::MAX, 297.0]).is_err());
    assert!(PageSize::parse(&[210.0]).is_err());
    assert!(Margins::parse(&[15.0; 3]).is_err());
    assert!(EvenMargins::parse(&[15.0; 5]).is_err());
    assert!(Spacing::parse(&[3.0]).is_err());
    assert!(Spacing::parse(&[0.0, 0.0]).is_ok());
    let source = fs::read_to_string(fixture("page-size-v1").join("a4-portrait.txt"))?;
    let mut settings = settings(&source)?;
    assert!(settings.resolve(0).is_err());
    let bounds = settings.resolve(1)?.margin_bounds;
    assert!(bounds.inset(-0.2, -0.2).is_err());
    assert!(bounds.inset(f64::NAN, f64::NAN).is_err());
    settings.set_even_margins(EvenMargins::parse(&[200.0, 200.0, 15.0, 15.0])?);
    assert!(settings.resolve(2).is_err());
    let temporary = tempfile::tempdir()?;
    let output = temporary.path().join("not-created.pdf");
    assert!(write_page_geometry_proof(&settings, 2, &output).is_err());
    assert!(
        !output.exists(),
        "validate all pages before writing any output"
    );
    assert!(write_page_geometry_proof(&settings, 0, &output).is_err());
    assert!(write_page_geometry_proof(&settings, 1, temporary.path()).is_err());
    assert!(Spacing::parse(&[0.0, 0.0])?.next_top(f64::NAN).is_err());
    near(points_to_mm(mm_to_points(210.25)), 210.25);
    Ok(())
}

#[test]
fn missing_commands_and_arguments_use_frozen_negative_inputs() -> Result<()> {
    for (family, inputs) in [
        ("page-size-v1", ["missing-size.txt", "missing-height.txt"]),
        ("margins-v1", ["missing-margins.txt", "missing-bottom.txt"]),
        (
            "spacing-v1",
            ["missing-spacing.txt", "missing-vertical.txt"],
        ),
    ] {
        for input in inputs {
            let source = fs::read_to_string(fixture(family).join(input))?;
            assert!(settings(&source).is_err(), "{family}/{input} must fail");
        }
    }
    Ok(())
}

fn attribute(node: roxmltree::Node<'_, '_>, name: &str) -> Result<f64> {
    Ok(node
        .attribute(name)
        .ok_or("missing trace attribute")?
        .parse()?)
}

#[test]
#[ignore = "requires mutool on PATH; verifies actual native strokes against frozen reference measurements"]
fn rendered_paths_match_margin_and_spacing_fixtures() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for family in ["margins-v1", "spacing-v1"] {
        let root = fixture(family);
        let manifest: Value = serde_json::from_slice(&fs::read(root.join("expected.json"))?)?;
        for case in manifest["cases"].as_array().ok_or("cases")? {
            if case["pdf_created"] != true {
                continue;
            }
            let source = fs::read_to_string(root.join(case["input"].as_str().ok_or("input")?))?;
            let settings = settings(&source)?;
            let pages = usize::try_from(case["pages"].as_u64().ok_or("pages")?)?;
            let output = temporary.path().join("proof.pdf");
            write_page_geometry_proof(&settings, pages, &output)?;
            let trace = std::process::Command::new("mutool")
                .args(["draw", "-q", "-F", "trace"])
                .arg(&output)
                .output()?;
            if !trace.stderr.is_empty() {
                eprintln!("mutool: {}", String::from_utf8_lossy(&trace.stderr));
            }
            assert!(trace.status.success(), "mutool failed: {}", trace.status);
            let text = String::from_utf8(trace.stdout)?;
            let document = roxmltree::Document::parse(&text)?;
            let drawn_pages: Vec<_> = document
                .descendants()
                .filter(|n| n.has_tag_name("page"))
                .collect();
            assert_eq!(drawn_pages.len(), pages);
            for (index, page) in drawn_pages.iter().enumerate() {
                let paths: Vec<_> = page
                    .children()
                    .filter(|n| n.has_tag_name("stroke_path"))
                    .collect();
                assert_eq!(paths.len(), 2, "one margin border and one content guide");
                let border: Vec<f64> = if family == "margins-v1" {
                    case["border_by_page_mm"][index]
                        .as_array()
                        .ok_or("border")?
                        .iter()
                        .map(|value| value.as_f64().ok_or("coordinate"))
                        .collect::<std::result::Result<_, _>>()?
                } else {
                    vec![15.1, 15.1, 194.9, 281.9]
                };
                let horizontal = settings.spacing().horizontal_mm();
                let content = [
                    border[0] + 0.1 + horizontal,
                    border[1] + 0.1,
                    border[2] - 0.1 - horizontal,
                    border[3] - 0.1,
                ];
                for (path_index, path) in paths.iter().enumerate() {
                    let transform: Vec<f64> = path
                        .attribute("transform")
                        .ok_or("transform")?
                        .split_whitespace()
                        .map(str::parse)
                        .collect::<std::result::Result<_, _>>()?;
                    assert_eq!(transform.len(), 6);
                    for (actual, expected) in
                        transform
                            .iter()
                            .zip([1.0, 0.0, 0.0, -1.0, 0.0, mm_to_points(297.0)])
                    {
                        assert!(
                            (actual - expected).abs() < 0.0001,
                            "unexpected page transform"
                        );
                    }
                    near(
                        points_to_mm(attribute(*path, "linewidth")?),
                        if path_index == 0 { 0.2 } else { 0.1 },
                    );
                    let points: Vec<_> = path
                        .children()
                        .filter(|n| n.has_tag_name("moveto") || n.has_tag_name("lineto"))
                        .collect();
                    assert_eq!(points.len(), 4);
                    let expected = if path_index == 0 {
                        border.as_slice()
                    } else {
                        &content
                    };
                    let corners = [
                        [expected[0], expected[1]],
                        [expected[2], expected[1]],
                        [expected[2], expected[3]],
                        [expected[0], expected[3]],
                    ];
                    for (point, expected) in points.iter().zip(corners) {
                        near(points_to_mm(attribute(*point, "x")?), expected[0]);
                        near(points_to_mm(attribute(*point, "y")?), expected[1]);
                    }
                    if family == "spacing-v1" && path_index == 1 {
                        near(
                            points_to_mm(attribute(points[0], "x")?),
                            case["text_anchors_mm"][0][0]
                                .as_f64()
                                .ok_or("text anchor")?,
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
