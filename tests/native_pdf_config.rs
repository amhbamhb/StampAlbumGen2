use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{
    config::NativePdfConfig, renderer::write_page_geometry_proof, units::points_to_mm,
};
use std::{error::Error, fs};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const PAGE: &str = r#"
color_album_border = "black"
[native_pdf.proof]
page_count = 3
guide_enabled = true
guide_width_mm = 0.1
guide_colour = "black"
[page]
album_page_width_mm = 210.25
album_page_height_mm = 297.75
page_left_margin_mm = 25.125
page_right_margin_mm = 12.25
page_top_margin_mm = 18.375
page_bottom_margin_mm = 21.5
album_pages_spacing_h = 6.25
album_pages_spacing_v = 8.75
page_border_outer = 0.1
page_border_inner1 = 0.5
page_border_inner2 = 0.2
page_border_spacing = 1.25
"#;

const EVEN: &str = r#"
[page.even_margins]
left_mm = 12.25
right_mm = 25.125
top_mm = 22.5
bottom_mm = 16.25
"#;

fn near(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 0.0000001,
        "{actual} != {expected}"
    );
}

#[test]
fn copied_template_values_reach_every_existing_command() -> Result<()> {
    let loaded = NativePdfConfig::from_toml(include_str!("../config_template.toml"))?;
    let settings = &loaded.page_settings;
    let page = settings.resolve(1)?;
    near(page.size.width_mm(), 216.0);
    near(page.size.height_mm(), 279.4);
    near(page.margin_bounds.left, 10.0);
    near(page.margin_bounds.bottom, 10.0);
    near(settings.spacing().horizontal_mm(), 3.0);
    near(settings.spacing().vertical_mm(), 3.0);
    near(settings.border().occupancy_mm(), 2.7);
    near(page.content_bounds.left, 15.7);
    near(page.content_bounds.top, 266.7);
    let strokes = settings.border().strokes(page.margin_bounds)?;
    assert_eq!(strokes.len(), 3);
    for (stroke, width) in strokes.iter().zip([0.1, 0.5, 0.1]) {
        near(stroke.width_mm, width);
    }
    assert!(!loaded
        .unapplied_keys
        .contains(&"header.album_pages_title_font".into()));
    assert!(loaded
        .unapplied_keys
        .contains(&"page.max_page_vspace_mm".into()));
    assert!(loaded.overridden_keys.is_empty());
    let full = NativePdfConfig::from_toml(include_str!("../config_full_example.toml"))?;
    near(full.page_settings.border().occupancy_mm(), 3.6);
    near(full.page_settings.spacing().horizontal_mm(), 0.0);
    Ok(())
}

#[test]
fn explicit_file_changes_preserve_precision_even_margins_and_native_output() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    let path = temporary.path().join("user config.toml");
    fs::write(&path, format!("{PAGE}{EVEN}"))?;
    let loaded = NativePdfConfig::load(&path)?;
    let settings = &loaded.page_settings;
    assert!(loaded.unapplied_keys.is_empty());
    let first = settings.resolve(1)?;
    let second = settings.resolve(2)?;
    let third = settings.resolve(3)?;
    near(first.size.width_mm(), 210.25);
    near(first.size.height_mm(), 297.75);
    near(first.margin_bounds.left, 25.125);
    near(first.margin_bounds.bottom, 21.5);
    near(first.margin_bounds.top, 279.375);
    near(first.margin_bounds.right, 198.0);
    near(first.content_bounds.left, 34.675);
    near(first.content_bounds.top, 276.075);
    near(second.margin_bounds.left, 12.25);
    near(second.margin_bounds.bottom, 16.25);
    near(second.margin_bounds.top, 275.25);
    near(second.margin_bounds.right, 185.125);
    near(third.margin_bounds.left, first.margin_bounds.left);
    near(settings.spacing().next_top(100.0)?, 91.25);
    let output = temporary.path().join("native.pdf");
    write_page_geometry_proof(settings, &loaded.proof, &output)?;
    let pdf = PdfDocument::open(&output)?;
    assert_eq!(pdf.page_count()?, 3);
    let media = pdf.get_page(0)?.media_box;
    assert!((points_to_mm(media[2] - media[0]) - 210.25).abs() < 0.01);
    assert!((points_to_mm(media[3] - media[1]) - 297.75).abs() < 0.01);
    fs::write(&path, PAGE.replace("210.25", "220"))?;
    let reloaded = NativePdfConfig::load(&path)?;
    near(reloaded.page_settings.resolve(1)?.size.width_mm(), 220.0);
    // An existing job keeps its snapshot; a new load sees the edited file.
    near(loaded.page_settings.resolve(1)?.size.width_mm(), 210.25);
    Ok(())
}

