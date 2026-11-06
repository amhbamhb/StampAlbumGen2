use stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    commands::page::album_pages_spacing::Spacing,
    config::NativePdfConfig,
    document::Bounds,
    stamp_box::StampBoxGeometry,
    stamp_image::StampTextProofSettings,
    stamp_text::{
        fit_text_rows, ExpandedWidthLimit, StampTextItem, StampTextPolicy, TextWidthPolicy,
    },
};

fn item() -> Result<StampTextItem, String> {
    Ok(StampTextItem {
        geometry: StampBoxGeometry::from_total_additions(
            Bounds {
                left: 0.0,
                bottom: 0.0,
                right: 22.0,
                top: 25.0,
            },
            6.0,
            6.0,
            0.0,
        )?,
        heading: Some(StampTextProofSettings::new(
            "A deliberately long heading",
            "HN",
            10.0,
            NamedColour::parse("black")?,
        )),
        caption: Some(StampTextProofSettings::new(
            "First line\nSecond line",
            "HN",
            10.0,
            NamedColour::parse("black")?,
        )),
    })
}

fn area(width: f64, height: f64) -> Bounds {
    Bounds {
        left: 12.0,
        bottom: 12.0,
        right: 12.0 + width,
        top: 12.0 + height,
    }
}

fn policy(mode: TextWidthPolicy) -> Result<StampTextPolicy, String> {
    StampTextPolicy::new(
        mode,
        ExpandedWidthLimit::new("percent", 200.0)?,
        2.0,
        1.0,
        3.0,
    )
}

#[test]
fn expanded_caps_wrap_four_source_lines_and_reserve_unlimited_measured_lines() -> Result<(), String>
{
    let text = "line text 1\nLINE text 2 that is really really really really really long.\nline 3 another long line\nline 4 short";
    for (unit, value, cap) in [("percent", 120.0, 33.6), ("mm", 30.0, 30.0)] {
        let mut item = item()?;
        item.heading = None;
        item.caption.as_mut().ok_or("caption")?.text = text.into();
        let policy = StampTextPolicy::new(
            TextWidthPolicy::ExpandFootprint,
            ExpandedWidthLimit::new(unit, value)?,
            2.0,
            1.0,
            3.0,
        )?;
        let rows = fit_text_rows(
            area(180.0, 240.0),
            &[item],
            Spacing::parse(&[3.0, 4.0])?,
            &policy,
        )?;
        let placed = &rows[0][0];
        let caption = placed.caption.as_ref().ok_or("caption")?;
        assert!(caption.lines.len() > 4);
        assert!(caption.lines.iter().all(|line| line.width_mm <= cap));
        assert_eq!(
            caption.lines.first().ok_or("first line")?.text,
            "line text 1"
        );
        assert_eq!(
            caption.lines.last().ok_or("last line")?.text,
            "line 4 short"
        );
        assert_eq!(
            caption
                .lines
                .iter()
                .flat_map(|l| l.text.chars())
                .filter(|c| !c.is_whitespace())
                .collect::<String>(),
            text.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
        );
        assert!(placed.footprint.right - placed.footprint.left <= cap + 4.0 + 1e-9);
        assert!(
            (placed.geometry.frame_bounds().right - placed.geometry.frame_bounds().left - 28.0)
                .abs()
                < 1e-9
        );
        assert!((caption.bounds.bottom - placed.footprint.bottom - 1.0).abs() < 1e-9);
    }
    let mut many = item()?;
    many.caption.as_mut().ok_or("caption")?.text = vec!["line"; 30].join("\n");
    let rows = fit_text_rows(
        area(180.0, 240.0),
        &[many],
        Spacing::parse(&[3.0, 4.0])?,
        &policy(TextWidthPolicy::ExpandFootprint)?,
    )?;
    assert_eq!(
        rows[0][0].caption.as_ref().ok_or("caption")?.lines.len(),
        30
    );
    Ok(())
}

