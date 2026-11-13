use super::album_pages_header::{AlbumPagesHeader, HeaderLayout};
use crate::native_pdf::{document::Bounds, numbering::PageNumbering, Result};

/// ALBUM_PAGES_HEADER_NUMBER shares header geometry, not footer label state.
#[derive(Clone, Debug)]
pub struct AlbumPagesHeaderNumber {
    style: AlbumPagesHeader,
    numbering: PageNumbering,
}

impl AlbumPagesHeaderNumber {
    pub fn new(style: AlbumPagesHeader, start: i32, before: &str, after: &str) -> Result<Self> {
        style
            .with_text(&format!("{before}{start}{after}"))
            .map_err(|error| format!("native_pdf.header_number.before/after: {error}"))?;
        Ok(Self {
            style,
            numbering: PageNumbering::new(start, before, after),
        })
    }

    pub fn layout(
        &self,
        margin: Bounds,
        physical_page: Bounds,
        page: usize,
        total: usize,
    ) -> Result<HeaderLayout> {
        let text = self.numbering.text(
            page,
            total,
            "ALBUM_PAGES_HEADER_NUMBER",
            "native_pdf.header_number.start",
        )?;
        self.style.with_text(&text)
            .and_then(|header| header.layout(margin, physical_page, page))
            .map_err(|error| format!("ALBUM_PAGES_HEADER_NUMBER (native_pdf.header_number font/size_pt/position/start/before/after; native_pdf.header.padding_mm/minimum_height_mm): {error}"))
    }
}
