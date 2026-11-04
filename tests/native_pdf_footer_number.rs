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
[footer]
album_pages_footer_number_enabled = true
album_pages_footer_number_font = "HN"
album_pages_footer_number_size = 8
album_pages_footer_number_position = "C"
album_pages_footer_number_start = 1
album_pages_footer_number_before = "Page "
album_pages_footer_number_after = " of $PAGES$"
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
position = "L"
text = "Collection"
padding_mm = 0
minimum_height_mm = 0
[native_pdf.footer_number]
enabled = true
"#;

#[test]
fn numbering_requires_known_total_and_uses_printed_start_not_parity() -> Result<()> {
    let config = NativePdfConfig::from_toml(CONFIG)?;
    assert!(config.page_settings.resolve(1).is_err());
    for page in [1, 2] {
        let layout = config.page_settings.resolve_with_page_count(page, 2)?;
        let number = layout.footer_number.ok_or("number")?;
        assert_eq!(number.lines[0].text, format!("Page {page} of 2"));
        assert!((number.lines[0].baseline_mm - (15.0 - 0.5 - 8.0 * 25.4 / 72.0)).abs() < 0.000001);
    }

    let source = CONFIG
        .replace("number_start = 1", "number_start = 8")
        .replace("number_position = \"C\"", "number_position = \"LR\"")
        .replace("number_before = \"Page \"", "number_before = \"\"")
        .replace("number_after = \" of $PAGES$\"", "number_after = \"\"");
    let config = NativePdfConfig::from_toml(&source)?;
    for (page, expected) in [(1, "8"), (2, "9"), (3, "10")] {
        let layout = config.page_settings.resolve_with_page_count(page, 3)?;
        let number = layout.footer_number.ok_or("number")?;
        assert_eq!(number.lines[0].text, expected);
        let line = &number.lines[0];
        let x = if page % 2 == 1 {
            15.0
        } else {
            195.0 - line.width_mm
        };
        assert!((line.left_mm - x).abs() < 0.000001);
    }
    Ok(())
}

#[test]
fn signed_starts_tokens_and_hints_do_not_change_number_glyphs() -> Result<()> {
    let source = CONFIG
        .replace("number_start = 1", "number_start = -1")
        .replace(
            "number_before = \"Page \"",
            "number_before = \"$PAGES$ / \"",
        )
        .replace(
            "number_after = \" of $PAGES$\"",
            "number_after = \" / $PAGES$ / $PAGES$\"",
        );
    let config = NativePdfConfig::from_toml(&source)?;
    for (page, label) in [(1, -1), (2, 0), (3, 1)] {
        let layout = config.page_settings.resolve_with_page_count(page, 3)?;
        assert_eq!(
            layout.footer_number.ok_or("number")?.lines[0].text,
            format!("3 / {label} / 3 / 3")
        );
    }
    let auto = config.page_settings.resolve_with_page_count(1, 3)?;
    let hinted = NativePdfConfig::from_toml(
        &source.replace("minimum_height_mm = 0", "minimum_height_mm = 25"),
    )?
    .page_settings
    .resolve_with_page_count(1, 3)?;
    let a = auto.footer_number.ok_or("number")?;
    let h = hinted.footer_number.ok_or("number")?;
    assert_eq!(h.lines[0].text, a.lines[0].text);
    assert_eq!(h.lines[0].left_mm, a.lines[0].left_mm);
    assert_eq!(h.lines[0].baseline_mm, a.lines[0].baseline_mm);
    assert_eq!(h.reserved_height_mm, 25.0);
    assert!((hinted.content_bounds.bottom - h.reservation_top_mm).abs() < 0.000001);
    assert!(hinted.content_bounds.bottom > auto.content_bounds.bottom);
    let multi = NativePdfConfig::from_toml(&CONFIG.replace(
        "number_before = \"Page \"",
        "number_before = \"First\\nPage \"",
    ))?
    .page_settings
    .resolve_with_page_count(1, 2)?
    .footer_number
    .ok_or("number")?;
    assert_eq!(multi.lines.len(), 2);
    assert_eq!(multi.lines[1].text, "Page 1 of 2");
    Ok(())
}

