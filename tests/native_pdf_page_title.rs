use stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
};
use std::{error::Error, fs};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn frozen_title_reference_identities_are_unchanged() -> Result<()> {
    use sha2::{Digest, Sha256};
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/title-v1");
    let manifest = fs::read_to_string(root.join("sha256.txt"))?;
    assert_eq!(manifest.lines().count(), 10);
    for line in manifest.lines() {
        let (expected, name) = line.split_once("  ").ok_or("hash entry")?;
        let hash: String = Sha256::digest(fs::read(root.join(name))?)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(hash, expected, "{name}");
    }
    Ok(())
}

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
page_border_outer = 0.2
page_border_inner1 = 0
page_border_inner2 = 0
page_border_spacing = 0
[header]
album_pages_title_font = "HN"
album_pages_title_size = 8
[colors]
color_album_border = "black"
color_album_title = "black"
[native_pdf.proof]
page_count = 2
guide_enabled = true
guide_width_mm = 0.1
guide_colour = "black"
[native_pdf.title]
enabled = true
text = "Agyp TITLE-8"
vspace_after_mm = 0
minimum_height_mm = 0
"#;

#[test]
fn title_measures_once_and_reserves_font_dependent_height() -> Result<()> {
    let small = NativePdfConfig::from_toml(CONFIG)?;
    let large = NativePdfConfig::from_toml(&CONFIG.replace("title_size = 8", "title_size = 25"))?;
    let small_page = small.page_settings.resolve(1)?;
    let large_page = large.page_settings.resolve(1)?;
    let a = small_page.title.as_ref().ok_or("title")?;
    let b = large_page.title.as_ref().ok_or("title")?;
    assert!(b.measured_height_mm > a.measured_height_mm);
    assert!((a.measured_height_mm - 3.4064222222).abs() < 0.000001);
    assert!((b.measured_height_mm - 10.6450694444).abs() < 0.000001);
    assert!(large_page.content_bounds.top < small_page.content_bounds.top);
    assert_eq!(a.reserved_height_mm, a.measured_height_mm);
    assert!(small_page.content_bounds.top <= a.bounds.bottom);
    assert_eq!(a.lines.len(), 1);
    assert!(!small
        .unapplied_keys
        .contains(&"header.album_pages_title_font".into()));
    assert!(!small
        .unapplied_keys
        .contains(&"colors.color_album_title".into()));
    let text = "Agyp FIRST\\nSecond LINE";
    let multi = NativePdfConfig::from_toml(&CONFIG.replace("Agyp TITLE-8", text))?;
    assert_eq!(
        multi
            .page_settings
            .resolve(1)?
            .title
            .ok_or("title")?
            .lines
            .len(),
        2
    );
    let blank = NativePdfConfig::from_toml(&CONFIG.replace("Agyp TITLE-8", "First\\n\\nLast"))?;
    assert_eq!(
        blank
            .page_settings
            .resolve(1)?
            .title
            .ok_or("title")?
            .lines
            .len(),
        3
    );
    Ok(())
}

