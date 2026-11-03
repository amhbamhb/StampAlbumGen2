use super::{document::PageSettings, proof::GeometryProofSettings, units::mm_to_points, Result};
use oxidize_pdf::{Color, Document, Page};
use std::path::Path;

/// Isolated proof: configured border and optional configured content-area guide.
/// This is not an album renderer; text and rows are pending.
pub fn write_page_geometry_proof(
    settings: &PageSettings,
    proof: &GeometryProofSettings,
    output: &Path,
) -> Result<()> {
    proof.validate()?;
    let layouts = (1..=proof.page_count)
        .map(|number| {
            let layout = settings.resolve(number)?;
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
        if let Some(title) = layout.title {
            let [red, green, blue] = title.colour.rgb();
            for line in title.lines {
                page.text()
                    .set_font(title.font.clone(), title.size_pt)
                    .set_fill_color(Color::rgb(red, green, blue))
                    .at(mm_to_points(line.left_mm), mm_to_points(line.baseline_mm))
                    .write(&line.text)
                    .map_err(|error| format!("Cannot draw native page title: {error}"))?;
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
