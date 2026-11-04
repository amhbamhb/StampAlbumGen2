use super::album_pages_footer::{AlbumPagesFooter, FooterLayout};
use crate::native_pdf::{document::Bounds, Result};

/// ALBUM_PAGES_FOOTER_NUMBER uses an explicit style and final document count.
#[derive(Clone, Debug)]
pub struct AlbumPagesFooterNumber {
    style: AlbumPagesFooter,
    start: i32,
    before: String,
    after: String,
}

impl AlbumPagesFooterNumber {
    pub fn new(style: AlbumPagesFooter, start: i32, before: &str, after: &str) -> Result<Self> {
        style
            .with_text(&format!("{before}{start}{after}"))
            .map_err(|error| format!("album_pages_footer_number_before/after: {error}"))?;
        Ok(Self {
            style,
            start,
            before: before.into(),
            after: after.into(),
        })
    }

    pub fn layout(
        &self,
        margin: Bounds,
        physical_page: Bounds,
        page_number: usize,
        total_pages: usize,
    ) -> Result<FooterLayout> {
        if total_pages == 0
            || u32::try_from(total_pages).is_err()
            || page_number == 0
            || page_number > total_pages
        {
            return Err("ALBUM_PAGES_FOOTER_NUMBER requires a positive final PDF page count and a physical page within it".into());
        }
        let offset = i64::try_from(page_number - 1)
            .map_err(|_| "album_pages_footer_number_start overflows printed numbering")?;
        let number = i64::from(self.start)
            .checked_add(offset)
            .and_then(|n| i32::try_from(n).ok())
            .ok_or("album_pages_footer_number_start overflows signed 32-bit printed numbering")?;
        let text = format!("{}{number}{}", self.before, self.after)
            .replace("$PAGES$", &total_pages.to_string());
        self.style.with_text(&text)
            .and_then(|footer| footer.layout(margin,physical_page,page_number))
            .map_err(|error| format!("ALBUM_PAGES_FOOTER_NUMBER (album_pages_footer_number_font/size/position/start/before/after): {error}"))
    }
}
