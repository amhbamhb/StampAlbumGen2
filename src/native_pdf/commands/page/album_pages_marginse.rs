use super::album_pages_margins::Margins;
use crate::native_pdf::Result;

#[derive(Clone, Copy, Debug)]
pub struct EvenMargins(pub(crate) Margins);

impl EvenMargins {
    pub fn parse(arguments_mm: &[f64]) -> Result<Self> {
        Margins::from_arguments("ALBUM_PAGES_MARGINSE", arguments_mm).map(Self)
    }
}
