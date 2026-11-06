//! Frame-only, ordered single-page fitting. Text and pagination are separate.

use super::{
    commands::page::album_pages_spacing::Spacing, document::Bounds, stamp_box::StampBoxGeometry,
    Result,
};

pub(crate) fn row_prefix_len(
    width: f64,
    widths: impl Iterator<Item = f64>,
    spacing: Spacing,
) -> usize {
    let mut used = 0.0;
    let mut count = 0;
    for item_width in widths {
        let next =
            used + if count == 0 {
                0.0
            } else {
                spacing.horizontal_mm()
            } + item_width;
        if next > width {
            break;
        }
        used = next;
        count += 1;
    }
    count
}

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
    let mut row_end = 0;
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
        if index == row_end {
            if !row.is_empty() {
                rows.push(std::mem::take(&mut row));
                top = spacing.next_top(top - row_height)?;
                left = content.left;
                row_height = 0.0;
            }
            row_end = index
                + row_prefix_len(
                    content.right - content.left,
                    stamps[index..].iter().map(|s| {
                        let b = s.frame_bounds();
                        b.right - b.left
                    }),
                    spacing,
                );
        }
        if height > top - content.bottom {
            return Err(format!(
                "native_pdf.stamp_rows item {} frame height {height} mm exceeds remaining page height; pagination is not implemented",
                index + 1
            ));
        }
        let placed = stamp.at_frame_top_left(left, top)?;
        left = placed.frame_bounds().right + spacing.horizontal_mm();
        row_height = row_height.max(height);
        row.push(placed);
    }
    if !row.is_empty() {
        rows.push(row);
    }
    Ok(rows)
}
