use stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    config::NativePdfConfig,
    document::Bounds,
    pagination::paginate_stamp_items,
    stamp_box::StampBoxGeometry,
    stamp_image::StampTextProofSettings,
    stamp_text::{ExpandedWidthLimit, StampTextItem, StampTextPolicy, TextWidthPolicy},
};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn config() -> Result<NativePdfConfig> {
    let mut value: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
    value["page"]["album_page_width_mm"] = toml::Value::Float(215.9);
    value["page"]["album_page_height_mm"] = toml::Value::Float(279.4);
    Ok(NativePdfConfig::from_toml(&toml::to_string(&value)?)?)
}

fn policy() -> Result<StampTextPolicy> {
    Ok(StampTextPolicy::new(
        TextWidthPolicy::ExpandFootprint,
        ExpandedWidthLimit::new("percent", 120.0)?,
        2.0,
        1.0,
        3.0,
    )?)
}

fn item(width: f64, height: f64) -> Result<StampTextItem> {
    Ok(StampTextItem {
        geometry: StampBoxGeometry::from_total_additions(
            Bounds {
                left: 0.0,
                bottom: 0.0,
                right: width,
                top: height,
            },
            6.0,
            6.0,
            0.0,
        )?,
        heading: None,
        caption: None,
    })
}

fn text(value: impl Into<String>) -> Result<StampTextProofSettings> {
    Ok(StampTextProofSettings::new(
        value,
        "HN",
        10.0,
        NamedColour::parse("black")?,
    ))
}

#[test]
fn pagination_preserves_order_dimensions_and_fits_each_resolved_page() -> Result<()> {
    let config = config()?;
    let items = vec![item(40.0, 35.0)?; 40];
    let pages = paginate_stamp_items(&config.page_settings, &items, &policy()?, None)?;
    assert!(pages.len() > 1);
    let mut indexes = Vec::new();
    for (number, page) in pages.iter().enumerate() {
        let resolved = config
            .page_settings
            .resolve_with_context(number + 1, pages.len(), None)?;
        assert_eq!(
            page.layout.content_bounds.left,
            resolved.content_bounds.left
        );
        for fragment in &page.items {
            indexes.push(fragment.source_index);
            let b = fragment.footprint;
            assert!(
                b.left >= resolved.content_bounds.left && b.right <= resolved.content_bounds.right
            );
            assert!(
                b.bottom >= resolved.content_bounds.bottom && b.top <= resolved.content_bounds.top
            );
            let physical = fragment.geometry.ok_or("missing stamp")?.physical_bounds();
            assert!((physical.right - physical.left - 40.0).abs() < 1e-9);
            assert!((physical.top - physical.bottom - 35.0).abs() < 1e-9);
        }
    }
    assert_eq!(indexes, (0..40).collect::<Vec<_>>());
    Ok(())
}

#[test]
fn oversized_caption_continues_without_line_loss_or_repeating_stamp() -> Result<()> {
    let config = config()?;
    let mut stamp = item(40.0, 35.0)?;
    stamp.caption = Some(text(
        (0..180)
            .map(|i| format!("Line {i:03}"))
            .collect::<Vec<_>>()
            .join("\n"),
    )?);
    let pages = paginate_stamp_items(&config.page_settings, &[stamp], &policy()?, None)?;
    assert!(pages.len() >= 3);
    let fragments: Vec<_> = pages.iter().flat_map(|p| &p.items).collect();
    assert_eq!(fragments.iter().filter(|f| f.geometry.is_some()).count(), 1);
    let lines: Vec<_> = fragments
        .iter()
        .flat_map(|f| f.caption.iter())
        .flat_map(|t| &t.lines)
        .map(|l| l.text.clone())
        .collect();
    assert_eq!(
        lines,
        (0..180).map(|i| format!("Line {i:03}")).collect::<Vec<_>>()
    );
    for page in &pages {
        for fragment in &page.items {
            assert!(fragment.footprint.bottom >= page.layout.content_bounds.bottom);
            for block in fragment.heading.iter().chain(&fragment.caption) {
                assert_eq!(block.measured.size_pt, 10.0);
                assert!(block.bounds.bottom >= fragment.footprint.bottom);
            }
        }
    }
    Ok(())
}

