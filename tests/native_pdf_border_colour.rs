use serde_json::Value;
use sha2::{Digest, Sha256};
use stamp_album_gen2::native_pdf::{
    commands::colour::colour_album_border::AlbumBorderColour, config::NativePdfConfig,
    renderer::write_page_geometry_proof, units::points_to_mm,
};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PAGE: &str = r#"
[page]
album_page_width_mm = 210
album_page_height_mm = 297
page_left_margin_mm = 15
page_right_margin_mm = 15
page_top_margin_mm = 15
page_bottom_margin_mm = 15
album_pages_spacing_h = 3
album_pages_spacing_v = 4
page_border_outer = 0.1
page_border_inner1 = 0.5
page_border_inner2 = 0.2
page_border_spacing = 1.25
"#;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/border-colour-v1")
}

fn manifest() -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(
        root().join("expected.json"),
    )?)?)
}

// Test-only adapter; the native production TXT interpreter is still pending.
fn source_colour(source: &str) -> Result<AlbumBorderColour> {
    let mut colour = AlbumBorderColour::default();
    for line in source.lines() {
        let Some((name, values)) = line.split_once('(') else {
            continue;
        };
        if matches!(name.trim(), "COLOUR_ALBUM_BORDER" | "COLOR_ALBUM_BORDER") {
            let (values, _) = values.split_once(')').ok_or("closing parenthesis")?;
            let args = values.split_whitespace().collect::<Vec<_>>();
            colour = AlbumBorderColour::parse(&args)?;
        }
    }
    Ok(colour)
}

fn near(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        actual.is_finite() && (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}, tolerance {tolerance}"
    );
}

#[test]
fn named_colours_defaults_aliases_and_frozen_identities_match() -> Result<()> {
    let manifest = manifest()?;
    for (name, hash) in manifest["sha256"].as_object().ok_or("hashes")? {
        let bytes = fs::read(root().join(name))?;
        let actual: String = Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(actual, hash.as_str().ok_or("hash")?, "{name}");
    }
    for case in manifest["cases"].as_array().ok_or("cases")? {
        let name = case["name"].as_str().ok_or("name")?;
        let colour = source_colour(&fs::read_to_string(root().join(format!("{name}.txt")))?)?;
        for (actual, expected) in colour
            .rgb()
            .iter()
            .zip(case["rgb"].as_array().ok_or("rgb")?)
        {
            near(*actual, expected.as_f64().ok_or("component")?, 0.00001);
        }
        let reference = oxidize_pdf::parser::PdfDocument::open(root().join(format!("{name}.pdf")))?;
        assert_eq!(
            reference.page_count()?,
            u32::try_from(case["pages"].as_u64().ok_or("pages")?)?
        );
        let text = reference.extract_text()?;
        for page in text {
            assert!(!page.truncated);
            assert!(page.text.contains("COLOUR-V1"));
        }
    }
    for case in manifest["invalid"].as_array().ok_or("invalid")? {
        let name = case["name"].as_str().ok_or("name")?;
        assert!(source_colour(&fs::read_to_string(root().join(format!("{name}.txt")))?).is_err());
    }
    Ok(())
}

#[test]
fn colour_validation_rejects_unsupported_forms_without_echoing_values() -> Result<()> {
    assert_eq!(
        AlbumBorderColour::parse(&[" green "])?.rgb(),
        [0.0, 128.0 / 255.0, 0.0]
    );
    assert_eq!(
        AlbumBorderColour::parse(&["grey"])?.rgb(),
        AlbumBorderColour::parse(&["gray"])?.rgb()
    );
    assert!(AlbumBorderColour::parse(&[]).is_err());
    assert!(AlbumBorderColour::parse(&["red", "blue"]).is_err());
    for invalid in [
        "",
        "private-value",
        "#F80",
        "#1E4A7A",
        "#GG00FF",
        "rgb(0,0,0)",
        "transparent",
        "none",
        "currentColor",
        "rebeccapurple",
        "\"red\"",
    ] {
        let error = AlbumBorderColour::parse(&[invalid])
            .err()
            .ok_or("must fail")?;
        assert!(!error.contains("private-value"));
    }
    Ok(())
}

