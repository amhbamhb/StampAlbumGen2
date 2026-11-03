use crate::native_pdf::{commands::arguments, document::Bounds, units::mm_to_points, Result};

/// Omitted ALBUM_PAGES_BORDER means no border and no reserved border space.
#[derive(Clone, Copy, Debug, Default)]
pub struct PageBorder {
    widths_mm: [f64; 3],
    gap_mm: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct BorderStroke {
    pub bounds: Bounds,
    pub width_mm: f64,
}

impl PageBorder {
    pub fn parse(arguments_mm: &[f64]) -> Result<Self> {
        if !matches!(arguments_mm.len(), 3 | 4) {
            return Err("ALBUM_PAGES_BORDER requires three or four numeric arguments".into());
        }
        let expanded;
        let values = if let [outer, inner, gap] = arguments_mm {
            expanded = [*outer, *inner, 0.0, *gap];
            &expanded
        } else {
            arguments_mm
        };
        let [outer, inner1, inner2, gap_mm] = arguments("ALBUM_PAGES_BORDER", values, false)?;
        // Include disabled slots: AlbumEasy retains their drawing gaps.
        let maximum_inset = outer + inner1 + inner2 + 2.0 * gap_mm;
        if !mm_to_points(maximum_inset).is_finite() {
            return Err("ALBUM_PAGES_BORDER overflows PDF point dimensions".into());
        }
        Ok(Self {
            widths_mm: [outer, inner1, inner2],
            gap_mm,
        })
    }

    pub fn occupancy_mm(self) -> f64 {
        // Unlike drawing slots, content reserves a gap only for enabled inner lines.
        self.widths_mm[0]
            + self.widths_mm[1..]
                .iter()
                .filter(|width| **width > 0.0)
                .map(|width| width + self.gap_mm)
                .sum::<f64>()
    }

    pub fn strokes(self, margin_bounds: Bounds) -> Result<Vec<BorderStroke>> {
        let mut strokes = Vec::new();
        let mut inset = 0.0;
        for width_mm in self.widths_mm {
            if width_mm > 0.0 {
                let center = inset + width_mm / 2.0;
                // Validate the entire stroke footprint, not just its centreline.
                margin_bounds.inset(inset + width_mm, inset + width_mm)?;
                strokes.push(BorderStroke {
                    bounds: margin_bounds.inset(center, center)?,
                    width_mm,
                });
            }
            inset += width_mm + self.gap_mm;
        }
        Ok(strokes)
    }
}
