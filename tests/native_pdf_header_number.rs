use stamp_album_gen2::native_pdf::config::NativePdfConfig;
use std::{error::Error, fs, path::Path};
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
enabled = false
font = "HN"
size_pt = 8
text = "Collection"
padding_mm = 0
minimum_height_mm = 0
[native_pdf.header_number]
enabled = true
font = "HN"
size_pt = 8
position = "R"
start = 1
before = "Page "
after = " of $PAGES$"
"#;

#[test]
fn header_numbers_use_explicit_count_and_independent_style() -> Result<()> {
    let config = NativePdfConfig::from_toml(CONFIG)?;
    assert!(config.page_settings.resolve(1).is_err());
    for page in [1, 2] {
        let layout = config.page_settings.resolve_with_page_count(page, 2)?;
        let number = layout.header_number.ok_or("number")?;
        assert_eq!(number.lines[0].text, format!("Page {page} of 2"));
        assert!((number.lines[0].baseline_mm - 284.0).abs() < 0.000001);
        assert!((number.lines[0].left_mm + number.lines[0].width_mm - 195.0).abs() < 0.000001);
    }
    Ok(())
}

#[test]
fn overlapping_header_values_report_page_and_correction_keys() -> Result<()> {
    let enabled = CONFIG.replace(
        "[native_pdf.header]\nenabled = false",
        "[native_pdf.header]\nenabled = true",
    );
    assert!(NativePdfConfig::from_toml(&enabled).is_ok());
    let collision = enabled.replace("position = \"R\"", "position = \"L\"");
    let error = NativePdfConfig::from_toml(&collision)
        .err()
        .ok_or("must fail")?;
    assert!(
        error.contains("page 1")
            && error.contains("overlaps")
            && error.contains("native_pdf.header_number.position")
            && error.contains("album_pages_header_position"),
        "{error}"
    );
    let even = enabled.replace("position = \"R\"", "position = \"RL\"");
    assert!(NativePdfConfig::from_toml(&even)
        .err()
        .ok_or("must fail")?
        .contains("page 2"));
    Ok(())
}

