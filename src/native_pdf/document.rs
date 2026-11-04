use super::{
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder,
        album_pages_footer::{AlbumPagesFooter, FooterLayout},
        album_pages_footer_number::AlbumPagesFooterNumber,
        album_pages_header::{AlbumPagesHeader, HeaderLayout},
        album_pages_header_date::AlbumPagesHeaderDate,
        album_pages_header_number::AlbumPagesHeaderNumber,
        album_pages_margins::Margins,
        album_pages_marginse::EvenMargins,
        album_pages_size::PageSize,
        album_pages_spacing::Spacing,
        album_pages_title::{AlbumPagesTitle, TitleLayout},
    },
    date::DateSource,
    text::text_lines_overlap,
    Result,
};

/// Millimetres in PDF bottom-left coordinates.
#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub left: f64,
    pub bottom: f64,
    pub right: f64,
    pub top: f64,
}

impl Bounds {
    pub fn inset(self, horizontal_mm: f64, vertical_mm: f64) -> Result<Self> {
        if !horizontal_mm.is_finite()
            || !vertical_mm.is_finite()
            || horizontal_mm < 0.0
            || vertical_mm < 0.0
        {
            return Err("Page inset must be finite and nonnegative".into());
        }
        let bounds = Self {
            left: self.left + horizontal_mm,
            right: self.right - horizontal_mm,
            bottom: self.bottom + vertical_mm,
            top: self.top - vertical_mm,
        };
        bounds.validate()?;
        Ok(bounds)
    }

