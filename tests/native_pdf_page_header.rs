use stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
};
use std::{error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn frozen_header_reference_identities_are_unchanged() -> Result<()> {
    use sha2::{Digest, Sha256};
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/header-v1");
    let manifest = fs::read_to_string(root.join("sha256.txt"))?;
    assert_eq!(manifest.lines().count(), 14);
    for line in manifest.lines() {
        let (expected, name) = line.split_once("  ").ok_or("hash entry")?;
        let hash: String = Sha256::digest(fs::read(root.join(name))?)
            .iter()
            .map(|byte| format!("{byte:02x}"))
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
album_pages_header_position = "L"
[colors]
color_album_border = "black"
color_album_header = "black"
[native_pdf.proof]
page_count = 2
guide_enabled = true
guide_width_mm = 0.1
guide_colour = "green"
[native_pdf.header]
enabled = true
font = "HN"
size_pt = 8
text = "Agyp HEADER"
padding_mm = 0
minimum_height_mm = 0
"#;

fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 0.000001, "{a} != {b}");
}

#[test]
fn header_uses_margin_coordinates_without_consuming_normal_content() -> Result<()> {
    let config = NativePdfConfig::from_toml(CONFIG)?;
    let page = config.page_settings.resolve(1)?;
    let header = page.header.as_ref().ok_or("header")?;
    near(header.lines[0].left_mm, 15.0);
    near(header.lines[0].baseline_mm, 282.0 + 2.0);
    near(page.content_bounds.top, 281.8);
    assert!(header.bounds.bottom > page.content_bounds.top);
    let disabled =
        NativePdfConfig::from_toml(&CONFIG.replace("enabled = true", "enabled = false"))?;
    near(
        disabled.page_settings.resolve(1)?.content_bounds.top,
        page.content_bounds.top,
    );
    assert!(disabled.page_settings.resolve(1)?.header.is_none());
    let absent = CONFIG.split("[native_pdf.header]").next().ok_or("config")?;
    assert!(NativePdfConfig::from_toml(absent)?
        .page_settings
        .resolve(1)?
        .header
        .is_none());
    Ok(())
}

#[test]
fn multiline_padding_and_minimum_hint_share_resolved_measurements() -> Result<()> {
    let source = CONFIG
        .replace("Agyp HEADER", "Agyp FIRST\\nSecond HEADER")
        .replace("padding_mm = 0", "padding_mm = 3");
    let config = NativePdfConfig::from_toml(&source)?;
    let page = config.page_settings.resolve(1)?;
    let h = page.header.as_ref().ok_or("header")?;
    assert_eq!(h.lines.len(), 2);
    near(
        h.lines[0].baseline_mm - h.lines[1].baseline_mm,
        points_to_mm(8.0),
    );
    near(h.lines[1].baseline_mm, 282.0 + 3.0 + 2.0);
    for minimum in [0.1_f64, 25.0] {
        let hinted = NativePdfConfig::from_toml(&source.replace(
            "minimum_height_mm = 0",
            &format!("minimum_height_mm = {minimum}"),
        ))?;
        let hinted = hinted.page_settings.resolve(1)?;
        let hh = hinted.header.as_ref().ok_or("header")?;
        near(hh.reserved_height_mm, minimum.max(hh.measured_height_mm));
        near(hh.lines[0].baseline_mm, h.lines[0].baseline_mm);
        near(hh.measured_height_mm, h.measured_height_mm);
        near(
            hinted.content_bounds.top,
            page.content_bounds.top.min(hh.reservation_bottom_mm),
        );
    }
    Ok(())
}

#[test]
fn parity_alignment_tracks_physical_page_and_even_margins() -> Result<()> {
    let even = "\n[page.even_margins]\nleft_mm = 12\nright_mm = 25\ntop_mm = 18\nbottom_mm = 15\n";
    for (position, odd_left, even_left) in [
        ("L", true, true),
        ("R", false, false),
        ("LR", true, false),
        ("RL", false, true),
    ] {
        let config = NativePdfConfig::from_toml(&format!(
            "{}{even}",
            CONFIG.replace("position = \"L\"", &format!("position = \"{position}\""))
        ))?;
        for (number, left) in [(1, odd_left), (2, even_left), (3, odd_left)] {
            let p = config.page_settings.resolve(number)?;
            let h = p.header.ok_or("header")?;
            let line = &h.lines[0];
            near(
                line.left_mm,
                if left {
                    p.margin_bounds.left
                } else {
                    p.margin_bounds.right - line.width_mm
                },
            );
        }
    }
    Ok(())
}

#[test]
fn missing_invalid_and_unusable_header_values_fail_with_redacted_errors() -> Result<()> {
    for (key, original, values) in [
        ("font", "\"HN\"", vec!["\"\"", "\"BAD\""]),
        ("size_pt", "8", vec!["0", "-1", "nan", "inf", "200"]),
        (
            "text",
            "\"Agyp HEADER\"",
            vec!["\"\"", "\"\\t\"", "\"漢字\""],
        ),
        ("padding_mm", "0", vec!["nan", "inf", "1000", "-1000"]),
        ("minimum_height_mm", "0", vec!["-1", "nan", "inf", "1000"]),
        ("enabled", "true", vec!["\"private-value\""]),
        (
            "album_pages_header_position",
            "\"L\"",
            vec!["\"\"", "\"BAD\""],
        ),
        (
            "color_album_header",
            "\"black\"",
            vec!["\"\"", "\"private-value\""],
        ),
    ] {
        for value in values {
            let invalid =
                CONFIG.replace(&format!("{key} = {original}"), &format!("{key} = {value}"));
            let error = NativePdfConfig::from_toml(&invalid)
                .err()
                .ok_or_else(|| format!("{key}: must fail"))?;
            assert!(error.contains(key) || error.contains("header"), "{error}");
            assert!(!error.contains("private-value"));
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
    let oversized = CONFIG.replace("Agyp HEADER", &"W".repeat(400));
    assert!(NativePdfConfig::from_toml(&oversized).is_err());
    let config = NativePdfConfig::from_toml(&format!(
        "album_pages_header_position = \"BAD\"\ncolor_album_header = \"BAD\"\n{CONFIG}"
    ))?;
    assert_eq!(
        config.overridden_keys,
        ["album_pages_header_position", "color_album_header"]
    );
    Ok(())
}

#[test]
fn defaults_snapshots_and_unknown_header_keys_are_explicit() -> Result<()> {
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let disabled = NativePdfConfig::from_toml(source)?;
        assert!(disabled.page_settings.resolve(1)?.header.is_none());
        let enabled = source.replace(
            "[native_pdf.header]\nenabled = false",
            "[native_pdf.header]\nenabled = true",
        );
        let enabled = NativePdfConfig::from_toml(&enabled)?;
        let header = enabled.page_settings.resolve(1)?.header.ok_or("header")?;
        assert_eq!(header.colour.rgb(), [0.0; 3]);
        near(header.size_pt, 8.0);
        assert_eq!(header.lines[0].text, "Stamp Album Header");
    }
    let temporary = tempfile::tempdir()?;
    let path = temporary.path().join("settings.toml");
    fs::write(&path, format!("{CONFIG}\nfuture_hint = 1\n"))?;
    let snapshot = NativePdfConfig::load(&path)?;
    assert_eq!(snapshot.unapplied_keys, ["native_pdf.header.future_hint"]);
    fs::write(&path, CONFIG.replace("Agyp HEADER", "Updated"))?;
    assert_eq!(
        snapshot
            .page_settings
            .resolve(1)?
            .header
            .ok_or("header")?
            .lines[0]
            .text,
        "Agyp HEADER"
    );
    assert_eq!(
        NativePdfConfig::load(&path)?
            .page_settings
            .resolve(1)?
            .header
            .ok_or("header")?
            .lines[0]
            .text,
        "Updated"
    );
    Ok(())
}

#[test]
fn header_fit_spill_and_all_standard_fonts_reserve_without_clipping() -> Result<()> {
    use stamp_album_gen2::native_pdf::commands::page::album_pages_header_pad::HeaderPadding;
    for args in [vec![], vec![0.0, 1.0], vec![f64::NAN], vec![f64::INFINITY]] {
        assert!(HeaderPadding::parse(&args).is_err());
    }
    near(HeaderPadding::parse(&[-3.0])?.mm(), -3.0);
    for font in [
        "HN", "HB", "HI", "HS", "TN", "TB", "TI", "TS", "CN", "CB", "CI", "CS",
    ] {
        let source = CONFIG
            .replace("font = \"HN\"", &format!("font = \"{font}\""))
            .replace("Agyp HEADER", "Agyp FIRST\\r\\n\\r\\nLast LINE")
            .replace("padding_mm = 0", "padding_mm = -8");
        let page = NativePdfConfig::from_toml(&source)?
            .page_settings
            .resolve(1)?;
        let header = page.header.ok_or("header")?;
        assert_eq!(header.lines.len(), 3);
        assert!(header.lines[1].text.is_empty());
        near(page.content_bounds.top, header.bounds.bottom);
        assert!(header.bounds.bottom < page.margin_bounds.top);
    }
    let header = NativePdfConfig::from_toml(CONFIG)?
        .page_settings
        .resolve(1)?
        .header
        .ok_or("header")?;
    let maximum = 297.0 - header.bounds.top;
    // Just inside and outside the physical edge must remain distinguishable.
    assert!(NativePdfConfig::from_toml(&CONFIG.replace(
        "padding_mm = 0",
        &format!("padding_mm = {}", maximum - 0.000001)
    ))
    .is_ok());
    assert!(NativePdfConfig::from_toml(&CONFIG.replace(
        "padding_mm = 0",
        &format!("padding_mm = {}", maximum + 0.000001)
    ))
    .is_err());
    let narrow_even = format!("{CONFIG}\n[page.even_margins]\nleft_mm = 100\nright_mm = 100\ntop_mm = 15\nbottom_mm = 15\n");
    let error = NativePdfConfig::from_toml(&narrow_even)
        .err()
        .ok_or("must fail")?;
    assert!(
        error.contains("page 2") && error.contains("header.text"),
        "{error}"
    );
    // The renderer preflights every page before saving: invalid later pages
    // must neither create output nor truncate an existing output.
    use stamp_album_gen2::native_pdf::commands::page::album_pages_marginse::EvenMargins;
    let mut config = NativePdfConfig::from_toml(CONFIG)?;
    config
        .page_settings
        .set_even_margins(EvenMargins::parse(&[100.0, 100.0, 15.0, 15.0])?);
    let temporary = tempfile::tempdir()?;
    let path = temporary.path().join("proof.pdf");
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &path).is_err());
    assert!(!path.exists());
    fs::write(&path, b"existing output")?;
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &path).is_err());
    assert_eq!(fs::read(&path)?, b"existing output");
    Ok(())
}