#[test]
fn defaults_missing_invalid_values_and_snapshot_are_explicit() -> Result<()> {
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let config = NativePdfConfig::from_toml(source)?;
        assert!(config.page_settings.resolve(1)?.header_number.is_none());
        let enabled = NativePdfConfig::from_toml(&source.replace(
            "[native_pdf.header_number]\nenabled = false",
            "[native_pdf.header_number]\nenabled = true",
        ))?;
        let layout = enabled
            .page_settings
            .resolve_with_page_count(1, enabled.proof.page_count)?
            .header_number
            .ok_or("number")?;
        assert_eq!(
            layout.lines[0].text,
            format!("Page 1 of {}", enabled.proof.page_count)
        );
        assert_eq!(layout.size_pt, 8.0);
        assert_eq!(layout.colour.rgb(), [0.0; 3]);
    }
    let (base, _) = CONFIG
        .split_once("[native_pdf.header_number]")
        .ok_or("section")?;
    let fields = [
        ("enabled", "true"),
        ("font", "\"HN\""),
        ("size_pt", "8"),
        ("position", "\"R\""),
        ("start", "1"),
        ("before", "\"Page \""),
        ("after", "\" of $PAGES$\""),
    ];
    for (key, original) in fields {
        let section = format!(
            "{base}[native_pdf.header_number]\n{}",
            fields
                .iter()
                .filter(|(k, _)| *k != key)
                .map(|(k, v)| format!("{k} = {v}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        let error = NativePdfConfig::from_toml(&section)
            .err()
            .ok_or("missing must fail")?;
        assert!(
            error.contains(&format!("native_pdf.header_number.{key}")),
            "{error}"
        );
        let section = format!(
            "{base}[native_pdf.header_number]\n{}",
            fields
                .iter()
                .map(|(k, v)| format!("{k} = {}", if *k == key { "123.5" } else { v }))
                .collect::<Vec<_>>()
                .join("\n")
        );
        // A positive numeric size is valid; every other field rejects a float.
        if key != "size_pt" {
            assert!(
                NativePdfConfig::from_toml(&section).is_err(),
                "{key}={original}"
            );
        }
    }
    for (key, original, invalid) in [
        ("font", "\"HN\"", "\"private-value\""),
        ("size_pt", "8", "0"),
        ("size_pt", "8", "nan"),
        ("size_pt", "8", "-8"),
        ("position", "\"R\"", "\"private-value\""),
        ("start", "1", "2147483648"),
        ("start", "1", "-2147483649"),
        ("start", "1", "2147483647"),
        ("before", "\"Page \"", "\"\\t\""),
        ("after", "\" of $PAGES$\"", "\"\\t\""),
        ("before", "\"Page \"", "\"漢字\""),
        ("after", "\" of $PAGES$\"", "\"漢字\""),
    ] {
        let section = format!(
            "{base}[native_pdf.header_number]\n{}",
            fields
                .iter()
                .map(|(k, v)| format!("{k} = {}", if *k == key { invalid } else { v }))
                .collect::<Vec<_>>()
                .join("\n")
        );
        let error = NativePdfConfig::from_toml(&section)
            .err()
            .ok_or_else(|| format!("{key}={original} must reject {invalid}"))?;
        assert!(error.contains("native_pdf.header_number"), "{error}");
        assert!(!error.contains("private-value"));
    }
    assert!(NativePdfConfig::from_toml(
        &CONFIG.replace("[native_pdf.header]", "[unsupported_header]")
    )
    .is_err());
    let config = NativePdfConfig::from_toml(&(CONFIG.to_owned() + "\nfuture_option = 4\n"))?;
    assert!(config
        .unapplied_keys
        .contains(&"native_pdf.header_number.future_option".into()));
    assert!(!config
        .unapplied_keys
        .contains(&"native_pdf.header_number.font".into()));
    for (page, total) in [(0, 2), (1, 0), (3, 2)] {
        assert!(config
            .page_settings
            .resolve_with_page_count(page, total)
            .is_err());
    }
    let max = NativePdfConfig::from_toml(
        &CONFIG
            .replace("start = 1", "start = 2147483647")
            .replace("page_count = 2", "page_count = 1"),
    )?;
    assert!(max.page_settings.resolve_with_page_count(2, 2).is_err());
    let mut snapshot = CONFIG.to_owned();
    let loaded = NativePdfConfig::from_toml(&snapshot)?;
    snapshot = snapshot.replace("start = 1", "start = 8");
    assert_eq!(
        loaded
            .page_settings
            .resolve_with_page_count(1, 2)?
            .header_number
            .ok_or("number")?
            .lines[0]
            .text,
        "Page 1 of 2"
    );
    assert_eq!(
        NativePdfConfig::from_toml(&snapshot)?
            .page_settings
            .resolve_with_page_count(1, 2)?
            .header_number
            .ok_or("number")?
            .lines[0]
            .text,
        "Page 8 of 2"
    );
    Ok(())
}

#[test]
fn signed_tokens_parity_multiline_and_shared_hints_are_measured() -> Result<()> {
    let source = CONFIG
        .replace("start = 1", "start = -1")
        .replace("position = \"R\"", "position = \"LR\"")
        .replace("before = \"Page \"", "before = \"$PAGES$ / \"")
        .replace(
            "after = \" of $PAGES$\"",
            "after = \" / $PAGES$ / $PAGES$\"",
        );
    let config = NativePdfConfig::from_toml(&source)?;
    for (page, label) in [(1, -1), (2, 0), (3, 1)] {
        let layout = config
            .page_settings
            .resolve_with_page_count(page, 3)?
            .header_number
            .ok_or("number")?;
        let line = &layout.lines[0];
        assert_eq!(line.text, format!("3 / {label} / 3 / 3"));
        let left = if page % 2 == 1 {
            15.0
        } else {
            195.0 - line.width_mm
        };
        assert!((line.left_mm - left).abs() < 0.000001);
    }
    let automatic = config.page_settings.resolve_with_page_count(1, 3)?;
    let hinted = NativePdfConfig::from_toml(
        &source.replace("minimum_height_mm = 0", "minimum_height_mm = 25"),
    )?
    .page_settings
    .resolve_with_page_count(1, 3)?;
    let a = automatic.header_number.ok_or("number")?;
    let h = hinted.header_number.ok_or("number")?;
    assert_eq!(a.lines[0].baseline_mm, h.lines[0].baseline_mm);
    assert_eq!(a.lines[0].left_mm, h.lines[0].left_mm);
    assert_eq!(h.reserved_height_mm, 25.0);
    assert!((hinted.content_bounds.top - h.reservation_bottom_mm).abs() < 0.000001);
    assert!(hinted.content_bounds.top < automatic.content_bounds.top);
    let multiline = NativePdfConfig::from_toml(
        &CONFIG.replace("before = \"Page \"", "before = \"First\\nPage \""),
    )?
    .page_settings
    .resolve_with_page_count(1, 2)?
    .header_number
    .ok_or("number")?;
    assert_eq!(multiline.lines.len(), 2);
    assert_eq!(multiline.lines[1].text, "Page 1 of 2");
    assert!(multiline.lines[0].baseline_mm > multiline.lines[1].baseline_mm);
    Ok(())
}

#[test]
fn late_width_collision_preflight_preserves_existing_output() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_header::AlbumPagesHeader, album_pages_header_pad::HeaderPadding,
        },
        renderer::write_page_geometry_proof,
        text::TextPosition,
    };
    let mut config = NativePdfConfig::from_toml(
        &CONFIG
            .replace("page_count = 2", "page_count = 3")
            .replace("start = 1", "start = 8")
            .replace("before = \"Page \"", "before = \"\"")
            .replace("after = \" of $PAGES$\"", "after = \"\""),
    )?;
    let first = config
        .page_settings
        .resolve_with_page_count(1, 3)?
        .header_number
        .ok_or("number")?;
    let last = config
        .page_settings
        .resolve_with_page_count(3, 3)?
        .header_number
        .ok_or("number")?;
    // Right edge of left text lies strictly between single-/double-digit labels.
    let width = (first.lines[0].left_mm + last.lines[0].left_mm) / 2.0 - 15.0;
    let text = "Collection ".repeat(18);
    let measured = stamp_album_gen2::native_pdf::text::MeasuredText::new("HN", 8.0, &text)?;
    let size = 8.0 * width / measured.lines[0].width_mm;
    config.page_settings.set_header(AlbumPagesHeader::new(
        "HN",
        size,
        TextPosition::Left,
        &text,
        HeaderPadding::parse(&[0.0])?,
        0.0,
        NamedColour::parse("black")?,
    )?);
    assert!(config.page_settings.resolve_with_page_count(1, 3).is_ok());
    assert!(config.page_settings.resolve_with_page_count(2, 3).is_ok());
    let error = config
        .page_settings
        .resolve_with_page_count(3, 3)
        .err()
        .ok_or("late collision")?;
    assert!(
        error.contains("page 3") && error.contains("overlaps"),
        "{error}"
    );
    let dir = tempfile::tempdir()?;
    let output = dir.path().join("native.pdf");
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &output).is_err());
    assert!(!output.exists());
    fs::write(&output, b"existing output")?;
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &output).is_err());
    assert_eq!(fs::read(&output)?, b"existing output");
    Ok(())
}

