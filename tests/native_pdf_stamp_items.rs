use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    config::NativePdfConfig,
    document::Bounds,
    renderer::write_stamp_items_geometry_proof,
    stamp_box::StampBoxGeometry,
    stamp_image::{StampImageRowItem, StampTextProofSettings},
    stamp_text::{
        fit_text_rows, ExpandedWidthLimit, StampTextItem, StampTextPolicy, TextWidthPolicy,
    },
    units::{mm_to_points, points_to_mm},
};
use std::{error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn config() -> Result<NativePdfConfig> {
    let mut source: toml::Value = toml::from_str(include_str!("../config_template.toml"))?;
    source["page"]["album_page_width_mm"] = toml::Value::Float(215.9);
    source["page"]["album_page_height_mm"] = toml::Value::Float(279.4);
    let mut config = NativePdfConfig::from_toml(&toml::to_string(&source)?)?;
    config.proof.page_count = 1;
    config.proof.guide_enabled = false;
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

fn items(directory: &Path) -> Result<Vec<StampImageRowItem>> {
    let mut items = Vec::new();
    for (index, (width, height, pixels_w, pixels_h, scale)) in [
        (22.0, 25.0, 220, 250, 0.05),
        (22.0, 25.0, 220, 250, 0.2),
        (40.0, 25.0, 220, 250, 0.2),
        (25.0, 40.0, 300, 150, 0.2),
    ]
    .into_iter()
    .enumerate()
    {
        let path = directory.join(format!("image-{index}.png"));
        image::RgbImage::from_pixel(
            pixels_w,
            pixels_h,
            image::Rgb([40 + index as u8 * 30, 100, 150]),
        )
        .save(&path)?;
        items.push(StampImageRowItem {
            stamp: StampTextItem {
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
                heading: Some(StampTextProofSettings::new(
                    format!("Stamp {}", index + 1),
                    "HB",
                    10.0,
                    NamedColour::parse("black")?,
                )),
                caption: Some(StampTextProofSettings::new(
                    format!("Known {width} x {height} mm\nA deliberately long caption"),
                    "HN",
                    10.0,
                    NamedColour::parse("black")?,
                )),
            },
            image_path: path,
            image_scale_pt_per_pixel: mm_to_points(scale),
        });
    }
    Ok(items)
}

#[test]
fn multiple_images_use_unique_resources_and_fit_physical_not_text_bounds() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let items = items(temp.path())?;
    let config = config()?;
    for mode in [
        TextWidthPolicy::WrapToFrame,
        TextWidthPolicy::ExpandFootprint,
    ] {
        let policy = policy(mode)?;
        let output = temp.path().join(format!("{mode:?}.pdf"));
        write_stamp_items_geometry_proof(
            &config.page_settings,
            &config.proof,
            &items,
            &policy,
            &output,
        )?;
        let pdf = PdfDocument::open(&output)?;
        assert_eq!(pdf.page_count()?, 1);
        let page = pdf.get_page(0)?;
        assert!((points_to_mm(page.media_box[2]) - 215.9).abs() < 0.001);
        let stamps: Vec<_> = items.iter().map(|i| i.stamp.clone()).collect();
        let content = config
            .page_settings
            .resolve_with_context(1, 1, None)?
            .content_bounds;
        let placed = fit_text_rows(content, &stamps, config.page_settings.spacing(), &policy)?;
        let mut matrices = Vec::new();
        for stream in pdf.get_page_content_streams(&page)? {
            let text = std::str::from_utf8(&stream)?;
            let tokens: Vec<_> = text.split_whitespace().collect();
            for w in tokens.windows(9) {
                if w[6] == "cm" && w[7].starts_with("/native_stamp_image") && w[8] == "Do" {
                    matrices.push((
                        w[7].to_string(),
                        w[..6]
                            .iter()
                            .map(|v| v.parse::<f64>())
                            .collect::<std::result::Result<Vec<_>, _>>()?,
                    ));
                }
            }
        }
        assert_eq!(matrices.len(), 4);
        let mut names = std::collections::HashSet::new();
        for (index, ((name, matrix), placed)) in
            matrices.iter().zip(placed.iter().flatten()).enumerate()
        {
            assert!(names.insert(name));
            let (width, height) = match index {
                0 => (11.0, 12.5),
                1 | 2 => (22.0, 25.0),
                3 => (25.0, 12.5),
                _ => unreachable!(),
            };
            let bounds = placed.geometry.physical_bounds();
            for (actual, expected) in [
                (points_to_mm(matrix[0]), width),
                (points_to_mm(matrix[3]), height),
                (
                    points_to_mm(matrix[4]),
                    (bounds.left + bounds.right - width) / 2.0,
                ),
                (
                    points_to_mm(matrix[5]),
                    (bounds.bottom + bounds.top - height) / 2.0,
                ),
            ] {
                assert!((actual - expected).abs() < 0.002, "{actual} != {expected}");
            }
        }
        let text = pdf.extract_text()?;
        for index in 1..=4 {
            assert!(text[0].text.contains(&format!("Stamp {index}")));
        }
    }
    Ok(())
}

#[test]
fn any_bad_image_or_scale_preserves_existing_output_and_creates_no_new_pdf() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let originals = items(temp.path())?;
    let config = config()?;
    let policy = policy(TextWidthPolicy::ExpandFootprint)?;
    for failure in 0..4 {
        let mut items = originals.clone();
        match failure {
            0 => items[3].image_path = temp.path().join("missing.png"),
            1 => {
                let bad = temp.path().join("corrupt.png");
                fs::write(&bad, b"not an image")?;
                items[3].image_path = bad;
            }
            2 => items[3].image_scale_pt_per_pixel = f64::NAN,
            _ => items[3].image_scale_pt_per_pixel = 0.0,
        }
        let output = temp.path().join("existing.pdf");
        fs::write(&output, b"original")?;
        assert!(write_stamp_items_geometry_proof(
            &config.page_settings,
            &config.proof,
            &items,
            &policy,
            &output
        )
        .is_err());
        assert_eq!(fs::read(&output)?, b"original");
        let missing = temp.path().join("must-not-exist.pdf");
        assert!(write_stamp_items_geometry_proof(
            &config.page_settings,
            &config.proof,
            &items,
            &policy,
            &missing
        )
        .is_err());
        assert!(!missing.exists());
    }
    Ok(())
}

