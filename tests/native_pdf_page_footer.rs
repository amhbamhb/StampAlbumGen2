use stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn frozen_footer_reference_identities_are_unchanged() -> Result<()> {
    use sha2::{Digest, Sha256};
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/footer-v1");
    let manifest = fs::read_to_string(root.join("sha256.txt"))?;
    assert_eq!(manifest.lines().count(), 14);
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
[colors]
color_album_border = "black"
color_album_footer = "black"
[native_pdf.proof]
page_count = 2
guide_enabled = true
guide_width_mm = 0.1
guide_colour = "green"
[native_pdf.footer]
enabled = true
font = "HN"
size_pt = 8
position = "L"
text = "Agyp FOOTER"
padding_mm = 0
minimum_height_mm = 0
"#;

fn near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.000001,
        "{actual} != {expected}"
    );
}

#[test]
fn footer_anchor_and_hint_share_layout_without_moving_glyphs() -> Result<()> {
    let config = NativePdfConfig::from_toml(CONFIG)?;
    let page = config.page_settings.resolve(1)?;
    let footer = page.footer.as_ref().ok_or("footer")?;
    near(footer.lines[0].baseline_mm, 15.0 - 0.5 - points_to_mm(8.0));
    near(footer.lines[0].left_mm, 15.0);
    near(page.content_bounds.bottom, 15.2);
    for minimum in [0.1_f64, 25.0] {
        let hinted = NativePdfConfig::from_toml(&CONFIG.replace(
            "minimum_height_mm = 0",
            &format!("minimum_height_mm = {minimum}"),
        ))?
        .page_settings
        .resolve(1)?;
        let f = hinted.footer.ok_or("footer")?;
        near(f.reserved_height_mm, minimum.max(f.measured_height_mm));
        near(f.lines[0].baseline_mm, footer.lines[0].baseline_mm);
        near(f.measured_height_mm, footer.measured_height_mm);
        near(
            hinted.content_bounds.bottom,
            page.content_bounds.bottom.max(f.reservation_top_mm),
        );
    }
    assert!(
        NativePdfConfig::from_toml(&CONFIG.replace("enabled = true", "enabled = false"))?
            .page_settings
            .resolve(1)?
            .footer
            .is_none()
    );
    assert!(
        NativePdfConfig::from_toml(CONFIG.split("[native_pdf.footer]").next().ok_or("config")?)?
            .page_settings
            .resolve(1)?
            .footer
            .is_none()
    );
    Ok(())
}

#[test]
fn footer_multiline_padding_and_parity_are_independent() -> Result<()> {
    let even = "\n[page.even_margins]\nleft_mm = 12\nright_mm = 25\ntop_mm = 18\nbottom_mm = 20\n";
    for (position, odd_left, even_left) in [
        ("L", true, true),
        ("R", false, false),
        ("LR", true, false),
        ("RL", false, true),
    ] {
        let source = CONFIG
            .replace("position = \"L\"", &format!("position = \"{position}\""))
            .replace("Agyp FOOTER", "First\\nSecond")
            .replace("padding_mm = 0", "padding_mm = 3")
            + even;
        let config = NativePdfConfig::from_toml(&source)?;
        for (number, left) in [(1, odd_left), (2, even_left), (3, odd_left)] {
            let page = config.page_settings.resolve(number)?;
            let footer = page.footer.ok_or("footer")?;
            near(
                footer.lines[0].left_mm,
                if left {
                    page.margin_bounds.left
                } else {
                    page.margin_bounds.right - footer.lines[0].width_mm
                },
            );
            near(
                footer.lines[0].baseline_mm,
                page.margin_bounds.bottom - 0.5 - points_to_mm(8.0) - 3.0,
            );
            near(
                footer.lines[0].baseline_mm - footer.lines[1].baseline_mm,
                points_to_mm(8.0),
            );
        }
    }
    Ok(())
}