#[test]
fn wide_and_tall_titles_remain_below_both_header_elements() -> Result<()> {
    let source=CONFIG.replace("[native_pdf.header]\nenabled = false","[native_pdf.header]\nenabled = true")
        .replace("[header]","[header]\nalbum_pages_title_font = \"TB\"\nalbum_pages_title_size = 25")
        .replace("[colors]","[colors]\ncolor_album_title = \"black\"")
        + "\n[native_pdf.title]\nenabled = true\ntext = \"Agyp TITLE\\nSecond line\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
    for padding in [0, -6] {
        for hint in [0, 25] {
            let source = source
                .replace("padding_mm = 0", &format!("padding_mm = {padding}"))
                .replacen(
                    "minimum_height_mm = 0",
                    &format!("minimum_height_mm = {hint}"),
                    1,
                );
            let layout = NativePdfConfig::from_toml(&source)?
                .page_settings
                .resolve_with_page_count(1, 2)?;
            let title = layout.title.ok_or("title")?;
            for header in [
                layout.header.ok_or("header")?,
                layout.header_number.ok_or("number")?,
            ] {
                assert!(title.bounds.top < header.bounds.bottom);
            }
        }
    }
    Ok(())
}

#[test]
fn all_fonts_remeasure_changing_digit_width_and_reject_overflow() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_header::AlbumPagesHeader,
            album_pages_header_number::AlbumPagesHeaderNumber,
            album_pages_header_pad::HeaderPadding,
        },
        document::Bounds,
        text::TextPosition,
    };
    let page = Bounds {
        left: 0.0,
        bottom: 0.0,
        right: 210.0,
        top: 297.0,
    };
    let margin = Bounds {
        left: 15.0,
        bottom: 15.0,
        right: 195.0,
        top: 282.0,
    };
    for font in [
        "HN", "HB", "HI", "HS", "TN", "TB", "TI", "TS", "CN", "CB", "CI", "CS",
    ] {
        let style = AlbumPagesHeader::new(
            font,
            8.0,
            TextPosition::Right,
            "9",
            HeaderPadding::parse(&[0.0])?,
            0.0,
            NamedColour::parse("black")?,
        )?;
        let number = AlbumPagesHeaderNumber::new(style, 9, "", "")?;
        let first = number.layout(margin, page, 1, 2)?;
        let exact = Bounds {
            left: 0.0,
            right: first.lines[0].width_mm,
            ..margin
        };
        assert!(number.layout(exact, page, 1, 2).is_ok());
        assert!(number.layout(exact, page, 2, 2).is_err());
        assert!(number.layout(margin, page, 0, 2).is_err());
        assert!(number.layout(margin, page, 1, 0).is_err());
        assert!(
            number.layout(margin, page, 1, 2)?.lines[0].width_mm
                < number.layout(margin, page, 2, 2)?.lines[0].width_mm
        );
    }
    Ok(())
}

