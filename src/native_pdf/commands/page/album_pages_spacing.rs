use crate::native_pdf::{Result, commands::arguments};

#[derive(Clone, Copy, Debug)]
pub struct Spacing {
    horizontal_mm: f64,
    vertical_mm: f64,
}

impl Spacing {
    pub fn parse(arguments_mm: &[f64]) -> Result<Self> {
        let [horizontal_mm, vertical_mm] = arguments("ALBUM_PAGES_SPACING", arguments_mm, false)?;
        Ok(Self {
            horizontal_mm,
            vertical_mm,
        })
    }

    pub fn horizontal_mm(self) -> f64 {
        self.horizontal_mm
    }

    pub fn vertical_mm(self) -> f64 {
        self.vertical_mm
    }

    /// Apply one inter-element gap, not an implicit top-of-page gap.
    pub fn next_top(self, previous_bottom_mm: f64) -> Result<f64> {
        let next = previous_bottom_mm - self.vertical_mm;
        if !previous_bottom_mm.is_finite() || !next.is_finite() {
            return Err("ALBUM_PAGES_SPACING requires finite cursor coordinates".into());
        }
        Ok(next)
    }
}
