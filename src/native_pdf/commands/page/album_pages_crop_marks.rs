use crate::native_pdf::{
    Result, colour::NamedColour, commands::page::album_pages_size::PageSize, units::mm_to_points,
};

/// Explicit ALBUM_PAGES_CROP_MARKS values; optional TXT defaults belong in TOML.
#[derive(Clone, Copy, Debug)]
pub struct AlbumPagesCropMarks {
    line_width_mm: f64,
    horizontal_padding_mm: f64,
    vertical_padding_mm: f64,
    arms_mm: f64,
    colour: NamedColour,
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
    pub fn parse(arguments_mm: &[f64], colour: NamedColour) -> Result<Self> {
        let [line_width_mm, horizontal_padding_mm, vertical_padding_mm, arms_mm]: [f64; 4] =
            arguments_mm.try_into().map_err(|_| {
                "ALBUM_PAGES_CROP_MARKS requires four explicit numeric values; optional TXT defaults must be resolved from TOML"
            })?;
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