#[test]
fn legacy_root_and_section_precedence_are_explicit() -> Result<()> {
    let mut flat: toml::Table = toml::from_str(PAGE)?;
    let page = flat.remove("page").ok_or("page")?;
    flat.extend(page.as_table().ok_or("page table")?.clone());
    let flat = toml::to_string(&flat)?;
    let loaded = NativePdfConfig::from_toml(&flat)?;
    near(loaded.page_settings.resolve(1)?.size.width_mm(), 210.25);
    let loaded = NativePdfConfig::from_toml(&format!(
        "album_page_width_mm = \"stale invalid root value\"\n{PAGE}"
    ))?;
    near(loaded.page_settings.resolve(1)?.size.width_mm(), 210.25);
    assert_eq!(loaded.overridden_keys, ["album_page_width_mm"]);
    assert!(loaded.unapplied_keys.is_empty());
    let conflicting = format!(
        "{}\n[page]\nalbum_page_width_mm = \"invalid section value\"\n",
        flat
    );
    assert!(NativePdfConfig::from_toml(&conflicting).is_err());
    Ok(())
}

#[test]
fn borders_are_required_and_even_margins_are_complete_or_absent() -> Result<()> {
    let bare = PAGE
        .lines()
        .filter(|line| !line.starts_with("page_border_"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(NativePdfConfig::from_toml(&bare).is_err());
    let loaded = NativePdfConfig::from_toml(PAGE)?;
    near(loaded.page_settings.resolve(2)?.margin_bounds.left, 25.125);
    for key in [
        "page_border_outer",
        "page_border_inner1",
        "page_border_inner2",
        "page_border_spacing",
    ] {
        let incomplete = PAGE
            .lines()
            .filter(|line| !line.starts_with(key))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(NativePdfConfig::from_toml(&incomplete).is_err(), "{key}");
    }
    for field in ["left_mm", "right_mm", "top_mm", "bottom_mm"] {
        let incomplete = EVEN
            .lines()
            .filter(|line| !line.starts_with(field))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(NativePdfConfig::from_toml(&format!("{PAGE}{incomplete}")).is_err());
    }
    assert!(NativePdfConfig::from_toml(&format!("{PAGE}\n[page.even_margins]\n")).is_err());
    assert!(NativePdfConfig::from_toml(&format!("{PAGE}\neven_margins = false\n")).is_err());
    let bad_even = EVEN.replace("left_mm = 12.25", "left_mm = 200");
    assert!(NativePdfConfig::from_toml(&format!("{PAGE}{bad_even}")).is_err());
    Ok(())
}

#[test]
fn invalid_files_keys_and_geometry_return_actionable_redacted_errors() -> Result<()> {
    for key in [
        "album_page_width_mm",
        "album_page_height_mm",
        "page_left_margin_mm",
        "page_right_margin_mm",
        "page_top_margin_mm",
        "page_bottom_margin_mm",
        "album_pages_spacing_h",
        "album_pages_spacing_v",
    ] {
        let missing = PAGE
            .lines()
            .filter(|line| !line.starts_with(key))
            .collect::<Vec<_>>()
            .join("\n");
        let error = NativePdfConfig::from_toml(&missing)
            .err()
            .ok_or("must fail")?;
        assert!(error.contains(key), "{error}");
    }
    for value in [
        "\"private-value\"",
        "true",
        "[]",
        "{}",
        "-1",
        "inf",
        "nan",
        "0",
    ] {
        let invalid = PAGE.replace(
            "album_page_width_mm = 210.25",
            &format!("album_page_width_mm = {value}"),
        );
        let error = NativePdfConfig::from_toml(&invalid)
            .err()
            .ok_or("must fail")?;
        assert!(
            error.contains("album_page_width_mm") || error.contains("ALBUM_PAGES_SIZE"),
            "{error}"
        );
        assert!(!error.contains("private-value"), "{error}");
    }
    assert!(NativePdfConfig::from_toml("page = false").is_err());
    assert!(NativePdfConfig::from_toml(&PAGE.replace("6.25", "1000")).is_err());
    let duplicate = format!("{PAGE}\nalbum_page_width_mm = 100\n");
    assert!(NativePdfConfig::from_toml(&duplicate).is_err());
    let temporary = tempfile::tempdir()?;
    let missing = temporary.path().join("missing-config.toml");
    assert!(NativePdfConfig::load(&missing)
        .err()
        .ok_or("must fail")?
        .contains("missing-config.toml"));
    let malformed = temporary.path().join("malformed.toml");
    fs::write(
        &malformed,
        "[stampworld]\npassword = \"private-value\" invalid\n",
    )?;
    let error = NativePdfConfig::load(&malformed).err().ok_or("must fail")?;
    assert!(error.contains("malformed.toml"));
    assert!(!error.contains("private-value"), "{error}");
    Ok(())
}

#[test]
fn unapplied_settings_are_reported_by_key_not_value() -> Result<()> {
    let loaded = NativePdfConfig::from_toml(&format!(
        "{PAGE}\nunsupported_setting = 42\n[stampworld]\npassword = \"private-value\"\n"
    ))?;
    assert_eq!(
        loaded.unapplied_keys,
        ["page.unsupported_setting", "stampworld.password"]
    );
    assert!(!format!("{:?}", loaded.unapplied_keys).contains("private-value"));
    Ok(())
}

#[test]
#[ignore = "requires mutool on PATH; verifies configuration controls actual native page strokes"]
fn user_config_controls_actual_rendered_paths() -> Result<()> {
    let config = NativePdfConfig::from_toml(&format!("{PAGE}{EVEN}"))?;
    let settings = config.page_settings;
    let temporary = tempfile::tempdir()?;
    let output = temporary.path().join("configured.pdf");
    write_page_geometry_proof(&settings, &config.proof, &output)?;
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
    let pages: Vec<_> = document
        .descendants()
        .filter(|n| n.has_tag_name("page"))
        .collect();
    assert_eq!(pages.len(), 3);
    for (page_number, (page, lefts)) in pages
        .iter()
        .zip([
            [25.175, 26.725, 28.325, 34.675],
            [12.3, 13.85, 15.45, 21.8],
            [25.175, 26.725, 28.325, 34.675],
        ])
        .enumerate()
    {
        let paths: Vec<_> = page
            .children()
            .filter(|n| n.has_tag_name("stroke_path"))
            .collect();
        assert_eq!(paths.len(), 4);
        let (bottom, top, right) = if page_number == 1 {
            (16.25, 275.25, 185.125)
        } else {
            (21.5, 279.375, 198.0)
        };
        for (index, ((path, expected_x), expected_width)) in paths
            .iter()
            .zip(lefts)
            .zip([0.1, 0.5, 0.2, 0.1])
            .enumerate()
        {
            let point = path
                .children()
                .find(|n| n.has_tag_name("moveto"))
                .ok_or("moveto")?;
            let x: f64 = point.attribute("x").ok_or("x")?.parse()?;
            let width: f64 = path.attribute("linewidth").ok_or("width")?.parse()?;
            assert!((points_to_mm(x) - expected_x).abs() < 0.01);
            assert!((points_to_mm(width) - expected_width).abs() < 0.01);
            let inset = [0.05, 1.6, 3.2, 3.3][index];
            let expected_right = right - inset - if index == 3 { 6.25 } else { 0.0 };
            let points: Vec<_> = path
                .children()
                .filter(|n| n.has_tag_name("moveto") || n.has_tag_name("lineto"))
                .collect();
            assert_eq!(points.len(), 4);
            for (point, (x, y)) in points.iter().zip([
                (expected_x, bottom + inset),
                (expected_right, bottom + inset),
                (expected_right, top - inset),
                (expected_x, top - inset),
            ]) {
                for (attribute, expected) in [("x", x), ("y", y)] {
                    let value: f64 = point.attribute(attribute).ok_or("coordinate")?.parse()?;
                    assert!((points_to_mm(value) - expected).abs() < 0.01);
                }
            }
        }
    }
    Ok(())
}
