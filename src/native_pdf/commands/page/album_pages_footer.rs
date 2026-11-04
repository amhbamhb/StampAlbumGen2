use super::album_pages_footer_pad::FooterPadding;
use crate::native_pdf::{
    colour::NamedColour,
    document::Bounds,
    text::{MeasuredText, PositionedTextLine, TextPosition},
    units::{mm_to_points, points_to_mm},
    Result,
};
use oxidize_pdf::text::Font;

/// ALBUM_PAGES_FOOTER, positioned below the bottom margin independently of titles.
#[derive(Clone, Debug)]
pub struct AlbumPagesFooter {
    text: MeasuredText,
    position: TextPosition,
    padding: FooterPadding,
    minimum_height_mm: f64,
    colour: NamedColour,
}

#[derive(Clone, Debug)]
pub struct FooterLayout {
    pub font: Font,
    pub size_pt: f64,
    pub colour: NamedColour,
    pub lines: Vec<PositionedTextLine>,
    pub bounds: Bounds,
    pub measured_height_mm: f64,
    pub reserved_height_mm: f64,
    pub reservation_top_mm: f64,
}

impl AlbumPagesFooter {
    pub fn new(
        font: &str,
        size_pt: f64,
        position: TextPosition,
        text: &str,
        padding: FooterPadding,
        minimum_height_mm: f64,
        colour: NamedColour,
    ) -> Result<Self> {
        if !minimum_height_mm.is_finite()
            || minimum_height_mm < 0.0
            || !mm_to_points(minimum_height_mm).is_finite()
        {
            return Err(
                "native_pdf.footer.minimum_height_mm must be finite nonnegative millimetres".into(),
            );
        }
        let text = MeasuredText::new(font, size_pt, text)
            .map_err(|error| format!("native_pdf.footer font/size_pt/text: {error}"))?;
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
    ) -> Result<FooterLayout> {
        margin.inset(0.0, 0.0)?;
        physical_page.inset(0.0, 0.0)?;
        if page_number == 0 {
            return Err("ALBUM_PAGES_FOOTER requires a one-based physical page number".into());
        }
        // Verified AlbumEasy first baseline: one em and 0.5 mm below the
        // margin boundary; multiline footers grow downwards.
        let first_baseline =
            margin.bottom - points_to_mm(self.text.size_pt) - 0.5 - self.padding.mm();
        let last_baseline =
            first_baseline - (self.text.lines.len() - 1) as f64 * self.text.line_advance_mm;
        let top = first_baseline + self.text.ascent_mm;
        let bottom = last_baseline - self.text.descent_mm;
        let measured_height_mm = top - bottom;
        let reserved_height_mm = measured_height_mm.max(self.minimum_height_mm);
        let reservation_top_mm = top + (reserved_height_mm - measured_height_mm);
        if ![
            first_baseline,
            last_baseline,
            top,
            bottom,
            measured_height_mm,
            reserved_height_mm,
            reservation_top_mm,
        ]
        .iter()
        .all(|v| v.is_finite())
            || top > physical_page.top
            || bottom < physical_page.bottom
            || reservation_top_mm >= physical_page.top
        {
            return Err("native_pdf.footer size_pt/text/padding_mm/minimum_height_mm exceed physical page bounds".into());
        }
        let mut left = margin.right;
        let mut right = margin.left;
        let mut lines = Vec::new();
        for (index, line) in self.text.lines.iter().enumerate() {
            if line.width_mm > margin.right - margin.left {
                return Err("native_pdf.footer.text exceeds margin width; use explicit newlines or a smaller font (automatic wrapping is not implemented)".into());
            }
            let x = self
                .position
                .line_left(page_number, margin.left, margin.right, line.width_mm);
            left = left.min(x);
            right = right.max(x + line.width_mm);
            lines.push(PositionedTextLine {
                text: line.text.clone(),
                left_mm: x,
                baseline_mm: first_baseline - index as f64 * self.text.line_advance_mm,
                width_mm: line.width_mm,
            });
        }
        Ok(FooterLayout {
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
            reservation_top_mm,
        })
    }
}
