use super::album_pages_footer::{AlbumPagesFooter, FooterLayout};
use crate::native_pdf::{
    Result,
    document::Bounds,
    numbering::{NumberingContext, PageNumbering},
};

/// ALBUM_PAGES_FOOTER_NUMBER uses an explicit style and final document count.
#[derive(Clone, Debug)]
pub struct AlbumPagesFooterNumber {
    style: AlbumPagesFooter,
    numbering: PageNumbering,
}

impl AlbumPagesFooterNumber {
    pub fn new(style: AlbumPagesFooter, start: i32, before: &str, after: &str) -> Result<Self> {
        style
            .with_text(&format!("{before}{start}{after}"))
            .map_err(|error| format!("album_pages_footer_number_before/after: {error}"))?;
        Ok(Self {
            style,
            numbering: PageNumbering::new(start, before, after),
        })
    }

    pub fn layout(
        &self,
        margin: Bounds,
        physical_page: Bounds,
        page_number: usize,
        total_pages: usize,
    ) -> Result<FooterLayout> {
        let text = self.numbering.text(
            page_number,
            total_pages,
            "ALBUM_PAGES_FOOTER_NUMBER",
            "album_pages_footer_number_start",
        )?;
        self.layout_text(margin, physical_page, page_number, &text)
    }

    pub(crate) fn layout_with_numbering_context(
        &self,
        margin: Bounds,
        physical_page: Bounds,
        page_number: usize,
        context: NumberingContext,
    ) -> Result<FooterLayout> {
        let text = self.numbering.text_with_context(
            context,
            "ALBUM_PAGES_FOOTER_NUMBER",
            "album_pages_footer_number_start",
        )?;
        self.layout_text(margin, physical_page, page_number, &text)
    }

    fn layout_text(
        &self,
        margin: Bounds,
        physical_page: Bounds,
        page_number: usize,
        text: &str,
    ) -> Result<FooterLayout> {
        self.style.with_text(text)
            .and_then(|footer| footer.layout(margin,physical_page,page_number))
            .map_err(|error| format!("ALBUM_PAGES_FOOTER_NUMBER (album_pages_footer_number_font/size/position/start/before/after): {error}"))
    }
}