#[test]
fn expanded_caps_validate_units_values_and_never_shrink_a_frame() -> Result<(), String> {
    for (unit, value) in [
        ("wrong", 120.0),
        ("percent", 99.0),
        ("percent", f64::INFINITY),
        ("mm", 0.0),
        ("mm", -1.0),
        ("mm", f64::NAN),
        ("mm", f64::MAX),
    ] {
        assert!(ExpandedWidthLimit::new(unit, value).is_err());
    }
    let limit = ExpandedWidthLimit::new("mm", 27.0)?;
    let policy = StampTextPolicy::new(TextWidthPolicy::ExpandFootprint, limit, 0.0, 0.0, 0.0)?;
    let error = fit_text_rows(
        area(180.0, 240.0),
        &[item()?],
        Spacing::parse(&[3.0, 4.0])?,
        &policy,
    )
    .expect_err("cap smaller than mounting frame");
    assert!(error.contains("mounting frame"), "{error}");
    Ok(())
}

#[test]
fn percentage_and_mm_caps_have_distinct_measurable_wrap_boundaries() -> Result<(), String> {
    let mut stamp = item()?;
    stamp.heading = None;
    stamp.caption = Some(StampTextProofSettings::new(
        "MMMMMMMMMMMMMMM",
        "CN",
        10.0,
        NamedColour::parse("black")?,
    ));
    let spacing = Spacing::parse(&[3.0, 4.0])?;
    for (unit, value, expected_lines) in [("percent", 120.0, 1), ("mm", 30.0, 2)] {
        let policy = StampTextPolicy::new(
            TextWidthPolicy::ExpandFootprint,
            ExpandedWidthLimit::new(unit, value)?,
            2.0,
            1.0,
            3.0,
        )?;
        let rows = fit_text_rows(
            area(180.0, 240.0),
            std::slice::from_ref(&stamp),
            spacing,
            &policy,
        )?;
        let text = rows[0][0].caption.as_ref().ok_or("caption")?;
        assert_eq!(text.lines.len(), expected_lines);
        if unit == "percent" {
            assert!(text.lines[0].width_mm > 30.0 && text.lines[0].width_mm <= 33.6);
        } else {
            assert!(text.lines.iter().all(|l| l.width_mm <= 30.0));
        }
    }
    Ok(())
}

#[test]
fn wrapping_preserves_text_and_reserves_full_height_without_changing_frame() -> Result<(), String> {
    let rows = fit_text_rows(
        area(64.0, 200.0),
        &[item()?, item()?],
        Spacing::parse(&[3.0, 4.0])?,
        &policy(TextWidthPolicy::WrapToFrame)?,
    )?;
    assert_eq!(rows.len(), 2, "32 mm footprint + 3 gap + 32 mm footprint");
    for placed in rows.iter().flatten() {
        assert!((placed.footprint.right - placed.footprint.left - 32.0).abs() < 1e-9);
        let frame = placed.geometry.frame_bounds();
        let physical = placed.geometry.physical_bounds();
        assert!((frame.right - frame.left - 28.0).abs() < 1e-9);
        assert!((physical.right - physical.left - 22.0).abs() < 1e-9);
        let heading = placed.heading.as_ref().ok_or("heading")?;
        assert!(heading.lines.len() > 1);
        assert_eq!(
            heading
                .lines
                .iter()
                .map(|l| l.text.as_str())
                .collect::<Vec<_>>()
                .join(" "),
            "A deliberately long heading"
        );
        assert!(heading.lines.iter().all(|l| l.width_mm <= 28.0));
        assert!(placed.footprint.top > frame.top);
        assert!(placed.footprint.bottom < frame.bottom);
    }
    let first = rows[0][0].footprint;
    let next = rows[1][0].footprint;
    assert!((first.bottom - next.top - 4.0).abs() < 1e-9);
    Ok(())
}

