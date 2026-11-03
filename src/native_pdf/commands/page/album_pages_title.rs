use crate::native_pdf::{
    colour::NamedColour,
    document::Bounds,
    text::MeasuredText,
    units::{mm_to_points, points_to_mm},
    Result,
};
use oxidize_pdf::text::Font;

/// ALBUM_PAGES_TITLE with an explicit native reservation hint and colour.
#[derive(Clone, Debug)]
pub struct AlbumPagesTitle {
    text: MeasuredText,
    vspace_after_mm: f64,
    minimum_height_mm: f64,
    colour: NamedColour,
}

#[derive(Clone, Debug)]
pub struct PositionedTitleLine {
    pub text: String,
    pub left_mm: f64,
    pub baseline_mm: f64,
    pub width_mm: f64,
}

/// One result drives both PDF positions and the remaining-content boundary.
#[derive(Clone, Debug)]
pub struct TitleLayout {
    pub font: Font,
    pub size_pt: f64,
    pub colour: NamedColour,
    pub lines: Vec<PositionedTitleLine>,
    pub bounds: Bounds,
    pub measured_height_mm: f64,
    pub reserved_height_mm: f64,
    pub next_content_top_mm: f64,
}

impl AlbumPagesTitle {
    pub fn new(
        font: &str,
        size_pt: f64,
        text: &str,
        vspace_after_mm: f64,
        minimum_height_mm: f64,
        colour: NamedColour,
    ) -> Result<Self> {
        if !vspace_after_mm.is_finite() || !mm_to_points(vspace_after_mm).is_finite() {
            return Err("native_pdf.title.vspace_after_mm must be finite millimetres".into());
        }
        if !minimum_height_mm.is_finite()
            || minimum_height_mm < 0.0
            || !mm_to_points(minimum_height_mm).is_finite()
        {
            return Err(
                "native_pdf.title.minimum_height_mm must be finite nonnegative millimetres".into(),
            );
        }
        Ok(Self {
            text: MeasuredText::new(font, size_pt, text)?,
            vspace_after_mm,
            minimum_height_mm,
            colour,
        })
    }

    pub fn layout(&self, content: Bounds, vertical_spacing_mm: f64) -> Result<TitleLayout> {
        content.inset(0.0, 0.0)?;
        if !vertical_spacing_mm.is_finite() || vertical_spacing_mm < 0.0 {
            return Err(
                "ALBUM_PAGES_SPACING vertical title gap must be finite and nonnegative".into(),
            );
        }
        let text_top = content.top - vertical_spacing_mm;
        // AlbumEasy starts the baseline one em below this top, not one ascender.
        let first_baseline = text_top - points_to_mm(self.text.size_pt);
        let mut lines = Vec::new();
        let mut max_width: f64 = 0.0;
        for (index, line) in self.text.lines.iter().enumerate() {
            if line.width_mm > content.right - content.left {
                return Err("native_pdf.title.text exceeds available width; use explicit newlines or a smaller font (automatic wrapping is not implemented)".into());
            }
            max_width = max_width.max(line.width_mm);
            lines.push(PositionedTitleLine {
                text: line.text.clone(),
                left_mm: (content.left + content.right - line.width_mm) / 2.0,
                baseline_mm: first_baseline - index as f64 * self.text.line_advance_mm,
                width_mm: line.width_mm,
            });
        }
        let last = lines
            .last()
            .ok_or("native_pdf.title.text has no measured lines")?;
        let bottom = last.baseline_mm - self.text.descent_mm;
        let measured_height_mm = text_top - bottom;
        let reserved_height_mm = measured_height_mm.max(self.minimum_height_mm);
        let next_content_top_mm = text_top - reserved_height_mm - self.vspace_after_mm;
        if ![
            bottom,
            measured_height_mm,
            reserved_height_mm,
            next_content_top_mm,
        ]
        .iter()
        .all(|v| v.is_finite())
            || bottom < content.bottom
            || next_content_top_mm <= content.bottom
            || next_content_top_mm > content.top
        {
            return Err("native_pdf.title text/minimum_height_mm/vspace_after_mm leave no finite usable content area".into());
        }
        Ok(TitleLayout {
            font: self.text.font.clone(),
            size_pt: self.text.size_pt,
            colour: self.colour,
            lines,
            bounds: Bounds {
                left: (content.left + content.right - max_width) / 2.0,
                right: (content.left + content.right + max_width) / 2.0,
                top: first_baseline + self.text.ascent_mm,
                bottom,
            },
            measured_height_mm,
            reserved_height_mm,
            next_content_top_mm,
        })
    }
}
