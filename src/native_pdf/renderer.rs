use super::{document::PageSettings, units::mm_to_points, Result};
use oxidize_pdf::{Color, Document, Page};
use std::path::Path;

/// Isolated proof: configured border plus a 0.1 mm content-area guide.
/// This is not an album renderer; text and rows are pending.
pub fn write_page_geometry_proof(
    settings: &PageSettings,
    pages: usize,
    output: &Path,
) -> Result<()> {
    if pages == 0 {
        return Err("Native PDF proof requires at least one page".into());
    }
    let layouts = (1..=pages)
        .map(|number| settings.resolve(number))
        .collect::<Result<Vec<_>>>()?;
    let mut document = Document::new();
    for layout in layouts {
        let mut page = Page::new(
            mm_to_points(layout.size.width_mm()),
            mm_to_points(layout.size.height_mm()),
        );
        let mut strokes = settings.border().strokes(layout.margin_bounds)?;
        let border_count = strokes.len();
        strokes.push(super::commands::page::album_pages_border::BorderStroke {
            bounds: layout.content_bounds,
            width_mm: 0.1,
        });
        for (index, stroke) in strokes.iter().enumerate() {
            let bounds = stroke.bounds;
            let [red, green, blue] = if index < border_count {
                settings.border_colour().rgb()
            } else {
                [0.0; 3]
            };
            page.graphics()
                .set_stroke_color(Color::rgb(red, green, blue))
                .set_line_width(mm_to_points(stroke.width_mm))
                .rect(
                    mm_to_points(bounds.left),
                    mm_to_points(bounds.bottom),
                    mm_to_points(bounds.right - bounds.left),
                    mm_to_points(bounds.top - bounds.bottom),
                )
                .stroke();
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