#[test]
fn expanding_reserves_width_padding_height_and_keeps_explicit_line_breaks() -> Result<(), String> {
    let rows = fit_text_rows(
        area(180.0, 200.0),
        &[item()?],
        Spacing::parse(&[3.0, 4.0])?,
        &policy(TextWidthPolicy::ExpandFootprint)?,
    )?;
    let placed = &rows[0][0];
    let heading = placed.heading.as_ref().ok_or("heading")?;
    assert_eq!(heading.lines.len(), 1);
    assert_eq!(placed.caption.as_ref().ok_or("caption")?.lines.len(), 2);
    assert!(
        (placed.footprint.right - placed.footprint.left - heading.lines[0].width_mm - 4.0).abs()
            < 1e-9
    );
    assert!(placed.footprint.right - placed.footprint.left > 32.0);
    let frame = placed.geometry.frame_bounds();
    assert!((frame.right - frame.left - 28.0).abs() < 1e-9);
    let caption = placed.caption.as_ref().ok_or("caption")?;
    assert!((frame.bottom - caption.bounds.top - 3.0).abs() < 1e-9);
    assert!((caption.bounds.bottom - placed.footprint.bottom - 1.0).abs() < 1e-9);
    Ok(())
}

#[test]
fn invalid_policy_oversized_text_and_page_overflow_are_explicit_errors() -> Result<(), String> {
    for v in [-1.0, f64::NAN, f64::INFINITY, f64::MAX] {
        let limit = ExpandedWidthLimit::new("percent", 120.0)?;
        assert!(StampTextPolicy::new(TextWidthPolicy::WrapToFrame, limit, v, 0.0, 0.0).is_err());
        assert!(StampTextPolicy::new(TextWidthPolicy::WrapToFrame, limit, 0.0, v, 0.0).is_err());
        assert!(StampTextPolicy::new(TextWidthPolicy::WrapToFrame, limit, 0.0, 0.0, v).is_err());
    }
    let spacing = Spacing::parse(&[3.0, 4.0])?;
    assert!(fit_text_rows(
        area(180.0, 20.0),
        &[item()?],
        spacing,
        &policy(TextWidthPolicy::WrapToFrame)?
    )
    .is_err());
    assert!(fit_text_rows(
        area(32.0, 200.0),
        &[item()?],
        spacing,
        &policy(TextWidthPolicy::ExpandFootprint)?
    )
    .is_err());
    let mut tiny = item()?;
    tiny.geometry = StampBoxGeometry::from_total_additions(area(0.1, 25.0), 0.0, 0.0, 0.0)?;
    assert!(fit_text_rows(
        area(180.0, 200.0),
        &[tiny],
        spacing,
        &policy(TextWidthPolicy::WrapToFrame)?
    )
    .is_err());
    Ok(())
}

#[test]
fn template_policy_is_explicit_validated_and_not_reported_as_unapplied() -> Result<(), String> {
    let template = include_str!("../config_template.toml");
    let config = NativePdfConfig::from_toml(template)?;
    assert!(
        config.stamp_text.is_none(),
        "opt-in; production behavior unchanged"
    );
    let mut enabled: toml::Value = toml::from_str(template).map_err(|error| error.to_string())?;
    enabled["native_pdf"]["stamp_text"]["enabled"] = toml::Value::Boolean(true);
    let enabled = toml::to_string(&enabled).map_err(|error| error.to_string())?;
    for mode in ["wrap_to_frame", "expand_footprint"] {
        let text = enabled.replace(
            "width_policy = \"wrap_to_frame\"",
            &format!("width_policy = \"{mode}\""),
        );
        let config = NativePdfConfig::from_toml(&text)?;
        assert!(config.stamp_text.is_some());
        assert!(!config
            .unapplied_keys
            .iter()
            .any(|k| k.starts_with("native_pdf.stamp_text.")));
    }
    assert!(NativePdfConfig::from_toml(&enabled.replace(
        "width_policy = \"wrap_to_frame\"",
        "width_policy = \"wrong\""
    ))
    .is_err());
    assert!(NativePdfConfig::from_toml(&enabled.replace(
        "horizontal_padding_mm = 0.0",
        "horizontal_padding_mm = -1.0"
    ))
    .is_err());
    assert!(NativePdfConfig::from_toml(&enabled.replace("text_gap_mm = 3.0", "")).is_err());
    for (old, new) in [
        (
            "expanded_width_limit = \"percent\"",
            "expanded_width_limit = \"inches\"",
        ),
        (
            "expanded_width_value = 120.0",
            "expanded_width_value = 99.0",
        ),
        ("expanded_width_limit = \"percent\"", ""),
        ("expanded_width_value = 120.0", ""),
    ] {
        assert!(NativePdfConfig::from_toml(&enabled.replace(old, new)).is_err());
    }
    let mm = enabled
        .replace(
            "expanded_width_limit = \"percent\"",
            "expanded_width_limit = \"mm\"",
        )
        .replace(
            "expanded_width_value = 120.0",
            "expanded_width_value = 30.0",
        );
    assert!(NativePdfConfig::from_toml(&mm)?.stamp_text.is_some());
    assert!(NativePdfConfig::from_toml(&template.replace(
        "expanded_width_value = 120.0",
        "expanded_width_value = -1.0"
    ))
    .is_err());
    Ok(())
}

