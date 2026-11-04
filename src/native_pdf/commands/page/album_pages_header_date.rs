use super::album_pages_header::{AlbumPagesHeader, HeaderLayout};
use crate::native_pdf::{
    date::{DateFormat, DateSource},
    document::Bounds,
    Result,
};
use chrono::NaiveDate;

/// ALBUM_PAGES_HEADER_DATE uses one document date and explicit header geometry.
#[derive(Clone, Debug)]
pub struct AlbumPagesHeaderDate {
    style: AlbumPagesHeader,
    format: DateFormat,
    separator: String,
    before: String,
    after: String,
    source: DateSource,
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
        // Empty fragments are valid, but unsupported characters must fail even
        // for a disabled date. Reuse measurement's strict encoding validation.
        for (key, value) in [
            ("separator", separator),
            ("before", before),
            ("after", after),
        ] {
            style
                .with_text(&format!("X{value}"))
                .map_err(|error| format!("native_pdf.header_date.{key}: {error}"))?;
        }
        Ok(Self {
            style,
            format,
            separator: separator.into(),
            before: before.into(),
            after: after.into(),
            source,
        })
    }

    pub fn source(&self) -> DateSource {
        self.source
    }

    pub fn layout(
        &self,
        margin: Bounds,
        physical_page: Bounds,
        page: usize,
        generation_date: Option<NaiveDate>,
    ) -> Result<HeaderLayout> {
        let date = self.source.resolve(generation_date)?;
        let date = self.format.format(date, &self.separator)?;
        let text = format!("{}{date}{}", self.before, self.after);
        self.style.with_text(&text)
            .and_then(|header| header.layout(margin, physical_page, page))
            .map_err(|error| format!("ALBUM_PAGES_HEADER_DATE (native_pdf.header_date font/size_pt/position/format/separator/before/after; native_pdf.header.padding_mm/minimum_height_mm): {error}"))
    }
}