fn trace(path: &Path) -> Result<String> {
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
}

#[test]
#[ignore = "requires mutool; compares header/title glyphs and native reservation guide against frozen references"]
fn serialized_headers_and_title_interactions_match_references() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for (name, position, size, padding, multiline, asymmetric) in [
        ("header-left", "L", 8, 0, false, false),
        ("header-centre", "C", 25, 5, false, false),
        ("header-right", "R", 8, -1, false, false),
        ("header-lr", "LR", 8, 3, false, true),
        ("header-rl", "RL", 8, 3, false, true),
        ("header-title", "C", 16, 3, true, false),
        ("header-omitted-pad", "L", 8, 0, false, false),
    ] {
        let mut source = CONFIG
            .replace("position = \"L\"", &format!("position = \"{position}\""))
            .replace("size_pt = 8", &format!("size_pt = {size}"))
            .replace("padding_mm = 0", &format!("padding_mm = {padding}"));
        if asymmetric {
            source = source.replace("page_left_margin_mm = 15", "page_left_margin_mm = 25")
                .replace("page_right_margin_mm = 15", "page_right_margin_mm = 12")
                + "\n[page.even_margins]\nleft_mm = 12\nright_mm = 25\ntop_mm = 18\nbottom_mm = 15\n";
        }
        if multiline {
            source = source.replace("font = \"HN\"", "font = \"TB\"")
                .replace("Agyp HEADER", "Agyp FIRST\\nSecond HEADER")
                .replace("page_border_outer = 0.2", "page_border_outer = 0.1")
                .replace("page_border_inner1 = 0", "page_border_inner1 = 0.5")
                .replace("page_border_inner2 = 0", "page_border_inner2 = 0.2")
                .replace("page_border_spacing = 0", "page_border_spacing = 1.25")
                .replace("[header]", "[header]\nalbum_pages_title_font = \"HN\"\nalbum_pages_title_size = 10")
                .replace("[colors]", "[colors]\ncolor_album_title = \"black\"")
                + "\n[native_pdf.title]\nenabled = true\ntext = \"Agyp TITLE\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
        }
        for hint in [0, 25] {
            let source = source
                .replacen(
                    "minimum_height_mm = 0",
                    &format!("minimum_height_mm = {hint}"),
                    1,
                )
                .replace(
                    "color_album_header = \"black\"",
                    if hint > 0 {
                        "color_album_header = \"pink\""
                    } else {
                        "color_album_header = \"black\""
                    },
                );
            let path = temporary.path().join("user settings.toml");
            fs::write(&path, &source)?;
            let config = NativePdfConfig::load(&path)?;
            let output = temporary.path().join("native.pdf");
            write_page_geometry_proof(&config.page_settings, &config.proof, &output)?;
            let actual = trace(&output)?;
            let reference = trace(
                &Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests/fixtures/albumeasy/header-v1")
                    .join(format!("{name}.pdf")),
            )?;
            let actual = roxmltree::Document::parse(&actual)?;
            let reference = roxmltree::Document::parse(&reference)?;
            let reference_pages: Vec<_> = reference
                .descendants()
                .filter(|n| n.has_tag_name("page"))
                .collect();
            for (i, page) in actual
                .descendants()
                .filter(|n| n.has_tag_name("page"))
                .enumerate()
            {
                let spans: Vec<_> = page
                    .descendants()
                    .filter(|n| n.has_tag_name("span"))
                    .collect();
                let wanted: Vec<_> = reference_pages[i % reference_pages.len()]
                    .descendants()
                    .filter(|n| n.has_tag_name("span"))
                    .take(spans.len())
                    .collect();
                assert_eq!(spans.len(), if multiline { 3 } else { 1 });
                let layout = config.page_settings.resolve(i + 1)?;
                let header = layout.header.as_ref().ok_or("header")?;
                let colour = spans[0]
                    .parent()
                    .ok_or("fill")?
                    .attribute("color")
                    .ok_or("colour")?;
                let colour = colour
                    .split_whitespace()
                    .map(str::parse::<f64>)
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                for (actual, expected) in colour.iter().zip(header.colour.rgb()) {
                    assert!((actual - expected).abs() < 0.00051);
                }
                if let Some(title) = &layout.title {
                    let first = spans[2]
                        .children()
                        .find(|n| n.has_tag_name("g"))
                        .ok_or("title glyph")?;
                    let baseline: f64 = first.attribute("y").ok_or("baseline")?.parse()?;
                    assert!((points_to_mm(baseline) - title.lines[0].baseline_mm).abs() < 0.01);
                }
                // A minimum reservation may move the title but never the header.
                for (a, e) in spans.iter().zip(wanted).take(if hint > 0 && multiline {
                    2
                } else {
                    spans.len()
                }) {
                    assert_eq!(a.attribute("font"), e.attribute("font"));
                    assert_eq!(a.attribute("trm"), e.attribute("trm"));
                    let a: Vec<_> = a.children().filter(|n| n.has_tag_name("g")).collect();
                    let e: Vec<_> = e.children().filter(|n| n.has_tag_name("g")).collect();
                    assert_eq!(a.len(), e.len());
                    for (a, e) in a.iter().zip(e) {
                        assert_eq!(a.attribute("unicode"), e.attribute("unicode"));
                        for axis in ["x", "y"] {
                            let a: f64 = a.attribute(axis).ok_or("axis")?.parse()?;
                            let e: f64 = e.attribute(axis).ok_or("axis")?.parse()?;
                            assert!(
                                points_to_mm(a - e).abs() < 0.01,
                                "{name} {axis}: {a} != {e}"
                            );
                        }
                    }
                }
                let guide = page
                    .children()
                    .rfind(|n| n.has_tag_name("stroke_path"))
                    .ok_or("guide")?;
                let top = guide
                    .children()
                    .filter_map(|n| n.attribute("y"))
                    .map(str::parse::<f64>)
                    .collect::<std::result::Result<Vec<_>, _>>()?
                    .into_iter()
                    .reduce(f64::max)
                    .ok_or("guide top")?;
                assert!(
                    (points_to_mm(top) - config.page_settings.resolve(i + 1)?.content_bounds.top)
                        .abs()
                        < 0.01
                );
            }
            let pdf = oxidize_pdf::parser::PdfDocument::open(output)?;
            assert_eq!(pdf.page_count()?, 2);
            for p in pdf.extract_text()? {
                assert!(!p.truncated);
                assert!(p.text.contains("Agyp"));
            }
        }
    }
    Ok(())
}