#[test]
fn config_colour_is_applied_with_section_precedence_and_no_geometry_changes() -> Result<()> {
    let base = NativePdfConfig::from_toml(PAGE)?.page_settings;
    let root = NativePdfConfig::from_toml(&format!("color_album_border = \"green\"\n{PAGE}"))?;
    assert_eq!(
        root.page_settings.border_colour().rgb(),
        [0.0, 128.0 / 255.0, 0.0]
    );
    let section = NativePdfConfig::from_toml(&format!(
        "color_album_border = \"stale-invalid\"\n{PAGE}\n[colors]\ncolor_album_border = \"DaRkBlUe\"\ncolor_album_footer = \"red\"\n"
    ))?;
    assert_eq!(
        section.page_settings.border_colour().rgb(),
        [0.0, 0.0, 139.0 / 255.0]
    );
    assert_eq!(section.overridden_keys, ["color_album_border"]);
    assert_eq!(section.unapplied_keys, ["colors.color_album_footer"]);
    let template = NativePdfConfig::from_toml(include_str!("../config_template.toml"))?;
    assert_eq!(
        template.page_settings.border_colour().rgb(),
        [0.0, 0.0, 139.0 / 255.0]
    );
    assert!(!template
        .unapplied_keys
        .contains(&"colors.color_album_border".into()));
    assert_eq!(base.border_colour().rgb(), [0.0; 3]);
    for number in [1, 2, 3] {
        let original = base.resolve(number)?;
        let changed = section.page_settings.resolve(number)?;
        near(
            original.content_bounds.left,
            changed.content_bounds.left,
            0.0,
        );
        near(original.content_bounds.top, changed.content_bounds.top, 0.0);
    }
    for invalid in ["\"private-value\"", "42", "true", "\"#F80\""] {
        let result = NativePdfConfig::from_toml(&format!(
            "{PAGE}\n[colors]\ncolor_album_border = {invalid}\n"
        ));
        let error = result.err().ok_or("must fail")?;
        assert!(error.contains("color_album_border") || error.contains("COLOUR_ALBUM_BORDER"));
        assert!(!error.contains("private-value"));
    }
    assert!(NativePdfConfig::from_toml(&format!("{PAGE}\ncolors = false\n")).is_ok());
    assert!(NativePdfConfig::from_toml(&format!("colors = false\n{PAGE}")).is_err());
    assert!(NativePdfConfig::from_toml(&format!(
        "color_album_border = \"green\"\n{PAGE}\n[colors]\ncolor_album_border = \"invalid\"\n"
    ))
    .is_err());
    Ok(())
}

fn trace(path: &Path) -> Result<String> {
    let output = std::process::Command::new("mutool")
        .args(["draw", "-q", "-F", "trace"])
        .arg(path)
        .output()?;
    if !output.stderr.is_empty() {
        eprintln!("mutool: {}", String::from_utf8_lossy(&output.stderr));
    }
    assert!(output.status.success());
    Ok(String::from_utf8(output.stdout)?)
}

