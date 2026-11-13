use crate::native_pdf::{commands::arguments, units::mm_to_points, Result};

#[derive(Clone, Copy, Debug)]
pub struct PageSize {
    width_mm: f64,
    height_mm: f64,
}

impl PageSize {
    pub fn parse(arguments_mm: &[f64]) -> Result<Self> {
        let [width_mm, height_mm] = arguments("ALBUM_PAGES_SIZE", arguments_mm, true)?;
        if !mm_to_points(width_mm).is_finite() || !mm_to_points(height_mm).is_finite() {
            return Err("ALBUM_PAGES_SIZE overflows PDF point dimensions".into());
        }
        Ok(Self {
            width_mm,
            height_mm,
        })
    }

    pub fn width_mm(self) -> f64 {
        self.width_mm
    }

    pub fn height_mm(self) -> f64 {
        self.height_mm
    }
}
