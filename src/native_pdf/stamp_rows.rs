//! Frame-only, ordered single-page fitting. Text and pagination are separate.

use super::{
    commands::page::album_pages_spacing::Spacing, document::Bounds, stamp_box::StampBoxGeometry,
    Result,
};

/// Left-aligned rows with top-aligned frames. No stamp is scaled or rotated.
/// A frame wider than the content area, or a row below it, is an explicit error.
pub fn fit_known_size_rows(
    content: Bounds,
    stamps: &[StampBoxGeometry],
    spacing: Spacing,
) -> Result<Vec<Vec<StampBoxGeometry>>> {
    StampBoxGeometry::from_total_additions(content, 0.0, 0.0, 0.0)
        .map_err(|error| format!("native_pdf.stamp_rows content bounds: {error}"))?;
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut left = content.left;
    let mut top = content.top;
    let mut row_height: f64 = 0.0;
    for (index, stamp) in stamps.iter().enumerate() {
        let frame = stamp.frame_bounds();
        let width = frame.right - frame.left;
        let height = frame.top - frame.bottom;
        if width > content.right - content.left {
            return Err(format!(
                "native_pdf.stamp_rows item {} frame width {width} mm exceeds content width",
                index + 1
            ));
        }
        if !row.is_empty() && width > content.right - left {
            rows.push(std::mem::take(&mut row));
            top = spacing.next_top(top - row_height)?;
            left = content.left;
            row_height = 0.0;
        }
        if height > top - content.bottom {
            return Err(format!(
                "native_pdf.stamp_rows item {} frame height {height} mm exceeds remaining page height; pagination is not implemented",
                index + 1
            ));
        }
        let physical = stamp.physical_bounds();
        let x_clearance = stamp.horizontal_clearance_mm();
        let y_clearance = stamp.vertical_clearance_mm();
        let placed = StampBoxGeometry::from_total_additions(
            Bounds {
                left: left + x_clearance,
                right: left + x_clearance + (physical.right - physical.left),
                bottom: top - y_clearance - (physical.top - physical.bottom),
                top: top - y_clearance,
            },
            x_clearance * 2.0,
            y_clearance * 2.0,
            0.0,
        )?;
        left = placed.frame_bounds().right + spacing.horizontal_mm();
        row_height = row_height.max(height);
        row.push(placed);
    }
    if !row.is_empty() {
        rows.push(row);
    }
    Ok(rows)
}