#[test]
#[ignore = "requires mutool on PATH; compares actual native and AlbumEasy border RGB and unchanged geometry"]
fn rendered_colours_match_reference_and_guide_stays_black() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for case in manifest()?["cases"].as_array().ok_or("cases")? {
        let name = case["name"].as_str().ok_or("name")?;
        let source = fs::read_to_string(root().join(format!("{name}.txt")))?;
        let colour = source_colour(&source)?;
        let mut settings = NativePdfConfig::from_toml(PAGE)?.page_settings;
        settings.set_border_colour(colour);
        let pages = usize::try_from(case["pages"].as_u64().ok_or("pages")?)?;
        let output = temporary.path().join("native.pdf");
        write_page_geometry_proof(&settings, pages, &output)?;
        let actual_text = trace(&output)?;
        let reference_text = trace(&root().join(format!("{name}.pdf")))?;
        let actual = roxmltree::Document::parse(&actual_text)?;
        let reference = roxmltree::Document::parse(&reference_text)?;
        let actual_pages: Vec<_> = actual
            .descendants()
            .filter(|n| n.has_tag_name("page"))
            .collect();
        let reference_pages: Vec<_> = reference
            .descendants()
            .filter(|n| n.has_tag_name("page"))
            .collect();
        assert_eq!(actual_pages.len(), pages);
        assert_eq!(reference_pages.len(), pages);
        for (page, reference) in actual_pages.iter().zip(reference_pages) {
            let paths: Vec<_> = page
                .children()
                .filter(|n| n.has_tag_name("stroke_path"))
                .collect();
            let expected: Vec<_> = reference
                .children()
                .filter(|n| n.has_tag_name("stroke_path"))
                .collect();
            assert_eq!(paths.len(), 4);
            assert_eq!(expected.len(), 3);
            for (index, path) in paths.iter().enumerate() {
                assert_eq!(path.attribute("colorspace"), Some("DeviceRGB"));
                let values: Vec<f64> = path
                    .attribute("color")
                    .ok_or("color")?
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                assert_eq!(values.len(), 3);
                for (value, wanted) in values.iter().zip(colour.rgb()) {
                    near(*value, if index < 3 { wanted } else { 0.0 }, 0.00051);
                }
                if index == 3 {
                    continue;
                }
                let reference = expected[index];
                for (value, wanted) in values.iter().zip(
                    reference
                        .attribute("color")
                        .ok_or("color")?
                        .split_whitespace(),
                ) {
                    near(*value, wanted.parse()?, 0.00051);
                }
                let width: f64 = path.attribute("linewidth").ok_or("width")?.parse()?;
                let reference_width: f64 =
                    reference.attribute("linewidth").ok_or("width")?.parse()?;
                near(points_to_mm(width), points_to_mm(reference_width), 0.01);
                let points: Vec<_> = path
                    .children()
                    .filter(|n| n.has_tag_name("moveto") || n.has_tag_name("lineto"))
                    .collect();
                let expected_points: Vec<_> = reference
                    .children()
                    .filter(|n| n.has_tag_name("moveto") || n.has_tag_name("lineto"))
                    .collect();
                assert_eq!(points.len(), 4);
                assert_eq!(expected_points.len(), 4);
                for (point, expected) in points.iter().zip(expected_points) {
                    for attribute in ["x", "y"] {
                        let value: f64 = point.attribute(attribute).ok_or("coordinate")?.parse()?;
                        let wanted: f64 =
                            expected.attribute(attribute).ok_or("coordinate")?.parse()?;
                        near(points_to_mm(value), points_to_mm(wanted), 0.01);
                    }
                }
            }
        }
    }
    // Exercise config -> state -> actual PDF too, not only direct command state.
    let config = NativePdfConfig::from_toml(&format!(
        "{PAGE}\n[colors]\ncolor_album_border = \"green\"\n"
    ))?;
    let output = temporary.path().join("config.pdf");
    write_page_geometry_proof(&config.page_settings, 1, &output)?;
    let text = trace(&output)?;
    let document = roxmltree::Document::parse(&text)?;
    let first = document
        .descendants()
        .find(|n| n.has_tag_name("stroke_path"))
        .ok_or("stroke")?;
    assert_eq!(first.attribute("color"), Some("0 .502 0"));
    let disabled = PAGE
        .replace("page_border_outer = 0.1", "page_border_outer = 0")
        .replace("page_border_inner1 = 0.5", "page_border_inner1 = 0")
        .replace("page_border_inner2 = 0.2", "page_border_inner2 = 0");
    let config = NativePdfConfig::from_toml(&format!(
        "{disabled}\n[colors]\ncolor_album_border = \"red\"\n"
    ))?;
    let output = temporary.path().join("no-border.pdf");
    write_page_geometry_proof(&config.page_settings, 2, &output)?;
    let text = trace(&output)?;
    let document = roxmltree::Document::parse(&text)?;
    let paths: Vec<_> = document
        .descendants()
        .filter(|n| n.has_tag_name("stroke_path"))
        .collect();
    assert_eq!(paths.len(), 2, "one diagnostic guide per page, no border");
    for path in paths {
        assert_eq!(path.attribute("color"), Some("0 0 0"));
    }
    Ok(())
}