#[test]
fn missing_invalid_and_clipped_footer_values_fail_explicitly() -> Result<()> {
    for (key, original, invalid) in [
        ("font", "\"HN\"", vec!["\"\"", "\"BAD\""]),
        ("size_pt", "8", vec!["0", "-1", "nan", "inf", "200"]),
        ("position", "\"L\"", vec!["\"\"", "\"BAD\""]),
        (
            "text",
            "\"Agyp FOOTER\"",
            vec!["\"\"", "\"\\t\"", "\"漢字\""],
        ),
        ("padding_mm", "0", vec!["nan", "inf", "1000", "-1000"]),
        ("minimum_height_mm", "0", vec!["-1", "nan", "inf", "1000"]),
        ("enabled", "true", vec!["\"private-value\""]),
        (
            "color_album_footer",
            "\"black\"",
            vec!["\"\"", "\"private-value\""],
        ),
    ] {
        for value in invalid {
            let source =
                CONFIG.replace(&format!("{key} = {original}"), &format!("{key} = {value}"));
            let error = NativePdfConfig::from_toml(&source)
                .err()
                .ok_or_else(|| format!("{key} must fail"))?;
            assert!(error.contains(key) || error.contains("footer"), "{error}");
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
    // AlbumEasy produced this clipped combination; native must protect descenders.
    let clipped = CONFIG
        .replace("size_pt = 8", "size_pt = 25")
        .replace("padding_mm = 0", "padding_mm = 5");
    assert!(NativePdfConfig::from_toml(&clipped).is_err());
    assert!(NativePdfConfig::from_toml(&CONFIG.replace("Agyp FOOTER", &"W".repeat(400))).is_err());
    Ok(())
}

#[test]
fn footer_defaults_colour_precedence_and_snapshots_preserve_number_settings() -> Result<()> {
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let disabled = NativePdfConfig::from_toml(source)?;
        assert!(disabled.page_settings.resolve(1)?.footer.is_none());
        let enabled = source.replace(
            "[native_pdf.footer]\nenabled = false",
            "[native_pdf.footer]\nenabled = true",
        );
        let enabled = NativePdfConfig::from_toml(&enabled)?;
        let footer = enabled.page_settings.resolve(1)?.footer.ok_or("footer")?;
        assert_eq!(footer.colour.rgb(), [0.0; 3]);
        assert_eq!(footer.lines[0].text, "Stamp Album Footer");
        near(footer.size_pt, 8.0);
    }
    let source = format!(
        "color_album_footer = \"private-value\"\n{CONFIG}\nfuture_hint = 1\n[footer]\nalbum_pages_footer_number_position = \"R\"\n"
    );
    let temporary = tempfile::tempdir()?;
    let path = temporary.path().join("user settings.toml");
    fs::write(&path, source)?;
    let snapshot = NativePdfConfig::load(&path)?;
    assert_eq!(snapshot.overridden_keys, ["color_album_footer"]);
    assert_eq!(
        snapshot.unapplied_keys,
        [
            "footer.album_pages_footer_number_position",
            "native_pdf.footer.future_hint"
        ]
    );
    fs::write(&path, CONFIG.replace("Agyp FOOTER", "Updated"))?;
    assert_eq!(
        snapshot
            .page_settings
            .resolve(1)?
            .footer
            .ok_or("footer")?
            .lines[0]
            .text,
        "Agyp FOOTER"
    );
    assert_eq!(
        NativePdfConfig::load(&path)?
            .page_settings
            .resolve(1)?
            .footer
            .ok_or("footer")?
            .lines[0]
            .text,
        "Updated"
    );
    Ok(())
}

#[test]
fn footer_fonts_fit_spill_and_no_partial_output() -> Result<()> {
    use stamp_album_gen2::native_pdf::commands::page::{
        album_pages_footer_pad::FooterPadding, album_pages_marginse::EvenMargins,
    };
    for args in [vec![], vec![0.0, 1.0], vec![f64::NAN], vec![f64::INFINITY]] {
        assert!(FooterPadding::parse(&args).is_err());
    }
    near(FooterPadding::parse(&[-3.0])?.mm(), -3.0);
    for font in [
        "HN", "HB", "HI", "HS", "TN", "TB", "TI", "TS", "CN", "CB", "CI", "CS",
    ] {
        let source = CONFIG
            .replace("font = \"HN\"", &format!("font = \"{font}\""))
            .replace("Agyp FOOTER", "First\\r\\n\\r\\nLast")
            .replace("padding_mm = 0", "padding_mm = -8");
        let page = NativePdfConfig::from_toml(&source)?
            .page_settings
            .resolve(1)?;
        let footer = page.footer.ok_or("footer")?;
        assert_eq!(footer.lines.len(), 3);
        assert!(footer.lines[1].text.is_empty());
        near(page.content_bounds.bottom, footer.bounds.top);
        assert!(footer.bounds.top > page.margin_bounds.bottom);
    }
    let footer = NativePdfConfig::from_toml(CONFIG)?
        .page_settings
        .resolve(1)?
        .footer
        .ok_or("footer")?;
    let maximum = footer.bounds.bottom;
    assert!(
        NativePdfConfig::from_toml(&CONFIG.replace(
            "padding_mm = 0",
            &format!("padding_mm = {}", maximum - 0.000001)
        ))
        .is_ok()
    );
    assert!(
        NativePdfConfig::from_toml(&CONFIG.replace(
            "padding_mm = 0",
            &format!("padding_mm = {}", maximum + 0.000001)
        ))
        .is_err()
    );
    // A later physical page may have a smaller bottom margin.
    let even = "\n[page.even_margins]\nleft_mm = 15\nright_mm = 15\ntop_mm = 15\nbottom_mm = 1\n";
    let error = NativePdfConfig::from_toml(&format!("{CONFIG}{even}"))
        .err()
        .ok_or("must fail")?;
    assert!(
        error.contains("page 2") && error.contains("footer"),
        "{error}"
    );
    let mut config = NativePdfConfig::from_toml(CONFIG)?;
    config
        .page_settings
        .set_even_margins(EvenMargins::parse(&[15.0, 15.0, 15.0, 1.0])?);
    let temporary = tempfile::tempdir()?;
    let path = temporary.path().join("proof.pdf");
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &path).is_err());
    assert!(!path.exists());
    fs::write(&path, b"existing output")?;
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &path).is_err());
    assert_eq!(fs::read(&path)?, b"existing output");
    Ok(())
}

