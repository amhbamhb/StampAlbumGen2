use stamp_album_gen2::native_pdf::{
    commands::page::album_pages_spacing::Spacing, document::Bounds, stamp_box::StampBoxGeometry,
    stamp_rows::fit_known_size_rows,
};

fn bounds(width: f64, height: f64) -> Bounds {
    Bounds {
        left: 12.0,
        bottom: 12.0,
        right: 12.0 + width,
        top: 12.0 + height,
    }
}

fn stamp(width: f64, height: f64, extra: f64) -> Result<StampBoxGeometry, String> {
    StampBoxGeometry::from_total_additions(bounds(width, height), extra, extra, 0.0)
}

fn near(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-9, "{actual} != {expected}");
}

#[test]
fn exact_fit_uses_frame_width_and_gaps_without_trailing_gap() -> Result<(), String> {
    let stamps = [stamp(22.0, 25.0, 6.0)?, stamp(22.0, 25.0, 6.0)?];
    let rows = fit_known_size_rows(bounds(59.0, 31.0), &stamps, Spacing::parse(&[3.0, 4.0])?)?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].len(), 2);
    near(rows[0][0].frame_bounds().left, 12.0);
    near(rows[0][1].frame_bounds().left, 43.0);
    near(rows[0][1].frame_bounds().right, 71.0);
    near(rows[0][0].frame_bounds().bottom, 12.0);
    Ok(())
}

#[test]
fn wraps_in_order_below_tallest_frame_without_scaling_or_rotation() -> Result<(), String> {
    let stamps = [
        stamp(22.0, 25.0, 6.0)?,
        stamp(25.0, 40.0, 6.0)?,
        stamp(40.0, 25.0, 6.0)?,
    ];
    let rows = fit_known_size_rows(bounds(62.0, 100.0), &stamps, Spacing::parse(&[3.0, 4.0])?)?;
    assert_eq!(rows.iter().map(Vec::len).collect::<Vec<_>>(), [2, 1]);
    near(rows[0][0].frame_bounds().top, 112.0);
    near(rows[0][1].frame_bounds().top, 112.0);
    near(rows[1][0].frame_bounds().top, 62.0);
    for (placed, source) in rows.iter().flatten().zip(stamps) {
        let p = placed.physical_bounds();
        let s = source.physical_bounds();
        near(p.right - p.left, s.right - s.left);
        near(p.top - p.bottom, s.top - s.bottom);
        near(
            placed.horizontal_clearance_mm(),
            source.horizontal_clearance_mm(),
        );
        near(
            placed.vertical_clearance_mm(),
            source.vertical_clearance_mm(),
        );
    }
    Ok(())
}

#[test]
fn fractional_clearance_and_zero_spacing_keep_independent_dimensions() -> Result<(), String> {
    let stamps = [
        StampBoxGeometry::from_total_additions(bounds(22.5, 25.0), 3.0, 5.0, 0.5)?,
        stamp(125.0, 66.0, 0.0)?,
    ];
    let rows = fit_known_size_rows(bounds(151.0, 66.0), &stamps, Spacing::parse(&[0.0, 0.0])?)?;
    assert_eq!(rows.len(), 1);
    near(
        rows[0][0].frame_bounds().right,
        rows[0][1].frame_bounds().left,
    );
    near(rows[0][1].frame_bounds().right, 163.0);
    near(rows[0][1].physical_bounds().bottom, 12.0);
    Ok(())
}

#[test]
fn width_and_vertical_overflow_fail_instead_of_shrinking_or_discarding() -> Result<(), String> {
    let spacing = Spacing::parse(&[3.0, 4.0])?;
    for (area, stamps, message) in [
        (
            bounds(27.999, 100.0),
            vec![stamp(22.0, 25.0, 6.0)?],
            "width",
        ),
        (
            bounds(28.0, 30.999),
            vec![stamp(22.0, 25.0, 6.0)?],
            "height",
        ),
        (
            bounds(28.0, 65.999),
            vec![stamp(22.0, 25.0, 6.0)?, stamp(22.0, 25.0, 6.0)?],
            "height",
        ),
    ] {
        let error = fit_known_size_rows(area, &stamps, spacing).expect_err("must reject overflow");
        assert!(error.contains("native_pdf.stamp_rows"), "{error}");
        assert!(error.contains(message), "{error}");
    }
    Ok(())
}

#[test]
fn empty_input_is_empty_but_invalid_content_is_not_silently_accepted() -> Result<(), String> {
    let spacing = Spacing::parse(&[3.0, 4.0])?;
    assert!(fit_known_size_rows(bounds(100.0, 100.0), &[], spacing)?.is_empty());
    for area in [
        bounds(0.0, 100.0),
        bounds(100.0, -1.0),
        bounds(f64::INFINITY, 100.0),
        Bounds {
            left: f64::NAN,
            ..bounds(100.0, 100.0)
        },
        Bounds {
            left: -3.5e307,
            right: 3.5e307,
            ..bounds(100.0, 100.0)
        },
    ] {
        assert!(fit_known_size_rows(area, &[], spacing).is_err());
    }
    Ok(())
}

