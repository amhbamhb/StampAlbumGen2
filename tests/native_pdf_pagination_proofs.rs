use oxidize_pdf::parser::PdfDocument;
use serde::Deserialize;
use stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    commands::page::album_pages_title::AlbumPagesTitle,
    config::NativePdfConfig,
    document::Bounds,
    pagination::{StampItemPage, StampRowSection},
    renderer::{
        write_paginated_stamp_items_geometry_proof, write_paginated_stamp_sections_geometry_proof,
    },
    stamp_box::StampBoxGeometry,
    stamp_image::{StampImageRowItem, StampTextProofSettings},
    stamp_rows::RowAlignment,
    stamp_text::{ExpandedWidthLimit, StampTextItem, StampTextPolicy, TextWidthPolicy},
    units::{mm_to_points, points_to_mm},
};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Deserialize)]
struct Sources {
    records: Vec<Record>,
}

#[derive(Deserialize)]
struct Record {
    record_id: i64,
    year: i64,
    sg_number: String,
    width_mm: f64,
    height_mm: f64,
    image_file: String,
}

fn directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/stamp-pagination-v1")
}

fn sources() -> Result<Sources> {
    Ok(serde_json::from_str(&fs::read_to_string(
        directory().join("source-records.json"),
    )?)?)
}

fn inputs(sources: &Sources) -> Result<Vec<StampImageRowItem>> {
    let mut result = Vec::new();
    for record in &sources.records {
        let image_path = directory().join(&record.image_file);
        let (pw, ph) = image::image_dimensions(&image_path)?;
        let frame_w = record.width_mm + 6.0;
        let frame_h = record.height_mm + 6.0;
        let black = NamedColour::parse("black")?;
        result.push(StampImageRowItem {
            frame_style: None,
            labels: None,
            stamp: StampTextItem {
                geometry: StampBoxGeometry::from_total_additions(
                    Bounds { left: 0.0, bottom: 0.0, right: record.width_mm, top: record.height_mm },
                    6.0, 6.0, 0.0,
                )?,
                heading: Some(StampTextProofSettings::new(format!("US {} / SG {}", record.year, record.sg_number), "HB", 10.0, black)),
                caption: Some(StampTextProofSettings::new(
                    format!("DB ID {}\nPhysical {} x {} mm\nFrame {frame_w} x {frame_h} mm\n3 mm clearance each edge",
                        record.record_id, record.width_mm, record.height_mm), "HN", 10.0, black)),
            },
            image_path,
            image_scale_pt_per_pixel: mm_to_points(
                (record.width_mm / f64::from(pw)).max(record.height_mm / f64::from(ph)),
            ),
            options: Default::default(),
        });
    }
    Ok(result)
}

fn config(name: &str) -> Result<NativePdfConfig> {
    let mut source: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
    source["page"]["album_page_width_mm"] = toml::Value::Float(215.9);
    source["page"]["album_page_height_mm"] = toml::Value::Float(279.4);
    source["footer"]["album_pages_footer_number_size"] = toml::Value::Float(10.0);
    source["native_pdf"]["footer_number"]["enabled"] = toml::Value::Boolean(true);
    let mut config = NativePdfConfig::from_toml(&toml::to_string(&source)?)?;
    config
        .page_settings
        .set_row_alignment(stamp_album_gen2::native_pdf::stamp_rows::RowAlignment::Left);
    config.proof.guide_enabled = false;
    config.page_settings.set_title(AlbumPagesTitle::new("HN", 10.0,
        &format!("{name}: real US stamps / recorded Colnect sizes\nBlue = physical / black = mounting frame / green = text footprint\nUS Letter / print Actual size 100% / no Fit / frame adds 6 mm total"),
        4.0, 0.0, NamedColour::parse("black")?)?);
    Ok(config)
}

fn policy(mode: TextWidthPolicy) -> Result<StampTextPolicy> {
    Ok(StampTextPolicy::new(
        mode,
        ExpandedWidthLimit::new("percent", 120.0)?,
        2.0,
        1.0,
        3.0,
    )?)
}

