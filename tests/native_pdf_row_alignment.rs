use stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    config::NativePdfConfig,
    document::Bounds,
    pagination::{paginate_stamp_items, paginate_stamp_sections, StampRowSection},
    stamp_box::StampBoxGeometry,
    stamp_image::StampTextProofSettings,
    stamp_rows::RowAlignment,
    stamp_text::{ExpandedWidthLimit, StampTextItem, StampTextPolicy, TextWidthPolicy},
};
use std::error::Error;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn config() -> Result<NativePdfConfig> {
    let mut source: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
    source["page"]["album_page_width_mm"] = toml::Value::Float(215.9);
    source["page"]["album_page_height_mm"] = toml::Value::Float(279.4);
    Ok(NativePdfConfig::from_toml(&toml::to_string(&source)?)?)
}

fn policy() -> Result<StampTextPolicy> {
    Ok(StampTextPolicy::new(
        TextWidthPolicy::WrapToFrame,
        ExpandedWidthLimit::new("percent", 120.0)?,
        2.0,
        1.0,
        3.0,
    )?)
}

fn item(width: f64) -> Result<StampTextItem> {
    Ok(StampTextItem {
        geometry: StampBoxGeometry::from_total_additions(
            Bounds {
                left: 0.0,
                bottom: 0.0,
                right: width,
                top: 25.0,
            },
            6.0,
            6.0,
            0.0,
        )?,
        heading: None,
        caption: None,
    })
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-8, "{a} != {b}");
}

#[test]
fn all_modes_use_complete_footprints_and_keep_frames_and_text_together() -> Result<()> {
    let mut config = config()?;
    let mut inputs = vec![item(22.0)?, item(40.0)?, item(25.0)?];
    inputs[1].heading = Some(StampTextProofSettings::new(
        "Heading",
        "HN",
        10.0,
        NamedColour::parse("black")?,
    ));
    for mode in [
        RowAlignment::Left,
        RowAlignment::Center,
        RowAlignment::Right,
        RowAlignment::Justified,
    ] {
        config.page_settings.set_row_alignment(mode);
        let pages = paginate_stamp_items(&config.page_settings, &inputs, &policy()?, None)?;
        assert_eq!(pages.len(), 1);
        let page = &pages[0];
        let b = page.layout.content_bounds;
        let row = &page.items;
        assert_eq!(row.len(), 3);
        let start = row[0].footprint.left - b.left;
        let end = b.right - row[2].footprint.right;
        let gap = row[1].footprint.left - row[0].footprint.right;
        let gap2 = row[2].footprint.left - row[1].footprint.right;
        close(gap, gap2);
        match mode {
            RowAlignment::Left => {
                close(start, 0.0);
                close(gap, config.page_settings.spacing().horizontal_mm());
            }
            RowAlignment::Right => {
                close(end, 0.0);
                close(gap, config.page_settings.spacing().horizontal_mm());
            }
            RowAlignment::Center => {
                close(start, end);
                close(start, gap);
            }
            RowAlignment::Justified => {
                close(start, 0.0);
                close(end, 0.0);
            }
        }
        for (i, f) in row.iter().enumerate() {
            assert_eq!(f.source_index, i);
            let g = f.geometry.ok_or("geometry")?;
            close(
                g.physical_bounds().right - g.physical_bounds().left,
                inputs[i].geometry.physical_bounds().right
                    - inputs[i].geometry.physical_bounds().left,
            );
            close(
                (g.frame_bounds().left + g.frame_bounds().right) / 2.0,
                (f.footprint.left + f.footprint.right) / 2.0,
            );
            if let Some(text) = &f.heading {
                close(
                    (text.bounds.left + text.bounds.right) / 2.0,
                    (f.footprint.left + f.footprint.right) / 2.0,
                );
            }
        }
    }
    Ok(())
}

#[test]
fn sections_start_rows_and_apply_to_last_rows_and_text_continuations() -> Result<()> {
    let config = config()?;
    let mut items = vec![item(22.0)?; 7];
    items[6].caption = Some(StampTextProofSettings::new(
        (0..160)
            .map(|i| format!("Line {i:03}"))
            .collect::<Vec<_>>()
            .join("\n"),
        "HN",
        10.0,
        NamedColour::parse("black")?,
    ));
    let sections = [
        StampRowSection {
            item_count: 2,
            alignment: RowAlignment::Right,
        },
        StampRowSection {
            item_count: 4,
            alignment: RowAlignment::Justified,
        },
        StampRowSection {
            item_count: 1,
            alignment: RowAlignment::Center,
        },
    ];
    let pages =
        paginate_stamp_sections(&config.page_settings, &items, &sections, &policy()?, None)?;
    let first = &pages[0];
    assert!(first.items[2].footprint.top < first.items[0].footprint.bottom);
    close(
        first.items[1].footprint.right,
        first.layout.content_bounds.right,
    );
    close(
        first.items[2].footprint.left,
        first.layout.content_bounds.left,
    );
    close(
        first.items[5].footprint.right,
        first.layout.content_bounds.right,
    );
    for p in &pages {
        for f in p.items.iter().filter(|f| f.source_index == 6) {
            close(
                f.footprint.left - p.layout.content_bounds.left,
                p.layout.content_bounds.right - f.footprint.right,
            );
        }
    }
    Ok(())
}

