use super::{
    Result,
    document::Bounds,
    units::{mm_to_points, points_to_mm},
};
use oxidize_pdf::text::{Font, TextEncoding, fonts::get_standard_font_metrics, measure_text};

#[derive(Clone, Copy, Debug)]
pub enum TextPosition {
    Left,
    Centre,
    Right,
    LeftRight,
    RightLeft,
}

impl TextPosition {
    /// Retains the original header command's public parsing contract.
    pub fn parse(token: &str) -> Result<Self> {
        Self::parse_for(token, "album_pages_header_position")
    }

    pub fn parse_for(token: &str, key: &str) -> Result<Self> {
        match token {
            "L" => Ok(Self::Left),
            "C" => Ok(Self::Centre),
            "R" => Ok(Self::Right),
            "LR" => Ok(Self::LeftRight),
            "RL" => Ok(Self::RightLeft),
            _ => Err(format!("{key} must be L, C, R, LR or RL")),
        }
    }

    pub(crate) fn line_left(self, page_number: usize, left: f64, right: f64, width: f64) -> f64 {
        let position = match (self, page_number.is_multiple_of(2)) {
            (Self::LeftRight, false) | (Self::RightLeft, true) => Self::Left,
            (Self::LeftRight, true) | (Self::RightLeft, false) => Self::Right,
            _ => self,
        };
        match position {
            Self::Left => left,
            Self::Right => right - width,
            _ => (left + right - width) / 2.0,
        }
    }
}

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

/// Intersects actual per-line AFM boxes, not the larger reserved hint rectangle.
pub(crate) fn text_lines_overlap(
    a: &[PositionedTextLine],
    a_bounds: Bounds,
    b: &[PositionedTextLine],
    b_bounds: Bounds,
) -> Result<bool> {
    let a_first = a
        .first()
        .ok_or("Native text has no positioned lines")?
        .baseline_mm;
    let a_last = a
        .last()
        .ok_or("Native text has no positioned lines")?
        .baseline_mm;
    let b_first = b
        .first()
        .ok_or("Native text has no positioned lines")?
        .baseline_mm;
    let b_last = b
        .last()
        .ok_or("Native text has no positioned lines")?
        .baseline_mm;
    Ok(a.iter().any(|a| {
        b.iter().any(|b| {
            !a.text.trim().is_empty()
                && !b.text.trim().is_empty()
                && a.left_mm < b.left_mm + b.width_mm
                && b.left_mm < a.left_mm + a.width_mm
                && a.baseline_mm - (a_last - a_bounds.bottom)
                    < b.baseline_mm + (b_bounds.top - b_first)
                && b.baseline_mm - (b_last - b_bounds.bottom)
                    < a.baseline_mm + (a_bounds.top - a_first)
        })
    }))
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
        Self::new_with_font(font, size_pt, text)
    }

    pub fn with_text(&self, text: &str) -> Result<Self> {
        Self::new_with_font(self.font.clone(), self.size_pt, text)
    }

    /// Word wrapping preserves explicit lines; repeated spaces are normalized.
    /// An overlong word breaks at character boundaries, never by reducing font size.
    pub fn wrapped_to_width(&self, width_mm: f64) -> Result<Self> {
        if !width_mm.is_finite() || width_mm <= 0.0 {
            return Err("Text wrap width must be finite and positive".into());
        }
        let mut lines = Vec::new();
        for source in &self.lines {
            let mut line = String::new();
            for word in source.text.split_whitespace() {
                let candidate = if line.is_empty() {
                    word.to_string()
                } else {
                    format!("{line} {word}")
                };
                if self.with_text(&candidate)?.lines[0].width_mm <= width_mm {
                    line = candidate;
                    continue;
                }
                if !line.is_empty() {
                    lines.push(std::mem::take(&mut line));
                }
                for character in word.chars() {
                    let candidate = format!("{line}{character}");
                    if self.with_text(&candidate)?.lines[0].width_mm > width_mm {
                        if line.is_empty() {
                            return Err("Text wrap width is narrower than a single glyph".into());
                        }
                        lines.push(std::mem::take(&mut line));
                        if self.with_text(&character.to_string())?.lines[0].width_mm > width_mm {
                            return Err("Text wrap width is narrower than a single glyph".into());
                        }
                    }
                    line.push(character);
                }
            }
            lines.push(line);
        }
        self.with_text(&lines.join("\n"))
    }

    fn new_with_font(font: Font, size_pt: f64, text: &str) -> Result<Self> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_collisions_ignore_blank_lines_hints_and_touching_edges() -> Result<()> {
        let a = vec![PositionedTextLine {
            text: "A".into(),
            left_mm: 0.0,
            baseline_mm: 10.0,
            width_mm: 5.0,
        }];
        let bounds = Bounds {
            left: 0.0,
            right: 5.0,
            bottom: 9.0,
            top: 12.0,
        };
        let b = vec![PositionedTextLine {
            left_mm: 5.0,
            ..a[0].clone()
        }];
        assert!(!text_lines_overlap(&a, bounds, &b, bounds)?);
        let b = vec![PositionedTextLine {
            left_mm: 4.0,
            ..a[0].clone()
        }];
        assert!(text_lines_overlap(&a, bounds, &b, bounds)?);
        let b = vec![PositionedTextLine {
            baseline_mm: 13.0,
            ..a[0].clone()
        }];
        let upper = Bounds {
            bottom: 12.0,
            top: 15.0,
            ..bounds
        };
        assert!(!text_lines_overlap(&a, bounds, &b, upper)?);
        let b = vec![PositionedTextLine {
            text: " ".into(),
            ..a[0].clone()
        }];
        assert!(!text_lines_overlap(&a, bounds, &b, bounds)?);
        let b = vec![
            PositionedTextLine {
                text: " ".into(),
                baseline_mm: 15.0,
                ..a[0].clone()
            },
            PositionedTextLine {
                text: "B".into(),
                baseline_mm: 5.0,
                ..a[0].clone()
            },
        ];
        assert!(!text_lines_overlap(
            &a,
            bounds,
            &b,
            Bounds {
                bottom: 4.0,
                top: 17.0,
                ..bounds
            }
        )?);
        Ok(())
    }
}
