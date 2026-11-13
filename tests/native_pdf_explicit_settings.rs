use stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
};
use std::{error::Error, fs};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const CONFIG: &str = r#"
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
[colors]
color_album_border = "pink"
[native_pdf.proof]
page_count = 2
guide_enabled = true
guide_width_mm = 0.35
guide_colour = "green"
"#;

const BORDER_COLOURS: &str = r#"
[native_pdf.border_colours]
outer = "red"
inner1 = "green"
inner2 = "blue"
"#;

#[test]
fn toml_defaults_and_independent_border_colours_are_explicit() -> Result<()> {
    use stamp_album_gen2::native_pdf::commands::page::album_pages_border::BorderLine;
    for template in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let config = NativePdfConfig::from_toml(template)?;
        for line in [BorderLine::Outer, BorderLine::Inner1, BorderLine::Inner2] {
            assert_eq!(
                config.page_settings.border_line_colour(line).rgb(),
                [0.0; 3]
            );
        }
        assert_eq!(config.proof.guide_colour.rgb(), [0.0; 3]);
    }
    let legacy = NativePdfConfig::from_toml(CONFIG)?;
    for (from, to, key) in [
        (
            "color_album_border = \"pink\"",
            "color_album_border = \"\"",
            "color_album_border",
        ),
        (
            "guide_colour = \"green\"",
            "guide_colour = \"\"",
            "native_pdf.proof.guide_colour",
        ),
        (
            "page_border_outer = 0.1",
            "page_border_outer = \"\"",
            "page_border_outer",
        ),
    ] {
        let error = NativePdfConfig::from_toml(&CONFIG.replace(from, to))
            .err()
            .ok_or("blank setting must fail")?;
        assert!(error.contains(key), "{error}");
    }
    for line in [BorderLine::Outer, BorderLine::Inner1, BorderLine::Inner2] {
        assert_eq!(
            legacy.page_settings.border_line_colour(line).rgb(),
            [1.0, 192.0 / 255.0, 203.0 / 255.0]
        );
    }
    for key in ["outer", "inner1", "inner2"] {
        for invalid in [None, Some("\"\""), Some("\"private-value\""), Some("false")] {
            let mut table: toml::Table = toml::from_str(&format!("{CONFIG}{BORDER_COLOURS}"))?;
            let colours = table
                .get_mut("native_pdf")
                .and_then(toml::Value::as_table_mut)
                .and_then(|native| native.get_mut("border_colours"))
                .and_then(toml::Value::as_table_mut)
                .ok_or("colours table")?;
            match invalid {
                None => {
                    colours.remove(key);
                }
                Some(value) => {
                    let value: toml::Table = toml::from_str(&format!("value = {value}"))?;
                    colours.insert(key.into(), value["value"].clone());
                }
            }
            let error = NativePdfConfig::from_toml(&toml::to_string(&table)?)
                .err()
                .ok_or("invalid must fail")?;
            assert!(
                error.contains(&format!("native_pdf.border_colours.{key}")),
                "{error}"
            );
            assert!(!error.contains("private-value"), "{error}");
        }
    }
    assert!(
        NativePdfConfig::from_toml(&format!("{CONFIG}\n[native_pdf.border_colours]\n")).is_err()
    );
    let mut config = NativePdfConfig::from_toml(&format!("{CONFIG}{BORDER_COLOURS}"))?;
    assert!(config.unapplied_keys.is_empty());
    // Applying the shared AlbumEasy command explicitly resets native overrides.
    config.page_settings.set_border_colour(
        stamp_album_gen2::native_pdf::commands::colour::colour_album_border::AlbumBorderColour::parse(&["navy"])?
    );
    for line in [BorderLine::Outer, BorderLine::Inner1, BorderLine::Inner2] {
        assert_eq!(
            config.page_settings.border_line_colour(line).rgb(),
            [0.0, 0.0, 128.0 / 255.0]
        );
    }
    Ok(())
}