#[test]
fn numbering_defaults_enablement_precedence_and_snapshots_are_explicit() -> Result<()> {
    for source in [
        include_str!("../config_template.toml"),
        include_str!("../config_full_example.toml"),
    ] {
        let disabled = NativePdfConfig::from_toml(source)?;
        assert!(disabled.page_settings.resolve(1)?.footer_number.is_none());
        let source = source.replace(
            "[native_pdf.footer_number]\nenabled = false",
            "[native_pdf.footer_number]\nenabled = true",
        );
        let enabled = NativePdfConfig::from_toml(&source)?;
        let n = enabled
            .page_settings
            .resolve_with_page_count(1, enabled.proof.page_count)?
            .footer_number
            .ok_or("number")?;
        assert_eq!(
            n.lines[0].text,
            format!("Page 1 of {}", enabled.proof.page_count)
        );
        assert_eq!(n.size_pt, 8.0);
        assert_eq!(n.colour.rgb(), [0.0; 3]);
    }
    let disabled = CONFIG.replace("number_enabled = true", "number_enabled = false");
    assert!(NativePdfConfig::from_toml(&disabled)?
        .page_settings
        .resolve(1)?
        .footer_number
        .is_none());
    let absent = CONFIG
        .split("[native_pdf.footer_number]")
        .next()
        .ok_or("config")?;
    assert!(NativePdfConfig::from_toml(absent)?
        .page_settings
        .resolve(1)?
        .footer_number
        .is_none());
    let source =
        format!("album_pages_footer_number_start = 100\n{CONFIG}\nfuture_number_setting = 1\n");
    let temporary = tempfile::tempdir()?;
    let path = temporary.path().join("settings.toml");
    fs::write(&path, source)?;
    let snapshot = NativePdfConfig::load(&path)?;
    assert_eq!(
        snapshot.overridden_keys,
        ["album_pages_footer_number_start"]
    );
    assert_eq!(
        snapshot.unapplied_keys,
        ["native_pdf.footer_number.future_number_setting"]
    );
    fs::write(
        &path,
        CONFIG.replace("number_start = 1", "number_start = 7"),
    )?;
    assert_eq!(
        snapshot
            .page_settings
            .resolve_with_page_count(1, 2)?
            .footer_number
            .ok_or("number")?
            .lines[0]
            .text,
        "Page 1 of 2"
    );
    assert_eq!(
        NativePdfConfig::load(&path)?
            .page_settings
            .resolve_with_page_count(1, 2)?
            .footer_number
            .ok_or("number")?
            .lines[0]
            .text,
        "Page 7 of 2"
    );
    Ok(())
}

