use super::{
    units::{mm_to_points, points_to_mm},
    Result,
};
use oxidize_pdf::text::{fonts::get_standard_font_metrics, measure_text, Font, TextEncoding};

#[derive(Clone, Debug)]
pub struct MeasuredLine {
    pub text: String,
    pub width_mm: f64,
}

#[derive(Clone, Debug)]
pub struct PositionedTextLine {
    pub text: String,
    pub left_mm: f64,
    pub baseline_mm: f64,
    pub width_mm: f64,
}

/// Standard-font measurements shared by layout and drawing; no wrapping fallback.
#[derive(Clone, Debug)]
pub struct MeasuredText {
    pub font: Font,
    pub size_pt: f64,
    pub lines: Vec<MeasuredLine>,
    pub ascent_mm: f64,
    pub descent_mm: f64,
    pub line_advance_mm: f64,
}

impl MeasuredText {
    pub fn new(token: &str, size_pt: f64, text: &str) -> Result<Self> {
        let font = match token {
            "HN" => Font::Helvetica,
            "HB" => Font::HelveticaBold,
            "HI" => Font::HelveticaOblique,
            "HS" => Font::HelveticaBoldOblique,
            "TN" => Font::TimesRoman,
            "TB" => Font::TimesBold,
            "TI" => Font::TimesItalic,
            "TS" => Font::TimesBoldItalic,
            "CN" => Font::Courier,
            "CB" => Font::CourierBold,
            "CI" => Font::CourierOblique,
            "CS" => Font::CourierBoldOblique,
            _ => return Err("Text font must be a supported built-in AlbumEasy font token".into()),
        };
        if !size_pt.is_finite() || size_pt <= 0.0 {
            return Err("Text size must be finite and positive points".into());
        }
        let text = text.replace("\r\n", "\n");
        if text.trim().is_empty()
            || text.chars().any(|c| c.is_control() && c != '\n')
            || TextEncoding::WinAnsiEncoding
                .encode_strict(&text.replace('\n', ""))
                .is_err()
        {
            return Err(
                "Text must be nonempty WinAnsi text; only newline controls are supported".into(),
            );
        }
        let metrics = get_standard_font_metrics(&font)
            .ok_or("Text font has no verified standard-font metrics")?;
        let ascent_mm = points_to_mm(metrics.to_user_space(metrics.ascender, size_pt));
        let descent_mm = -points_to_mm(metrics.to_user_space(metrics.descender, size_pt));
        let line_advance_mm = points_to_mm(size_pt).max(ascent_mm + descent_mm);
        let lines = text
            .split('\n')
            .map(|line| MeasuredLine {
                text: line.into(),
                width_mm: points_to_mm(measure_text(line, &font, size_pt)),
            })
            .collect::<Vec<_>>();
        if ![ascent_mm, descent_mm, line_advance_mm]
            .iter()
            .all(|v| v.is_finite())
            || lines
                .iter()
                .any(|line| !line.width_mm.is_finite() || !mm_to_points(line.width_mm).is_finite())
        {
            return Err("Text size overflows native text measurements".into());
        }
        Ok(Self {
            font,
            size_pt,
            lines,
            ascent_mm,
            descent_mm,
            line_advance_mm,
        })
    }
}