#[test]
fn title_fit_boundaries_and_hint_preserve_measured_text() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        colour::NamedColour, commands::page::album_pages_title::AlbumPagesTitle, document::Bounds,
    };
    for token in [
        "HN", "HB", "HI", "HS", "TN", "TB", "TI", "TS", "CN", "CB", "CI", "CS",
    ] {
        let title = AlbumPagesTitle::new(
            token,
            25.0,
            "Agyp Éire",
            0.0,
            0.0,
            NamedColour::parse("black")?,
        )?;
        let area = Bounds {
            left: 0.0,
            right: 200.0,
            bottom: 0.0,
            top: 100.0,
        };
        let layout = title.layout(area, 0.0)?;
        let width = layout.lines[0].width_mm;
        assert!(
            title
                .layout(
                    Bounds {
                        right: width,
                        ..area
                    },
                    0.0
                )
                .is_ok(),
            "{token}"
        );
        assert!(
            title
                .layout(
                    Bounds {
                        right: width - 0.001,
                        ..area
                    },
                    0.0
                )
                .is_err(),
            "{token}"
        );
        assert!(
            title
                .layout(
                    Bounds {
                        bottom: layout.next_content_top_mm - 0.001,
                        ..area
                    },
                    0.0
                )
                .is_ok(),
            "{token}"
        );
        assert!(
            title
                .layout(
                    Bounds {
                        bottom: layout.next_content_top_mm,
                        ..area
                    },
                    0.0
                )
                .is_err(),
            "{token}"
        );
        assert!(title.layout(area, f64::NAN).is_err());
    }
    let config = NativePdfConfig::from_toml(CONFIG)?;
    let mut settings = config.page_settings;
    let huge = AlbumPagesTitle::new(
        "HN",
        25.0,
        "Agyp",
        0.0,
        1000.0,
        NamedColour::parse("black")?,
    )?;
    settings.set_title(huge);
    let temporary = tempfile::tempdir()?;
    let output = temporary.path().join("invalid.pdf");
    assert!(write_page_geometry_proof(&settings, &config.proof, &output).is_err());
    assert!(!output.exists());
    Ok(())
}

#[test]
fn title_precedence_snapshot_and_optional_extension_are_explicit() -> Result<()> {
    let config = NativePdfConfig::from_toml(&format!(
        "album_pages_title_font = \"invalid\"\nalbum_pages_title_size = \"invalid\"\ncolor_album_title = \"invalid\"\n{CONFIG}"
    ))?;
    assert_eq!(
        config.overridden_keys,
        [
            "album_pages_title_font",
            "album_pages_title_size",
            "color_album_title"
        ]
    );
    let absent = CONFIG.split("[native_pdf.title]").next().ok_or("config")?;
    assert!(NativePdfConfig::from_toml(absent)?
        .page_settings
        .resolve(1)?
        .title
        .is_none());
    let temporary = tempfile::tempdir()?;
    let path = temporary.path().join("user config.toml");
    fs::write(&path, CONFIG)?;
    let first = NativePdfConfig::load(&path)?;
    fs::write(&path, CONFIG.replace("title_size = 8", "title_size = 25"))?;
    let second = NativePdfConfig::load(&path)?;
    assert_eq!(
        first
            .page_settings
            .resolve(1)?
            .title
            .ok_or("title")?
            .size_pt,
        8.0
    );
    assert_eq!(
        second
            .page_settings
            .resolve(1)?
            .title
            .ok_or("title")?
            .size_pt,
        25.0
    );
    let parity = NativePdfConfig::from_toml(&format!(
        "{CONFIG}\n[page.even_margins]\nleft_mm = 25\nright_mm = 15\ntop_mm = 15\nbottom_mm = 15\n"
    ))?;
    let odd = parity.page_settings.resolve(1)?.title.ok_or("title")?;
    let even = parity.page_settings.resolve(2)?.title.ok_or("title")?;
    assert!((even.lines[0].left_mm - odd.lines[0].left_mm - 5.0).abs() < 0.000001);
    let too_narrow_even = format!(
        "{CONFIG}\n[page.even_margins]\nleft_mm = 95\nright_mm = 95\ntop_mm = 15\nbottom_mm = 15\n"
    );
    let error = NativePdfConfig::from_toml(&too_narrow_even)
        .err()
        .ok_or("even title must fail")?;
    assert!(
        error.contains("page 2") && error.contains("native_pdf.title.text"),
        "{error}"
    );
    Ok(())
}