#[test]
fn invalid_missing_and_overflow_values_fail_with_redacted_keys() -> Result<()> {
    for (key, original, invalid) in [
        (
            "album_pages_footer_number_font",
            "\"HN\"",
            vec!["\"\"", "\"BAD\""],
        ),
        (
            "album_pages_footer_number_size",
            "8",
            vec!["0", "-1", "nan", "inf", "200"],
        ),
        (
            "album_pages_footer_number_position",
            "\"C\"",
            vec!["\"\"", "\"BAD\""],
        ),
        (
            "album_pages_footer_number_start",
            "1",
            vec![
                "1.5",
                "2147483648",
                "-2147483649",
                "2147483647",
                "\"private-value\"",
            ],
        ),
        (
            "album_pages_footer_number_before",
            "\"Page \"",
            vec!["\"\\t\"", "\"漢字\""],
        ),
        (
            "album_pages_footer_number_after",
            "\" of $PAGES$\"",
            vec!["\"\\t\"", "\"漢字\""],
        ),
        (
            "album_pages_footer_number_enabled",
            "true",
            vec!["\"private-value\""],
        ),
    ] {
        for value in invalid {
            let source =
                CONFIG.replace(&format!("{key} = {original}"), &format!("{key} = {value}"));
            let error = NativePdfConfig::from_toml(&source)
                .err()
                .ok_or_else(|| format!("{key} must fail"))?;
            assert!(
                error.contains(key) || error.contains("album_pages_footer_number"),
                "{error}"
            );
            assert!(!error.contains("private-value"));
        }
        let source = CONFIG
            .lines()
            .filter(|line| !line.starts_with(key))
            .collect::<Vec<_>>()
            .join("\n");
        let error = NativePdfConfig::from_toml(&source)
            .err()
            .ok_or("missing must fail")?;
        assert!(error.contains(key), "{error}");
    }
    let source = CONFIG.replace(
        "[native_pdf.footer_number]\nenabled = true",
        "[native_pdf.footer_number]\nenabled = \"private-value\"",
    );
    let error = NativePdfConfig::from_toml(&source)
        .err()
        .ok_or("must fail")?;
    assert!(error.contains("native_pdf.footer_number.enabled") && !error.contains("private-value"));
    assert!(NativePdfConfig::from_toml(&CONFIG.replace(
        "[native_pdf.footer_number]\nenabled = true",
        "[native_pdf.footer_number]"
    ))
    .is_err());
    let no_policy = CONFIG.replace("[native_pdf.footer]", "[unapplied_footer]");
    assert!(NativePdfConfig::from_toml(&no_policy).is_err());
    let config = NativePdfConfig::from_toml(CONFIG)?;
    for (page, total) in [(0, 2), (1, 0), (3, 2)] {
        assert!(config
            .page_settings
            .resolve_with_page_count(page, total)
            .is_err());
    }
    let at_max = NativePdfConfig::from_toml(
        &CONFIG
            .replace("number_start = 1", "number_start = 2147483647")
            .replace("page_count = 2", "page_count = 1"),
    )?;
    assert!(at_max.page_settings.resolve_with_page_count(2, 2).is_err());
    Ok(())
}

#[test]
fn coexistence_collisions_and_later_page_preflight_are_checked() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        commands::page::album_pages_marginse::EvenMargins, renderer::write_page_geometry_proof,
    };
    let coexist = CONFIG
        .replace(
            "[native_pdf.footer]\nenabled = false",
            "[native_pdf.footer]\nenabled = true",
        )
        .replace("number_position = \"C\"", "number_position = \"R\"");
    let config = NativePdfConfig::from_toml(&coexist)?;
    let layout = config.page_settings.resolve_with_page_count(1, 2)?;
    assert!(layout.footer.is_some() && layout.footer_number.is_some());
    let collision = coexist.replace("number_position = \"R\"", "number_position = \"L\"");
    assert!(NativePdfConfig::from_toml(&collision)
        .err()
        .ok_or("collision")?
        .contains("overlaps"));
    let even_collision = coexist.replace("number_position = \"R\"", "number_position = \"RL\"");
    let error = NativePdfConfig::from_toml(&even_collision)
        .err()
        .ok_or("collision")?;
    assert!(
        error.contains("page 2") && error.contains("overlaps"),
        "{error}"
    );
    let mut config = NativePdfConfig::from_toml(CONFIG)?;
    config
        .page_settings
        .set_even_margins(EvenMargins::parse(&[15.0, 15.0, 15.0, 1.0])?);
    let temporary = tempfile::tempdir()?;
    let path = temporary.path().join("native.pdf");
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &path).is_err());
    assert!(!path.exists());
    fs::write(&path, b"existing output")?;
    assert!(write_page_geometry_proof(&config.page_settings, &config.proof, &path).is_err());
    assert_eq!(fs::read(&path)?, b"existing output");
    Ok(())
}

