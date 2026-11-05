use super::{
    colour::NamedColour, document::PageSettings, proof::GeometryProofSettings,
    stamp_box::StampBoxGeometry, stamp_image::StampImageProofSettings, text::PositionedTextLine,
    units::mm_to_points, Result,
};
use oxidize_pdf::{Color, Document, Page};
use std::path::Path;

/// Isolated geometry proof. Stamp-image content is available through
/// `write_stamp_image_proof`; production row layout is not wired up.
pub fn write_page_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    output: &Path,
) -> Result<()> {
    let generation_date = settings.capture_generation_date();
    write_page_geometry_proof_with_date(settings, proof, output, generation_date)
}

/// Explicit context entry point for deterministic generation-date testing and
/// callers that also report the resolved layout. Fixed TOML dates take precedence.
pub fn write_page_geometry_proof_with_date(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    output: &Path,
    generation_date: Option<chrono::NaiveDate>,
) -> Result<()> {
    write_proof(settings, proof, output, generation_date, None, None)
}

/// Writes a one-page diagnostic proof with one explicitly sized stamp image.
pub fn write_stamp_image_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    stamp: &StampImageProofSettings,
    output: &Path,
) -> Result<()> {
    if proof.page_count != 1 {
        return Err("native_pdf.stamp_image proof requires exactly one page".into());
    }
    write_proof(
        settings,
        proof,
        output,
        settings.capture_generation_date(),
        Some(stamp),
        None,
    )
}

/// Diagnostic frame/physical outlines for a known-size stamp.
/// These guides are not production stamp-frame styling.
pub fn write_stamp_box_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    stamp: &StampImageProofSettings,
    geometry: &StampBoxGeometry,
    output: &Path,
) -> Result<()> {
    if proof.page_count != 1 {
        return Err("native_pdf.stamp_box proof requires exactly one page".into());
    }
    let stamp = stamp.clone().with_stamp_box(geometry);
    write_proof(
        settings,
        proof,
        output,
        settings.capture_generation_date(),
        Some(&stamp),
        Some(geometry),
    )
}