#[test]
fn rendered_letter_rows_match_positions_and_overflow_preserves_output(
) -> Result<(), Box<dyn std::error::Error>> {
    use oxidize_pdf::parser::PdfDocument;
    use stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::{
            colour::colour_album_border::AlbumBorderColour,
            page::{
                album_pages_border::PageBorder, album_pages_margins::Margins,
                album_pages_size::PageSize,
            },
        },
        document::PageSettings,
        proof::GeometryProofSettings,
        renderer::write_stamp_rows_geometry_proof,
        units::points_to_mm,
    };
    let mut settings = PageSettings::new(
        PageSize::parse(&[215.9, 279.4])?,
        Margins::parse(&[12.0; 4])?,
        Spacing::parse(&[3.0, 4.0])?,
        PageBorder::parse(&[0.2, 0.0, 0.0, 0.0])?,
        AlbumBorderColour::parse(&["black"])?,
    );
    settings.set_row_alignment(stamp_album_gen2::native_pdf::stamp_rows::RowAlignment::Left);
    let proof = GeometryProofSettings {
        page_count: 1,
        guide_enabled: false,
        guide_width_mm: 0.1,
        guide_colour: NamedColour::parse("black")?,
    };
    let stamps = [
        stamp(125.0, 66.0, 6.0)?,
        stamp(22.0, 25.0, 6.0)?,
        stamp(103.0, 51.0, 6.0)?,
    ];
    let layout = settings.resolve_with_context(1, 1, None)?;
    let rows = fit_known_size_rows(layout.content_bounds, &stamps, settings.spacing())?;
    assert_eq!(rows.iter().map(Vec::len).collect::<Vec<_>>(), [2, 1]);
    let temporary = tempfile::tempdir()?;
    let output = temporary.path().join("rows.pdf");
    write_stamp_rows_geometry_proof(&settings, &proof, &stamps, &output)?;
    let original = std::fs::read(&output)?;
    let pdf = PdfDocument::open(&output)?;
    assert_eq!(pdf.page_count()?, 1);
    let page = pdf.get_page(0)?;
    assert!((points_to_mm(page.media_box[2]) - 215.9).abs() < 0.001);
    assert!((points_to_mm(page.media_box[3]) - 279.4).abs() < 0.001);
    let mut rectangles = Vec::new();
    for stream in pdf.get_page_content_streams(&page)? {
        let text = std::str::from_utf8(&stream)?;
        let tokens: Vec<_> = text.split_whitespace().collect();
        for window in tokens.windows(5).filter(|window| window[4] == "re") {
            rectangles.push(
                window[..4]
                    .iter()
                    .map(|token| token.parse::<f64>().map(points_to_mm))
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
    }
    let expected: Vec<_> = rows
        .iter()
        .flatten()
        .flat_map(|item| [item.frame_bounds(), item.physical_bounds()])
        .collect();
    assert_eq!(
        rectangles.len(),
        1 + expected.len(),
        "page border and frame/physical outlines"
    );
    for (actual, expected) in rectangles.iter().skip(1).zip(expected) {
        for (actual, expected) in actual.iter().zip([
            expected.left,
            expected.bottom,
            expected.right - expected.left,
            expected.top - expected.bottom,
        ]) {
            assert!((actual - expected).abs() < 0.002, "{actual} != {expected}");
        }
    }
    let oversized = [stamp(200.0, 25.0, 6.0)?];
    assert!(write_stamp_rows_geometry_proof(&settings, &proof, &oversized, &output).is_err());
    assert_eq!(std::fs::read(&output)?, original);
    let missing = temporary.path().join("must-not-create.pdf");
    assert!(write_stamp_rows_geometry_proof(&settings, &proof, &oversized, &missing).is_err());
    assert!(!missing.exists());
    let multi_page = GeometryProofSettings {
        page_count: 2,
        ..proof
    };
    assert!(write_stamp_rows_geometry_proof(&settings, &multi_page, &stamps, &output).is_err());
    assert_eq!(std::fs::read(&output)?, original);
    Ok(())
}

#[test]
#[ignore = "Regenerates persistent Letter proof fixtures; run explicitly"]
fn regenerate_letter_row_fixtures() -> Result<(), Box<dyn std::error::Error>> {
    use stamp_album_gen2::native_pdf::{
        colour::NamedColour,
        commands::{
            colour::colour_album_border::AlbumBorderColour,
            page::{
                album_pages_border::PageBorder, album_pages_margins::Margins,
                album_pages_size::PageSize, album_pages_title::AlbumPagesTitle,
            },
        },
        document::PageSettings,
        proof::GeometryProofSettings,
        renderer::write_stamp_rows_geometry_proof,
    };
    use std::{fs, path::Path};
    let output =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/albumeasy/stamp-rows-v1");
    fs::create_dir_all(&output)?;
    let cases = [
        ("standard-wrap", vec![(22.0, 25.0, 6.0); 7]),
        (
            "mixed-heights",
            vec![
                (22.0, 25.0, 6.0),
                (25.0, 40.0, 6.0),
                (40.0, 25.0, 6.0),
                (44.0, 90.0, 6.0),
                (103.0, 51.0, 6.0),
                (125.0, 66.0, 6.0),
            ],
        ),
        (
            "large-wrap",
            vec![
                (125.0, 66.0, 6.0),
                (22.0, 25.0, 6.0),
                (103.0, 51.0, 6.0),
                (25.0, 40.0, 6.0),
            ],
        ),
        (
            "zero-clearance",
            vec![(22.5, 25.0, 0.0), (40.0, 25.0, 0.0), (125.0, 66.0, 0.0)],
        ),
    ];
    let mut expected = Vec::new();
    for (name, sizes) in cases {
        let mut settings = PageSettings::new(
            PageSize::parse(&[215.9, 279.4])?,
            Margins::parse(&[12.0; 4])?,
            Spacing::parse(&[3.0, 4.0])?,
            PageBorder::parse(&[0.2, 0.0, 0.0, 0.0])?,
            AlbumBorderColour::parse(&["black"])?,
        );
        settings.set_row_alignment(stamp_album_gen2::native_pdf::stamp_rows::RowAlignment::Left);
        settings.set_title(AlbumPagesTitle::new(
            "HN", 10.0,
            &format!("{name} - frame-only rows\nBlack: frame; blue: physical; input order left to right\nLetter / Actual size 100% / no Fit"),
            4.0, 0.0, NamedColour::parse("black")?,
        )?);
        let stamps = sizes
            .iter()
            .map(|&(w, h, e)| stamp(w, h, e))
            .collect::<Result<Vec<_>, _>>()?;
        let content = settings.resolve_with_context(1, 1, None)?.content_bounds;
        let rows = fit_known_size_rows(content, &stamps, settings.spacing())?;
        let proof = GeometryProofSettings {
            page_count: 1,
            guide_enabled: false,
            guide_width_mm: 0.1,
            guide_colour: NamedColour::parse("black")?,
        };
        write_stamp_rows_geometry_proof(
            &settings,
            &proof,
            &stamps,
            &output.join(format!("{name}.pdf")),
        )?;
        let mut reference = format!(
            "# Frame geometry comparison ONLY; not native position/text parity.\n\
             ALBUM_PAGES_SIZE (215.9 279.4)\nALBUM_PAGES_MARGINS (12 12 12 12)\n\
             ALBUM_PAGES_SPACING (3 4)\nALBUM_PAGES_BORDER (0.2 0 0 0)\n\
             STAMP_BOXES_SIZE_ADJUST (0)\nPAGE_START\n\
             PAGE_TEXT_CENTRE (HN 10 \"{name}: mounting frames only\" 4)\n"
        );
        let mut row_data = Vec::new();
        for row in rows {
            reference.push_str("ROW_START_ES (HN 10 0.1)\n");
            let mut item_data = Vec::new();
            for item in row {
                let f = item.frame_bounds();
                let p = item.physical_bounds();
                reference.push_str(&format!(
                    "STAMP_ADD ({} {} \"\" \"\" \"\" \"\")\n",
                    f.right - f.left,
                    f.top - f.bottom
                ));
                item_data.push(serde_json::json!({
                    "frame_bounds_mm": [f.left, f.bottom, f.right, f.top],
                    "physical_bounds_mm": [p.left, p.bottom, p.right, p.top]
                }));
            }
            row_data.push(item_data);
        }
        fs::write(output.join(format!("{name}.txt")), reference)?;
        expected.push(serde_json::json!({
            "name": name,
            "content_bounds_mm": [content.left, content.bottom, content.right, content.top],
            "rows": row_data
        }));
    }
    let expected = serde_json::json!({
        "reference": "Calculated native frame-only rows; AlbumEasy TXT not reference-rendered",
        "page_size_mm": [215.9, 279.4], "tolerance_mm": 0.004,
        "gap_tolerance_mm": 0.006,
        "horizontal_gap_mm": 3.0, "vertical_gap_mm": 4.0,
        "cases": expected
    });
    fs::write(
        output.join("expected.json"),
        serde_json::to_string_pretty(&expected)? + "\n",
    )?;
    Ok(())
}
