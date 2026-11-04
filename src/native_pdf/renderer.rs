use super::{
    colour::NamedColour, document::PageSettings, proof::GeometryProofSettings,
    text::PositionedTextLine, units::mm_to_points, Result,
};
use oxidize_pdf::{Color, Document, Page};
use std::path::Path;

/// Isolated proof: configured border and optional configured content-area guide.
/// Optional measured title/header/footer text is supported; stamp rows are pending.
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
    let mut document = Document::new();
    for layout in layouts {
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
