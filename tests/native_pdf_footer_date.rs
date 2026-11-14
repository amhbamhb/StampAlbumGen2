use stamp_album_gen2::native_pdf::config::NativePdfConfig;
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
#[ignore = "requires mutool; compares actual footer-date and coexistence reference glyphs"]
fn serialized_footer_dates_match_frozen_fonts_glyphs_colour_and_geometry() -> Result<()> {
    use stamp_album_gen2::native_pdf::{renderer::write_page_geometry_proof, units::points_to_mm};
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/footer-date-v1");
    let captured = fs::read_to_string(root.join("generation-date.txt"))?;
    let dir = tempfile::tempdir()?;
    for (
        name,
        position,
        format,
        separator,
        before,
        after,
        font,
        size,
        padding,
        pages,
        asymmetric,
        coexist,
    ) in [
        (
            "date-basic",
            "L",
            "YMD",
            "-",
            "Generated: ",
            "",
            "HN",
            8,
            0,
            2,
            false,
            false,
        ),
        (
            "date-centre",
            "C",
            "DMY",
            "/",
            "Created\\n",
            " END",
            "TB",
            16,
            1,
            1,
            false,
            false,
        ),
        (
            "date-right",
            "R",
            "MDY",
            ".",
            "",
            "",
            "HN",
            25,
            3,
            1,
            false,
            false,
        ),
        (
            "date-lr", "LR", "YMD", "--", "Date ", "", "HN", 8, 3, 2, true, false,
        ),
        (
            "date-rl", "RL", "DMY", "", "", "", "HN", 8, -1, 2, true, false,
        ),
        (
            "date-coexist",
            "L",
            "YMD",
            "-",
            "Generated: ",
            "",
            "HN",
            8,
            0,
            2,
            false,
            true,
        ),
        (
            "date-omitted",
            "R",
            "YMD",
            "",
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
            .split_once("[native_pdf.footer_date]")
            .ok_or("section")?;
        let mut source=format!("{base}[native_pdf.footer_date]\nenabled = true\nfont = \"{font}\"\nsize_pt = {size}\nposition = \"{position}\"\nformat = \"{format}\"\nseparator = \"{separator}\"\nbefore = \"{before}\"\nafter = \"{after}\"\ndate_source = \"fixed\"\nfixed_date = \"{}\"\n",captured.trim())
            .replace("padding_mm = 0",&format!("padding_mm = {padding}"))
            .replace("page_count = 2",&format!("page_count = {pages}"));
        if asymmetric {
            source = source
                .replace("page_left_margin_mm = 15", "page_left_margin_mm = 25")
                .replace("page_right_margin_mm = 15", "page_right_margin_mm = 12")
                + "\n[page.even_margins]\nleft_mm = 12\nright_mm = 25\ntop_mm = 18\nbottom_mm = 20\n";
        }
        if coexist {
            source = source
                .replace(
                    "[native_pdf.footer]\nenabled = false",
                    "[native_pdf.footer]\nenabled = true",
                )
                .replace(
                    "[colors]",
                    "[colors]\ncolor_album_title = \"black\"\ncolor_album_header = \"black\"",
                )
                + &HEADER_DATE
                    .replace(
                        "[native_pdf.header]\nenabled = false",
                        "[native_pdf.header]\nenabled = true",
                    )
                    .replace("date_source = \"generation\"", "date_source = \"fixed\"")
                    .replace("2025-01-01", captured.trim())
                    .replace(
                        "[header]",
                        "[header]\nalbum_pages_title_font = \"HN\"\nalbum_pages_title_size = 10",
                    )
                + "\n[native_pdf.header_number]\nenabled = true\nfont = \"HN\"\nsize_pt = 8\nposition = \"R\"\nstart = 7\nbefore = \"Page \"\nafter = \" of $PAGES$\"\n[footer]\nalbum_pages_footer_number_enabled = true\nalbum_pages_footer_number_font = \"HN\"\nalbum_pages_footer_number_size = 8\nalbum_pages_footer_number_position = \"R\"\nalbum_pages_footer_number_start = 1\nalbum_pages_footer_number_before = \"Sheet \"\nalbum_pages_footer_number_after = \" / $PAGES$\"\n[native_pdf.footer_number]\nenabled = true\n[native_pdf.title]\nenabled = true\ntext = \"TITLE\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
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
            let path = dir.path().join("native.pdf");
            write_page_geometry_proof(&config.page_settings, &config.proof, &path)?;
            let actual_xml = trace(&path)?;
            let reference_xml = trace(&root.join(format!("{name}.pdf")))?;
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
                        7
                    } else if name == "date-centre" {
                        2
                    } else {
                        1
                    }
                );
                assert_eq!(spans.len(), expected.len());
                for (a, e) in spans.iter().zip(expected) {
                    assert_eq!(a.attribute("font"), e.attribute("font"), "{name}");
                    assert_eq!(a.attribute("trm"), e.attribute("trm"), "{name}");
                    let a_glyphs: Vec<_> = a.children().filter(|n| n.has_tag_name("g")).collect();
                    let e_glyphs: Vec<_> = e.children().filter(|n| n.has_tag_name("g")).collect();
                    assert_eq!(a_glyphs.len(), e_glyphs.len(), "{name}");
                    for (a, e) in a_glyphs.iter().zip(e_glyphs) {
                        assert_eq!(a.attribute("unicode"), e.attribute("unicode"), "{name}");
                        for axis in ["x", "y"] {
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
                let date = layout.footer_date.ok_or("date")?;
                let date_span = &spans[if coexist { 3 } else { 0 }];
                let colour: Vec<f64> = date_span
                    .parent()
                    .ok_or("fill")?
                    .attribute("color")
                    .ok_or("colour")?
                    .split_whitespace()
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                assert_eq!(colour.len(), 3);
                for (a, e) in colour.iter().zip(date.colour.rgb()) {
                    assert!((a - e).abs() < 0.00051);
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
            let date_text = stamp_album_gen2::native_pdf::date::DateFormat::parse_for(
                format,
                "native_pdf.footer_date",
            )?
            .format(
                chrono::NaiveDate::parse_from_str(captured.trim(), "%Y-%m-%d")?,
                separator,
            )?;
            for page in pdf.extract_text()? {
                assert!(!page.truncated);
                assert!(page.text.contains(&date_text));
            }
        }
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
enabled = false
font = "HN"
size_pt = 8
position = "C"
text = "Footer"
padding_mm = 0
minimum_height_mm = 0
[native_pdf.footer_date]
enabled = true
font = "HN"
size_pt = 8
position = "L"
format = "YMD"
separator = "-"
before = "Generated: "
after = ""
date_source = "fixed"
fixed_date = "2024-02-29"
"#;

const HEADER_DATE: &str = r#"
[header]
album_pages_header_position = "C"
[native_pdf.header]
enabled = false
font = "HN"
size_pt = 8
text = "Collection"
padding_mm = 0
minimum_height_mm = 0
[native_pdf.header_date]
enabled = true
font = "HN"
size_pt = 8
position = "L"
format = "YMD"
separator = "-"
before = "Generated: "
after = ""
date_source = "generation"
fixed_date = "2025-01-01"
"#;

#[test]
fn header_and_footer_share_one_generation_date_with_independent_fixed_overrides() -> Result<()> {
    use stamp_album_gen2::native_pdf::renderer::{
        write_page_geometry_proof, write_page_geometry_proof_with_date,
    };
    let header = HEADER_DATE.replace("date_source = \"generation\"", "date_source = \"fixed\"");
    let footer = CONFIG.replace("date_source = \"fixed\"", "date_source = \"generation\"");
    let context = chrono::NaiveDate::from_ymd_opt(2024, 12, 31).ok_or("date")?;
    for (source, expected_header, expected_footer, capture) in [
        (
            format!("{footer}{HEADER_DATE}"),
            "2024-12-31",
            "2024-12-31",
            true,
        ),
        (
            format!("{CONFIG}{HEADER_DATE}"),
            "2024-12-31",
            "2024-02-29",
            true,
        ),
        (
            format!("{footer}{header}"),
            "2025-01-01",
            "2024-12-31",
            true,
        ),
        (
            format!("{CONFIG}{header}"),
            "2025-01-01",
            "2024-02-29",
            false,
        ),
    ] {
        let source = source.replace("[colors]", "[colors]\ncolor_album_header = \"black\"");
        let config = NativePdfConfig::from_toml(&source)?;
        assert_eq!(
            config.page_settings.capture_generation_date().is_some(),
            capture
        );
        for page in [1, 2] {
            let layout = config
                .page_settings
                .resolve_with_context(page, 2, Some(context))?;
            assert_eq!(
                layout.header_date.ok_or("header date")?.lines[0].text,
                format!("Generated: {expected_header}")
            );
            assert_eq!(
                layout.footer_date.ok_or("footer date")?.lines[0].text,
                format!("Generated: {expected_footer}")
            );
        }
        if capture {
            assert!(config.page_settings.resolve_with_page_count(1, 2).is_err());
        }
    }
    let source = format!("{footer}{HEADER_DATE}")
        .replace("[colors]", "[colors]\ncolor_album_header = \"black\"");
    let config = NativePdfConfig::from_toml(&source)?;
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("native.pdf");
    assert!(
        write_page_geometry_proof_with_date(&config.page_settings, &config.proof, &path, None)
            .is_err()
    );
    assert!(!path.exists());
    write_page_geometry_proof_with_date(
        &config.page_settings,
        &config.proof,
        &path,
        Some(context),
    )?;
    let pdf = oxidize_pdf::parser::PdfDocument::open(&path)?;
    for page in pdf.extract_text()? {
        assert_eq!(page.text.matches("2024-12-31").count(), 2);
        assert!(!page.truncated);
    }
    let next = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).ok_or("date")?;
    let layout = config
        .page_settings
        .resolve_with_context(1, 2, Some(next))?;
    assert_eq!(
        layout.header_date.ok_or("header")?.lines[0].text,
        "Generated: 2025-01-01"
    );
    assert_eq!(
        layout.footer_date.ok_or("footer")?.lines[0].text,
        "Generated: 2025-01-01"
    );
    let before = chrono::Local::now().date_naive().to_string();
    let automatic = dir.path().join("automatic.pdf");
    write_page_geometry_proof(&config.page_settings, &config.proof, &automatic)?;
    let after = chrono::Local::now().date_naive().to_string();
    let pdf = oxidize_pdf::parser::PdfDocument::open(&automatic)?;
    let pages = pdf.extract_text()?;
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].text, pages[1].text);
    assert!(
        pages[0].text.matches(&before).count() == 2 || pages[0].text.matches(&after).count() == 2
    );
    assert!(pages.iter().all(|page| !page.truncated));
    // Footer-only generation must consult the clock even without a header.
    let footer_only = NativePdfConfig::from_toml(&footer)?;
    assert!(
        footer_only
            .page_settings
            .capture_generation_date()
            .is_some()
    );
    assert!(
        footer_only
            .page_settings
            .resolve(1)
            .err()
            .ok_or("context")?
            .contains("native_pdf.footer_date.date_source")
    );
    Ok(())
}

#[test]
fn defaults_enablement_snapshots_and_colour_precedence_are_explicit() -> Result<()> {
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let disabled = NativePdfConfig::from_toml(source)?;
        assert!(disabled.page_settings.resolve(1)?.footer_date.is_none());
        let enabled = NativePdfConfig::from_toml(&source.replace(
            "[native_pdf.footer_date]\nenabled = false",
            "[native_pdf.footer_date]\nenabled = true",
        ))?;
        let date = chrono::NaiveDate::from_ymd_opt(2024, 2, 29).ok_or("date")?;
        let layout = enabled
            .page_settings
            .resolve_with_context(1, enabled.proof.page_count, Some(date))?
            .footer_date
            .ok_or("date")?;
        assert_eq!(layout.lines[0].text, "Generated: 2024-02-29");
        assert_eq!(layout.size_pt, 8.0);
        assert_eq!(layout.colour.rgb(), [0.0; 3]);
        assert_eq!(
            layout.lines[0].left_mm,
            enabled
                .page_settings
                .resolve_with_context(1, enabled.proof.page_count, Some(date))?
                .margin_bounds
                .left
        );
    }
    let (base, _) = CONFIG
        .split_once("[native_pdf.footer_date]")
        .ok_or("section")?;
    assert!(
        NativePdfConfig::from_toml(base)?
            .page_settings
            .resolve(1)?
            .footer_date
            .is_none()
    );
    let source = "color_album_footer = \"red\"\n".to_owned()
        + &CONFIG.replace(
            "color_album_footer = \"black\"",
            "color_album_footer = \"pink\"",
        )
        + "\nfuture_option = 1\n";
    let loaded = NativePdfConfig::from_toml(&source)?;
    assert!(
        loaded
            .overridden_keys
            .contains(&"color_album_footer".into())
    );
    assert!(
        loaded
            .unapplied_keys
            .contains(&"native_pdf.footer_date.future_option".into())
    );
    assert!(
        !loaded
            .unapplied_keys
            .contains(&"native_pdf.footer_date.format".into())
    );
    assert_eq!(
        loaded
            .page_settings
            .resolve(1)?
            .footer_date
            .ok_or("date")?
            .colour
            .rgb(),
        [1.0, 192.0 / 255.0, 203.0 / 255.0]
    );
    let changed = source.replace("2024-02-29", "2025-01-01");
    assert_eq!(
        loaded
            .page_settings
            .resolve(1)?
            .footer_date
            .ok_or("date")?
            .lines[0]
            .text,
        "Generated: 2024-02-29"
    );
    assert_eq!(
        NativePdfConfig::from_toml(&changed)?
            .page_settings
            .resolve(1)?
            .footer_date
            .ok_or("date")?
            .lines[0]
            .text,
        "Generated: 2025-01-01"
    );
    Ok(())
}

#[test]
fn missing_invalid_dates_and_fragments_report_footer_keys_without_values() -> Result<()> {
    let (base, _) = CONFIG
        .split_once("[native_pdf.footer_date]")
        .ok_or("section")?;
    let fields = [
        ("enabled", "true"),
        ("font", "\"HN\""),
        ("size_pt", "8"),
        ("position", "\"L\""),
        ("format", "\"YMD\""),
        ("separator", "\"-\""),
        ("before", "\"Generated: \""),
        ("after", "\"\""),
        ("date_source", "\"fixed\""),
        ("fixed_date", "\"2024-02-29\""),
    ];
    for (key, _) in fields {
        let source = format!(
            "{base}[native_pdf.footer_date]\n{}",
            fields
                .iter()
                .filter(|(k, _)| *k != key)
                .map(|(k, v)| format!("{k} = {v}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        assert!(
            NativePdfConfig::from_toml(&source)
                .err()
                .ok_or("missing")?
                .contains(&format!("native_pdf.footer_date.{key}"))
        );
    }
    for (key, value) in [
        ("enabled", "\"private-value\""),
        ("font", "\"private-value\""),
        ("size_pt", "0"),
        ("size_pt", "-1"),
        ("size_pt", "nan"),
        ("size_pt", "inf"),
        ("position", "\"private-value\""),
        ("format", "\"private-value\""),
        ("separator", "\"\\t\""),
        ("separator", "\"漢字\""),
        ("before", "\"\\t\""),
        ("after", "\"漢字\""),
        ("date_source", "\"private-value\""),
        ("fixed_date", "\"private-value\""),
        ("fixed_date", "\"2023-02-29\""),
        ("fixed_date", "\"1900-02-29\""),
        ("fixed_date", "\"2024-04-31\""),
        ("fixed_date", "\"2024-2-29\""),
        ("fixed_date", "\"0000-01-01\""),
        ("fixed_date", "\"10000-01-01\""),
        ("fixed_date", "\"2024-02-29 \""),
        ("before", "1"),
        ("after", "false"),
        ("separator", "1"),
    ] {
        for enabled in ["true", "false"] {
            let source = format!(
                "{base}[native_pdf.footer_date]\n{}",
                fields
                    .iter()
                    .map(|(k, v)| format!(
                        "{k} = {}",
                        if *k == key {
                            value
                        } else if *k == "enabled" {
                            enabled
                        } else {
                            v
                        }
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            let error = NativePdfConfig::from_toml(&source)
                .err()
                .ok_or_else(|| format!("{key} must fail"))?;
            assert!(error.contains("native_pdf.footer_date"), "{key}: {error}");
            assert!(!error.contains("private-value"));
            assert!(!error.contains("native_pdf.header_date"), "{error}");
        }
    }
    assert!(
        NativePdfConfig::from_toml(&CONFIG.replace("[native_pdf.footer]", "[unsupported_footer]"))
            .is_err()
    );
    assert!(
        NativePdfConfig::from_toml(
            &CONFIG
                .replace("date_source = \"fixed\"", "date_source = \"generation\"")
                .replace("2024-02-29", "2023-02-29")
        )
        .is_err()
    );
    for date in ["0001-01-01", "2000-02-29", "9999-12-31"] {
        assert!(NativePdfConfig::from_toml(&CONFIG.replace("2024-02-29", date)).is_ok());
    }
    Ok(())
}

#[test]
fn measured_footer_hints_parity_and_multiline_do_not_move_glyphs() -> Result<()> {
    let source = CONFIG
        .replace("position = \"L\"", "position = \"LR\"")
        .replace("separator = \"-\"", "separator = \"--\"")
        .replace("before = \"Generated: \"", "before = \"Created\\n\"")
        .replace("after = \"\"", "after = \" END\"");
    let config = NativePdfConfig::from_toml(&source)?;
    for page in [1, 2] {
        let date = config
            .page_settings
            .resolve_with_page_count(page, 2)?
            .footer_date
            .ok_or("date")?;
        assert_eq!(date.lines.len(), 2);
        assert_eq!(date.lines[1].text, "2024--02--29 END");
        assert!(date.lines[0].baseline_mm > date.lines[1].baseline_mm);
        for line in date.lines {
            assert!(
                (line.left_mm
                    - if page % 2 == 1 {
                        15.0
                    } else {
                        195.0 - line.width_mm
                    })
                .abs()
                    < 0.000001
            );
        }
    }
    let automatic = config.page_settings.resolve(1)?;
    let hinted = NativePdfConfig::from_toml(
        &source.replace("minimum_height_mm = 0", "minimum_height_mm = 25"),
    )?
    .page_settings
    .resolve(1)?;
    let a = automatic.footer_date.ok_or("date")?;
    let h = hinted.footer_date.ok_or("date")?;
    assert_eq!(h.reserved_height_mm, 25.0);
    assert!((hinted.content_bounds.bottom - h.reservation_top_mm).abs() < 0.000001);
    for (a, h) in a.lines.iter().zip(h.lines) {
        assert_eq!(a.text, h.text);
        assert_eq!(a.baseline_mm, h.baseline_mm);
        assert_eq!(a.left_mm, h.left_mm);
    }
    assert!(hinted.content_bounds.bottom > automatic.content_bounds.bottom);
    let empty = CONFIG
        .replace("separator = \"-\"", "separator = \"\"")
        .replace("before = \"Generated: \"", "before = \"\"");
    assert_eq!(
        NativePdfConfig::from_toml(&empty)?
            .page_settings
            .resolve(1)?
            .footer_date
            .ok_or("date")?
            .lines[0]
            .text,
        "20240229"
    );
    Ok(())
}

#[test]
fn footer_date_text_and_number_collisions_are_actionable_and_page_specific() -> Result<()> {
    let text = CONFIG.replace(
        "[native_pdf.footer]\nenabled = false",
        "[native_pdf.footer]\nenabled = true",
    );
    assert!(NativePdfConfig::from_toml(&text).is_ok());
    let error = NativePdfConfig::from_toml(&text.replace("position = \"L\"", "position = \"C\""))
        .err()
        .ok_or("collision")?;
    assert!(
        error.contains("page 1")
            && error.contains("overlaps")
            && error.contains("native_pdf.footer_date.position")
            && error.contains("native_pdf.footer.position"),
        "{error}"
    );
    let parity = text
        .replace("position = \"L\"", "position = \"LR\"")
        .replace("position = \"C\"", "position = \"R\"");
    assert!(
        NativePdfConfig::from_toml(&parity)
            .err()
            .ok_or("even")?
            .contains("page 2")
    );
    let number = CONFIG.to_owned()
        + "\n[footer]\nalbum_pages_footer_number_enabled = true\nalbum_pages_footer_number_font = \"HN\"\nalbum_pages_footer_number_size = 8\nalbum_pages_footer_number_position = \"L\"\nalbum_pages_footer_number_start = 1\nalbum_pages_footer_number_before = \"Page \"\nalbum_pages_footer_number_after = \" of $PAGES$\"\n[native_pdf.footer_number]\nenabled = true\n";
    let error = NativePdfConfig::from_toml(&number).err().ok_or("number")?;
    assert!(
        error.contains("ALBUM_PAGES_FOOTER_NUMBER")
            && error.contains("album_pages_footer_number_position"),
        "{error}"
    );
    assert!(
        NativePdfConfig::from_toml(
            &number.replace("number_position = \"L\"", "number_position = \"R\"")
        )
        .is_ok()
    );
    Ok(())
}

#[test]
fn late_footer_date_failure_preserves_existing_output_and_title_clearance() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        commands::page::album_pages_marginse::EvenMargins, renderer::write_page_geometry_proof,
    };
    let title = CONFIG.replace("[colors]", "[colors]\ncolor_album_title = \"black\"")
        + "\n[header]\nalbum_pages_title_font = \"TB\"\nalbum_pages_title_size = 25\n[native_pdf.title]\nenabled = true\ntext = \"Agyp TITLE\\nSecond line\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
    for padding in [0, -200] {
        let config = NativePdfConfig::from_toml(
            &title.replace("padding_mm = 0", &format!("padding_mm = {padding}")),
        )?;
        let layout = config.page_settings.resolve(1)?;
        assert!(
            layout.title.ok_or("title")?.bounds.bottom
                > layout.footer_date.ok_or("date")?.bounds.top
        );
    }
    let mut config = NativePdfConfig::from_toml(CONFIG)?;
    config
        .page_settings
        .set_even_margins(EvenMargins::parse(&[15.0, 15.0, 15.0, 1.0])?);
    assert!(config.page_settings.resolve_with_page_count(1, 2).is_ok());
    assert!(config.page_settings.resolve_with_page_count(2, 2).is_err());
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("native.pdf");
    let error = write_page_geometry_proof(&config.page_settings, &config.proof, &path)
        .err()
        .ok_or("late error")?;
    assert!(
        error.contains("page 2") && error.contains("ALBUM_PAGES_FOOTER_DATE"),
        "{error}"
    );
    assert!(!path.exists());
    fs::write(&path, b"existing output")?;
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &path).is_err());
    assert_eq!(fs::read(&path)?, b"existing output");
    // AlbumEasy accepts this clipping case; native must explicitly reject it.
    let clipping = CONFIG
        .replace(
            "[native_pdf.footer_date]\nenabled = true\nfont = \"HN\"\nsize_pt = 8",
            "[native_pdf.footer_date]\nenabled = true\nfont = \"HN\"\nsize_pt = 25",
        )
        .replace("padding_mm = 0", "padding_mm = 5");
    assert!(NativePdfConfig::from_toml(&clipping).is_err());
    Ok(())
}

#[test]
fn all_font_variants_validate_exact_width_and_keyed_generation_range() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_footer::AlbumPagesFooter, album_pages_footer_date::AlbumPagesFooterDate,
            album_pages_footer_pad::FooterPadding,
        },
        date::{DateFormat, DateSource},
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
    let source = DateSource::parse_for("fixed", "2024-02-29", "native_pdf.footer_date")?;
    for font in [
        "HN", "HB", "HI", "HS", "TN", "TB", "TI", "TS", "CN", "CB", "CI", "CS",
    ] {
        let style = AlbumPagesFooter::new(
            font,
            8.0,
            TextPosition::Right,
            "2024-02-29",
            FooterPadding::parse(&[0.0])?,
            0.0,
            NamedColour::parse("black")?,
        )?;
        let date = AlbumPagesFooterDate::new(
            style.clone(),
            DateFormat::YearMonthDay,
            "-",
            "",
            "",
            source,
        )?;
        let first = date.layout(margin, page, 1, None)?;
        let exact = Bounds {
            left: 0.0,
            right: first.lines[0].width_mm,
            ..margin
        };
        assert!(date.layout(exact, page, 1, None).is_ok());
        assert!(
            date.layout(
                Bounds {
                    right: exact.right - 0.00001,
                    ..exact
                },
                page,
                1,
                None
            )
            .is_err()
        );
        assert!(date.layout(margin, page, 0, None).is_err());
        let generated = AlbumPagesFooterDate::new(
            style,
            DateFormat::YearMonthDay,
            "-",
            "",
            "",
            DateSource::Generation,
        )?;
        for year in [0, -1, 10000] {
            let date = chrono::NaiveDate::from_ymd_opt(year, 1, 1).ok_or("date")?;
            let error = generated
                .layout(margin, page, 1, Some(date))
                .err()
                .ok_or("range")?;
            assert!(
                error.contains("native_pdf.footer_date")
                    && !error.contains("native_pdf.header_date"),
                "{error}"
            );
        }
    }
    Ok(())
}

