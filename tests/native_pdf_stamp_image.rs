use oxidize_pdf::parser::PdfDocument;
use stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    commands::{
        colour::colour_album_border::AlbumBorderColour,
        page::{
            album_pages_border::PageBorder, album_pages_margins::Margins,
            album_pages_size::PageSize, album_pages_spacing::Spacing,
        },
    },
    document::{Bounds, PageSettings},
    proof::GeometryProofSettings,
    renderer::{write_stamp_box_geometry_proof, write_stamp_image_proof},
    stamp_box::StampBoxGeometry,
    stamp_image::{StampImageProofSettings, StampTextProofSettings},
    units::{mm_to_points, points_to_mm},
};
use std::{error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn settings() -> Result<PageSettings> {
    Ok(PageSettings::new(
        PageSize::parse(&[210.0, 297.0])?,
        Margins::parse(&[12.0; 4])?,
        Spacing::parse(&[3.0, 4.0])?,
        PageBorder::parse(&[0.5, 0.0, 0.0, 0.0])?,
        AlbumBorderColour::parse(&["black"])?,
    ))
}

fn proof() -> Result<GeometryProofSettings> {
    Ok(GeometryProofSettings {
        page_count: 1,
        guide_enabled: true,
        guide_width_mm: 0.1,
        guide_colour: NamedColour::parse("black")?,
    })
}

fn make_stamp_image(path: &Path) -> Result<()> {
    let image = image::RgbImage::from_fn(72, 100, |x, y| {
        if !(5..67).contains(&x) || !(5..95).contains(&y) {
            image::Rgb([30, 50, 120])
        } else if (x + y) % 2 == 0 {
            image::Rgb([220, 180, 90])
        } else {
            image::Rgb([245, 230, 180])
        }
    });
    image.save(path)?;
    Ok(())
}

#[test]
fn indexed_png_and_rgba_transparency_render_without_backend_png_decode() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    let indexed = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/albumeasy/denmark-txt-native-v1/images/flag_placeholder.png");
    let bytes = fs::read(&indexed)?;
    assert_eq!(
        (bytes[24], bytes[25]),
        (4, 3),
        "4-bit indexed PNG regression"
    );
    let rgba = temporary.path().join("transparent.png");
    image::RgbaImage::from_fn(16, 16, |x, _| {
        image::Rgba([30, 50, 120, if x < 8 { 0 } else { 255 }])
    })
    .save(&rgba)?;
    for (index, image_path) in [indexed, rgba].into_iter().enumerate() {
        let output = temporary.path().join(format!("png-{index}.pdf"));
        write_stamp_image_proof(
            &settings()?,
            &proof()?,
            &StampImageProofSettings {
                image_path,
                box_bounds: Bounds {
                    left: 80.0,
                    right: 130.0,
                    bottom: 116.0,
                    top: 176.0,
                },
                image_scale_pt_per_pixel: 1.0,
                options: Default::default(),
                image_padding_horizontal_mm: 2.0,
                image_padding_vertical_mm: 2.0,
                text_gap_mm: 2.0,
                heading: None,
                caption: None,
            },
            &output,
        )?;
        let pdf = PdfDocument::open(&output)?;
        assert_eq!(pdf.page_count()?, 1);
        if index == 1 {
            assert!(String::from_utf8_lossy(&fs::read(output)?).contains("/SMask"));
        }
    }
    Ok(())
}

#[test]
fn stamp_image_heading_caption_and_page_border_render_with_physical_geometry() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    let image_path = temporary.path().join("proof-stamp.png");
    make_stamp_image(&image_path)?;
    let output = temporary.path().join("stamp-proof.pdf");
    let black = NamedColour::parse("black")?;
    let stamp = StampImageProofSettings {
        image_path: image_path.clone(),
        box_bounds: Bounds {
            left: 80.0,
            bottom: 116.0,
            right: 130.0,
            top: 176.0,
        },
        image_scale_pt_per_pixel: 1.0,
        options: Default::default(),
        image_padding_horizontal_mm: 2.0,
        image_padding_vertical_mm: 2.0,
        text_gap_mm: 2.0,
        heading: Some(StampTextProofSettings::new(
            "NATIVE STAMP PROOF",
            "HB",
            8.0,
            black,
        )),
        caption: Some(StampTextProofSettings::new(
            "72 x 100 px; no upscale",
            "HN",
            6.0,
            black,
        )),
    };

    write_stamp_image_proof(&settings()?, &proof()?, &stamp, &output)?;

    let pdf = PdfDocument::open(&output)?;
    assert_eq!(pdf.page_count()?, 1);
    let page = pdf.get_page(0)?;
    assert!((points_to_mm(page.media_box[2] - page.media_box[0]) - 210.0).abs() < 0.01);
    assert!((points_to_mm(page.media_box[3] - page.media_box[1]) - 297.0).abs() < 0.01);
    let streams = pdf.get_page_content_streams(&page)?;
    assert!(streams.iter().any(|stream| {
        stream
            .windows(b"native_stamp_image".len())
            .any(|window| window == b"native_stamp_image")
    }));
    let extracted = pdf.extract_text()?;
    assert_eq!(extracted.len(), 1);
    assert!(extracted[0].text.contains("NATIVE STAMP PROOF"));
    assert!(extracted[0].text.contains("72 x 100 px; no upscale"));
    assert!(fs::metadata(output)?.len() > 0);
    Ok(())
}