#[test]
fn footer_and_header_hints_share_usable_content_and_title_fit() -> Result<()> {
    let with_title = CONFIG.replace("[colors]",
        "[header]\nalbum_pages_title_font = \"HN\"\nalbum_pages_title_size = 10\nalbum_pages_header_position = \"L\"\n[colors]\ncolor_album_title = \"black\"\ncolor_album_header = \"black\"")
        + "\n[native_pdf.header]\nenabled = true\nfont = \"HN\"\nsize_pt = 8\ntext = \"Header\"\npadding_mm = 0\nminimum_height_mm = 25\n[native_pdf.title]\nenabled = true\ntext = \"Title\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
    let automatic = NativePdfConfig::from_toml(&with_title)?
        .page_settings
        .resolve(1)?;
    let hinted = with_title.replacen("minimum_height_mm = 0", "minimum_height_mm = 25", 1);
    let hinted = NativePdfConfig::from_toml(&hinted)?
        .page_settings
        .resolve(1)?;
    near(
        hinted.content_bounds.bottom,
        hinted.footer.as_ref().ok_or("footer")?.reservation_top_mm,
    );
    near(hinted.content_bounds.top, automatic.content_bounds.top);
    near(
        hinted.title.as_ref().ok_or("title")?.lines[0].baseline_mm,
        automatic.title.as_ref().ok_or("title")?.lines[0].baseline_mm,
    );
    assert!(hinted.content_bounds.top > hinted.content_bounds.bottom);
    // Both reservations can individually fit the physical page while their
    // combined content rectangle cannot hold the title.
    let collision = with_title.replacen("minimum_height_mm = 0", "minimum_height_mm = 265", 1);
    assert!(NativePdfConfig::from_toml(&collision).is_err());
    // A footer alone may fit physically while reserving the entire usable area.
    assert!(
        NativePdfConfig::from_toml(
            &CONFIG.replace("minimum_height_mm = 0", "minimum_height_mm = 280")
        )
        .is_err()
    );
    Ok(())
}