#[test]
fn frozen_number_reference_identities_are_unchanged() -> Result<()> {
    use sha2::{Digest, Sha256};
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/footer-number-v1");
    let manifest = fs::read_to_string(root.join("sha256.txt"))?;
    assert_eq!(manifest.lines().count(), 14);
    for line in manifest.lines() {
        let (expected, name) = line.split_once("  ").ok_or("hash")?;
        let hash: String = Sha256::digest(fs::read(root.join(name))?)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(hash, expected, "{name}");
    }
    Ok(())
}

#[test]
fn changing_digit_width_and_font_metrics_are_validated_at_final_count() -> Result<()> {
    use stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::page::{
            album_pages_footer::AlbumPagesFooter,
            album_pages_footer_number::AlbumPagesFooterNumber,
            album_pages_footer_pad::FooterPadding,
        },
        document::Bounds,
        text::TextPosition,
    };
    let source = CONFIG
        .replace("number_start = 1", "number_start = 9")
        .replace("number_position = \"C\"", "number_position = \"R\"")
        .replace("number_before = \"Page \"", "number_before = \"\"")
        .replace("number_after = \" of $PAGES$\"", "number_after = \"\"");
    let config = NativePdfConfig::from_toml(&source)?;
    let one = config
        .page_settings
        .resolve_with_page_count(1, 2)?
        .footer_number
        .ok_or("number")?;
    let two = config
        .page_settings
        .resolve_with_page_count(2, 2)?
        .footer_number
        .ok_or("number")?;
    assert_eq!(one.lines[0].text, "9");
    assert_eq!(two.lines[0].text, "10");
    assert!(two.lines[0].width_mm > one.lines[0].width_mm);
    assert!(two.lines[0].left_mm < one.lines[0].left_mm);
    let page = Bounds {
        left: 0.0,
        bottom: 0.0,
        right: 210.0,
        top: 297.0,
    };
    for font in [
        "HN", "HB", "HI", "HS", "TN", "TB", "TI", "TS", "CN", "CB", "CI", "CS",
    ] {
        let style = AlbumPagesFooter::new(
            font,
            8.0,
            TextPosition::Right,
            "9",
            FooterPadding::parse(&[0.0])?,
            0.0,
            NamedColour::parse("black")?,
        )?;
        let number = AlbumPagesFooterNumber::new(style, 9, "", "")?;
        let margin = Bounds {
            left: 15.0,
            bottom: 15.0,
            right: 195.0,
            top: 282.0,
        };
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
#[ignore = "requires mutool; compares numbering and coexistence glyphs against actual reference PDFs"]
fn serialized_numbers_tokens_and_footer_coexistence_match_references() -> Result<()> {
    use stamp_album_gen2::native_pdf::{renderer::write_page_geometry_proof, units::points_to_mm};
    let temporary = tempfile::tempdir()?;
    for (name, position, start, before, after, size, padding, pages, asymmetric, coexist) in [
        (
            "number-basic",
            "C",
            1,
            "Page ",
            " of $PAGES$",
            8,
            0,
            2,
            false,
            false,
        ),
        ("number-lr", "LR", 8, "", "", 8, 3, 3, true, false),
        ("number-rl", "RL", 7, "Sheet ", "", 8, -1, 2, true, false),
        (
            "number-tokens",
            "R",
            7,
            "$PAGES$ / ",
            " / $PAGES$ / $PAGES$",
            16,
            1,
            2,
            false,
            false,
        ),
        (
            "number-coexist",
            "R",
            1,
            "Page ",
            " of $PAGES$",
            8,
            0,
            2,
            false,
            true,
        ),
        ("number-signed", "C", -1, "", "", 8, 0, 3, false, false),
        ("number-omitted", "C", 1, "", "", 8, 0, 1, false, false),
    ] {
        let mut source = CONFIG
            .replace(
                "number_position = \"C\"",
                &format!("number_position = \"{position}\""),
            )
            .replace("number_start = 1", &format!("number_start = {start}"))
            .replace(
                "number_before = \"Page \"",
                &format!("number_before = \"{before}\""),
            )
            .replace(
                "number_after = \" of $PAGES$\"",
                &format!("number_after = \"{after}\""),
            )
            .replace("number_size = 8", &format!("number_size = {size}"))
            .replace("padding_mm = 0", &format!("padding_mm = {padding}"))
            .replace("page_count = 2", &format!("page_count = {pages}"));
        if name == "number-tokens" {
            source = source.replace("number_font = \"HN\"", "number_font = \"TB\"");
        }
        if asymmetric {
            source=source.replace("page_left_margin_mm = 15","page_left_margin_mm = 25")
                    .replace("page_right_margin_mm = 15","page_right_margin_mm = 12")
                    + "\n[page.even_margins]\nleft_mm = 12\nright_mm = 25\ntop_mm = 18\nbottom_mm = 20\n";
        }
        if coexist {
            source=source.replace("[native_pdf.footer]\nenabled = false","[native_pdf.footer]\nenabled = true")
                    .replace("[colors]","[header]\nalbum_pages_header_position = \"L\"\nalbum_pages_title_font = \"HN\"\nalbum_pages_title_size = 10\n[colors]\ncolor_album_header = \"black\"\ncolor_album_title = \"black\"")
                    + "\n[native_pdf.header]\nenabled = true\nfont = \"HN\"\nsize_pt = 8\ntext = \"HEADER\"\npadding_mm = 0\nminimum_height_mm = 0\n[native_pdf.title]\nenabled = true\ntext = \"TITLE\"\nvspace_after_mm = 2\nminimum_height_mm = 0\n";
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
                    .join("tests/fixtures/albumeasy/footer-number-v1")
                    .join(format!("{name}.pdf")),
            )?;
            let actual = roxmltree::Document::parse(&actual_xml)?;
            let reference = roxmltree::Document::parse(&reference_xml)?;
            let reference_pages: Vec<_> = reference
                .descendants()
                .filter(|n| n.has_tag_name("page"))
                .collect();
            assert_eq!(reference_pages.len(), pages);
            let actual_pages: Vec<_> = actual
                .descendants()
                .filter(|n| n.has_tag_name("page"))
                .collect();
            assert_eq!(actual_pages.len(), pages);
            for (i, page) in actual_pages.iter().enumerate() {
                let spans: Vec<_> = page
                    .descendants()
                    .filter(|n| n.has_tag_name("span"))
                    .collect();
                let expected: Vec<_> = reference_pages[i]
                    .descendants()
                    .filter(|n| n.has_tag_name("span"))
                    .take(spans.len())
                    .collect();
                assert_eq!(spans.len(), if coexist { 4 } else { 1 });
                assert_eq!(expected.len(), spans.len());
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
                let layout = config.page_settings.resolve_with_page_count(i + 1, pages)?;
                let number = layout.footer_number.ok_or("number")?;
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
                let guide = page
                    .children()
                    .rfind(|n| n.has_tag_name("stroke_path"))
                    .ok_or("guide")?;
                let ys: Vec<f64> = guide
                    .children()
                    .filter_map(|n| n.attribute("y"))
                    .map(str::parse)
                    .collect::<std::result::Result<_, _>>()?;
                for (actual, expected) in [
                    (
                        ys.iter().copied().reduce(f64::min).ok_or("bottom")?,
                        layout.content_bounds.bottom,
                    ),
                    (
                        ys.iter().copied().reduce(f64::max).ok_or("top")?,
                        layout.content_bounds.top,
                    ),
                ] {
                    assert!((points_to_mm(actual) - expected).abs() < 0.01);
                }
            }
            let pdf = oxidize_pdf::parser::PdfDocument::open(output)?;
            assert_eq!(pdf.page_count()? as usize, pages);
            for page in pdf.extract_text()? {
                assert!(!page.truncated);
            }
        }
    }
    Ok(())
}