fn write_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    output: &Path,
    generation_date: Option<chrono::NaiveDate>,
    stamp: Option<&StampImageProofSettings>,
    geometry: Option<&StampBoxGeometry>,
) -> Result<()> {
    proof.validate()?;
    let layouts = (1..=proof.page_count)
        .map(|number| {
            let layout = settings
                .resolve_with_context(number, proof.page_count, generation_date)
                .map_err(|error| format!("Native PDF page {number}: {error}"))?;
            proof.validate_layout(&layout)?;
            Ok(layout)
        })
        .collect::<Result<Vec<_>>>()?;
    let stamp = stamp.map(|stamp| stamp.prepare(&layouts[0])).transpose()?;
    let mut document = Document::new();
    for (index, layout) in layouts.into_iter().enumerate() {
        let mut page = Page::new(
            mm_to_points(layout.size.width_mm()),
            mm_to_points(layout.size.height_mm()),
        );
        let mut strokes: Vec<_> = settings
            .border()
            .strokes(layout.margin_bounds)?
            .into_iter()
            .map(|stroke| {
                (
                    stroke.bounds,
                    stroke.width_mm,
                    settings.border_line_colour(stroke.line).rgb(),
                )
            })
            .collect();
        if proof.guide_enabled {
            strokes.push((
                layout.content_bounds,
                proof.guide_width_mm,
                proof.guide_colour.rgb(),
            ));
        }
        for (bounds, width_mm, [red, green, blue]) in strokes {
            page.graphics()
                .set_stroke_color(Color::rgb(red, green, blue))
                .set_line_width(mm_to_points(width_mm))
                .rect(
                    mm_to_points(bounds.left),
                    mm_to_points(bounds.bottom),
                    mm_to_points(bounds.right - bounds.left),
                    mm_to_points(bounds.top - bounds.bottom),
                )
                .stroke();
        }
        if let Some(border) = layout.decorative_border {
            let footprints = border.footprints();
            if footprints.iter().any(|b| {
                b.left < 0.0
                    || b.bottom < 0.0
                    || b.right > layout.size.width_mm()
                    || b.top > layout.size.height_mm()
            }) {
                eprintln!("Warning: Native PDF page {}: ALBUM_PAGES_DECORATIVE_BORDER crosses the physical page and will be clipped; adjust IMAGE_SCALE/IMAGE_CORNER_PAD/IMAGE_EDGE_PAD/BORDER_PAD or native_pdf.decorative_border policy. Page size is unchanged.",index+1);
            }
            if footprints.iter().any(|b| {
                b.left < layout.content_bounds.right
                    && b.right > layout.content_bounds.left
                    && b.bottom < layout.content_bounds.top
                    && b.top > layout.content_bounds.bottom
            }) {
                // TODO(native-pdf-ui): Show page/definition element and offer scale,
                // padding/end-offset or content-spacing/hint corrections. Do not silently move content.
                eprintln!("Warning: Native PDF page {}: ALBUM_PAGES_DECORATIVE_BORDER footprint intersects the content area; transparent image pixels may be clear. Adjust definition scale/corner/edge padding/end offsets or content spacing/height hints. Decorative borders do not reserve content space (AlbumEasy-compatible).",index+1);
            }
            for image in border.corners.iter().chain(&border.edges) {
                page.add_image(image.resource_name, image.image.as_ref().clone());
                let [x, y, width, height] = image.rect_mm;
                page.draw_image(
                    image.resource_name,
                    mm_to_points(x),
                    mm_to_points(y),
                    mm_to_points(width),
                    mm_to_points(height),
                )
                .map_err(|error| format!("Cannot draw native decorative corner: {error}"))?;
            }
            let [red, green, blue] = border.colour.rgb();
            for line in border.lines {
                let [a, b] = line.points;
                page.graphics()
                    .set_stroke_color(Color::rgb(red, green, blue))
                    .set_line_width(mm_to_points(line.width_mm))
                    .move_to(mm_to_points(a[0]), mm_to_points(a[1]))
                    .line_to(mm_to_points(b[0]), mm_to_points(b[1]))
                    .stroke();
            }
        }
        if let Some(marks) = layout.crop_marks {
            if marks.clipped {
                // TODO(native-pdf-ui): Show the physical-page clipping warning
                // and offer padding/arms corrections without resizing silently.
                eprintln!("Warning: Native PDF page {}: ALBUM_PAGES_CROP_MARKS extend partly or wholly outside the physical page and will be clipped; adjust native_pdf.crop_marks.horizontal_padding_mm/vertical_padding_mm/arms_mm/line_width_mm. Page size and content are unchanged (AlbumEasy-compatible).", index + 1);
            }
            let [red, green, blue] = marks.colour.rgb();
            for mark in marks.marks {
                let [a, b, c, d] = mark.points;
                page.graphics()
                    .set_stroke_color(Color::rgb(red, green, blue))
                    .set_line_width(mm_to_points(marks.line_width_mm))
                    .move_to(mm_to_points(a[0]), mm_to_points(a[1]))
                    .line_to(mm_to_points(b[0]), mm_to_points(b[1]))
                    .move_to(mm_to_points(c[0]), mm_to_points(c[1]))
                    .line_to(mm_to_points(d[0]), mm_to_points(d[1]))
                    .stroke();
            }
        }
        if let Some(number) = layout.header_number {
            draw_text_lines(
                &mut page,
                number.font,
                number.size_pt,
                number.colour,
                number.lines,
                "header number",
            )?;
        }
        if let Some(number) = layout.footer_number {
            draw_text_lines(
                &mut page,
                number.font,
                number.size_pt,
                number.colour,
                number.lines,
                "footer number",
            )?;
        }
        if let Some(date) = layout.header_date {
            draw_text_lines(
                &mut page,
                date.font,
                date.size_pt,
                date.colour,
                date.lines,
                "header date",
            )?;
        }
        if let Some(date) = layout.footer_date {
            draw_text_lines(
                &mut page,
                date.font,
                date.size_pt,
                date.colour,
                date.lines,
                "footer date",
            )?;
        }
        if let Some(header) = layout.header {
            draw_text_lines(
                &mut page,
                header.font,
                header.size_pt,
                header.colour,
                header.lines,
                "header",
            )?;
        }
        if let Some(footer) = layout.footer {
            draw_text_lines(
                &mut page,
                footer.font,
                footer.size_pt,
                footer.colour,
                footer.lines,
                "footer",
            )?;
        }
        if let Some(title) = layout.title {
            draw_text_lines(
                &mut page,
                title.font,
                title.size_pt,
                title.colour,
                title.lines,
                "title",
            )?;
        }
        if let Some(stamp) = &stamp {
            page.add_image("native_stamp_image", stamp.image.clone());
            let bounds = stamp.image_bounds;
            page.draw_image(
                "native_stamp_image",
                mm_to_points(bounds.left),
                mm_to_points(bounds.bottom),
                mm_to_points(bounds.right - bounds.left),
                mm_to_points(bounds.top - bounds.bottom),
            )
            .map_err(|error| format!("Cannot draw native stamp image: {error}"))?;
            for (text, kind) in [
                (stamp.heading.as_ref(), "stamp heading"),
                (stamp.caption.as_ref(), "stamp caption"),
            ] {
                if let Some(text) = text {
                    draw_text_lines(
                        &mut page,
                        text.font.clone(),
                        text.size_pt,
                        text.colour,
                        text.lines.clone(),
                        kind,
                    )?;
                }
            }
        }
        if let Some(geometry) = geometry {
            for (bounds, colour) in [
                (geometry.frame_bounds(), Color::rgb(0.0, 0.0, 0.0)),
                (geometry.physical_bounds(), Color::rgb(0.0, 0.3, 0.9)),
            ] {
                page.graphics()
                    .set_stroke_color(colour)
                    .set_line_width(mm_to_points(0.1))
                    .rect(
                        mm_to_points(bounds.left),
                        mm_to_points(bounds.bottom),
                        mm_to_points(bounds.right - bounds.left),
                        mm_to_points(bounds.top - bounds.bottom),
                    )
                    .stroke();
            }
        }

        document.add_page(page);
    }
    document.save(output).map_err(|error| {
        format!(
            "Cannot write native PDF proof {}: {error}",
            output.display()
        )
    })
}

fn draw_text_lines(
    page: &mut Page,
    font: oxidize_pdf::text::Font,
    size_pt: f64,
    colour: NamedColour,
    lines: Vec<PositionedTextLine>,
    kind: &str,
) -> Result<()> {
    let [red, green, blue] = colour.rgb();
    for line in lines {
        page.text()
            .set_font(font.clone(), size_pt)
            .set_fill_color(Color::rgb(red, green, blue))
            .at(mm_to_points(line.left_mm), mm_to_points(line.baseline_mm))
            .write(&line.text)
            .map_err(|error| format!("Cannot draw native page {kind}: {error}"))?;
    }
    Ok(())
}
