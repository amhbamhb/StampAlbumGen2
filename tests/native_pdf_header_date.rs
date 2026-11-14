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
#[ignore = "requires mutool; compares actual dated reference glyphs, colour and guide geometry"]
fn serialized_dates_and_coexistence_match_frozen_reference_pdfs() -> Result<()> {
    use stamp_album_gen2::native_pdf::{renderer::write_page_geometry_proof, units::points_to_mm};
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/header-date-v1");
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
            2,
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
            1,
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
            .split_once("[native_pdf.header_date]")
            .ok_or("section")?;
        let mut source=format!("{base}[native_pdf.header_date]\nenabled = true\nfont = \"{font}\"\nsize_pt = {size}\nposition = \"{position}\"\nformat = \"{format}\"\nseparator = \"{separator}\"\nbefore = \"{before}\"\nafter = \"{after}\"\ndate_source = \"fixed\"\nfixed_date = \"{}\"\n",captured.trim())
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
                    "[native_pdf.header]\nenabled = false",
                    "[native_pdf.header]\nenabled = true",
                )
                .replace(
                    "[header]",
                    "[header]\nalbum_pages_title_font = \"HN\"\nalbum_pages_title_size = 10",
                )
                .replace(
                    "[colors]",
                    "[colors]\ncolor_album_title = \"black\"\ncolor_album_footer = \"black\"",
                )
                + "\n[native_pdf.header_number]\nenabled = true\nfont = \"HN\"\nsize_pt = 8\nposition = \"R\"\nstart = 7\nbefore = \"Page \"\nafter = \" of $PAGES$\"\n[footer]\nalbum_pages_footer_number_enabled = true\nalbum_pages_footer_number_font = \"HN\"\nalbum_pages_footer_number_size = 8\nalbum_pages_footer_number_position = \"C\"\nalbum_pages_footer_number_start = 1\nalbum_pages_footer_number_before = \"Sheet \"\nalbum_pages_footer_number_after = \" / $PAGES$\"\n[native_pdf.footer]\nenabled = false\nfont = \"HN\"\nsize_pt = 8\nposition = \"C\"\ntext = \"Footer\"\npadding_mm = 0\nminimum_height_mm = 0\n[native_pdf.footer_number]\nenabled = true\n[native_pdf.title]\nenabled = true\ntext = \"TITLE\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
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
                        5
                    } else if name == "date-centre" {
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
                            // The explicit safety hint intentionally moves only the title.
                            if minimum > 0 && coexist && span_index == 4 && axis == "y" {
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
                let date = layout.header_date.ok_or("date")?;
                let date_span = &spans[if coexist { 2 } else { 0 }];
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
                if let Some(title) = layout.title {
                    let first = spans[4]
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
            let date_text = stamp_album_gen2::native_pdf::date::DateFormat::parse(format)?.format(
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
[header]
album_pages_header_position = "C"
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
[native_pdf.header_date]
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

#[test]
fn generation_date_requires_one_explicit_document_context() -> Result<()> {
    use stamp_album_gen2::native_pdf::renderer::{
        write_page_geometry_proof, write_page_geometry_proof_with_date,
    };
    let config = NativePdfConfig::from_toml(
        &CONFIG.replace("date_source = \"fixed\"", "date_source = \"generation\""),
    )?;
    assert!(config.page_settings.capture_generation_date().is_some());
    assert!(
        config
            .page_settings
            .resolve(1)
            .err()
            .ok_or("missing date")?
            .contains("resolve_with_context")
    );
    assert!(config.page_settings.resolve_with_page_count(1, 2).is_err());
    let date = chrono::NaiveDate::from_ymd_opt(2024, 12, 31).ok_or("date")?;
    for page in [1, 2] {
        let layout = config
            .page_settings
            .resolve_with_context(page, 2, Some(date))?;
        assert_eq!(
            layout.header_date.ok_or("date")?.lines[0].text,
            "Generated: 2024-12-31"
        );
    }
    let next = chrono::NaiveDate::from_ymd_opt(2025, 1, 1).ok_or("date")?;
    assert_eq!(
        config
            .page_settings
            .resolve_with_context(1, 2, Some(next))?
            .header_date
            .ok_or("date")?
            .lines[0]
            .text,
        "Generated: 2025-01-01"
    );
    let dir = tempfile::tempdir()?;
    let output = dir.path().join("native.pdf");
    assert!(
        write_page_geometry_proof_with_date(&config.page_settings, &config.proof, &output, None)
            .is_err()
    );
    assert!(!output.exists());
    write_page_geometry_proof_with_date(&config.page_settings, &config.proof, &output, Some(date))?;
    let pdf = oxidize_pdf::parser::PdfDocument::open(&output)?;
    assert_eq!(pdf.page_count()?, 2);
    for page in pdf.extract_text()? {
        assert!(!page.truncated);
        assert!(page.text.contains("2024-12-31"));
        assert!(!page.text.contains("2025-01-01"));
    }
    let before = chrono::Local::now().date_naive().to_string();
    let automatic = dir.path().join("automatic.pdf");
    write_page_geometry_proof(&config.page_settings, &config.proof, &automatic)?;
    let after = chrono::Local::now().date_naive().to_string();
    let pdf = oxidize_pdf::parser::PdfDocument::open(&automatic)?;
    let pages = pdf.extract_text()?;
    assert_eq!(pages.len(), 2);
    assert_eq!(pages[0].text, pages[1].text);
    assert!(pages[0].text.contains(&before) || pages[0].text.contains(&after));
    assert!(pages.iter().all(|page| !page.truncated));
    let fixed = NativePdfConfig::from_toml(CONFIG)?;
    assert!(fixed.page_settings.capture_generation_date().is_none());
    assert_eq!(
        fixed
            .page_settings
            .resolve_with_context(1, 2, Some(next))?
            .header_date
            .ok_or("date")?
            .lines[0]
            .text,
        "Generated: 2024-02-29"
    );
    Ok(())
}

#[test]
fn defaults_disabled_validation_snapshot_and_unapplied_keys_are_explicit() -> Result<()> {
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let config = NativePdfConfig::from_toml(source)?;
        assert!(config.page_settings.resolve(1)?.header_date.is_none());
        assert!(config.page_settings.capture_generation_date().is_none());
        let config = NativePdfConfig::from_toml(&source.replace(
            "[native_pdf.header_date]\nenabled = false",
            "[native_pdf.header_date]\nenabled = true",
        ))?;
        let date = chrono::NaiveDate::from_ymd_opt(2024, 2, 29).ok_or("date")?;
        let layout = config
            .page_settings
            .resolve_with_context(1, config.proof.page_count, Some(date))?
            .header_date
            .ok_or("date")?;
        assert_eq!(layout.lines[0].text, "Generated: 2024-02-29");
        assert_eq!(layout.size_pt, 8.0);
        assert_eq!(layout.colour.rgb(), [0.0; 3]);
        assert_eq!(
            layout.lines[0].left_mm,
            config
                .page_settings
                .resolve_with_context(1, config.proof.page_count, Some(date))?
                .margin_bounds
                .left
        );
    }
    let (base, _) = CONFIG
        .split_once("[native_pdf.header_date]")
        .ok_or("section")?;
    let absent = NativePdfConfig::from_toml(base)?;
    assert!(absent.page_settings.resolve(1)?.header_date.is_none());
    let config = NativePdfConfig::from_toml(&(CONFIG.to_owned() + "\nfuture_option = 1\n"))?;
    assert!(
        config
            .unapplied_keys
            .contains(&"native_pdf.header_date.future_option".into())
    );
    assert!(
        !config
            .unapplied_keys
            .contains(&"native_pdf.header_date.date_source".into())
    );
    let changed = CONFIG.replace("fixed_date = \"2024-02-29\"", "fixed_date = \"2025-01-01\"");
    assert_eq!(
        config
            .page_settings
            .resolve(1)?
            .header_date
            .ok_or("date")?
            .lines[0]
            .text,
        "Generated: 2024-02-29"
    );
    assert_eq!(
        NativePdfConfig::from_toml(&changed)?
            .page_settings
            .resolve(1)?
            .header_date
            .ok_or("date")?
            .lines[0]
            .text,
        "Generated: 2025-01-01"
    );
    Ok(())
}

#[test]
fn invalid_missing_date_fields_return_redacted_keyed_errors() -> Result<()> {
    let (base, _) = CONFIG
        .split_once("[native_pdf.header_date]")
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
            "{base}[native_pdf.header_date]\n{}",
            fields
                .iter()
                .filter(|(k, _)| *k != key)
                .map(|(k, v)| format!("{k} = {v}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        let error = NativePdfConfig::from_toml(&source)
            .err()
            .ok_or("must fail")?;
        assert!(
            error.contains(&format!("native_pdf.header_date.{key}")),
            "{error}"
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
        ("fixed_date", "\"2024-00-00\""),
        ("before", "1"),
        ("after", "false"),
        ("separator", "1"),
    ] {
        for enabled in ["true", "false"] {
            let source = format!(
                "{base}[native_pdf.header_date]\n{}",
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
            assert!(error.contains("native_pdf.header_date"), "{key}: {error}");
            assert!(!error.contains("private-value"));
        }
    }
    assert!(
        NativePdfConfig::from_toml(&CONFIG.replace("[native_pdf.header]", "[unsupported_header]"))
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
fn date_hints_parity_and_multiline_reuse_measured_header_layout() -> Result<()> {
    let source = CONFIG
        .replace("position = \"L\"", "position = \"LR\"")
        .replace("separator = \"-\"", "separator = \"--\"")
        .replace("before = \"Generated: \"", "before = \"Created\\n\"")
        .replace("after = \"\"", "after = \" END\"");
    let config = NativePdfConfig::from_toml(&source)?;
    for page in [1, 2] {
        let layout = config
            .page_settings
            .resolve_with_page_count(page, 2)?
            .header_date
            .ok_or("date")?;
        assert_eq!(layout.lines.len(), 2);
        assert_eq!(layout.lines[1].text, "2024--02--29 END");
        for line in &layout.lines {
            let left = if page % 2 == 1 {
                15.0
            } else {
                195.0 - line.width_mm
            };
            assert!((line.left_mm - left).abs() < 0.000001);
        }
        assert!(layout.lines[0].baseline_mm > layout.lines[1].baseline_mm);
    }
    let automatic = config.page_settings.resolve(1)?;
    let hinted = NativePdfConfig::from_toml(
        &source.replace("minimum_height_mm = 0", "minimum_height_mm = 25"),
    )?
    .page_settings
    .resolve(1)?;
    let a = automatic.header_date.ok_or("date")?;
    let h = hinted.header_date.ok_or("date")?;
    assert_eq!(h.reserved_height_mm, 25.0);
    for (a, h) in a.lines.iter().zip(h.lines) {
        assert_eq!(a.text, h.text);
        assert_eq!(a.baseline_mm, h.baseline_mm);
        assert_eq!(a.left_mm, h.left_mm);
    }
    assert!(hinted.content_bounds.top < automatic.content_bounds.top);
    let empty = CONFIG
        .replace("separator = \"-\"", "separator = \"\"")
        .replace("before = \"Generated: \"", "before = \"\"");
    assert_eq!(
        NativePdfConfig::from_toml(&empty)?
            .page_settings
            .resolve(1)?
            .header_date
            .ok_or("date")?
            .lines[0]
            .text,
        "20240229"
    );
    Ok(())
}

#[test]
fn date_collisions_with_text_and_numbers_identify_correction_keys() -> Result<()> {
    let text = CONFIG.replace(
        "[native_pdf.header]\nenabled = false",
        "[native_pdf.header]\nenabled = true",
    );
    assert!(NativePdfConfig::from_toml(&text).is_ok());
    let collision = text.replace("position = \"L\"", "position = \"C\"");
    let error = NativePdfConfig::from_toml(&collision)
        .err()
        .ok_or("must overlap")?;
    assert!(
        error.contains("page 1")
            && error.contains("overlaps")
            && error.contains("native_pdf.header_date.position")
            && error.contains("album_pages_header_position"),
        "{error}"
    );
    let parity = text
        .replace("position = \"L\"", "position = \"LR\"")
        .replace(
            "album_pages_header_position = \"C\"",
            "album_pages_header_position = \"R\"",
        );
    assert!(
        NativePdfConfig::from_toml(&parity)
            .err()
            .ok_or("even collision")?
            .contains("page 2")
    );
    let number = CONFIG.to_owned()
        + "\n[native_pdf.header_number]\nenabled = true\nfont = \"HN\"\nsize_pt = 8\nposition = \"L\"\nstart = 1\nbefore = \"Page \"\nafter = \" of $PAGES$\"\n";
    let error = NativePdfConfig::from_toml(&number)
        .err()
        .ok_or("number collision")?;
    assert!(
        error.contains("ALBUM_PAGES_HEADER_NUMBER")
            && error.contains("native_pdf.header_number.position"),
        "{error}"
    );
    assert!(
        NativePdfConfig::from_toml(&number.replace("position = \"L\"", "position = \"R\""))
            .is_err()
    );
    let (base, _) = number
        .split_once("[native_pdf.header_number]")
        .ok_or("number")?;
    let safe = base.to_owned()
        + "\n[native_pdf.header_number]\nenabled = true\nfont = \"HN\"\nsize_pt = 8\nposition = \"R\"\nstart = 1\nbefore = \"Page \"\nafter = \" of $PAGES$\"\n";
    assert!(NativePdfConfig::from_toml(&safe).is_ok());
    Ok(())
}

#[test]
fn title_separation_and_invalid_later_page_preserve_output() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        commands::page::album_pages_marginse::EvenMargins, renderer::write_page_geometry_proof,
    };
    let source = CONFIG
        .replace(
            "[native_pdf.header]\nenabled = false",
            "[native_pdf.header]\nenabled = true",
        )
        .replace(
            "[header]",
            "[header]\nalbum_pages_title_font = \"TB\"\nalbum_pages_title_size = 25",
        )
        .replace("[colors]", "[colors]\ncolor_album_title = \"black\"")
        + "\n[native_pdf.title]\nenabled = true\ntext = \"Agyp TITLE\\nSecond line\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
    for padding in [0, -6] {
        let layout = NativePdfConfig::from_toml(
            &source.replace("padding_mm = 0", &format!("padding_mm = {padding}")),
        )?
        .page_settings
        .resolve(1)?;
        let title = layout.title.ok_or("title")?;
        assert!(title.bounds.top < layout.header_date.ok_or("date")?.bounds.bottom);
        assert!(title.bounds.top < layout.header.ok_or("header")?.bounds.bottom);
    }
    let mut config = NativePdfConfig::from_toml(CONFIG)?;
    config
        .page_settings
        .set_even_margins(EvenMargins::parse(&[15.0, 15.0, 1.0, 15.0])?);
    assert!(config.page_settings.resolve_with_page_count(1, 2).is_ok());
    assert!(config.page_settings.resolve_with_page_count(2, 2).is_err());
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("native.pdf");
    let error = write_page_geometry_proof(&config.page_settings, &config.proof, &path)
        .err()
        .ok_or("late error")?;
    assert!(
        error.contains("page 2") && error.contains("ALBUM_PAGES_HEADER_DATE"),
        "{error}"
    );
    assert!(!path.exists());
    fs::write(&path, b"existing output")?;
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &path).is_err());
    assert_eq!(fs::read(&path)?, b"existing output");
    Ok(())
}

#[test]
fn all_font_variants_validate_exact_date_width_and_year_range() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_header::AlbumPagesHeader, album_pages_header_date::AlbumPagesHeaderDate,
            album_pages_header_pad::HeaderPadding,
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
    let source = DateSource::parse("fixed", "2024-02-29")?;
    for font in [
        "HN", "HB", "HI", "HS", "TN", "TB", "TI", "TS", "CN", "CB", "CI", "CS",
    ] {
        let style = AlbumPagesHeader::new(
            font,
            8.0,
            TextPosition::Right,
            "2024-02-29",
            HeaderPadding::parse(&[0.0])?,
            0.0,
            NamedColour::parse("black")?,
        )?;
        let date = AlbumPagesHeaderDate::new(style, DateFormat::YearMonthDay, "-", "", "", source)?;
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
    }
    for year in [0, -1, 10000] {
        let date = chrono::NaiveDate::from_ymd_opt(year, 1, 1).ok_or("date")?;
        assert!(DateFormat::YearMonthDay.format(date, "-").is_err());
    }
    Ok(())
}

#[test]
fn frozen_reference_date_and_hashes_are_unchanged() -> Result<()> {
    use sha2::{Digest, Sha256};
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/header-date-v1");
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
    let captured = fs::read_to_string(root.join("generation-date.txt"))?;
    assert!(chrono::NaiveDate::parse_from_str(captured.trim(), "%Y-%m-%d").is_ok());
    Ok(())
}

#[test]
fn fixed_date_uses_explicit_formats_and_shared_header_geometry() -> Result<()> {
    for (format, expected) in [
        ("YMD", "2024-02-29"),
        ("DMY", "29-02-2024"),
        ("MDY", "02-29-2024"),
    ] {
        let config = NativePdfConfig::from_toml(
            &CONFIG.replace("format = \"YMD\"", &format!("format = \"{format}\"")),
        )?;
        for page in [1, 2] {
            let layout = config.page_settings.resolve_with_page_count(page, 2)?;
            let date = layout.header_date.ok_or("date")?;
            assert_eq!(date.lines[0].text, format!("Generated: {expected}"));
            assert!((date.lines[0].baseline_mm - 284.0).abs() < 0.000001);
            assert_eq!(date.lines[0].left_mm, 15.0);
        }
    }
    Ok(())
}
