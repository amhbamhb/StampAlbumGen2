use crate::native_pdf::{
    Result, colour::NamedColour, document::Bounds, text::MeasuredText, units::mm_to_points,
};
use oxidize_pdf::text::Font;

#[derive(Clone, Copy, Debug)]
pub enum MarginPosition {
    Left,
    Right,
    LeftRight,
    RightLeft,
}

impl MarginPosition {
    pub fn parse(token: &str) -> Result<Self> {
        match token {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "LR" => Ok(Self::LeftRight),
            "RL" => Ok(Self::RightLeft),
            _ => Err("ALBUM_PAGES_MARGIN_TXT position must be L, R, LR or RL".into()),
        }
    }

    fn is_left(self, page: usize) -> bool {
        match self {
            Self::Left => true,
            Self::Right => false,
            Self::LeftRight => !page.is_multiple_of(2),
            Self::RightLeft => page.is_multiple_of(2),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MarginAlignment {
    Top,
    Centre,
    Bottom,
}

impl MarginAlignment {
    pub fn parse(token: &str) -> Result<Self> {
        match token {
            "TOP" => Ok(Self::Top),
            "CENTRE" => Ok(Self::Centre),
            "BOTTOM" => Ok(Self::Bottom),
            _ => Err("ALBUM_PAGES_MARGIN_TXT alignment must be TOP, CENTRE or BOTTOM".into()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MarginDirection {
    Up,
    Down,
}

impl MarginDirection {
    pub fn parse(token: &str) -> Result<Self> {
        match token {
            "UP" => Ok(Self::Up),
            "DOWN" => Ok(Self::Down),
            _ => Err("ALBUM_PAGES_MARGIN_TXT direction must be UP or DOWN".into()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AlbumPagesMarginText {
    text: MeasuredText,
    position: MarginPosition,
    alignment: MarginAlignment,
    direction: MarginDirection,
    border_gap_mm: f64,
    colour: NamedColour,
}

#[derive(Clone, Debug)]
pub struct RotatedMarginLine {
    pub text: String,
    pub x_mm: f64,
    pub y_mm: f64,
    pub bounds: Bounds,
}

#[derive(Clone, Debug)]
pub struct MarginTextLayout {
    pub font: Font,
    pub size_pt: f64,
    pub colour: NamedColour,
    pub direction: MarginDirection,
    pub lines: Vec<RotatedMarginLine>,
}

impl AlbumPagesMarginText {
    pub fn new(
        text: MeasuredText,
        position: MarginPosition,
        alignment: MarginAlignment,
        direction: MarginDirection,
        border_gap_mm: f64,
        colour: NamedColour,
    ) -> Result<Self> {
        if !border_gap_mm.is_finite() || !mm_to_points(border_gap_mm).is_finite() {
            return Err("ALBUM_PAGES_MARGIN_TXT border gap must be finite millimetres".into());
        }
        Ok(Self {
            text,
            position,
            alignment,
            direction,
            border_gap_mm,
            colour,
        })
    }

    pub fn layout(&self, margin: Bounds, paper: Bounds, page: usize) -> Result<MarginTextLayout> {
        if page == 0 {
            return Err("ALBUM_PAGES_MARGIN_TXT requires a one-based physical page".into());
        }
        margin.inset(0.0, 0.0)?;
        paper.inset(0.0, 0.0)?;
        let left = self.position.is_left(page);
        let up = matches!(self.direction, MarginDirection::Up);
        let base_x = if left {
            margin.left - self.border_gap_mm - if up { 0.0 } else { self.text.ascent_mm }
        } else {
            margin.right + self.border_gap_mm + if up { self.text.ascent_mm } else { 0.0 }
        };
        // Rotated multiline text shares one origin, aligned by the longest line.
        let longest = self
            .text
            .lines
            .iter()
            .map(|line| line.width_mm)
            .fold(0.0, f64::max);
        let base_y = match (self.alignment, up) {
            (MarginAlignment::Top, true) => margin.top - longest,
            (MarginAlignment::Top, false) => margin.top,
            (MarginAlignment::Bottom, true) => margin.bottom,
            (MarginAlignment::Bottom, false) => margin.bottom + longest,
            (MarginAlignment::Centre, true) => (margin.top + margin.bottom - longest) / 2.0,
            (MarginAlignment::Centre, false) => (margin.top + margin.bottom + longest) / 2.0,
        };
        let mut lines = Vec::new();
        for (index, line) in self.text.lines.iter().enumerate() {
            let x =
                base_x + if left { -1.0 } else { 1.0 } * index as f64 * self.text.line_advance_mm;
            let bounds = if up {
                Bounds {
                    left: x - self.text.ascent_mm,
                    right: x + self.text.descent_mm,
                    bottom: base_y,
                    top: base_y + line.width_mm,
                }
            } else {
                Bounds {
                    left: x - self.text.descent_mm,
                    right: x + self.text.ascent_mm,
                    bottom: base_y - line.width_mm,
                    top: base_y,
                }
            };
            if ![
                x,
                base_y,
                bounds.left,
                bounds.right,
                bounds.bottom,
                bounds.top,
            ]
            .iter()
            .all(|v| v.is_finite() && mm_to_points(*v).is_finite())
                || bounds.left < paper.left
                || bounds.right > paper.right
                || bounds.bottom < margin.bottom - 1e-9
                || bounds.top > margin.top + 1e-9
                || (left && bounds.right > margin.left)
                || (!left && bounds.left < margin.right)
            {
                return Err(format!(
                    "ALBUM_PAGES_MARGIN_TXT on physical page {page} exceeds paper/margin bounds or overlaps the page border; adjust text, font size or border gap"
                ));
            }
            lines.push(RotatedMarginLine {
                text: line.text.clone(),
                x_mm: x,
                y_mm: base_y,
                bounds,
            });
        }
        Ok(MarginTextLayout {
            font: self.text.font.clone(),
            size_pt: self.text.size_pt,
            colour: self.colour,
            direction: self.direction,
            lines,
        })
    }
}
