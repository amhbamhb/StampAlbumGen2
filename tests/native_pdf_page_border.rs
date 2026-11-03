use serde_json::Value;
use sha2::{Digest, Sha256};
use stamp_album_gen2::native_pdf::{
    commands::page::{
        album_pages_border::PageBorder, album_pages_margins::Margins, album_pages_size::PageSize,
        album_pages_spacing::Spacing,
    },
    document::{Bounds, PageSettings},
    renderer::write_page_geometry_proof,
    units::points_to_mm,
};
use std::{error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/border-v1")
}

fn manifest() -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(
        root().join("expected.json"),
    )?)?)
}

// Numeric fixture reader only; not a production TXT importer.
fn border(source: &str) -> Result<PageBorder> {
    let Some(line) = source
        .lines()
        .find(|line| line.starts_with("ALBUM_PAGES_BORDER "))
    else {
        return Ok(PageBorder::default());
    };
    let (_, values) = line.split_once('(').ok_or("opening parenthesis")?;
    let (values, _) = values.split_once(')').ok_or("closing parenthesis")?;
    let values = values
        .split_whitespace()
        .map(str::parse)
        .collect::<std::result::Result<Vec<f64>, _>>()?;
    Ok(PageBorder::parse(&values)?)
}

fn settings(border: PageBorder) -> Result<PageSettings> {
    let mut settings = PageSettings::new(
        PageSize::parse(&[210.0, 297.0])?,
        Margins::parse(&[15.0; 4])?,
        Spacing::parse(&[3.0, 4.0])?,
    );
    settings.set_border(border);
    Ok(settings)
}

fn near(actual: f64, expected: f64) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= 0.01,
        "expected {expected}, got {actual}"
    );
}

fn numbers(value: &Value) -> Result<Vec<f64>> {
    value
        .as_array()
        .ok_or("array")?
        .iter()
        .map(|v| v.as_f64().ok_or_else(|| "number".into()))
        .collect()
}

#[test]
fn frozen_reference_identities_and_text() -> Result<()> {
    let manifest = manifest()?;
    for (name, expected) in manifest["sha256"].as_object().ok_or("hashes")? {
        let bytes = fs::read(root().join(name))?;
        let hash: String = Sha256::digest(&bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(hash, expected.as_str().ok_or("hash")?, "{name}");
    }
    for case in manifest["cases"].as_array().ok_or("cases")? {
        let name = case["name"].as_str().ok_or("name")?;
        let reference = oxidize_pdf::parser::PdfDocument::open(root().join(format!("{name}.pdf")))?;
        assert_eq!(reference.page_count()?, 1);
        let pages = reference.extract_text()?;
        assert_eq!(pages.len(), 1);
        assert!(!pages[0].truncated);
        assert!(pages[0].text.contains("BORDER-V1"), "{name}");
    }
    Ok(())
}

#[test]
fn border_geometry_and_content_occupancy_match_reference() -> Result<()> {
    for case in manifest()?["cases"].as_array().ok_or("cases")? {
        let name = case["name"].as_str().ok_or("name")?;
        let source = fs::read_to_string(root().join(format!("{name}.txt")))?;
        let settings = settings(border(&source)?)?;
        let layout = settings.resolve(1)?;
        let strokes = settings.border().strokes(layout.margin_bounds)?;
        let expected = case["strokes"].as_array().ok_or("strokes")?;
        assert_eq!(strokes.len(), expected.len(), "{name}");
        for (stroke, expected) in strokes.iter().zip(expected) {
            near(
                stroke.width_mm,
                expected["width_mm"].as_f64().ok_or("width")?,
            );
            for (actual, wanted) in [
                stroke.bounds.left,
                stroke.bounds.bottom,
                stroke.bounds.right,
                stroke.bounds.top,
            ]
            .iter()
            .zip(numbers(&expected["bounds_mm"])?)
            {
                near(*actual, wanted);
            }
        }
        let glyph = numbers(&case["first_glyph_mm"])?;
        near(layout.content_bounds.left, glyph[0]);
        // All reference cases use the same HN10 text and margins.
        near(layout.content_bounds.top - glyph[1], 282.0 - 278.47222);
        near(layout.content_bounds.right, 210.0 - glyph[0]);
    }
    Ok(())
}

#[test]
fn invalid_border_arguments_and_unusable_strokes_fail_before_output() -> Result<()> {
    for count in [0, 1, 2, 5] {
        assert!(PageBorder::parse(&vec![0.1; count]).is_err());
    }
    for index in 0..4 {
        for invalid in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY, f64::MAX] {
            let mut args = [0.1; 4];
            args[index] = invalid;
            assert!(PageBorder::parse(&args).is_err());
        }
    }
    let invalid = fs::read_to_string(root().join("missing-gap.txt"))?;
    assert!(border(&invalid).is_err());
    assert!(PageBorder::parse(&[0.0; 4])?
        .strokes(Bounds {
            left: 15.0,
            bottom: 15.0,
            right: 195.0,
            top: 282.0,
        })?
        .is_empty());
    let settings = settings(PageBorder::parse(&[0.0, 0.0, 1.0, 50.0])?)?;
    let temporary = tempfile::tempdir()?;
    let output = temporary.path().join("must-not-exist.pdf");
    // Content would fit, but the sparse second-inner stroke does not.
    assert!(settings.resolve(1).is_err());
    assert!(write_page_geometry_proof(&settings, 1, &output).is_err());
    assert!(!output.exists());
    Ok(())
}