    fn validate(self) -> Result<()> {
        if ![self.left, self.bottom, self.right, self.top]
            .iter()
            .all(|v| v.is_finite())
            || self.left >= self.right
            || self.bottom >= self.top
        {
            return Err("Page settings leave no finite usable area".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct PageLayout {
    pub size: PageSize,
    pub margin_bounds: Bounds,
    pub content_bounds: Bounds,
    pub title: Option<TitleLayout>,
    pub header: Option<HeaderLayout>,
    pub header_number: Option<HeaderLayout>,
    pub header_date: Option<HeaderLayout>,
    pub footer: Option<FooterLayout>,
    pub footer_number: Option<FooterLayout>,
}

#[derive(Clone, Debug)]
pub struct PageSettings {
    size: PageSize,
    margins: Margins,
    even_margins: Option<EvenMargins>,
    spacing: Spacing,
    border: PageBorder,
    border_colour: AlbumBorderColour,
    border_line_colours: [AlbumBorderColour; 3],
    title: Option<AlbumPagesTitle>,
    header: Option<AlbumPagesHeader>,
    header_number: Option<AlbumPagesHeaderNumber>,
    header_date: Option<AlbumPagesHeaderDate>,
    footer: Option<AlbumPagesFooter>,
    footer_number: Option<AlbumPagesFooterNumber>,
}

impl PageSettings {
    pub fn new(
        size: PageSize,
        margins: Margins,
        spacing: Spacing,
        border: PageBorder,
        border_colour: AlbumBorderColour,
    ) -> Self {
        Self {
            size,
            margins,
            even_margins: None,
            spacing,
            border,
            border_colour,
            border_line_colours: [border_colour; 3],
            title: None,
            header: None,
            header_number: None,
            header_date: None,
            footer: None,
            footer_number: None,
        }
    }

    pub fn set_even_margins(&mut self, margins: EvenMargins) {
        self.even_margins = Some(margins);
    }

    pub fn set_title(&mut self, title: AlbumPagesTitle) {
        self.title = Some(title);
    }

    pub fn set_header(&mut self, header: AlbumPagesHeader) {
        self.header = Some(header);
    }

    pub fn set_header_number(&mut self, number: AlbumPagesHeaderNumber) {
        self.header_number = Some(number);
    }

    pub fn set_header_date(&mut self, date: AlbumPagesHeaderDate) {
        self.header_date = Some(date);
    }

    /// Capture local calendar date once, outside the page loop. Fixed/disabled
    /// dates do not consult the clock; callers can reuse this exact context.
    pub fn capture_generation_date(&self) -> Option<chrono::NaiveDate> {
        match self.header_date.as_ref().map(AlbumPagesHeaderDate::source) {
            Some(DateSource::Generation) => Some(chrono::Local::now().date_naive()),
            _ => None,
        }
    }

    pub fn set_footer(&mut self, footer: AlbumPagesFooter) {
        self.footer = Some(footer);
    }

    pub fn set_footer_number(&mut self, number: AlbumPagesFooterNumber) {
        self.footer_number = Some(number);
    }

    pub fn spacing(&self) -> Spacing {
        self.spacing
    }

    pub fn set_border(&mut self, border: PageBorder) {
        self.border = border;
    }

    pub fn border(&self) -> PageBorder {
        self.border
    }

    pub fn set_border_colour(&mut self, colour: AlbumBorderColour) {
        self.border_colour = colour;
        self.border_line_colours = [colour; 3];
    }

    /// Shared AlbumEasy command value; use border_line_colour for native strokes.
    pub fn border_colour(&self) -> AlbumBorderColour {
        self.border_colour
    }

    /// Native per-line overrides, independent of the shared AlbumEasy command.
    pub fn set_border_line_colours(&mut self, colours: [AlbumBorderColour; 3]) {
        self.border_line_colours = colours;
    }

    pub fn border_line_colour(
        &self,
        line: super::commands::page::album_pages_border::BorderLine,
    ) -> AlbumBorderColour {
        use super::commands::page::album_pages_border::BorderLine;
        match line {
            BorderLine::Outer => self.border_line_colours[0],
            BorderLine::Inner1 => self.border_line_colours[1],
            BorderLine::Inner2 => self.border_line_colours[2],
        }
    }

    /// Physical page numbers are one-based; no footer-label parity is inferred.
    pub fn resolve(&self, page_number: usize) -> Result<PageLayout> {
        if self.footer_number.is_some() {
            return Err("ALBUM_PAGES_FOOTER_NUMBER requires the final page count; use resolve_with_page_count".into());
        }
        if self.header_number.is_some() {
            return Err("ALBUM_PAGES_HEADER_NUMBER requires the final page count; use resolve_with_page_count".into());
        }
        self.resolve_page(page_number, None, None)
    }

    pub fn resolve_with_page_count(
        &self,
        page_number: usize,
        total_pages: usize,
    ) -> Result<PageLayout> {
        self.resolve_with_context(page_number, total_pages, None)
    }

    pub fn resolve_with_context(
        &self,
        page_number: usize,
        total_pages: usize,
        generation_date: Option<chrono::NaiveDate>,
    ) -> Result<PageLayout> {
        if total_pages == 0
            || u32::try_from(total_pages).is_err()
            || page_number == 0
            || page_number > total_pages
        {
            return Err(
                "Native PDF physical page must be within a positive final PDF page count".into(),
            );
        }
        self.resolve_page(page_number, Some(total_pages), generation_date)
    }

    fn resolve_page(
        &self,
        page_number: usize,
        total_pages: Option<usize>,
        generation_date: Option<chrono::NaiveDate>,
    ) -> Result<PageLayout> {
        if page_number == 0 {
            return Err("Native PDF page number must be one-based".into());
        }
        let margins = match (page_number.is_multiple_of(2), self.even_margins) {
            (true, Some(margins)) => margins.0,
            _ => self.margins,
        };
        let margin_bounds = Bounds {
            left: margins.left,
            bottom: margins.bottom,
            right: self.size.width_mm() - margins.right,
            top: self.size.height_mm() - margins.top,
        };
        margin_bounds.validate()?;
        self.border.strokes(margin_bounds)?;
        let border_occupancy_mm = self.border.occupancy_mm();
        // Vertical spacing belongs between elements, not at page edges.
        let mut content_bounds = margin_bounds
            .inset(border_occupancy_mm, border_occupancy_mm)?
            .inset(self.spacing.horizontal_mm(), 0.0)?;
        let physical_page = Bounds {
            left: 0.0,
            bottom: 0.0,
            right: self.size.width_mm(),
            top: self.size.height_mm(),
        };
        let header = self
            .header
            .as_ref()
            .map(|header| header.layout(margin_bounds, physical_page, page_number))
            .transpose()?;
        if let Some(header) = &header {
            content_bounds.top = content_bounds.top.min(header.reservation_bottom_mm);
            content_bounds
                .validate()
                .map_err(|error| format!("native_pdf.header.minimum_height_mm: {error}"))?;
        }
        let header_number = self
            .header_number
            .as_ref()
            .map(|number| {
                number.layout(
                    margin_bounds,
                    physical_page,
                    page_number,
                    total_pages.ok_or("ALBUM_PAGES_HEADER_NUMBER requires the final page count")?,
                )
            })
            .transpose()?;
        if let Some(number) = &header_number {
            content_bounds.top = content_bounds.top.min(number.reservation_bottom_mm);
            content_bounds.validate().map_err(|error| {
                format!("ALBUM_PAGES_HEADER_NUMBER/native_pdf.header.minimum_height_mm: {error}")
            })?;
        }
        if let (Some(header), Some(number)) = (&header, &header_number) {
            if text_lines_overlap(&header.lines, header.bounds, &number.lines, number.bounds)? {
                // TODO(native-pdf-ui): Show conflicting elements, physical page and
                // TOML keys; offer correction in the UI rather than silently moving text.
                return Err(format!("Native PDF page {page_number}: native_pdf.header.text overlaps ALBUM_PAGES_HEADER_NUMBER; adjust album_pages_header_position or native_pdf.header_number.position, native_pdf.header.text, native_pdf.header_number.before/after or font/size_pt values"));
            }
        }
        let header_date = self
            .header_date
            .as_ref()
            .map(|date| date.layout(margin_bounds, physical_page, page_number, generation_date))
            .transpose()?;
        if let Some(date) = &header_date {
            content_bounds.top = content_bounds.top.min(date.reservation_bottom_mm);
            content_bounds.validate().map_err(|error| {
                format!("ALBUM_PAGES_HEADER_DATE/native_pdf.header.minimum_height_mm: {error}")
            })?;
            for (element, layout, keys) in [
                (
                    "native_pdf.header.text",
                    header.as_ref(),
                    "album_pages_header_position or native_pdf.header.font/size_pt/text",
                ),
                (
                    "ALBUM_PAGES_HEADER_NUMBER",
                    header_number.as_ref(),
                    "native_pdf.header_number.position/font/size_pt/before/after",
                ),
            ] {
                if let Some(layout) = layout {
                    if text_lines_overlap(&date.lines, date.bounds, &layout.lines, layout.bounds)? {
                        // TODO(native-pdf-ui): Include date conflicts in the UI's
                        // physical-page warning and TOML correction choices.
                        return Err(format!("Native PDF page {page_number}: ALBUM_PAGES_HEADER_DATE overlaps {element}; adjust native_pdf.header_date.position/font/size_pt/before/after/separator or {keys}"));
                    }
                }
            }
        }
        let footer = self
            .footer
            .as_ref()
            .map(|footer| footer.layout(margin_bounds, physical_page, page_number))
            .transpose()?;
        let footer_number = self
            .footer_number
            .as_ref()
            .map(|number| {
                number.layout(
                    margin_bounds,
                    physical_page,
                    page_number,
                    total_pages.ok_or("ALBUM_PAGES_FOOTER_NUMBER requires the final page count")?,
                )
            })
            .transpose()?;
        if let (Some(footer), Some(number)) = (&footer, &footer_number) {
            if text_lines_overlap(&footer.lines, footer.bounds, &number.lines, number.bounds)? {
                return Err("native_pdf.footer.text overlaps ALBUM_PAGES_FOOTER_NUMBER; adjust native_pdf.footer.position or album_pages_footer_number_position, text or font sizes".into());
            }
        }
        if let Some(footer) = &footer {
            content_bounds.bottom = content_bounds.bottom.max(footer.reservation_top_mm);
            content_bounds
                .validate()
                .map_err(|error| format!("native_pdf.footer.minimum_height_mm: {error}"))?;
        }
        if let Some(number) = &footer_number {
            content_bounds.bottom = content_bounds.bottom.max(number.reservation_top_mm);
            content_bounds.validate().map_err(|error| {
                format!("ALBUM_PAGES_FOOTER_NUMBER/native_pdf.footer.minimum_height_mm: {error}")
            })?;
        }
        let title = self
            .title
            .as_ref()
            .map(|title| title.layout(content_bounds, self.spacing.vertical_mm()))
            .transpose()?;
        if let Some(title) = &title {
            for (element, layout, keys) in [
                ("native_pdf.header.text", header.as_ref(), "album_pages_header_position, native_pdf.header.font/size_pt/text/padding_mm"),
                ("ALBUM_PAGES_HEADER_NUMBER", header_number.as_ref(), "native_pdf.header_number.position/font/size_pt/before/after, native_pdf.header.padding_mm"),
                ("ALBUM_PAGES_HEADER_DATE", header_date.as_ref(), "native_pdf.header_date.position/font/size_pt/before/after/separator, native_pdf.header.padding_mm"),
            ] {
                if let Some(layout) = layout {
                    if text_lines_overlap(&layout.lines, layout.bounds, &title.lines, title.bounds)? {
                        return Err(format!("Native PDF page {page_number}: {element} overlaps native_pdf.title.text; adjust {keys}, album_pages_title_font/size or native_pdf.title.text"));
                    }
                }
            }
            content_bounds.top = title.next_content_top_mm;
            content_bounds.validate()?;
        }
        Ok(PageLayout {
            size: self.size,
            margin_bounds,
            content_bounds,
            title,
            header,
            header_number,
            header_date,
            footer,
            footer_number,
        })
    }
}