fn image_bounds(item: &StampImageRowItem, physical: Bounds) -> Result<Bounds> {
    let (pw, ph) = image::image_dimensions(&item.image_path)?;
    let w = points_to_mm(f64::from(pw) * item.image_scale_pt_per_pixel);
    let h = points_to_mm(f64::from(ph) * item.image_scale_pt_per_pixel);
    let scale = ((physical.right - physical.left) / w)
        .min((physical.top - physical.bottom) / h)
        .min(1.0);
    let (w, h) = (w * scale, h * scale);
    Ok(Bounds {
        left: (physical.left + physical.right - w) / 2.0,
        right: (physical.left + physical.right + w) / 2.0,
        bottom: (physical.bottom + physical.top - h) / 2.0,
        top: (physical.bottom + physical.top + h) / 2.0,
    })
}

#[test]
fn real_multi_page_pdf_has_each_image_once_correct_matrices_and_final_numbering() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let sources = sources()?;
    let inputs = inputs(&sources)?;
    let config = config("Pagination regression")?;
    let output = temp.path().join("real.pdf");
    let pages = write_paginated_stamp_items_geometry_proof(
        &config.page_settings,
        &config.proof,
        &inputs,
        &policy(TextWidthPolicy::ExpandFootprint)?,
        &output,
    )?;
    assert!(pages.len() > 1);
    let pdf = PdfDocument::open(&output)?;
    assert_eq!(pdf.page_count()?, u32::try_from(pages.len())?);
    let text = pdf.extract_text()?;
    let mut total_images = 0;
    for (index, placed_page) in pages.iter().enumerate() {
        let page = pdf.get_page(u32::try_from(index)?)?;
        assert!((points_to_mm(page.media_box[2]) - 215.9).abs() < 0.002);
        assert!((points_to_mm(page.media_box[3]) - 279.4).abs() < 0.002);
        assert!(text[index]
            .text
            .contains(&format!("Page {} of {}", index + 1, pages.len())));
        let mut matrices = Vec::new();
        for stream in pdf.get_page_content_streams(&page)? {
            let text = std::str::from_utf8(&stream)?;
            let tokens: Vec<_> = text.split_whitespace().collect();
            for window in tokens.windows(9) {
                if window[6] == "cm"
                    && window[7].starts_with("/native_stamp_image")
                    && window[8] == "Do"
                {
                    matrices.push(
                        window[..6]
                            .iter()
                            .map(|v| v.parse::<f64>())
                            .collect::<std::result::Result<Vec<_>, _>>()?,
                    );
                }
            }
        }
        let placed: Vec<_> = placed_page
            .items
            .iter()
            .filter(|f| f.geometry.is_some())
            .collect();
        assert_eq!(matrices.len(), placed.len());
        total_images += matrices.len();
        for (matrix, fragment) in matrices.iter().zip(placed) {
            let physical = fragment.geometry.ok_or("geometry")?.physical_bounds();
            let expected = image_bounds(&inputs[fragment.source_index], physical)?;
            for (value, correct) in [
                (matrix[0], expected.right - expected.left),
                (matrix[3], expected.top - expected.bottom),
                (matrix[4], expected.left),
                (matrix[5], expected.bottom),
            ] {
                assert!((points_to_mm(value) - correct).abs() < 0.002);
            }
        }
    }
    assert_eq!(total_images, sources.records.len());
    Ok(())
}

#[test]
fn invalid_last_page_asset_preserves_output_and_line_continuations_render() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let mut inputs = inputs(&sources()?)?;
    let config = config("Continuation regression")?;
    let policy = policy(TextWidthPolicy::ExpandFootprint)?;
    let output = temp.path().join("existing.pdf");
    fs::write(&output, b"unchanged")?;
    inputs.last_mut().ok_or("last")?.image_path = temp.path().join("missing.png");
    assert!(write_paginated_stamp_items_geometry_proof(
        &config.page_settings,
        &config.proof,
        &inputs,
        &policy,
        &output
    )
    .is_err());
    assert_eq!(fs::read(&output)?, b"unchanged");
    let fresh = temp.path().join("new.pdf");
    assert!(write_paginated_stamp_items_geometry_proof(
        &config.page_settings,
        &config.proof,
        &inputs,
        &policy,
        &fresh
    )
    .is_err());
    assert!(!fresh.exists());
    let mut inputs = inputs_from_first()?;
    inputs[0].stamp.caption = Some(StampTextProofSettings::new(
        (0..180)
            .map(|i| format!("Line {i:03}"))
            .collect::<Vec<_>>()
            .join("\n"),
        "HN",
        10.0,
        NamedColour::parse("black")?,
    ));
    let pages = write_paginated_stamp_items_geometry_proof(
        &config.page_settings,
        &config.proof,
        &inputs,
        &policy,
        &output,
    )?;
    let pdf = PdfDocument::open(&output)?;
    assert_eq!(pdf.page_count()?, u32::try_from(pages.len())?);
    let text = pdf.extract_text()?;
    let all_text = text
        .iter()
        .map(|p| p.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for i in 0..180 {
        assert_eq!(all_text.matches(&format!("Line {i:03}")).count(), 1);
    }
    Ok(())
}

