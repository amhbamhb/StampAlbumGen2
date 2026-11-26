use crate::native_pdf::{
    Result, colour::NamedColour, commands::page::album_pages_size::PageSize, units::mm_to_points,
};

/// Explicit ALBUM_PAGES_CROP_MARKS values; optional TXT defaults belong in TOML.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlbumPagesCropMarks {
    line_width_mm: f64,
    horizontal_padding_mm: f64,
    vertical_padding_mm: f64,
    arms_mm: f64,
    colour: NamedColour,
}

impl Default for AlbumPagesCropMarks {
    fn default() -> Self {
        Self::default_with_colour(NamedColour::BLACK)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CropMark {
    /// Two disconnected segments: horizontal start/end, vertical start/end.
    pub points: [[f64; 2]; 4],
}

#[derive(Clone, Debug)]
pub struct CropMarksLayout {
    pub marks: [CropMark; 4],
    pub line_width_mm: f64,
    pub colour: NamedColour,
    pub clipped: bool,
}

impl AlbumPagesCropMarks {
    pub const DEFAULT_LINE_WIDTH_MM: f64 = 0.1;
    pub const DEFAULT_HORIZONTAL_PADDING_MM: f64 = 1.0;
    pub const DEFAULT_VERTICAL_PADDING_MM: f64 = 1.0;
    pub const DEFAULT_ARMS_MM: f64 = 5.0;

    pub fn new(
        line_width_mm: f64,
        horizontal_padding_mm: f64,
        vertical_padding_mm: f64,
        arms_mm: f64,
        colour: NamedColour,
    ) -> Result<Self> {
        for (key, value) in [
            ("line_width_mm", line_width_mm),
            ("horizontal_padding_mm", horizontal_padding_mm),
            ("vertical_padding_mm", vertical_padding_mm),
            ("arms_mm", arms_mm),
        ] {
            if !value.is_finite() || value < 0.0 || !mm_to_points(value).is_finite() {
                return Err(format!(
                    "native_pdf.crop_marks.{key} must be finite, nonnegative and fit PDF point dimensions"
                ));
            }
        }
        Ok(Self {
            line_width_mm,
            horizontal_padding_mm,
            vertical_padding_mm,
            arms_mm,
            colour,
        })
    }

    pub fn default_with_colour(colour: NamedColour) -> Self {
        Self {
            line_width_mm: Self::DEFAULT_LINE_WIDTH_MM,
            horizontal_padding_mm: Self::DEFAULT_HORIZONTAL_PADDING_MM,
            vertical_padding_mm: Self::DEFAULT_VERTICAL_PADDING_MM,
            arms_mm: Self::DEFAULT_ARMS_MM,
            colour,
        }
    }

    pub fn parse(arguments_mm: &[f64], colour: NamedColour) -> Result<Self> {
        let [line_width_mm, horizontal_padding_mm, vertical_padding_mm, arms_mm]: [f64; 4] =
            arguments_mm.try_into().map_err(|_| {
                "ALBUM_PAGES_CROP_MARKS requires four explicit numeric values; optional TXT defaults must be resolved from TOML"
            })?;
        Self::new(
            line_width_mm,
            horizontal_padding_mm,
            vertical_padding_mm,
            arms_mm,
            colour,
        )
    }

    /// Parses command arguments for ALBUM_PAGES_CROP_MARKS / NATIVE_PAGES_CROP_MARKS.
    ///
    /// Supported signatures:
    /// - `()`: all defaults (line_width 0.1, h_pad 1.0, v_pad 1.0, arms 5.0, black)
    /// - `("none")`: disabled (`None`)
    /// - `(colour)`: all defaults with specified colour
    /// - `(line_width)`: custom line width, rest defaults, black
    /// - `(line_width, colour)`: custom line width with colour
    /// - `(line_width, h_pad)`: custom width and horizontal padding, black
    /// - `(line_width, h_pad, colour)`: custom width and horizontal padding with colour
    /// - `(line_width, h_pad, v_pad)`: custom width, h_pad and v_pad, black
    /// - `(line_width, h_pad, v_pad, colour)`: custom width, h_pad and v_pad with colour
    /// - `(line_width, h_pad, v_pad, arms)`: all 4 numeric values, black
    /// - `(line_width, h_pad, v_pad, arms, colour)`: all 4 numeric values with colour
    pub fn parse_command_args(args: &[&str]) -> Result<Option<Self>> {
        if args.is_empty() {
            return Ok(Some(Self::default_with_colour(NamedColour::BLACK)));
        }
        if args.len() == 1 {
            let single = args[0].trim().trim_matches('"').trim_matches('\'').trim();
            if single.eq_ignore_ascii_case("none")
                || single.eq_ignore_ascii_case("off")
                || single.eq_ignore_ascii_case("false")
            {
                return Ok(None);
            }
            if let Ok(lw) = single.parse::<f64>() {
                return Self::new(
                    lw,
                    Self::DEFAULT_HORIZONTAL_PADDING_MM,
                    Self::DEFAULT_VERTICAL_PADDING_MM,
                    Self::DEFAULT_ARMS_MM,
                    NamedColour::BLACK,
                )
                .map(Some);
            }
            let col = NamedColour::parse(single)?;
            return Ok(Some(Self::default_with_colour(col)));
        }

        if args.len() > 5 {
            return Err(
                "ALBUM_PAGES_CROP_MARKS accepts at most 5 arguments (line_width, horizontal_padding, vertical_padding, arms, colour)"
                    .to_string(),
            );
        }

        let last_str = args
            .last()
            .unwrap()
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .trim();
        let (num_args, colour) = if let Ok(_num) = last_str.parse::<f64>() {
            if args.len() > 4 {
                return Err(
                    "ALBUM_PAGES_CROP_MARKS accepts at most 4 numeric arguments plus an optional colour"
                        .to_string(),
                );
            }
            (args, NamedColour::BLACK)
        } else {
            let col = NamedColour::parse(last_str)?;
            (&args[..args.len() - 1], col)
        };

        let mut values = [
            Self::DEFAULT_LINE_WIDTH_MM,
            Self::DEFAULT_HORIZONTAL_PADDING_MM,
            Self::DEFAULT_VERTICAL_PADDING_MM,
            Self::DEFAULT_ARMS_MM,
        ];
        if num_args.len() > 4 {
            return Err("ALBUM_PAGES_CROP_MARKS accepts at most 4 numeric arguments".to_string());
        }
        for (i, arg_str) in num_args.iter().enumerate() {
            let trimmed = arg_str.trim().trim_matches('"').trim_matches('\'').trim();
            values[i] = trimmed.parse::<f64>().map_err(|_| {
                format!("Invalid numeric argument '{trimmed}' for ALBUM_PAGES_CROP_MARKS")
            })?;
        }

        Self::new(values[0], values[1], values[2], values[3], colour).map(Some)
    }

    pub fn line_width_mm(&self) -> f64 {
        self.line_width_mm
    }

    pub fn horizontal_padding_mm(&self) -> f64 {
        self.horizontal_padding_mm
    }

    pub fn vertical_padding_mm(&self) -> f64 {
        self.vertical_padding_mm
    }

    pub fn arms_mm(&self) -> f64 {
        self.arms_mm
    }

    pub fn colour(&self) -> NamedColour {
        self.colour
    }

    pub fn layout(self, size: PageSize) -> Result<CropMarksLayout> {
        // AlbumEasy offsets centres by half a line width, keeps the MediaBox,
        // and intentionally allows the outer arms to cross its edges.
        let half = self.line_width_mm / 2.0;
        let left = self.horizontal_padding_mm - half;
        let bottom = self.vertical_padding_mm - half;
        let right = size.width_mm() - left;
        let top = size.height_mm() - bottom;
        let extent = self.arms_mm.max(half);
        let mut clipped = false;
        let centres = [[left, bottom], [left, top], [right, top], [right, bottom]];
        let marks = centres.map(|[x, y]| CropMark {
            points: [
                [x - self.arms_mm, y],
                [x + self.arms_mm, y],
                [x, y + self.arms_mm],
                [x, y - self.arms_mm],
            ],
        });
        for ([x, y], mark) in centres.into_iter().zip(&marks) {
            let footprint = [x - extent, y - extent, x + extent, y + extent];
            if !mark
                .points
                .iter()
                .flatten()
                .chain(footprint.iter())
                .all(|value| value.is_finite() && mm_to_points(*value).is_finite())
            {
                return Err("ALBUM_PAGES_CROP_MARKS/native_pdf.crop_marks line_width_mm/padding/arms overflow PDF point coordinates".into());
            }
            // Zero-length butt-capped segments paint nothing.
            clipped |= self.arms_mm > 0.0
                && (footprint[0] < 0.0
                    || footprint[1] < 0.0
                    || footprint[2] > size.width_mm()
                    || footprint[3] > size.height_mm());
        }
        Ok(CropMarksLayout {
            marks,
            line_width_mm: self.line_width_mm,
            colour: self.colour,
            clipped,
        })
    }
}