#[test]
fn frozen_footer_date_reference_identities_are_unchanged() -> Result<()> {
    use sha2::{Digest, Sha256};
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/footer-date-v1");
    let manifest = fs::read_to_string(root.join("sha256.txt"))?;
    assert_eq!(manifest.lines().count(), 15);
    for line in manifest.lines() {
        let (expected, name) = line.split_once("  ").ok_or("hash")?;
        let actual: String = Sha256::digest(fs::read(root.join(name))?)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(actual, expected, "{name}");
    }
    assert!(
        chrono::NaiveDate::parse_from_str(
            fs::read_to_string(root.join("generation-date.txt"))?.trim(),
            "%Y-%m-%d"
        )
        .is_ok()
    );
    Ok(())
}

#[test]
fn fixed_footer_date_uses_formats_and_verified_bottom_geometry() -> Result<()> {
    for (format, expected) in [
        ("YMD", "2024-02-29"),
        ("DMY", "29-02-2024"),
        ("MDY", "02-29-2024"),
    ] {
        let config = NativePdfConfig::from_toml(
            &CONFIG.replace("format = \"YMD\"", &format!("format = \"{format}\"")),
        )?;
        for page in [1, 2] {
            let date = config
                .page_settings
                .resolve_with_page_count(page, 2)?
                .footer_date
                .ok_or("date")?;
            assert_eq!(date.lines[0].text, format!("Generated: {expected}"));
            assert!(
                (date.lines[0].baseline_mm - (15.0 - 8.0 * 25.4 / 72.0 - 0.5)).abs() < 0.000001
            );
            assert_eq!(date.lines[0].left_mm, 15.0);
        }
    }
    Ok(())
}