#[test]
#[ignore = "requires mutool; verifies independent border colours retain slot identity with disabled strokes"]
fn independent_colours_reach_pdf_for_full_and_sparse_borders() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for (disabled, expected) in [
        (
            None,
            vec![[1.0, 0.0, 0.0], [0.0, 128.0 / 255.0, 0.0], [0.0, 0.0, 1.0]],
        ),
        (
            Some(("page_border_outer = 0.1", "page_border_outer = 0")),
            vec![[0.0, 128.0 / 255.0, 0.0], [0.0, 0.0, 1.0]],
        ),
        (
            Some(("page_border_inner1 = 0.5", "page_border_inner1 = 0")),
            vec![[1.0, 0.0, 0.0], [0.0, 0.0, 1.0]],
        ),
        (
            Some(("page_border_inner2 = 0.2", "page_border_inner2 = 0")),
            vec![[1.0, 0.0, 0.0], [0.0, 128.0 / 255.0, 0.0]],
        ),
    ] {
        let mut source = format!("{CONFIG}{BORDER_COLOURS}")
            .replace("guide_enabled = true", "guide_enabled = false");
        if let Some((from, to)) = disabled {
            source = source.replace(from, to);
        }
        let path = temporary.path().join("settings.toml");
        fs::write(&path, source)?;
        let config = NativePdfConfig::load(&path)?;
        let output = temporary.path().join("proof.pdf");
        write_page_geometry_proof(&config.page_settings, &config.proof, &output)?;
        let trace = std::process::Command::new("mutool")
            .args(["draw", "-q", "-F", "trace"])
            .arg(&output)
            .output()?;
        assert!(
            trace.status.success(),
            "{}",
            String::from_utf8_lossy(&trace.stderr)
        );
        if !trace.stderr.is_empty() {
            eprintln!("mutool: {}", String::from_utf8_lossy(&trace.stderr));
        }
        let text = String::from_utf8(trace.stdout)?;
        let document = roxmltree::Document::parse(&text)?;
        for page in document.descendants().filter(|n| n.has_tag_name("page")) {
            let paths: Vec<_> = page
                .children()
                .filter(|n| n.has_tag_name("stroke_path"))
                .collect();
            assert_eq!(paths.len(), expected.len());
            for (path, wanted) in paths.iter().zip(&expected) {
                let values: Vec<f64> = path
                    .attribute("color")
                    .ok_or("colour")?
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                assert_eq!(values.len(), 3);
                for (value, wanted) in values.iter().zip(wanted) {
                    assert!((value - wanted).abs() <= 0.00051);
                }
            }
        }
    }
    Ok(())
}

