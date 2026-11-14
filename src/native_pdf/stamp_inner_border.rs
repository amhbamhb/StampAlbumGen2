use super::{Result, colour::NamedColour, document::Bounds, units::mm_to_points};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InnerBorderStyle {
    Solid,
    Dotted,
    Dashed,
    DotDash,
    Blank,
    Corner(u8),
}

impl InnerBorderStyle {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "SOLID" => Ok(Self::Solid),
            "DOTTED" => Ok(Self::Dotted),
            "DASHED" => Ok(Self::Dashed),
            "DOT_DASH" => Ok(Self::DotDash),
            "BLANK" => Ok(Self::Blank),
            "CORNER" => Ok(Self::Corner(15)),
            _ => Err("Border style must be SOLID, DOTTED, DASHED, DOT_DASH, BLANK or CORNER; SIDES is a separate mask command".into()),
        }
    }

    pub fn dash_points(self) -> Vec<f64> {
        match self {
            Self::Dotted => vec![1.0],
            Self::Dashed => vec![3.0, 2.0, 3.0, 2.0],
            Self::DotDash => vec![3.0, 2.0, 1.0, 2.0],
            Self::Solid | Self::Blank | Self::Corner(_) => Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BorderStyleState {
    pub style: InnerBorderStyle,
    sides: u8,
    // Preserve prior SOLID-only PDF streams; selector/dash modes use separate sides.
    segmented: bool,
}

impl BorderStyleState {
    pub fn new(style: InnerBorderStyle) -> Self {
        Self {
            style,
            sides: 15,
            segmented: false,
        }
    }

    pub fn apply(&mut self, arguments: &[&str]) -> Result<()> {
        let mode = *arguments.first().ok_or("Border style requires a mode")?;
        if matches!(mode, "CORNER" | "SIDES") {
            let names = if mode == "CORNER" {
                ["TL", "TR", "BL", "BR"]
            } else {
                ["L", "R", "T", "B"]
            };
            let mut mask = 0;
            for selector in &arguments[1..] {
                let index = names
                    .iter()
                    .position(|name| name == selector)
                    .ok_or_else(|| {
                        format!(
                            "Invalid {mode} selector {selector}; expected {}",
                            names.join("/")
                        )
                    })?;
                let bit = 1 << index;
                mask |= bit;
            }
            if mode == "SIDES" {
                self.sides = mask;
            } else {
                self.style = InnerBorderStyle::Corner(if arguments.len() == 1 { 15 } else { mask });
            }
            self.segmented = true;
        } else {
            if arguments.len() != 1 {
                return Err("Ordinary border styles do not accept selectors".into());
            }
            self.style = InnerBorderStyle::parse(mode)?;
            if !matches!(
                self.style,
                InnerBorderStyle::Solid | InnerBorderStyle::Blank
            ) {
                self.segmented = true;
            }
        }
        Ok(())
    }

    pub fn uses_legacy_rectangle(self) -> bool {
        self.style == InnerBorderStyle::Solid && self.sides == 15 && !self.segmented
    }

    pub fn outer_paths(
        self,
        bounds: Bounds,
        thickness_mm: f64,
        colour: NamedColour,
    ) -> Result<InnerBorderPaths> {
        if !thickness_mm.is_finite()
            || !mm_to_points(thickness_mm).is_finite()
            || thickness_mm < 0.0
        {
            return Err("Stamp border requires finite nonnegative thickness".into());
        }
        Ok(InnerBorderPaths::at(bounds, thickness_mm, self, colour))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StampInnerBorder {
    padding_mm: f64,
    thickness_mm: f64,
    pub colour: NamedColour,
    pub state: BorderStyleState,
}

#[derive(Clone, Debug)]
pub struct InnerBorderPaths {
    pub thickness_mm: f64,
    pub style: InnerBorderStyle,
    pub colour: NamedColour,
    pub segments: [(f64, f64, f64, f64); 4],
    state: BorderStyleState,
    corners: [[(f64, f64); 3]; 4],
}

impl InnerBorderPaths {
    fn at(bounds: Bounds, thickness_mm: f64, state: BorderStyleState, colour: NamedColour) -> Self {
        let half = thickness_mm / 2.0;
        let Bounds {
            left: l,
            right: r,
            top: t,
            bottom: b,
        } = bounds;
        // AlbumEasy's rectangular CORNER command has fixed 2 mm joined legs.
        let leg = 2.0;
        Self {
            thickness_mm,
            style: state.style,
            colour,
            state,
            segments: [
                (l, b - half, l, t + half),
                (r, b - half, r, t + half),
                (l - half, t, r + half, t),
                (l - half, b, r + half, b),
            ],
            corners: [
                [(l, t - leg), (l, t), (l + leg, t)],
                [(r - leg, t), (r, t), (r, t - leg)],
                [(l, b + leg), (l, b), (l + leg, b)],
                [(r - leg, b), (r, b), (r, b + leg)],
            ],
        }
    }

    pub fn stroke_paths(&self) -> Vec<Vec<(f64, f64)>> {
        match self.style {
            InnerBorderStyle::Blank => Vec::new(),
            InnerBorderStyle::Corner(mask) => self
                .corners
                .iter()
                .enumerate()
                .filter(|(index, _)| mask & (1 << index) != 0)
                .map(|(_, points)| points.to_vec())
                .collect(),
            _ => self
                .segments
                .iter()
                .enumerate()
                .filter(|(index, _)| self.state.sides & (1 << index) != 0)
                .map(|(_, &(x1, y1, x2, y2))| vec![(x1, y1), (x2, y2)])
                .collect(),
        }
    }
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
            colour,
            state: BorderStyleState::new(style),
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
        if matches!(self.state.style, InnerBorderStyle::Blank) {
            return Ok(None);
        }
        Ok(Some(InnerBorderPaths::at(
            inner,
            self.thickness_mm,
            self.state,
            self.colour,
        )))
    }
}