#[test]
fn wrapping_keeps_explicit_blank_lines_and_splits_words_without_losing_characters(
) -> Result<(), String> {
    use stamp_album_gen2::native_pdf::text::MeasuredText;
    let text = MeasuredText::new("HN", 10.0, "One  two\n\nABCDEFGHIJKLMNO")?;
    let wrapped = text.wrapped_to_width(12.0)?;
    assert!(wrapped.lines.iter().any(|line| line.text.is_empty()));
    assert_eq!(
        wrapped
            .lines
            .iter()
            .flat_map(|line| line.text.chars())
            .filter(|c| !c.is_whitespace())
            .collect::<String>(),
        "OnetwoABCDEFGHIJKLMNO"
    );
    assert!(wrapped.lines.iter().all(|line| line.width_mm <= 12.0));
    for width in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(text.wrapped_to_width(width).is_err());
    }
    Ok(())
}

#[test]
fn absent_text_has_no_phantom_gaps_and_empty_input_still_validates_bounds() -> Result<(), String> {
    let mut stamp = item()?;
    stamp.heading = None;
    stamp.caption = None;
    let spacing = Spacing::parse(&[3.0, 4.0])?;
    let policy = policy(TextWidthPolicy::ExpandFootprint)?;
    let rows = fit_text_rows(area(100.0, 100.0), &[stamp], spacing, &policy)?;
    let placed = &rows[0][0];
    assert!((placed.footprint.top - placed.footprint.bottom - 33.0).abs() < 1e-9);
    assert!(placed.heading.is_none() && placed.caption.is_none());
    assert!(fit_text_rows(area(100.0, 100.0), &[], spacing, &policy)?.is_empty());
    assert!(fit_text_rows(area(0.0, 100.0), &[], spacing, &policy).is_err());
    Ok(())
}

#[test]
fn both_policies_render_all_text_and_reject_overflow_before_touching_output(
) -> Result<(), Box<dyn std::error::Error>> {
    use oxidize_pdf::parser::PdfDocument;
    use stamp_album_gen2::native_pdf::renderer::write_stamp_text_geometry_proof;
    let config = NativePdfConfig::from_toml(include_str!("../config_template.toml"))?;
    let temporary = tempfile::tempdir()?;
    let mut proof = config.proof.clone();
    proof.page_count = 1;
    for mode in [
        TextWidthPolicy::WrapToFrame,
        TextWidthPolicy::ExpandFootprint,
    ] {
        let output = temporary.path().join(format!("{mode:?}.pdf"));
        let items = [item()?, item()?];
        write_stamp_text_geometry_proof(
            &config.page_settings,
            &proof,
            &items,
            &policy(mode)?,
            &output,
        )?;
        let pdf = PdfDocument::open(&output)?;
        let text = pdf.extract_text()?;
        let normalized = text
            .iter()
            .map(|page| page.text.as_str())
            .collect::<Vec<_>>()
            .join(" ")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            normalized.contains("A deliberately long heading"),
            "{normalized}"
        );
        assert!(normalized.contains("First line"), "{normalized}");
        assert!(normalized.contains("Second line"), "{normalized}");
        let original = std::fs::read(&output)?;
        let mut bad = item()?;
        bad.heading = Some(StampTextProofSettings::new(
            "X".repeat(10000),
            "HN",
            10.0,
            NamedColour::parse("black")?,
        ));
        let expand = policy(TextWidthPolicy::ExpandFootprint)?;
        assert!(write_stamp_text_geometry_proof(
            &config.page_settings,
            &proof,
            &[bad.clone()],
            &expand,
            &output
        )
        .is_err());
        assert_eq!(std::fs::read(&output)?, original);
        let missing = temporary.path().join("must-not-exist.pdf");
        assert!(write_stamp_text_geometry_proof(
            &config.page_settings,
            &proof,
            &[bad],
            &expand,
            &missing
        )
        .is_err());
        assert!(!missing.exists());
    }
    Ok(())
}

