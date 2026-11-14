use super::album_pages_footer::{AlbumPagesFooter, FooterLayout};
use crate::native_pdf::{
    Result,
    date::{DateFormat, DateSource, DateText},
    document::Bounds,
};
use chrono::NaiveDate;

/// ALBUM_PAGES_FOOTER_DATE shares document date text and measured footer geometry.
#[derive(Clone, Debug)]
pub struct AlbumPagesFooterDate {
    style: AlbumPagesFooter,
    text: DateText,
}

impl AlbumPagesFooterDate {
    pub fn new(
        style: AlbumPagesFooter,
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
            "native_pdf.footer_date",
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
    ) -> Result<FooterLayout> {
        let text = self.text.text(generation_date)?;
        self.style.with_text(&text)
            .and_then(|footer| footer.layout(margin, physical_page, page))
            .map_err(|error| format!("ALBUM_PAGES_FOOTER_DATE (native_pdf.footer_date font/size_pt/position/format/separator/before/after; native_pdf.footer.padding_mm/minimum_height_mm): {error}"))
    }
}