fn inputs_from_first() -> Result<Vec<StampImageRowItem>> {
    Ok(inputs(&sources()?)?.into_iter().take(1).collect())
}

fn bounds(b: Bounds) -> [f64; 4] {
    [b.left, b.bottom, b.right, b.top]
}

fn ledger(
    pages: &[StampItemPage],
    inputs: &[StampImageRowItem],
    sources: &Sources,
) -> Result<serde_json::Value> {
    let mut result = Vec::new();
    for (index, page) in pages.iter().enumerate() {
        let mut fragments = Vec::new();
        for fragment in &page.items {
            let source = &sources.records[fragment.source_index];
            let geometry = fragment.geometry;
            let illustration = geometry
                .map(|g| image_bounds(&inputs[fragment.source_index], g.physical_bounds()))
                .transpose()?;
            let blocks: Vec<_> = [
                ("heading", fragment.heading.as_ref()),
                ("caption", fragment.caption.as_ref()),
            ]
            .into_iter()
            .filter_map(|(kind, text)| {
                text.map(|t| {
                    serde_json::json!({
                        "kind":kind, "bounds_mm":bounds(t.bounds), "size_pt":t.measured.size_pt,
                        "lines": t.lines.iter().map(|l| serde_json::json!({
                            "text":l.text,"left_mm":l.left_mm,"baseline_mm":l.baseline_mm
                        })).collect::<Vec<_>>()
                    })
                })
            })
            .collect();
            fragments.push(serde_json::json!({
                "source_index":fragment.source_index,"record_id":source.record_id,
                "image_file":source.image_file,"footprint_mm":bounds(fragment.footprint),
                "physical_mm":geometry.map(|g|bounds(g.physical_bounds())),
                "frame_mm":geometry.map(|g|bounds(g.frame_bounds())),
                "image_bounds_mm":illustration.map(bounds),"text":blocks,
                "text_only_continuation":geometry.is_none()
            }));
        }
        result.push(serde_json::json!({"page":index+1,"content_bounds_mm":bounds(page.layout.content_bounds),"items":fragments}));
    }
    Ok(serde_json::json!(result))
}