#[test]
fn invalid_image_does_not_create_or_truncate_the_output() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    let missing_image = temporary.path().join("missing.png");
    let stamp = StampImageProofSettings {
        image_path: missing_image,
        box_bounds: Bounds {
            left: 80.0,
            bottom: 116.0,
            right: 130.0,
            top: 176.0,
        },
        image_scale_pt_per_pixel: 1.0,
        options: Default::default(),
        image_padding_horizontal_mm: 2.0,
        image_padding_vertical_mm: 2.0,
        text_gap_mm: 2.0,
        heading: None,
        caption: None,
    };
    let output = temporary.path().join("stamp-proof.pdf");
    assert!(write_stamp_image_proof(&settings()?, &proof()?, &stamp, &output).is_err());
    assert!(!output.exists());

    fs::write(&output, b"existing output")?;
    assert!(write_stamp_image_proof(&settings()?, &proof()?, &stamp, &output).is_err());
    assert_eq!(fs::read(output)?, b"existing output");
    Ok(())
}

#[test]
fn illustration_scale_changes_pdf_image_but_not_known_size_or_frame() -> Result<()> {
    let temporary = tempfile::tempdir()?;
    let image_path = temporary.path().join("known-stamp.png");
    image::RgbImage::from_pixel(220, 250, image::Rgb([30, 50, 120])).save(&image_path)?;
    let geometry = StampBoxGeometry::from_total_additions(
        Bounds {
            left: 80.0,
            bottom: 116.0,
            right: 102.0,
            top: 141.0,
        },
        6.0,
        6.0,
        0.0,
    )?;
    for (index, mm_per_pixel, expected_width, expected_height) in
        [(0, 0.05, 11.0, 12.5), (1, 0.2, 22.0, 25.0)]
    {
        let stamp = StampImageProofSettings {
            image_path: image_path.clone(),
            box_bounds: geometry.physical_bounds(),
            image_scale_pt_per_pixel: mm_to_points(mm_per_pixel),
            options: Default::default(),
            image_padding_horizontal_mm: 0.0,
            image_padding_vertical_mm: 0.0,
            text_gap_mm: 2.0,
            heading: None,
            caption: None,
        }
        .with_stamp_box(&geometry);
        assert_eq!(stamp.image_scale_pt_per_pixel, mm_to_points(mm_per_pixel));
        assert_eq!(stamp.image_padding_horizontal_mm, 3.0);
        assert_eq!(stamp.image_padding_vertical_mm, 3.0);
        assert_eq!(stamp.box_bounds.right - stamp.box_bounds.left, 28.0);
        assert_eq!(stamp.box_bounds.top - stamp.box_bounds.bottom, 31.0);
        let output = temporary.path().join(format!("known-stamp-{index}.pdf"));
        write_stamp_box_geometry_proof(&settings()?, &proof()?, &stamp, &geometry, &output)?;
        let pdf = PdfDocument::open(&output)?;
        let page = pdf.get_page(0)?;
        let streams = pdf.get_page_content_streams(&page)?;
        let mut image_matrix = None;
        let mut rectangles = 0;
        for stream in streams {
            let content = std::str::from_utf8(&stream)?;
            let tokens: Vec<_> = content.split_whitespace().collect();
            rectangles += tokens.iter().filter(|token| **token == "re").count();
            for window in tokens.windows(9) {
                if window[6..] == ["cm", "/native_stamp_image", "Do"] {
                    let values = window[..6]
                        .iter()
                        .map(|value| value.parse::<f64>())
                        .collect::<std::result::Result<Vec<_>, _>>()?;
                    assert!(image_matrix.replace(values).is_none());
                }
            }
        }
        let matrix = image_matrix.ok_or("PDF stamp image transform not found")?;
        assert_eq!(
            rectangles, 4,
            "page border, content guide, frame, physical outline"
        );
        // The PDF writer serializes matrix coordinates to two decimal points.
        let tolerance_mm = points_to_mm(0.005) + 1e-9;
        assert!((points_to_mm(matrix[0]) - expected_width).abs() < tolerance_mm);
        assert_eq!(matrix[1], 0.0);
        assert_eq!(matrix[2], 0.0);
        assert!(
            (points_to_mm(matrix[3]) - expected_height).abs() < tolerance_mm,
            "expected height {expected_height} mm; PDF matrix {matrix:?}"
        );
        assert!((points_to_mm(matrix[4]) - (91.0 - expected_width / 2.0)).abs() < tolerance_mm);
        assert!((points_to_mm(matrix[5]) - (128.5 - expected_height / 2.0)).abs() < tolerance_mm);
        let physical = geometry.physical_bounds();
        assert_eq!(physical.right - physical.left, 22.0);
        assert_eq!(physical.top - physical.bottom, 25.0);
    }
    Ok(())
}