#[test]
fn item_proof_rejects_multiple_pages_and_text_overflow_before_writing() -> Result<()> {
    let temp = tempfile::tempdir()?;
    let mut config = config()?;
    let mut items = items(temp.path())?;
    let policy = policy(TextWidthPolicy::ExpandFootprint)?;
    let output = temp.path().join("existing.pdf");
    fs::write(&output, b"original")?;
    config.proof.page_count = 2;
    assert!(write_stamp_items_geometry_proof(
        &config.page_settings,
        &config.proof,
        &items,
        &policy,
        &output
    )
    .is_err());
    config.proof.page_count = 1;
    items[0].stamp.caption.as_mut().ok_or("caption")?.text = "long text ".repeat(1000);
    assert!(write_stamp_items_geometry_proof(
        &config.page_settings,
        &config.proof,
        &items,
        &policy,
        &output
    )
    .is_err());
    assert_eq!(fs::read(output)?, b"original");
    Ok(())
}

#[test]
#[ignore = "Regenerates persistent combined image/text Letter proofs; run explicitly"]
fn regenerate_letter_item_fixtures() -> Result<()> {
    use stamp_album_gen2::native_pdf::commands::page::album_pages_title::AlbumPagesTitle;
    let output =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/stamp-items-v1");
    fs::create_dir_all(&output)?;
    let originals = items(&output)?;
    let inputs: Vec<_> = originals.iter().chain(originals.iter()).cloned().collect();
    let mut cases = Vec::new();
    for (name, mode) in [
        ("wrapped-images", TextWidthPolicy::WrapToFrame),
        ("expanded-images", TextWidthPolicy::ExpandFootprint),
    ] {
        let mut config = config()?;
        config.page_settings.set_title(AlbumPagesTitle::new("HN",10.0,
            &format!("{name}: complete image/text rows\nGreen footprint / black frame / blue physical\nSynthetic images; 120% expanded cap; padding 2 mm / 1 mm\nLetter / Actual size 100% / no Fit"),4.0,0.0,NamedColour::parse("black")?)?);
        let policy = policy(mode)?;
        write_stamp_items_geometry_proof(
            &config.page_settings,
            &config.proof,
            &inputs,
            &policy,
            &output.join(format!("{name}.pdf")),
        )?;
        let content = config
            .page_settings
            .resolve_with_context(1, 1, None)?
            .content_bounds;
        let stamps: Vec<_> = inputs.iter().map(|item| item.stamp.clone()).collect();
        let rows = fit_text_rows(content, &stamps, config.page_settings.spacing(), &policy)?;
        let mut source = inputs.iter();
        let mut data = Vec::new();
        let mut txt=format!("# COMPATIBILITY ADAPTATION: {name}; standalone text and image blocks, NOT native row/position parity.\n\
            # Native text cap/footprint policy is not an AlbumEasy command. All text and images retained.\n\
            ALBUM_PAGES_SIZE (215.9 279.4)\nALBUM_PAGES_MARGINS (12 12 12 12)\n\
            ALBUM_PAGES_SPACING (3 3)\nALBUM_PAGES_BORDER (0.2 0 0 0)\n\
            STAMP_BOXES_SIZE_ADJUST (0)\nPAGE_START\n");
        for row in rows {
            let mut row_data = Vec::new();
            for placed in row {
                let input = source.next().ok_or("fixture item mismatch")?;
                let physical = placed.geometry.physical_bounds();
                let frame = placed.geometry.frame_bounds();
                let b = placed.footprint;
                let (pixels_w, pixels_h) = image::image_dimensions(&input.image_path)?;
                let natural_w = points_to_mm(f64::from(pixels_w) * input.image_scale_pt_per_pixel);
                let natural_h = points_to_mm(f64::from(pixels_h) * input.image_scale_pt_per_pixel);
                let factor = ((physical.right - physical.left) / natural_w)
                    .min((physical.top - physical.bottom) / natural_h)
                    .min(1.0);
                let iw = natural_w * factor;
                let ih = natural_h * factor;
                let centre_x = (physical.left + physical.right) / 2.0;
                let centre_y = (physical.bottom + physical.top) / 2.0;
                let mut text_data = Vec::new();
                for text in [placed.heading.as_ref(), placed.caption.as_ref()]
                    .into_iter()
                    .flatten()
                {
                    let lines = text
                        .lines
                        .iter()
                        .map(|line| line.text.as_str())
                        .collect::<Vec<_>>()
                        .join("\n");
                    let quoted = lines
                        .replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\n', "\\n");
                    txt.push_str(&format!("PAGE_TEXT (HN 10 \"{quoted}\" 2)\n"));
                    text_data.push(serde_json::json!({
                        "lines":text.lines.iter().map(|line|serde_json::json!({
                            "text":line.text,"left_mm":line.left_mm,"baseline_mm":line.baseline_mm
                        })).collect::<Vec<_>>()
                    }));
                }
                let filename = input
                    .image_path
                    .file_name()
                    .and_then(|v| v.to_str())
                    .ok_or("filename")?;
                txt.push_str(&format!("ROW_START_ES (HN 10 0.1)\nSTAMP_ADD_IMG ({} {} \"{filename}\" \"\" \"\" \"\")\n",
                    frame.right-frame.left,frame.top-frame.bottom));
                row_data.push(serde_json::json!({
                    "footprint_mm":[b.left,b.bottom,b.right,b.top],
                    "frame_mm":[frame.left,frame.bottom,frame.right,frame.top],
                    "physical_mm":[physical.left,physical.bottom,physical.right,physical.top],
                    "image_bounds_mm":[centre_x-iw/2.0,centre_y-ih/2.0,centre_x+iw/2.0,centre_y+ih/2.0],
                    "image_pixels":[pixels_w,pixels_h],"image_scale_pt_per_pixel":input.image_scale_pt_per_pixel,
                    "image_file":filename,"text":text_data
                }));
            }
            data.push(row_data);
        }
        fs::write(output.join(format!("{name}.txt")), txt)?;
        cases.push(serde_json::json!({"name":name,"rows":data}));
    }
    fs::write(
        output.join("expected.json"),
        serde_json::to_string_pretty(&serde_json::json!({
            "reference":"Calculated native item layout; AlbumEasy comparison inputs not reference-rendered",
            "page_size_mm":[215.9,279.4],"tolerance_mm":0.004,"gap_tolerance_mm":0.006,
            "horizontal_gap_mm":3.0,"vertical_gap_mm":3.0,"cases":cases
        }))? + "\n",
    )?;
    Ok(())
}