#[test]
fn long_heading_blank_lines_and_caption_preserve_sequence() -> Result<()> {
    let config = config()?;
    let mut stamp = item(40.0, 35.0)?;
    let heading = format!("Heading\n{}Last heading", "\n".repeat(130));
    stamp.heading = Some(text(&heading)?);
    stamp.caption = Some(text("After the stamp")?);
    let pages = paginate_stamp_items(&config.page_settings, &[stamp], &policy()?, None)?;
    let fragments: Vec<_> = pages.iter().flat_map(|p| &p.items).collect();
    let lines: Vec<_> = fragments
        .iter()
        .flat_map(|f| f.heading.iter())
        .flat_map(|t| &t.lines)
        .map(|l| l.text.as_str())
        .collect();
    assert_eq!(lines, heading.split('\n').collect::<Vec<_>>());
    assert_eq!(fragments.iter().filter(|f| f.geometry.is_some()).count(), 1);
    assert_eq!(
        fragments
            .last()
            .ok_or("fragment")?
            .caption
            .as_ref()
            .ok_or("caption")?
            .lines[0]
            .text,
        "After the stamp"
    );
    Ok(())
}

#[test]
fn indivisible_frame_and_overwide_item_error_and_empty_input_is_one_page() -> Result<()> {
    let config = config()?;
    assert!(
        paginate_stamp_items(
            &config.page_settings,
            &[item(40.0, 400.0)?],
            &policy()?,
            None
        )
        .err()
        .ok_or("expected error")?
        .contains("item 1")
    );
    assert!(
        paginate_stamp_items(
            &config.page_settings,
            &[item(400.0, 40.0)?],
            &policy()?,
            None
        )
        .is_err()
    );
    let pages = paginate_stamp_items(&config.page_settings, &[], &policy()?, None)?;
    assert_eq!(pages.len(), 1);
    assert!(pages[0].items.is_empty());
    Ok(())
}

#[test]
fn fresh_even_pages_repack_pending_items_and_use_actual_total_in_numbering() -> Result<()> {
    let mut value: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
    value["page"]["album_page_width_mm"] = toml::Value::Float(215.9);
    value["page"]["album_page_height_mm"] = toml::Value::Float(279.4);
    value["page"].as_table_mut().ok_or("page table")?.insert(
        "even_margins".into(),
        toml::Value::Table(toml::map::Map::from_iter([
            ("left_mm".into(), toml::Value::Float(45.0)),
            ("right_mm".into(), toml::Value::Float(45.0)),
            ("top_mm".into(), toml::Value::Float(25.0)),
            ("bottom_mm".into(), toml::Value::Float(25.0)),
        ])),
    );
    value["native_pdf"]["footer_number"]["enabled"] = toml::Value::Boolean(true);
    let config = NativePdfConfig::from_toml(&toml::to_string(&value)?)?;
    let items = vec![item(40.0, 75.0)?; 80];
    let pages = paginate_stamp_items(&config.page_settings, &items, &policy()?, None)?;
    assert!(pages.len() >= 10);
    for (index, page) in pages.iter().enumerate() {
        assert_eq!(
            page.layout.footer_number.as_ref().ok_or("number")?.lines[0].text,
            format!("Page {} of {}", index + 1, pages.len())
        );
    }
    let first_row_count = |page: &stamp_album_gen2::native_pdf::pagination::StampItemPage| {
        let top = page.items[0].footprint.top;
        page.items
            .iter()
            .take_while(|i| i.footprint.top == top)
            .count()
    };
    assert_eq!(first_row_count(&pages[0]), 3);
    assert_eq!(first_row_count(&pages[1]), 2);
    let indexes: Vec<_> = pages
        .iter()
        .flat_map(|p| &p.items)
        .map(|i| i.source_index)
        .collect();
    assert_eq!(indexes, (0..80).collect::<Vec<_>>());
    Ok(())
}

#[test]
fn exact_page_fit_needs_no_trailing_gap_and_one_more_row_creates_new_page() -> Result<()> {
    let config = config()?;
    let bounds = config
        .page_settings
        .resolve_with_context(1, 1, None)?
        .content_bounds;
    let policy = StampTextPolicy::new(
        TextWidthPolicy::WrapToFrame,
        ExpandedWidthLimit::new("percent", 120.0)?,
        0.0,
        0.0,
        0.0,
    )?;
    let exact = item(
        bounds.right - bounds.left - 6.0,
        bounds.top - bounds.bottom - 6.0,
    )?;
    let pages = paginate_stamp_items(&config.page_settings, &[exact.clone()], &policy, None)?;
    assert_eq!(pages.len(), 1);
    assert!((pages[0].items[0].footprint.bottom - bounds.bottom).abs() < 1e-9);
    assert_eq!(
        paginate_stamp_items(
            &config.page_settings,
            &[exact.clone(), exact],
            &policy,
            None
        )?
        .len(),
        2
    );
    Ok(())
}
