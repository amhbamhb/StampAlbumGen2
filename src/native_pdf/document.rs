use super::{
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder,
        album_pages_margins::Margins,
        album_pages_marginse::EvenMargins,
        album_pages_size::PageSize,
        album_pages_spacing::Spacing,
        album_pages_title::{AlbumPagesTitle, TitleLayout},
    },
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
        }
    }

    pub fn set_even_margins(&mut self, margins: EvenMargins) {
        self.even_margins = Some(margins);
    }

    pub fn set_title(&mut self, title: AlbumPagesTitle) {
        self.title = Some(title);
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
        let title = self
            .title
            .as_ref()
            .map(|title| title.layout(content_bounds, self.spacing.vertical_mm()))
            .transpose()?;
        if let Some(title) = &title {
            content_bounds.top = title.next_content_top_mm;
            content_bounds.validate()?;
        }
        Ok(PageLayout {
            size: self.size,
            margin_bounds,
            content_bounds,
            title,
        })
    }
}