#[test]
fn singleton_justified_is_centered_and_near_full_rows_keep_minimum_gaps() -> Result<()> {
    let mut config = config()?;
    config
        .page_settings
        .set_row_alignment(RowAlignment::Justified);
    let pages = paginate_stamp_items(&config.page_settings, &[item(22.0)?], &policy()?, None)?;
    let p = &pages[0];
    let f = &p.items[0].footprint;
    close(
        f.left - p.layout.content_bounds.left,
        p.layout.content_bounds.right - f.right,
    );
    config.page_settings.set_row_alignment(RowAlignment::Center);
    let b = config
        .page_settings
        .resolve_with_context(1, 1, None)?
        .content_bounds;
    let width = (b.right - b.left - config.page_settings.spacing().horizontal_mm()) / 2.0 - 10.0;
    let pages = paginate_stamp_items(
        &config.page_settings,
        &[item(width)?, item(width)?],
        &policy()?,
        None,
    )?;
    let row = &pages[0].items;
    close(
        row[1].footprint.left - row[0].footprint.right,
        config.page_settings.spacing().horizontal_mm(),
    );
    close(row[0].footprint.left, b.left);
    Ok(())
}

#[test]
fn defaults_config_and_section_validation_are_explicit() -> Result<()> {
    let config = config()?;
    assert_eq!(config.page_settings.row_alignment(), RowAlignment::Center);
    for mode in ["left", "center", "right", "justified"] {
        let mut v: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
        v["native_pdf"]["rows"]["alignment"] = toml::Value::String(mode.into());
        let cfg = NativePdfConfig::from_toml(&toml::to_string(&v)?)?;
        assert_eq!(
            cfg.page_settings.row_alignment(),
            RowAlignment::parse(mode)?
        );
        assert!(!cfg
            .unapplied_keys
            .iter()
            .any(|k| k.contains("native_pdf.rows.alignment")));
    }
    assert!(RowAlignment::parse("justify_last_left").is_err());
    let mut source: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
    source["native_pdf"]
        .as_table_mut()
        .ok_or("native table")?
        .remove("rows");
    assert_eq!(
        NativePdfConfig::from_toml(&toml::to_string(&source)?)?
            .page_settings
            .row_alignment(),
        RowAlignment::Center
    );
    for bad in [
        toml::Value::String("justify_last_left".into()),
        toml::Value::Integer(3),
    ] {
        let mut source: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
        source["native_pdf"]["rows"]["alignment"] = bad;
        assert!(NativePdfConfig::from_toml(&toml::to_string(&source)?).is_err());
    }
    let mut source: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
    source["native_pdf"]["rows"]
        .as_table_mut()
        .ok_or("rows")?
        .remove("alignment");
    assert!(NativePdfConfig::from_toml(&toml::to_string(&source)?).is_err());
    assert!(
        paginate_stamp_sections(&config.page_settings, &[item(22.0)?], &[], &policy()?, None)
            .is_err()
    );
    assert!(paginate_stamp_sections(
        &config.page_settings,
        &[],
        &[StampRowSection {
            item_count: 0,
            alignment: RowAlignment::Left
        }],
        &policy()?,
        None
    )
    .is_err());
    Ok(())
}

#[test]
fn each_mode_applies_to_every_continuation_and_even_page_width() -> Result<()> {
    let mut value: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
    value["page"].as_table_mut().ok_or("page")?.insert(
        "even_margins".into(),
        toml::Value::Table(toml::map::Map::from_iter([
            ("left_mm".into(), toml::Value::Float(35.0)),
            ("right_mm".into(), toml::Value::Float(25.0)),
            ("top_mm".into(), toml::Value::Float(15.0)),
            ("bottom_mm".into(), toml::Value::Float(15.0)),
        ])),
    );
    let mut cfg = NativePdfConfig::from_toml(&toml::to_string(&value)?)?;
    let mut stamp = item(40.0)?;
    stamp.caption = Some(StampTextProofSettings::new(
        (0..160)
            .map(|i| format!("Line {i:03}"))
            .collect::<Vec<_>>()
            .join("\n"),
        "HN",
        10.0,
        NamedColour::parse("black")?,
    ));
    for mode in [
        RowAlignment::Left,
        RowAlignment::Center,
        RowAlignment::Right,
        RowAlignment::Justified,
    ] {
        cfg.page_settings.set_row_alignment(mode);
        let pages = paginate_stamp_items(
            &cfg.page_settings,
            std::slice::from_ref(&stamp),
            &policy()?,
            None,
        )?;
        assert!(pages.len() > 2);
        for page in &pages {
            let b = page.layout.content_bounds;
            for f in &page.items {
                match mode {
                    RowAlignment::Left => close(f.footprint.left, b.left),
                    RowAlignment::Right => close(f.footprint.right, b.right),
                    _ => close(f.footprint.left - b.left, b.right - f.footprint.right),
                }
                for block in f.heading.iter().chain(&f.caption) {
                    for line in &block.lines {
                        close(
                            line.left_mm + line.width_mm / 2.0,
                            (f.footprint.left + f.footprint.right) / 2.0,
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