#[test]
fn minimum_hint_only_expands_reservation_and_never_resizes_text() -> Result<()> {
    let automatic = NativePdfConfig::from_toml(CONFIG)?
        .page_settings
        .resolve(1)?;
    for hint in [0.1_f64, 25.0] {
        let config = NativePdfConfig::from_toml(&CONFIG.replace(
            "minimum_height_mm = 0",
            &format!("minimum_height_mm = {hint}"),
        ))?;
        let page = config.page_settings.resolve(1)?;
        let measured = automatic.title.as_ref().ok_or("title")?;
        let title = page.title.as_ref().ok_or("title")?;
        assert_eq!(title.measured_height_mm, measured.measured_height_mm);
        assert_eq!(title.lines[0].baseline_mm, measured.lines[0].baseline_mm);
        assert_eq!(title.reserved_height_mm, hint.max(title.measured_height_mm));
        assert_eq!(
            automatic.content_bounds.top - page.content_bounds.top,
            title.reserved_height_mm - measured.reserved_height_mm
        );
    }
    let disabled = NativePdfConfig::from_toml(
        &CONFIG.replace("enabled = true\ntext", "enabled = false\ntext"),
    )?;
    assert!(disabled.page_settings.resolve(1)?.title.is_none());
    Ok(())
}

#[test]
fn invalid_title_policy_fails_with_keyed_redacted_errors() -> Result<()> {
    for (key, original, values) in [
        ("album_pages_title_font", "\"HN\"", vec!["\"BAD\"", "\"\""]),
        ("album_pages_title_size", "8", vec!["0", "-1", "nan", "inf"]),
        (
            "color_album_title",
            "\"black\"",
            vec!["\"\"", "\"private-value\""],
        ),
        ("minimum_height_mm", "0", vec!["-1", "nan", "inf", "1000"]),
        ("vspace_after_mm", "0", vec!["nan", "inf", "1000"]),
        (
            "text",
            "\"Agyp TITLE-8\"",
            vec!["\"\"", "\"漢字\"", "\"\\t\""],
        ),
        ("enabled", "true", vec!["\"private-value\""]),
    ] {
        for value in values {
            let source =
                CONFIG.replace(&format!("{key} = {original}"), &format!("{key} = {value}"));
            let error = NativePdfConfig::from_toml(&source)
                .err()
                .ok_or_else(|| format!("{key} must fail"))?;
            assert!(!error.contains("private-value"), "{error}");
        }
        let missing = CONFIG
            .lines()
            .filter(|line| !line.starts_with(key))
            .collect::<Vec<_>>()
            .join("\n");
        let error = NativePdfConfig::from_toml(&missing)
            .err()
            .ok_or("missing must fail")?;
        assert!(error.contains(key), "{error}");
    }
    let too_wide = CONFIG.replace("Agyp TITLE-8", &"W".repeat(300));
    assert!(NativePdfConfig::from_toml(&too_wide).is_err());
    Ok(())
}

