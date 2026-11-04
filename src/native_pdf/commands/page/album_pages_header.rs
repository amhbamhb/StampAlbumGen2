use super::album_pages_header_pad::HeaderPadding;
use crate::native_pdf::{
    colour::NamedColour,
    document::Bounds,
    text::{MeasuredText, PositionedTextLine},
    units::mm_to_points,
    Result,
};
use oxidize_pdf::text::Font;

#[derive(Clone, Copy, Debug)]
pub enum HeaderPosition {
    Left,
    Centre,
    Right,
    LeftRight,
    RightLeft,
}

impl HeaderPosition {
    pub fn parse(token: &str) -> Result<Self> {
        match token {
            "L" => Ok(Self::Left),
            "C" => Ok(Self::Centre),
            "R" => Ok(Self::Right),
            "LR" => Ok(Self::LeftRight),
            "RL" => Ok(Self::RightLeft),
            _ => Err("album_pages_header_position must be L, C, R, LR or RL".into()),
        }
    }

    fn for_page(self, page_number: usize) -> Self {
        match (self, page_number.is_multiple_of(2)) {
            (Self::LeftRight, false) | (Self::RightLeft, true) => Self::Left,
            (Self::LeftRight, true) | (Self::RightLeft, false) => Self::Right,
            _ => self,
        }
    }
}

/// ALBUM_PAGES_HEADER, positioned in the margin independently of page titles.
#[derive(Clone, Debug)]
pub struct AlbumPagesHeader {
    text: MeasuredText,
    position: HeaderPosition,
    padding: HeaderPadding,
    minimum_height_mm: f64,
    colour: NamedColour,
}

#[derive(Clone, Debug)]
pub struct HeaderLayout {
    pub font: Font,
    pub size_pt: f64,
    pub colour: NamedColour,
    pub lines: Vec<PositionedTextLine>,
    pub bounds: Bounds,
    pub measured_height_mm: f64,
    pub reserved_height_mm: f64,
    pub reservation_bottom_mm: f64,
}

impl AlbumPagesHeader {
    pub fn new(
        font: &str,
        size_pt: f64,
        position: HeaderPosition,
        text: &str,
        padding: HeaderPadding,
        minimum_height_mm: f64,
        colour: NamedColour,
    ) -> Result<Self> {
        if !minimum_height_mm.is_finite()
            || minimum_height_mm < 0.0
            || !mm_to_points(minimum_height_mm).is_finite()
        {
            return Err(
                "native_pdf.header.minimum_height_mm must be finite nonnegative millimetres".into(),
            );
        }
        let text = MeasuredText::new(font, size_pt, text)
            .map_err(|error| format!("native_pdf.header font/size_pt/text: {error}"))?;
        Ok(Self {
            text,
            position,
            padding,
            minimum_height_mm,
            colour,
        })
    }

    pub fn layout(
        &self,
        margin: Bounds,
        physical_page: Bounds,
        page_number: usize,
    ) -> Result<HeaderLayout> {
        margin.inset(0.0, 0.0)?;
        physical_page.inset(0.0, 0.0)?;
        if page_number == 0 {
            return Err("ALBUM_PAGES_HEADER requires a one-based physical page number".into());
        }
        // Verified AlbumEasy baseline: 2 mm above the margin boundary;
        // multiline headers grow upwards and padding shifts the complete block.
        let last_baseline = margin.top + 2.0 + self.padding.mm();
        let first_baseline =
            last_baseline + (self.text.lines.len() - 1) as f64 * self.text.line_advance_mm;
        let top = first_baseline + self.text.ascent_mm;
        let bottom = last_baseline - self.text.descent_mm;
        let measured_height_mm = top - bottom;
        let reserved_height_mm = measured_height_mm.max(self.minimum_height_mm);
        let reservation_bottom_mm = bottom - (reserved_height_mm - measured_height_mm);
        if ![
            first_baseline,
            top,
            bottom,
            measured_height_mm,
            reserved_height_mm,
            reservation_bottom_mm,
        ]
        .iter()
        .all(|v| v.is_finite())
            || top > physical_page.top
            || bottom < physical_page.bottom
            || reservation_bottom_mm <= physical_page.bottom
        {
            return Err("native_pdf.header size_pt/text/padding_mm/minimum_height_mm exceed physical page bounds".into());
        }
        let position = self.position.for_page(page_number);
        let mut left = margin.right;
        let mut right = margin.left;
        let mut lines = Vec::new();
        for (index, line) in self.text.lines.iter().enumerate() {
            if line.width_mm > margin.right - margin.left {
                return Err("native_pdf.header.text exceeds margin width; use explicit newlines or a smaller font (automatic wrapping is not implemented)".into());
            }
            let x = match position {
                HeaderPosition::Left => margin.left,
                HeaderPosition::Right => margin.right - line.width_mm,
                _ => (margin.left + margin.right - line.width_mm) / 2.0,
            };
            left = left.min(x);
            right = right.max(x + line.width_mm);
            lines.push(PositionedTextLine {
                text: line.text.clone(),
                left_mm: x,
                baseline_mm: first_baseline - index as f64 * self.text.line_advance_mm,
                width_mm: line.width_mm,
            });
        }
        Ok(HeaderLayout {
            font: self.text.font.clone(),
            size_pt: self.text.size_pt,
            colour: self.colour,
            lines,
            bounds: Bounds {
                left,
                right,
                top,
                bottom,
            },
            measured_height_mm,
            reserved_height_mm,
            reservation_bottom_mm,
        })
    }
}
