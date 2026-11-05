//! Reproducible visual fixtures; no album data or production defaults are used.
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
    renderer::write_stamp_box_geometry_proof,
    stamp_box::StampBoxGeometry,
    stamp_image::{StampImageProofSettings, StampTextProofSettings},
    units::mm_to_points,
};
use std::{error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("Usage: cargo run --example stamp_frame_proofs -- OUTPUT_DIRECTORY")?;
    fs::create_dir_all(&output)?;
    let image_path = output.join("synthetic-stamp.png");
    let image = image::RgbImage::from_fn(220, 250, |x, y| {
        if x < 5 || y < 5 || x >= 215 || y >= 245 {
            image::Rgb([180, 90, 30])
        } else if (x / 20 + y / 20) % 2 == 0 {
            image::Rgb([235, 220, 170])
        } else {
            image::Rgb([170, 205, 190])
        }
    });
    image.save(&image_path)?;
    let settings = PageSettings::new(
        PageSize::parse(&[215.9, 279.4])?,
        Margins::parse(&[12.0; 4])?,
        Spacing::parse(&[3.0, 4.0])?,
        PageBorder::parse(&[0.2, 0.0, 0.0, 0.0])?,
        AlbumBorderColour::parse(&["black"])?,
    );
    let proof = GeometryProofSettings {
        page_count: 1,
        guide_enabled: false,
        guide_width_mm: 0.1,
        guide_colour: NamedColour::parse("black")?,
    };
    let cases = [
        ("01-standard-small-image", 22.0, 25.0, 6.0, 6.0, 0.0, 0.05),
        ("02-standard-fit-image", 22.0, 25.0, 6.0, 6.0, 0.0, 0.2),
        ("03-fractional", 22.5, 25.0, 3.0, 5.0, 0.5, 0.2),
        ("04-portrait", 25.0, 40.0, 6.0, 6.0, 0.0, 0.2),
        ("05-landscape", 40.0, 25.0, 6.0, 6.0, 0.0, 0.2),
        ("06-booklet", 44.0, 90.0, 6.0, 6.0, 0.0, 0.5),
        ("07-se-tenant", 125.0, 66.0, 6.0, 6.0, 0.0, 0.8),
        ("08-wide", 103.0, 51.0, 6.0, 6.0, 0.0, 0.5),
        ("09-zero-clearance", 22.0, 25.0, 0.0, 0.0, 0.0, 0.2),
        ("10-calibration", 100.0, 100.0, 0.0, 0.0, 0.0, 0.05),
    ];
    let mut measurements = String::from(
        "NATIVE FRAME GEOMETRY PROOFS - Letter 215.9 x 279.4 mm\n\
         Print ACTUAL SIZE / 100%; disable Fit, Shrink and borderless scaling.\n\
         Black = frame centreline; blue = physical perforation-tip bounds.\n\
         The synthetic picture is not a real stamp or a physical-size reference.\n\
         Case 10 blue/black outlines coincide: measure 100 mm on BOTH axes.\n\
         Existing extras are TOTAL additions, not per-edge amounts.\n\n\
         case,physical_w,physical_h,extra_w,extra_h,adjust,frame_w,frame_h,clearance_x,clearance_y,image_w,image_h\n"
    );
    let reference_preamble = String::from(
        "# Independent frame-size comparison, not native image/text parity.\n\
         ALBUM_PAGES_SIZE (215.9 279.4)\nALBUM_PAGES_MARGINS (12 12 12 12)\n\
         ALBUM_PAGES_SPACING (3 4)\nALBUM_PAGES_BORDER (0.2 0 0 0)\n\
         STAMP_BOXES_SIZE_ADJUST (0)\n",
    );
    let mut reference = reference_preamble.clone();
    let mut expected = Vec::new();
    for (name, width, height, ew, eh, adjustment, mm_per_pixel) in cases {
        let geometry = StampBoxGeometry::from_total_additions(
            Bounds {
                left: 107.95 - width / 2.0,
                right: 107.95 + width / 2.0,
                bottom: 185.0 - height / 2.0,
                top: 185.0 + height / 2.0,
            },
            ew,
            eh,
            adjustment,
        )?;
        let fw = width + ew + adjustment;
        let fh = height + eh + adjustment;
        let scale = (width / (220.0 * mm_per_pixel))
            .min(height / (250.0 * mm_per_pixel))
            .min(1.0);
        let iw = 220.0 * mm_per_pixel * scale;
        let ih = 250.0 * mm_per_pixel * scale;
        let caption = format!(
            "Physical\n{width} x {height}\nFrame\n{fw} x {fh}\n\
             Edge X / Y\n{} / {}\nImage\n{iw:.2} x {ih:.2}\n\
             All sizes mm\nBlack frame\nBlue physical\nPrint 100%\nNo fit",
            geometry.horizontal_clearance_mm(),
            geometry.vertical_clearance_mm()
        );
        let stamp = StampImageProofSettings {
            image_path: image_path.clone(),
            box_bounds: geometry.frame_bounds(),
            image_scale_pt_per_pixel: mm_to_points(mm_per_pixel),
            image_padding_horizontal_mm: 0.0,
            image_padding_vertical_mm: 0.0,
            text_gap_mm: 3.0,
            heading: Some(StampTextProofSettings::new(
                name.split_once('-').map_or(name, |(number, _)| number),
                "HB",
                14.0,
                NamedColour::parse("black")?,
            )),
            caption: Some(StampTextProofSettings::new(
                caption,
                "HN",
                10.0,
                NamedColour::parse("black")?,
            )),
        };
        write_stamp_box_geometry_proof(
            &settings,
            &proof,
            &stamp,
            &geometry,
            &output.join(format!("{name}.pdf")),
        )?;
        measurements.push_str(&format!(
            "{name},{width},{height},{ew},{eh},{adjustment},{fw},{fh},{},{},{iw:.6},{ih:.6}\n",
            geometry.horizontal_clearance_mm(),
            geometry.vertical_clearance_mm()
        ));
        let reference_case = format!(
            "\nPAGE_START\nPAGE_TEXT_CENTRE (HB 12 \"{name}\" 4)\n\
             PAGE_TEXT_CENTRE (HN 9 \"Frame {fw} x {fh} mm; physical {width} x {height} mm\" 4)\n\
             ROW_START_ES (HN 6 0.1)\nSTAMP_ADD ({fw} {fh} \"\" \"\" \"Frame\" \"\")\n\
             PAGE_VSPACE (10)\nROW_START_ES (HN 6 0.1)\n\
             STAMP_ADD ({width} {height} \"\" \"\" \"Physical reference\" \"\")\n"
        );
        reference.push_str(&reference_case);
        fs::write(
            output.join(format!("{name}.txt")),
            format!("{reference_preamble}{reference_case}"),
        )?;
        expected.push(serde_json::json!({
            "name": name,
            "physical_size_mm": [width, height],
            "frame_size_mm": [fw, fh],
            "clearance_per_edge_mm": [
                geometry.horizontal_clearance_mm(),
                geometry.vertical_clearance_mm()
            ],
            "image_size_mm": [iw, ih]
        }));
    }
    fs::write(output.join("measurements.txt"), measurements)?;
    fs::write(output.join("frame-reference-albumeasy.txt"), reference)?;
    let expected = serde_json::json!({
        "reference": "Calculated native geometry; AlbumEasy TXT not reference-rendered",
        "page_size_mm": [215.9, 279.4],
        "tolerance_mm": 0.004,
        "cases": expected
    });
    fs::write(
        output.join("expected.json"),
        serde_json::to_string_pretty(&expected)? + "\n",
    )?;
    println!(
        "Created 10 PDF/TXT pairs, expected geometry, measurement ledger, AlbumEasy frame reference, and synthetic PNG in {}",
        output.display()
    );
    Ok(())
}