#[test]
#[ignore = "Regenerates persistent Letter text-footprint fixtures; run explicitly"]
fn regenerate_letter_text_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    use stamp_album_gen2::native_pdf::{
        commands::page::album_pages_title::AlbumPagesTitle,
        renderer::write_stamp_text_geometry_proof,
    };
    use std::{fs, path::Path};
    let output =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/stamp-text-v1");
    fs::create_dir_all(&output)?;
    let mut cases = Vec::new();
    for (name, mode, unit, value, long_text) in [
        ("wrap_to_frame", "wrap_to_frame", "percent", 120.0, false),
        (
            "expand_footprint",
            "expand_footprint",
            "percent",
            200.0,
            false,
        ),
        (
            "expanded-percent-120",
            "expand_footprint",
            "percent",
            120.0,
            true,
        ),
        ("expanded-mm-30", "expand_footprint", "mm", 30.0, true),
    ] {
        let mut source: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
        source["page"]["album_page_width_mm"] = toml::Value::Float(215.9);
        source["page"]["album_page_height_mm"] = toml::Value::Float(279.4);
        source["native_pdf"]["stamp_text"]["enabled"] = toml::Value::Boolean(true);
        source["native_pdf"]["stamp_text"]["width_policy"] = toml::Value::String(mode.into());
        source["native_pdf"]["stamp_text"]["expanded_width_limit"] =
            toml::Value::String(unit.into());
        source["native_pdf"]["stamp_text"]["expanded_width_value"] = toml::Value::Float(value);
        source["native_pdf"]["stamp_text"]["horizontal_padding_mm"] = toml::Value::Float(2.0);
        source["native_pdf"]["stamp_text"]["vertical_padding_mm"] = toml::Value::Float(1.0);
        let mut config = NativePdfConfig::from_toml(&toml::to_string(&source)?)?;
        config.proof.page_count = 1;
        config.proof.guide_enabled = false;
        config.page_settings.set_title(AlbumPagesTitle::new(
            "HN",10.0,&format!(
                "{name}: cap {value} {unit}\nGreen: footprint; black: frame; blue: physical\nPadding per edge: 2 mm horizontal / 1 mm vertical\nLetter / Actual size 100% / no Fit"
            ),4.0,0.0,NamedColour::parse("black")?,
        )?);
        let policy = config.stamp_text.as_ref().ok_or("missing policy")?;
        let mut items = Vec::new();
        for index in 1..=if long_text { 2 } else { 6 } {
            let mut item = item()?;
            item.heading.as_mut().ok_or("heading")?.text =
                format!("Item {index}: commemorative issue");
            item.caption.as_mut().ok_or("caption")?.text =
                "Known size 22 x 25 mm\nFrame 28 x 31 mm".into();
            if long_text {
                item.heading.as_mut().ok_or("heading")?.text = format!("Example {index}");
                item.caption.as_mut().ok_or("caption")?.text =
                    "line text 1\nLINE text 2 that is really really really really really long.\nline 3 another long line\nline 4 short".into();
            }
            items.push(item);
        }
        write_stamp_text_geometry_proof(
            &config.page_settings,
            &config.proof,
            &items,
            policy,
            &output.join(format!("{name}.pdf")),
        )?;
        let content = config
            .page_settings
            .resolve_with_context(1, 1, None)?
            .content_bounds;
        let rows = fit_text_rows(content, &items, config.page_settings.spacing(), policy)?;
        let mut expected_rows = Vec::new();
        let mut reference=format!(
            "# COMPATIBILITY ADAPTATION: native {mode}, cap {value} {unit}; comparison input only, not certified layout parity.\n\
             # Native footprint padding and expanded width are NOT AlbumEasy commands.\n\
             # All text is retained below; row/page positions may differ. Native settings remain in TOML.\n\
             ALBUM_PAGES_SIZE (215.9 279.4)\nALBUM_PAGES_MARGINS (12 12 12 12)\n\
             ALBUM_PAGES_SPACING (3 3)\nALBUM_PAGES_BORDER (0.2 0 0 0)\n\
             STAMP_BOXES_SIZE_ADJUST (0)\nPAGE_START\n\
             PAGE_TEXT_CENTRE (HN 10 \"{name}: text-preserving compatibility comparison\" 4)\n"
        );
        for row in rows {
            let mut data = Vec::new();
            for placed in row {
                let f = placed.geometry.frame_bounds();
                let p = placed.geometry.physical_bounds();
                let b = placed.footprint;
                let mut blocks = Vec::new();
                // Standalone PAGE_TEXT commands preserve text without pretending to
                // reproduce native beside-frame positions or enlarging STAMP_ADD.
                for text in [placed.heading.as_ref(), placed.caption.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    let original_lines = text
                        .lines
                        .iter()
                        .map(|line| line.text.as_str())
                        .collect::<Vec<_>>()
                        .join("\n");
                    let quoted = original_lines
                        .replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\n', "\\n");
                    blocks.push(serde_json::json!({
                        "bounds_mm":[text.bounds.left,text.bounds.bottom,text.bounds.right,text.bounds.top],
                        "ascent_mm":text.measured.ascent_mm,"descent_mm":text.measured.descent_mm,
                        "lines":text.lines.iter().map(|line|serde_json::json!({
                            "text":line.text,"left_mm":line.left_mm,
                            "baseline_mm":line.baseline_mm,"width_mm":line.width_mm
                        })).collect::<Vec<_>>()
                    }));
                    reference.push_str(&format!("PAGE_TEXT (HN 10 \"{quoted}\" 2)\n"));
                }
                reference.push_str(&format!(
                    "ROW_START_ES (HN 10 0.1)\nSTAMP_ADD ({} {} \"\" \"\" \"\" \"\")\n",
                    f.right - f.left,
                    f.top - f.bottom
                ));
                data.push(serde_json::json!({
                    "footprint_mm":[b.left,b.bottom,b.right,b.top],
                    "frame_mm":[f.left,f.bottom,f.right,f.top],
                    "physical_mm":[p.left,p.bottom,p.right,p.top],
                    "text":blocks
                }));
            }
            expected_rows.push(data);
        }
        fs::write(output.join(format!("{name}.txt")), reference)?;
        fs::write(output.join(format!("{name}.toml")),
            format!("# Policy fragment; merge into a full native config. See config_template.toml for complete help.\n[native_pdf.stamp_text]\n{}\n",
                toml::to_string(&source["native_pdf"]["stamp_text"])?.trim_end()))?;
        cases.push(serde_json::json!({
            "name":name,"width_policy":mode,"expanded_width_limit":unit,
            "expanded_width_value":value,"rows":expected_rows
        }));
    }
    let expected = serde_json::json!({
        "reference":"Calculated native text layout; AlbumEasy inputs intentionally adapted, not parity",
        "page_size_mm":[215.9,279.4],"tolerance_mm":0.004,"gap_tolerance_mm":0.006,
        "horizontal_gap_mm":3.0,"vertical_gap_mm":3.0,
        "cases":cases
    });
    fs::write(
        output.join("expected.json"),
        serde_json::to_string_pretty(&expected)? + "\n",
    )?;
    Ok(())
}
