use super::{colour::NamedColour, document::Bounds, units::mm_to_points, Result};

#[derive(Clone, Copy, Debug)]
pub enum InnerBorderStyle {
    Solid,
    Dotted,
    Dashed,
    DotDash,
    Blank,
}

impl InnerBorderStyle {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "SOLID" => Ok(Self::Solid),
            "DOTTED" => Ok(Self::Dotted),
            "DASHED" => Ok(Self::Dashed),
            "DOT_DASH" => Ok(Self::DotDash),
            "BLANK" => Ok(Self::Blank),
            _ => Err("STAMP_INNER_BORDER_STYLE must be SOLID, DOTTED, DASHED, DOT_DASH or BLANK; CORNER/SIDES are not yet mapped".into()),
        }
    }

    pub fn dash_points(self) -> Vec<f64> {
        match self {
            Self::Dotted => vec![1.0],
            Self::Dashed => vec![3.0, 2.0, 3.0, 2.0],
            Self::DotDash => vec![3.0, 2.0, 1.0, 2.0],
            Self::Solid | Self::Blank => Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StampInnerBorder {
    padding_mm: f64,
    thickness_mm: f64,
    pub style: InnerBorderStyle,
    pub colour: NamedColour,
}

#[derive(Clone, Debug)]
pub struct InnerBorderPaths {
    pub thickness_mm: f64,
    pub style: InnerBorderStyle,
    pub colour: NamedColour,
    pub segments: [(f64, f64, f64, f64); 4],
}

impl StampInnerBorder {
    pub fn new(
        padding_mm: f64,
        thickness_mm: f64,
        style: InnerBorderStyle,
        colour: NamedColour,
    ) -> Result<Self> {
        if [padding_mm, thickness_mm]
            .iter()
            .any(|value| !value.is_finite() || !mm_to_points(*value).is_finite() || *value < 0.0)
        {
            return Err("STAMP_INNER_BORDER requires nonnegative finite padding and thickness; (0 0) cancels".into());
        }
        Ok(Self {
            padding_mm,
            thickness_mm,
            style,
            colour,
        })
    }

    pub fn layout(
        self,
        bounds: Bounds,
        outer_thickness_mm: f64,
    ) -> Result<Option<InnerBorderPaths>> {
        if self.thickness_mm == 0.0 && self.padding_mm == 0.0 {
            return Ok(None);
        }
        if !outer_thickness_mm.is_finite() || outer_thickness_mm < 0.0 {
            return Err("Inner border requires finite nonnegative outer thickness".into());
        }
        // AlbumEasy measures the gap between stroke edges, not their centrelines.
        let inset = self.padding_mm + (outer_thickness_mm + self.thickness_mm) / 2.0;
        let inner = bounds
            .inset(inset, inset)
            .map_err(|error| format!("Inner border exceeds stamp frame: {error}"))?;
        if matches!(self.style, InnerBorderStyle::Blank) {
            return Ok(None);
        }
        let half = self.thickness_mm / 2.0;
        Ok(Some(InnerBorderPaths {
            thickness_mm: self.thickness_mm,
            style: self.style,
            colour: self.colour,
            segments: [
                (
                    inner.left,
                    inner.bottom - half,
                    inner.left,
                    inner.top + half,
                ),
                (
                    inner.right,
                    inner.bottom - half,
                    inner.right,
                    inner.top + half,
                ),
                (inner.left - half, inner.top, inner.right + half, inner.top),
                (
                    inner.left - half,
                    inner.bottom,
                    inner.right + half,
                    inner.bottom,
                ),
            ],
        }))
    }
}
