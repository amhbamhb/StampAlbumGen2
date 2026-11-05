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
    renderer::write_stamp_image_proof,
    stamp_image::{StampImageProofSettings, StampTextProofSettings},
    units::points_to_mm,
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