fn trace(path: &Path) -> Result<String> {
    let output = std::process::Command::new("mutool")
        .args(["draw", "-q", "-F", "trace"])
        .arg(path)
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    if !output.stderr.is_empty() {
        eprintln!("mutool: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(String::from_utf8(output.stdout)?)
}

#[test]
#[ignore = "requires mutool; compares actual footer/header/title glyphs and content guides"]
fn serialized_footer_positions_colours_and_content_match_references() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    for (name, position, size, padding, multiline, asymmetric) in [
        ("footer-left", "L", 8, 0, false, false),
        ("footer-centre", "C", 25, 3, false, false),
        ("footer-right", "R", 8, -1, false, false),
        ("footer-lr", "LR", 8, 3, false, true),
        ("footer-rl", "RL", 8, 3, false, true),
        ("footer-multiline", "C", 16, 1, true, false),
        ("footer-omitted-pad", "L", 8, 0, false, false),
    ] {
        let mut source = CONFIG
            .replace("position = \"L\"", &format!("position = \"{position}\""))
            .replace("size_pt = 8", &format!("size_pt = {size}"))
            .replace("padding_mm = 0", &format!("padding_mm = {padding}"));
        if asymmetric {
            source = source
                .replace("page_left_margin_mm = 15", "page_left_margin_mm = 25")
                .replace("page_right_margin_mm = 15", "page_right_margin_mm = 12")
                + "\n[page.even_margins]\nleft_mm = 12\nright_mm = 25\ntop_mm = 18\nbottom_mm = 20\n";
        }
        if multiline {
            source = source.replace("font = \"HN\"","font = \"TB\"").replace("Agyp FOOTER","Agyp FIRST\\nSecond FOOTER")
                .replace("page_border_outer = 0.2","page_border_outer = 0.1")
                .replace("page_border_inner1 = 0","page_border_inner1 = 0.5")
                .replace("page_border_inner2 = 0","page_border_inner2 = 0.2")
                .replace("page_border_spacing = 0","page_border_spacing = 1.25")
                .replace("[colors]","[header]\nalbum_pages_header_position = \"L\"\nalbum_pages_title_font = \"HN\"\nalbum_pages_title_size = 10\n[colors]\ncolor_album_header = \"black\"\ncolor_album_title = \"black\"")
                + "\n[native_pdf.header]\nenabled = true\nfont = \"HN\"\nsize_pt = 8\ntext = \"Agyp HEADER\"\npadding_mm = 0\nminimum_height_mm = 0\n[native_pdf.title]\nenabled = true\ntext = \"Agyp TITLE\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
        }
        for minimum in [0, 25] {
            let source = source
                .replacen(
                    "minimum_height_mm = 0",
                    &format!("minimum_height_mm = {minimum}"),
                    1,
                )
                .replace(
                    "color_album_footer = \"black\"",
                    if minimum > 0 {
                        "color_album_footer = \"pink\""
                    } else {
                        "color_album_footer = \"black\""
                    },
                );
            let config = NativePdfConfig::from_toml(&source)?;
            let output = temporary.path().join("native.pdf");
            write_page_geometry_proof(&config.page_settings, &config.proof, &output)?;
            let actual_xml = trace(&output)?;
            let reference_xml = trace(
                &Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests/fixtures/albumeasy/footer-v1")
                    .join(format!("{name}.pdf")),
            )?;
            let actual = roxmltree::Document::parse(&actual_xml)?;
            let reference = roxmltree::Document::parse(&reference_xml)?;
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
                let expected: Vec<_> = reference_pages[i % reference_pages.len()]
                    .descendants()
                    .filter(|n| n.has_tag_name("span"))
                    .take(spans.len())
                    .collect();
                assert_eq!(spans.len(), if multiline { 4 } else { 1 });
                assert_eq!(spans.len(), expected.len());
                for (a, e) in spans.iter().zip(expected) {
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
                let layout = config.page_settings.resolve(i + 1)?;
                let footer = layout.footer.as_ref().ok_or("footer")?;
                let colour = spans[usize::from(multiline)]
                    .parent()
                    .ok_or("fill")?
                    .attribute("color")
                    .ok_or("colour")?
                    .split_whitespace()
                    .map(str::parse::<f64>)
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                assert_eq!(colour.len(), 3);
                for (actual, expected) in colour.iter().zip(footer.colour.rgb()) {
                    assert!((actual - expected).abs() < 0.00051);
                }
                let guide = page
                    .children()
                    .rfind(|n| n.has_tag_name("stroke_path"))
                    .ok_or("guide")?;
                let ys = guide
                    .children()
                    .filter_map(|n| n.attribute("y"))
                    .map(str::parse::<f64>)
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                let bottom = ys.iter().copied().reduce(f64::min).ok_or("bottom")?;
                let top = ys.iter().copied().reduce(f64::max).ok_or("top")?;
                assert!((points_to_mm(bottom) - layout.content_bounds.bottom).abs() < 0.01);
                assert!((points_to_mm(top) - layout.content_bounds.top).abs() < 0.01);
            }
            let pdf = oxidize_pdf::parser::PdfDocument::open(output)?;
            assert_eq!(pdf.page_count()?, 2);
            for page in pdf.extract_text()? {
                assert!(!page.truncated && page.text.contains("Agyp"));
            }
        }
    }
    Ok(())
}
