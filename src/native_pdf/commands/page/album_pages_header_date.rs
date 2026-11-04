use super::album_pages_header::{AlbumPagesHeader, HeaderLayout};
use crate::native_pdf::{
    date::{DateFormat, DateSource, DateText},
    document::Bounds,
    Result,
};
use chrono::NaiveDate;

/// ALBUM_PAGES_HEADER_DATE uses one document date and explicit header geometry.
#[derive(Clone, Debug)]
pub struct AlbumPagesHeaderDate {
    style: AlbumPagesHeader,
    text: DateText,
}

impl AlbumPagesHeaderDate {
    pub fn new(
        style: AlbumPagesHeader,
        format: DateFormat,
        separator: &str,
        before: &str,
        after: &str,
        source: DateSource,
    ) -> Result<Self> {
        let text = DateText::new(
            format,
            separator,
            before,
            after,
            source,
            "native_pdf.header_date",
            |text| style.with_text(text).map(|_| ()),
        )?;
        Ok(Self { style, text })
    }

    pub fn source(&self) -> DateSource {
        self.text.source()
    }

    pub fn layout(
        &self,
        margin: Bounds,
        physical_page: Bounds,
        page: usize,
        generation_date: Option<NaiveDate>,
    ) -> Result<HeaderLayout> {
        let text = self.text.text(generation_date)?;
        self.style.with_text(&text)
            .and_then(|header| header.layout(margin, physical_page, page))
            .map_err(|error| format!("ALBUM_PAGES_HEADER_DATE (native_pdf.header_date font/size_pt/position/format/separator/before/after; native_pdf.header.padding_mm/minimum_height_mm): {error}"))
    }
}