#[test]
#[ignore = "requires mutool; compares actual native title positions and font sizes with AlbumEasy references"]
fn title_pdf_positions_match_frozen_references_and_hint_controls_guide() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for (name, font, size, text, gap) in [
        ("title-8", "HN", 8, "Agyp TITLE-8", 0),
        ("title-25", "HN", 25, "Agyp TITLE-25", 0),
        ("title-multiline", "TB", 16, "Agyp FIRST\\nSecond LINE", 5),
        ("title-omitted-gap", "HN", 8, "Agyp TITLE-8", 0),
        ("title-negative-gap", "HN", 8, "Agyp TITLE-8", -1),
    ] {
        let source = CONFIG
            .replace("title_font = \"HN\"", &format!("title_font = \"{font}\""))
            .replace("title_size = 8", &format!("title_size = {size}"))
            .replace("Agyp TITLE-8", text)
            .replace("vspace_after_mm = 0", &format!("vspace_after_mm = {gap}"));
        for hint in [0, 25] {
            let source = source
                .replace(
                    "minimum_height_mm = 0",
                    &format!("minimum_height_mm = {hint}"),
                )
                .replace(
                    "color_album_title = \"black\"",
                    if hint == 25 {
                        "color_album_title = \"pink\""
                    } else {
                        "color_album_title = \"black\""
                    },
                );
            let config = NativePdfConfig::from_toml(&source)?;
            let output = temporary.path().join("title.pdf");
            write_page_geometry_proof(&config.page_settings, &config.proof, &output)?;
            let reference = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/albumeasy/title-v1")
                .join(format!("{name}.pdf"));
            let trace = |path: &std::path::Path| -> Result<String> {
                let result = std::process::Command::new("mutool")
                    .args(["draw", "-q", "-F", "trace"])
                    .arg(path)
                    .output()?;
                assert!(
                    result.status.success(),
                    "{}",
                    String::from_utf8_lossy(&result.stderr)
                );
                if !result.stderr.is_empty() {
                    eprintln!("mutool: {}", String::from_utf8_lossy(&result.stderr));
                }
                Ok(String::from_utf8(result.stdout)?)
            };
            let actual = trace(&output)?;
            let expected = trace(&reference)?;
            let actual = roxmltree::Document::parse(&actual)?;
            let expected = roxmltree::Document::parse(&expected)?;
            let expected_page = expected
                .descendants()
                .find(|n| n.has_tag_name("page"))
                .ok_or("page")?;
            let expected_spans: Vec<_> = expected_page
                .descendants()
                .filter(|n| n.has_tag_name("span"))
                .take(text.split("\\n").count())
                .collect();
            for page in actual.descendants().filter(|n| n.has_tag_name("page")) {
                let expected_rgb = if hint == 25 {
                    [1.0, 192.0 / 255.0, 203.0 / 255.0]
                } else {
                    [0.0; 3]
                };
                for fill in page.children().filter(|n| n.has_tag_name("fill_text")) {
                    let rgb: Vec<f64> = fill
                        .attribute("color")
                        .ok_or("text colour")?
                        .split_whitespace()
                        .map(str::parse)
                        .collect::<std::result::Result<_, _>>()?;
                    assert_eq!(rgb.len(), 3);
                    for (actual, wanted) in rgb.iter().zip(expected_rgb) {
                        assert!((actual - wanted).abs() <= 0.00051);
                    }
                }
                let guide = page
                    .children()
                    .rfind(|n| n.has_tag_name("stroke_path"))
                    .ok_or("guide")?;
                let ys: Vec<f64> = guide
                    .children()
                    .filter_map(|n| n.attribute("y"))
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                let top = ys.into_iter().reduce(f64::max).ok_or("guide top")?;
                let layout = config.page_settings.resolve(1)?;
                assert!((points_to_mm(top) - layout.content_bounds.top).abs() < 0.01);
                assert_eq!(
                    layout.title.as_ref().ok_or("title")?.reserved_height_mm,
                    (hint as f64).max(layout.title.as_ref().ok_or("title")?.measured_height_mm)
                );
                let spans: Vec<_> = page
                    .descendants()
                    .filter(|n| n.has_tag_name("span"))
                    .collect();
                assert_eq!(spans.len(), expected_spans.len());
                for (a, e) in spans.iter().zip(&expected_spans) {
                    assert_eq!(a.attribute("trm"), e.attribute("trm"));
                    assert_eq!(a.attribute("font"), e.attribute("font"));
                    let actual_glyphs: Vec<_> =
                        a.children().filter(|n| n.has_tag_name("g")).collect();
                    let expected_glyphs: Vec<_> =
                        e.children().filter(|n| n.has_tag_name("g")).collect();
                    assert_eq!(actual_glyphs.len(), expected_glyphs.len());
                    for (a, e) in actual_glyphs.iter().zip(&expected_glyphs) {
                        assert_eq!(a.attribute("unicode"), e.attribute("unicode"));
                        for axis in ["x", "y"] {
                            let a: f64 = a.attribute(axis).ok_or("axis")?.parse()?;
                            let e: f64 = e.attribute(axis).ok_or("axis")?.parse()?;
                            assert!(
                                (points_to_mm(a - e)).abs() < 0.01,
                                "{name} {axis}: {a} != {e}"
                            );
                        }
                    }
                }
            }
            let pdf = oxidize_pdf::parser::PdfDocument::open(&output)?;
            assert_eq!(pdf.page_count()?, 2);
            for page in pdf.extract_text()? {
                assert!(!page.truncated);
                assert!(page.text.contains("Agyp"));
            }
            fs::remove_file(output)?;
        }
    }
    Ok(())
}