#[test]
fn frozen_header_number_reference_identities_are_unchanged() -> Result<()> {
    use sha2::{Digest, Sha256};
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/header-number-v1");
    let manifest = fs::read_to_string(root.join("sha256.txt"))?;
    assert_eq!(manifest.lines().count(), 14);
    for line in manifest.lines() {
        let (expected, name) = line.split_once("  ").ok_or("hash")?;
        let actual: String = Sha256::digest(fs::read(root.join(name))?)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(actual, expected, "{name}");
    }
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
#[ignore = "requires mutool; verifies actual header-number reference glyphs and coexistence"]
fn serialized_header_numbers_match_reference_fonts_glyphs_and_geometry() -> Result<()> {
    use stamp_album_gen2::native_pdf::{renderer::write_page_geometry_proof, units::points_to_mm};
    let dir = tempfile::tempdir()?;
    for (name, position, start, before, after, font, size, padding, pages, asymmetric, coexist) in [
        (
            "header-number-basic",
            "R",
            1,
            "Page ",
            " of $PAGES$",
            "HN",
            8,
            0,
            2,
            false,
            false,
        ),
        (
            "header-number-lr",
            "LR",
            8,
            "",
            "",
            "HN",
            8,
            3,
            3,
            true,
            false,
        ),
        (
            "header-number-rl",
            "RL",
            -1,
            "$PAGES$ / ",
            " / $PAGES$",
            "TB",
            16,
            -1,
            3,
            true,
            false,
        ),
        (
            "header-number-coexist",
            "R",
            7,
            "Page ",
            " of $PAGES$",
            "HN",
            8,
            0,
            2,
            false,
            true,
        ),
        (
            "header-number-left",
            "L",
            0,
            "",
            "",
            "HN",
            8,
            0,
            1,
            false,
            false,
        ),
        (
            "header-number-centre",
            "C",
            1,
            "First\\nPage ",
            " of $PAGES$",
            "TB",
            16,
            2,
            2,
            false,
            false,
        ),
        (
            "header-number-omitted",
            "R",
            1,
            "",
            "",
            "HN",
            8,
            0,
            1,
            false,
            false,
        ),
    ] {
        let (base, _) = CONFIG
            .split_once("[native_pdf.header_number]")
            .ok_or("section")?;
        let mut source=format!("{base}[native_pdf.header_number]\nenabled = true\nfont = \"{font}\"\nsize_pt = {size}\nposition = \"{position}\"\nstart = {start}\nbefore = \"{before}\"\nafter = \"{after}\"\n")
            .replace("padding_mm = 0",&format!("padding_mm = {padding}"))
            .replace("page_count = 2",&format!("page_count = {pages}"));
        if asymmetric {
            source=source.replace("page_left_margin_mm = 15","page_left_margin_mm = 25").replace("page_right_margin_mm = 15","page_right_margin_mm = 12")
                + "\n[page.even_margins]\nleft_mm = 12\nright_mm = 25\ntop_mm = 18\nbottom_mm = 20\n";
        }
        if coexist {
            source=source.replace("[native_pdf.header]\nenabled = false","[native_pdf.header]\nenabled = true")
                .replace("[header]","[header]\nalbum_pages_title_font = \"HN\"\nalbum_pages_title_size = 10")
                .replace("[colors]","[colors]\ncolor_album_title = \"black\"\ncolor_album_footer = \"black\"")
                + "\n[footer]\nalbum_pages_footer_number_enabled = true\nalbum_pages_footer_number_font = \"HN\"\nalbum_pages_footer_number_size = 8\nalbum_pages_footer_number_position = \"C\"\nalbum_pages_footer_number_start = 1\nalbum_pages_footer_number_before = \"Sheet \"\nalbum_pages_footer_number_after = \" / $PAGES$\"\n[native_pdf.footer]\nenabled = false\nfont = \"HN\"\nsize_pt = 8\nposition = \"C\"\ntext = \"Footer\"\npadding_mm = 0\nminimum_height_mm = 0\n[native_pdf.footer_number]\nenabled = true\n[native_pdf.title]\nenabled = true\ntext = \"TITLE\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
        }
        for minimum in [0, 25] {
            let source = source
                .replacen(
                    "minimum_height_mm = 0",
                    &format!("minimum_height_mm = {minimum}"),
                    1,
                )
                .replace(
                    "color_album_header = \"black\"",
                    if minimum > 0 {
                        "color_album_header = \"pink\""
                    } else {
                        "color_album_header = \"black\""
                    },
                );
            let config = NativePdfConfig::from_toml(&source)?;
            let path = dir.path().join("native.pdf");
            write_page_geometry_proof(&config.page_settings, &config.proof, &path)?;
            let actual_xml = trace(&path)?;
            let reference_xml = trace(
                &Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests/fixtures/albumeasy/header-number-v1")
                    .join(format!("{name}.pdf")),
            )?;
            let actual = roxmltree::Document::parse(&actual_xml)?;
            let reference = roxmltree::Document::parse(&reference_xml)?;
            let a_pages: Vec<_> = actual
                .descendants()
                .filter(|n| n.has_tag_name("page"))
                .collect();
            let e_pages: Vec<_> = reference
                .descendants()
                .filter(|n| n.has_tag_name("page"))
                .collect();
            assert_eq!(a_pages.len(), pages);
            assert_eq!(e_pages.len(), pages);
            for (index, (a, e)) in a_pages.iter().zip(e_pages).enumerate() {
                let spans: Vec<_> = a.descendants().filter(|n| n.has_tag_name("span")).collect();
                let expected: Vec<_> = e
                    .descendants()
                    .filter(|n| n.has_tag_name("span"))
                    .take(spans.len())
                    .collect();
                assert_eq!(
                    spans.len(),
                    if coexist {
                        4
                    } else if name == "header-number-centre" {
                        2
                    } else {
                        1
                    }
                );
                assert_eq!(spans.len(), expected.len());
                for (span_index, (a, e)) in spans.iter().zip(expected).enumerate() {
                    assert_eq!(a.attribute("font"), e.attribute("font"), "{name}");
                    assert_eq!(a.attribute("trm"), e.attribute("trm"), "{name}");
                    let a_glyphs: Vec<_> = a.children().filter(|n| n.has_tag_name("g")).collect();
                    let e_glyphs: Vec<_> = e.children().filter(|n| n.has_tag_name("g")).collect();
                    assert_eq!(a_glyphs.len(), e_glyphs.len(), "{name}");
                    for (a, e) in a_glyphs.iter().zip(e_glyphs) {
                        assert_eq!(a.attribute("unicode"), e.attribute("unicode"), "{name}");
                        for axis in ["x", "y"] {
                            // The native safety hint intentionally lowers the title.
                            if minimum > 0 && coexist && span_index == 3 && axis == "y" {
                                continue;
                            }
                            let a: f64 = a.attribute(axis).ok_or("axis")?.parse()?;
                            let e: f64 = e.attribute(axis).ok_or("axis")?.parse()?;
                            assert!(
                                points_to_mm(a - e).abs() < 0.01,
                                "{name} page {} {axis}: {a} != {e}",
                                index + 1
                            );
                        }
                    }
                }
                let layout = config
                    .page_settings
                    .resolve_with_page_count(index + 1, pages)?;
                let number = layout.header_number.ok_or("number")?;
                let colour: Vec<f64> = spans[0]
                    .parent()
                    .ok_or("fill")?
                    .attribute("color")
                    .ok_or("colour")?
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                assert_eq!(colour.len(), 3);
                for (a, e) in colour.iter().zip(number.colour.rgb()) {
                    assert!((a - e).abs() < 0.00051);
                }
                if let Some(title) = layout.title {
                    let first = spans[3]
                        .children()
                        .find(|n| n.has_tag_name("g"))
                        .ok_or("title glyph")?;
                    let y: f64 = first.attribute("y").ok_or("y")?.parse()?;
                    assert!((points_to_mm(y) - title.lines[0].baseline_mm).abs() < 0.01);
                }
                let guide = a
                    .children()
                    .rfind(|n| n.has_tag_name("stroke_path"))
                    .ok_or("guide")?;
                let ys: Vec<f64> = guide
                    .children()
                    .filter_map(|n| n.attribute("y"))
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                for (a, e) in [
                    (
                        ys.iter().copied().reduce(f64::min).ok_or("bottom")?,
                        layout.content_bounds.bottom,
                    ),
                    (
                        ys.iter().copied().reduce(f64::max).ok_or("top")?,
                        layout.content_bounds.top,
                    ),
                ] {
                    assert!((points_to_mm(a) - e).abs() < 0.01);
                }
            }
            let pdf = oxidize_pdf::parser::PdfDocument::open(&path)?;
            assert_eq!(pdf.page_count()? as usize, pages);
            for page in pdf.extract_text()? {
                assert!(!page.truncated);
            }
        }
    }
    Ok(())
}
