//! Frame-only, ordered single-page fitting. Text and pagination are separate.

use super::{
    Result, commands::page::album_pages_spacing::Spacing, document::Bounds,
    stamp_box::StampBoxGeometry,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowAlignment {
    Left,
    Center,
    Right,
    Justified,
}

impl RowAlignment {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            "justified" => Ok(Self::Justified),
            _ => Err("native_pdf.rows.alignment must be left, center, right or justified".into()),
        }
    }

    pub(crate) fn positions(
        self,
        content: Bounds,
        widths: &[f64],
        minimum_gap: f64,
    ) -> Result<Vec<f64>> {
        if widths.is_empty() {
            return Ok(Vec::new());
        }
        let count = widths.len();
        let sum: f64 = widths.iter().sum();
        let available = content.right - content.left;
        let used = sum + minimum_gap * (count - 1) as f64;
        if !used.is_finite() || used > available + 1e-9 {
            return Err("native_pdf.rows aligned row exceeds usable width".into());
        }
        let spare = (available - used).max(0.0);
        let (offset, gap) = match self {
            Self::Left => (0.0, minimum_gap),
            Self::Right => (spare, minimum_gap),
            Self::Justified if count > 1 => (0.0, minimum_gap + spare / (count - 1) as f64),
            Self::Justified => (spare / 2.0, minimum_gap),
            Self::Center => {
                let even = (available - sum) / (count + 1) as f64;
                if count == 1 || even >= minimum_gap {
                    (even, even)
                } else {
                    // Nearly full rows retain minimum internal gaps; only the
                    // remaining width can be shared by the two outer edges.
                    (spare / 2.0, minimum_gap)
                }
            }
        };
        let mut left = content.left + offset;
        Ok(widths
            .iter()
            .map(|width| {
                let result = left;
                left += width + gap;
                result
            })
            .collect())
    }
}

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
    fit_known_size_rows_with_alignment(content, stamps, spacing, RowAlignment::Left)
}

pub fn fit_known_size_rows_with_alignment(
    content: Bounds,
    stamps: &[StampBoxGeometry],
    spacing: Spacing,
    alignment: RowAlignment,
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
    for row in &mut rows {
        let widths: Vec<_> = row
            .iter()
            .map(|g| {
                let b = g.frame_bounds();
                b.right - b.left
            })
            .collect();
        let positions = alignment.positions(content, &widths, spacing.horizontal_mm())?;
        for (geometry, left) in row.iter_mut().zip(positions) {
            *geometry = geometry.at_frame_top_left(left, geometry.frame_bounds().top)?;
        }
    }
    Ok(rows)
}