#[test]
fn rendering_policy_has_no_missing_key_fallbacks() -> Result<()> {
    for key in [
        "page_border_outer",
        "page_border_inner1",
        "page_border_inner2",
        "page_border_spacing",
        "color_album_border",
        "page_count",
        "guide_enabled",
        "guide_width_mm",
        "guide_colour",
    ] {
        let missing = CONFIG
            .lines()
            .filter(|line| !line.starts_with(key))
            .collect::<Vec<_>>()
            .join("\n");
        let error = NativePdfConfig::from_toml(&missing)
            .err()
            .ok_or("missing value must fail")?;
        assert!(error.contains(key), "{key}: {error}");
    }
    let no_border = CONFIG
        .lines()
        .filter(|line| !line.starts_with("page_border_"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(NativePdfConfig::from_toml(&no_border).is_err());
    let disabled = CONFIG
        .replace("page_border_outer = 0.1", "page_border_outer = 0")
        .replace("page_border_inner1 = 0.5", "page_border_inner1 = 0")
        .replace("page_border_inner2 = 0.2", "page_border_inner2 = 0");
    let loaded = NativePdfConfig::from_toml(&disabled)?;
    assert!(loaded
        .page_settings
        .border()
        .strokes(loaded.page_settings.resolve(1)?.margin_bounds)?
        .is_empty());
    let no_proof = CONFIG.split("[native_pdf.proof]").next().ok_or("config")?;
    assert!(NativePdfConfig::from_toml(no_proof).is_err());
    let unknown = NativePdfConfig::from_toml(&format!("{CONFIG}\nfuture_setting = 42\n"))?;
    assert_eq!(unknown.unapplied_keys, ["native_pdf.proof.future_setting"]);
    assert!(NativePdfConfig::from_toml(CONFIG)?
        .unapplied_keys
        .is_empty());
    Ok(())
}

#[test]
fn proof_options_are_validated_without_output_or_secret_echoes() -> Result<()> {
    for (key, original, values) in [
        (
            "page_count",
            "2",
            vec!["0", "-1", "1.5", "true", "\"private-value\""],
        ),
        ("guide_enabled", "true", vec!["0", "\"private-value\""]),
        (
            "guide_width_mm",
            "0.35",
            vec!["0", "-1", "nan", "inf", "1e308", "\"private-value\""],
        ),
        (
            "guide_colour",
            "\"green\"",
            vec!["false", "\"private-value\"", "\"#F80\""],
        ),
    ] {
        for value in values {
            let source =
                CONFIG.replace(&format!("{key} = {original}"), &format!("{key} = {value}"));
            let error = NativePdfConfig::from_toml(&source)
                .err()
                .ok_or("invalid must fail")?;
            assert!(!error.contains("private-value"), "{error}");
        }
    }
    let temporary = tempfile::tempdir()?;
    let output = temporary.path().join("no-output.pdf");
    let config = NativePdfConfig::from_toml(CONFIG)?;
    let mut invalid = config.proof.clone();
    invalid.page_count = 0;
    assert!(write_page_geometry_proof(&config.page_settings, &invalid, &output).is_err());
    assert!(!output.exists());
    Ok(())
}

#[test]
#[ignore = "requires mutool; verifies TOML page count, independent guide colour/width and disabled guide"]
fn all_proof_policy_controls_actual_output() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for (enabled, pages, width, colour, rgb) in [
        (true, 2, 0.35, "green", [0.0, 128.0 / 255.0, 0.0]),
        (false, 4, 0.35, "green", [0.0, 128.0 / 255.0, 0.0]),
        (true, 1, 0.65, "navy", [0.0, 0.0, 128.0 / 255.0]),
    ] {
        let source = CONFIG
            .replace(
                "guide_enabled = true",
                &format!("guide_enabled = {enabled}"),
            )
            .replace("page_count = 2", &format!("page_count = {pages}"))
            .replace(
                "guide_width_mm = 0.35",
                &format!("guide_width_mm = {width}"),
            )
            .replace(
                "guide_colour = \"green\"",
                &format!("guide_colour = \"{colour}\""),
            );
        let path = temporary.path().join("settings.toml");
        fs::write(&path, source)?;
        let config = NativePdfConfig::load(&path)?;
        let output = temporary.path().join("proof.pdf");
        write_page_geometry_proof(&config.page_settings, &config.proof, &output)?;
        let pdf = oxidize_pdf::parser::PdfDocument::open(&output)?;
        assert_eq!(pdf.page_count()?, pages);
        let trace = std::process::Command::new("mutool")
            .args(["draw", "-q", "-F", "trace"])
            .arg(&output)
            .output()?;
        assert!(
            trace.status.success(),
            "{}",
            String::from_utf8_lossy(&trace.stderr)
        );
        if !trace.stderr.is_empty() {
            eprintln!("mutool: {}", String::from_utf8_lossy(&trace.stderr));
        }
        let text = String::from_utf8(trace.stdout)?;
        let trace = roxmltree::Document::parse(&text)?;
        let traced_pages: Vec<_> = trace
            .descendants()
            .filter(|n| n.has_tag_name("page"))
            .collect();
        assert_eq!(traced_pages.len(), pages as usize);
        for page in traced_pages {
            let paths: Vec<_> = page
                .children()
                .filter(|n| n.has_tag_name("stroke_path"))
                .collect();
            assert_eq!(paths.len(), if enabled { 4 } else { 3 });
            for path in &paths[..3] {
                let rgb: Vec<f64> = path
                    .attribute("color")
                    .ok_or("rgb")?
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                for (value, wanted) in rgb.iter().zip([1.0, 192.0 / 255.0, 203.0 / 255.0]) {
                    assert!((value - wanted).abs() <= 0.00051);
                }
            }
            if enabled {
                let guide = paths[3];
                let values: Vec<f64> = guide
                    .attribute("color")
                    .ok_or("rgb")?
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                assert_eq!(values.len(), 3);
                for (value, wanted) in values.iter().zip(rgb) {
                    assert!((value - wanted).abs() <= 0.00051);
                }
                let actual_width: f64 = guide.attribute("linewidth").ok_or("width")?.parse()?;
                assert!((points_to_mm(actual_width) - width).abs() < 0.01);
            }
        }
    }
    Ok(())
}
