use crate::native_pdf::{Result, commands::arguments};

#[derive(Clone, Copy, Debug)]
pub struct Margins {
    pub(crate) left: f64,
    pub(crate) right: f64,
    pub(crate) top: f64,
    pub(crate) bottom: f64,
}

impl Margins {
    pub fn parse(arguments_mm: &[f64]) -> Result<Self> {
        Self::from_arguments("ALBUM_PAGES_MARGINS", arguments_mm)
    }

    pub(crate) fn from_arguments(command: &str, arguments_mm: &[f64]) -> Result<Self> {
        let [left, right, top, bottom] = arguments(command, arguments_mm, false)?;
        Ok(Self {
            left,
            right,
            top,
            bottom,
        })
    }
}