fn attribute(node: roxmltree::Node<'_, '_>, name: &str) -> Result<f64> {
    Ok(node.attribute(name).ok_or("attribute")?.parse()?)
}

fn trace(path: &Path) -> Result<String> {
    let output = std::process::Command::new("mutool")
        .args(["draw", "-q", "-F", "trace"])
        .arg(path)
        .output()?;
    if !output.stderr.is_empty() {
        eprintln!("mutool: {}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success(), "mutool failed: {}", output.status);
    Ok(String::from_utf8(output.stdout)?)
}

fn path_bounds(path: roxmltree::Node<'_, '_>) -> Result<[f64; 4]> {
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for point in path
        .children()
        .filter(|n| n.has_tag_name("moveto") || n.has_tag_name("lineto"))
    {
        xs.push(points_to_mm(attribute(point, "x")?));
        ys.push(points_to_mm(attribute(point, "y")?));
    }
    assert_eq!(xs.len(), 4);
    let bounds = [
        xs.iter().copied().fold(f64::INFINITY, f64::min),
        ys.iter().copied().fold(f64::INFINITY, f64::min),
        xs.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        ys.iter().copied().fold(f64::NEG_INFINITY, f64::max),
    ];
    let mut corners = std::collections::BTreeSet::new();
    for (x, y) in xs.iter().zip(&ys) {
        let right = (x - bounds[2]).abs() < 0.01;
        let top = (y - bounds[3]).abs() < 0.01;
        near(*x, if right { bounds[2] } else { bounds[0] });
        near(*y, if top { bounds[3] } else { bounds[1] });
        corners.insert((right, top));
    }
    assert_eq!(corners.len(), 4, "all four rectangle corners must be drawn");
    Ok(bounds)
}

#[test]
#[ignore = "requires mutool on PATH; verifies actual native and AlbumEasy border paths"]
fn rendered_border_paths_match_frozen_references() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for case in manifest()?["cases"].as_array().ok_or("cases")? {
        let name = case["name"].as_str().ok_or("name")?;
        let source = fs::read_to_string(root().join(format!("{name}.txt")))?;
        let settings = settings(border(&source)?)?;
        let output = temporary.path().join("native.pdf");
        write_page_geometry_proof(&settings, 1, &output)?;
        let reference_text = trace(&root().join(format!("{name}.pdf")))?;
        let native_text = trace(&output)?;
        let reference = roxmltree::Document::parse(&reference_text)?;
        let native = roxmltree::Document::parse(&native_text)?;
        let expected = case["strokes"].as_array().ok_or("strokes")?;
        for (document, native) in [(&reference, false), (&native, true)] {
            let paths: Vec<_> = document
                .descendants()
                .filter(|n| n.has_tag_name("stroke_path"))
                .collect();
            assert_eq!(paths.len(), expected.len() + usize::from(native), "{name}");
            for (index, path) in paths.iter().enumerate() {
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
                        .zip([1.0, 0.0, 0.0, -1.0, 0.0, 297.0 * 72.0 / 25.4])
                {
                    assert!((actual - expected).abs() < 0.0001);
                }
                let (width, bounds) = if index < expected.len() {
                    (
                        expected[index]["width_mm"].as_f64().ok_or("width")?,
                        numbers(&expected[index]["bounds_mm"])?,
                    )
                } else {
                    // Reference text anchors independently determine this guide.
                    let glyph = numbers(&case["first_glyph_mm"])?;
                    let occupied = glyph[0] - 18.0;
                    (
                        0.1,
                        vec![
                            glyph[0],
                            15.0 + occupied,
                            210.0 - glyph[0],
                            282.0 - occupied,
                        ],
                    )
                };
                near(points_to_mm(attribute(*path, "linewidth")?), width);
                for (actual, wanted) in path_bounds(*path)?.iter().zip(bounds) {
                    near(*actual, wanted);
                }
            }
        }
    }
    Ok(())
}
