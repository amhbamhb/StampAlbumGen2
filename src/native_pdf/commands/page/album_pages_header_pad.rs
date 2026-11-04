use crate::native_pdf::{units::mm_to_points, Result};

/// ALBUM_PAGES_HEADER_PAD shifts the whole header away from the top border.
#[derive(Clone, Copy, Debug)]
pub struct HeaderPadding(f64);

impl HeaderPadding {
    pub fn parse(arguments_mm: &[f64]) -> Result<Self> {
        let [padding] = arguments_mm else {
            return Err("ALBUM_PAGES_HEADER_PAD requires one numeric padding_mm argument".into());
        };
        if !padding.is_finite() || !mm_to_points(*padding).is_finite() {
            return Err("ALBUM_PAGES_HEADER_PAD padding_mm must be finite millimetres".into());
        }
        Ok(Self(*padding))
    }

    pub fn mm(self) -> f64 {
        self.0
    }
}