#[test]
#[ignore = "Regenerates persistent real-stamp US Letter PDF/TXT pagination proofs; run explicitly"]
fn regenerate_real_letter_pagination_proofs() -> Result<()> {
    let sources = sources()?;
    let all = inputs(&sources)?;
    let mut cases = Vec::new();
    for (name, mode, long) in [
        ("real-stamps-wrapped", TextWidthPolicy::WrapToFrame, false),
        (
            "real-stamps-expanded",
            TextWidthPolicy::ExpandFootprint,
            false,
        ),
        ("overflow-caption", TextWidthPolicy::ExpandFootprint, true),
    ] {
        let config = config(name)?;
        let policy = policy(mode)?;
        let mut inputs = if long { all[..1].to_vec() } else { all.clone() };
        if long {
            let caption = inputs[0].stamp.caption.as_mut().ok_or("caption")?;
            caption.text.push_str(&format!(
                "\n{}",
                (0..180)
                    .map(|i| format!("Line {i:03}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }

        let pages = write_paginated_stamp_items_geometry_proof(
            &config.page_settings,
            &config.proof,
            &inputs,
            &policy,
            &directory().join(format!("{name}.pdf")),
        )?;
        let mut txt = String::from("# COMPATIBILITY ADAPTATION: separate image/text blocks, NOT native position/page parity.\n# Full source text retained; native footprint/continuation policy is not an AlbumEasy command.\nALBUM_PAGES_SIZE (215.9 279.4)\nALBUM_PAGES_MARGINS (12 12 12 12)\nALBUM_PAGES_SPACING (3 3)\nALBUM_PAGES_BORDER (0.2 0 0 0)\nSTAMP_BOXES_SIZE_ADJUST (0)\n");
        for (i, input) in inputs.iter().enumerate() {
            let source = &sources.records[i];
            txt.push_str(&format!("PAGE_START\n# US DB ID {} / SG {} / recorded Colnect {} x {} mm\nROW_START_ES (HN 10 0.1)\nSTAMP_ADD_IMG ({} {} \"{}\" \"\" \"\" \"\")\n",
                source.record_id,source.sg_number,source.width_mm,source.height_mm,
                source.width_mm+6.0,source.height_mm+6.0,source.image_file));
            for text in input.stamp.heading.iter().chain(&input.stamp.caption) {
                let quoted = text
                    .text
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n");
                txt.push_str(&format!("PAGE_TEXT (HN 10 \"{quoted}\" 2)\n"));
            }
        }
        fs::write(directory().join(format!("{name}.txt")), txt)?;
        cases.push(serde_json::json!({"name":name,"page_count":pages.len(),"pages":ledger(&pages,&inputs,&sources)?}));
    }
    fs::write(
        directory().join("expected.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "reference":"Native calculated geometry; TXT is an unrendered compatibility adaptation",
            "page_size_mm":[215.9,279.4],"tolerance_mm":0.004,"cases":cases
        }))? + "\n",
    )?;
    Ok(())
}

#[test]
fn all_alignment_modes_render_images_and_text_at_their_resolved_positions() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let all = inputs(&sources()?)?;
    for mode in [
        RowAlignment::Left,
        RowAlignment::Center,
        RowAlignment::Right,
        RowAlignment::Justified,
    ] {
        let mut config = config("Alignment regression")?;
        config.page_settings.set_row_alignment(mode);
        let output = temp.path().join(format!("{mode:?}.pdf"));
        let pages = write_paginated_stamp_items_geometry_proof(
            &config.page_settings,
            &config.proof,
            &all[..5],
            &policy(TextWidthPolicy::ExpandFootprint)?,
            &output,
        )?;
        let pdf = PdfDocument::open(&output)?;
        let extracted = pdf.extract_text()?;
        for (page_index, planned) in pages.iter().enumerate() {
            let page = pdf.get_page(u32::try_from(page_index)?)?;
            let mut matrices = Vec::new();
            for stream in pdf.get_page_content_streams(&page)? {
                let content = std::str::from_utf8(&stream)?;
                let tokens: Vec<_> = content.split_whitespace().collect();
                for w in tokens.windows(9) {
                    if w[6] == "cm" && w[7].starts_with("/native_stamp_image") && w[8] == "Do" {
                        matrices.push(
                            w[..6]
                                .iter()
                                .map(|n| n.parse::<f64>())
                                .collect::<std::result::Result<Vec<_>, _>>()?,
                        );
                    }
                }
            }
            assert_eq!(matrices.len(), planned.items.len());
            for (matrix, fragment) in matrices.iter().zip(&planned.items) {
                let expected = image_bounds(
                    &all[fragment.source_index],
                    fragment.geometry.ok_or("geometry")?.physical_bounds(),
                )?;
                for (value, correct) in [
                    (matrix[0], expected.right - expected.left),
                    (matrix[3], expected.top - expected.bottom),
                    (matrix[4], expected.left),
                    (matrix[5], expected.bottom),
                ] {
                    assert!((points_to_mm(value) - correct).abs() < 0.002);
                }
                for text in fragment.heading.iter().chain(&fragment.caption) {
                    for line in &text.lines {
                        assert!(extracted[page_index].text.contains(&line.text));
                    }
                }
            }
        }
    }
    Ok(())
}

#[test]
#[ignore = "Regenerates real US Letter row-alignment PDF/TXT proofs; run explicitly"]
fn regenerate_real_letter_alignment_proofs() -> Result<()> {
    let output =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/row-alignment-v1");
    fs::create_dir_all(&output)?;
    let sources = sources()?;
    let all = inputs(&sources)?;
    let mut cases = Vec::new();
    for (name, mode) in [
        ("left", RowAlignment::Left),
        ("center", RowAlignment::Center),
        ("right", RowAlignment::Right),
        ("justified", RowAlignment::Justified),
        ("mixed-sections", RowAlignment::Center),
        ("center-continuation", RowAlignment::Center),
    ] {
        let mut config = config(name)?;
        config.page_settings.set_row_alignment(mode);
        let mut items = all[..5].to_vec();
        let sections = if name == "mixed-sections" {
            vec![
                StampRowSection {
                    item_count: 2,
                    alignment: RowAlignment::Justified,
                },
                StampRowSection {
                    item_count: 2,
                    alignment: RowAlignment::Left,
                },
                StampRowSection {
                    item_count: 1,
                    alignment: RowAlignment::Right,
                },
            ]
        } else if name == "center-continuation" {
            items.truncate(1);
            let caption = items[0].stamp.caption.as_mut().ok_or("caption")?;
            caption.text.push_str(&format!(
                "\n{}",
                (0..130)
                    .map(|i| format!("Line {i:03}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
            vec![StampRowSection {
                item_count: 1,
                alignment: mode,
            }]
        } else {
            vec![StampRowSection {
                item_count: items.len(),
                alignment: mode,
            }]
        };
        let pages = write_paginated_stamp_sections_geometry_proof(
            &config.page_settings,
            &config.proof,
            &items,
            &sections,
            &policy(TextWidthPolicy::ExpandFootprint)?,
            &output.join(format!("{name}.pdf")),
        )?;
        let mut txt=String::from("# COMPATIBILITY ADAPTATION: full images/text retained in separate blocks; NOT native row/page/position parity.\n# Native alignment and section settings are in companion TOML; not AlbumEasy commands.\nALBUM_PAGES_SIZE (215.9 279.4)\nALBUM_PAGES_MARGINS (12 12 12 12)\nALBUM_PAGES_SPACING (3 3)\nALBUM_PAGES_BORDER (0.2 0 0 0)\nSTAMP_BOXES_SIZE_ADJUST (0)\n");
        for (i, item) in items.iter().enumerate() {
            let record = &sources.records[i];
            txt.push_str(&format!("PAGE_START\n# DB ID {} / SG {}\nROW_START_ES (HN 10 0.1)\nSTAMP_ADD_IMG ({} {} \"../stamp-pagination-v1/{}\" \"\" \"\" \"\")\n",
                        record.record_id,record.sg_number,record.width_mm+6.0,record.height_mm+6.0,record.image_file));
            for block in item.stamp.heading.iter().chain(&item.stamp.caption) {
                let escaped = block
                    .text
                    .replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace('\n', "\\n");
                txt.push_str(&format!("PAGE_TEXT (HN 10 \"{escaped}\" 2)\n"));
            }
        }
        fs::write(output.join(format!("{name}.txt")), txt)?;
        let mut toml=format!("# Native override fragment; merge into a copy of config_template.toml.\n# Section-specific alignment shown below is fixture/API metadata, not imported commands.\n[native_pdf.rows]\nalignment = \"{}\"\n",if name.starts_with("center") || name=="mixed-sections" {"center"} else {name});
        for section in &sections {
            toml.push_str(&format!(
                "# Section: {} consecutive items, {:?}\n",
                section.item_count, section.alignment
            ));
        }
        fs::write(output.join(format!("{name}.toml")), toml)?;
        cases.push(serde_json::json!({"name":name,"page_count":pages.len(),
                    "sections":sections.iter().map(|s|serde_json::json!({"item_count":s.item_count,"alignment":format!("{:?}",s.alignment)})).collect::<Vec<_>>(),
                    "pages":ledger(&pages,&items,&sources)?}));
    }
    fs::write(
        output.join("expected.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "page_size_mm":[215.9,279.4],"tolerance_mm":0.004,
            "source_manifest":"../stamp-pagination-v1/source-records.json",
            "image_base":"../stamp-pagination-v1","cases":cases
        }))? + "\n",
    )?;
    Ok(())
}
