use super::{document::PageSettings, units::mm_to_points, Result};
use oxidize_pdf::{Document, Page};
use std::path::Path;

/// Isolated proof: 0.2 mm margin border plus a 0.1 mm content-area guide.
/// This is not an album renderer; text, rows and the border command are pending.
pub fn write_page_geometry_proof(
    settings: &PageSettings,
    pages: usize,
    output: &Path,
) -> Result<()> {
    if pages == 0 {
        return Err("Native PDF proof requires at least one page".into());
    }
    let layouts = (1..=pages)
        .map(|number| settings.resolve(number, 0.2))
        .collect::<Result<Vec<_>>>()?;
    let mut document = Document::new();
    for layout in layouts {
        let mut page = Page::new(
            mm_to_points(layout.size.width_mm()),
            mm_to_points(layout.size.height_mm()),
        );
        for (bounds, stroke) in [
            (layout.margin_bounds.inset(0.1, 0.1)?, 0.2),
            (layout.content_bounds, 0.1),
        ] {
            page.graphics()
                .set_line_width(mm_to_points(stroke))
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
