use crate::native_pdf::{
    colour::NamedColour,
    document::Bounds,
    units::{mm_to_points, points_to_mm},
    Result,
};
use oxidize_pdf::graphics::Image;
use std::{fs, io::Cursor, path::Path, sync::Arc};

/// Explicit TOML policy for commands omitted from a definition file.
#[derive(Clone, Copy, Debug)]
pub struct DecorativeBorderPolicy {
    values: [f64; 11],
    colour: NamedColour,
}

impl DecorativeBorderPolicy {
    pub(crate) const KEYS: [&'static str; 11] = [
        "image_scale",
        "corner_padding_h_mm",
        "corner_padding_v_mm",
        "line_outer_mm",
        "line_inner1_mm",
        "line_inner2_mm",
        "line_spacing_mm",
        "line_padding_h_mm",
        "line_padding_v_mm",
        "end_offset_h_mm",
        "end_offset_v_mm",
    ];

    pub fn parse(values: &[f64], colour: NamedColour) -> Result<Self> {
        let values: [f64; 11] = values.try_into().map_err(|_| {
            "native_pdf.decorative_border requires eleven explicit numeric settings"
        })?;
        for (index, key) in Self::KEYS.into_iter().enumerate() {
            let value = values[index];
            let signed = matches!(index, 1 | 2 | 7 | 8);
            if !value.is_finite()
                || !mm_to_points(value).is_finite()
                || (!signed && value < 0.0)
                || (index == 0 && value == 0.0)
            {
                return Err(format!("native_pdf.decorative_border.{key} must be finite and fit PDF points; scale must be positive, widths/spacing/end offsets nonnegative"));
            }
        }
        Ok(Self { values, colour })
    }
}

#[derive(Clone, Debug)]
pub struct AlbumPagesDecorativeBorder {
    policy: DecorativeBorderPolicy,
    image: Arc<Image>,
}

#[derive(Clone, Debug)]
pub struct DecorativeLine {
    pub points: [[f64; 2]; 2],
    pub width_mm: f64,
}

#[derive(Clone, Debug)]
pub struct DecorativeBorderLayout {
    /// x/y/width/height in mm; signed dimensions mirror the original PNG.
    pub corners: [[f64; 4]; 4],
    pub image: Arc<Image>,
    pub lines: Vec<DecorativeLine>,
    pub colour: NamedColour,
}

impl DecorativeBorderLayout {
    /// Conservative rectangular image boxes plus exact butt-capped line footprints.
    /// Alpha-transparent pixels are not treated as a certified collision mask.
    pub fn footprints(&self) -> Vec<Bounds> {
        let mut bounds: Vec<_> = self
            .corners
            .iter()
            .map(|[x, y, w, h]| Bounds {
                left: x.min(x + w),
                right: x.max(x + w),
                bottom: y.min(y + h),
                top: y.max(y + h),
            })
            .collect();
        bounds.extend(self.lines.iter().map(|line| {
            let [[x0, y0], [x1, y1]] = line.points;
            let (dx, dy) = if y0 == y1 {
                (0.0, line.width_mm / 2.0)
            } else {
                (line.width_mm / 2.0, 0.0)
            };
            Bounds {
                left: x0.min(x1) - dx,
                right: x0.max(x1) + dx,
                bottom: y0.min(y1) - dy,
                top: y0.max(y1) + dy,
            }
        }));
        bounds
    }
}

impl AlbumPagesDecorativeBorder {
    pub fn load(path: &Path, mut policy: DecorativeBorderPolicy) -> Result<Self> {
        let source = fs::read_to_string(path).map_err(|error| {
            format!(
                "Cannot read native_pdf.decorative_border.definition_file {}: {error}",
                path.display()
            )
        })?;
        let mut image_name = None;
        for (index, raw) in source.lines().enumerate() {
            let error = |message: &str| {
                format!(
                    "Decorative border {} line {}: {message}",
                    path.display(),
                    index + 1
                )
            };
            let mut quoted = false;
            let end = raw
                .char_indices()
                .find_map(|(i, c)| {
                    if c == '"' {
                        quoted = !quoted;
                    }
                    (c == '#' && !quoted).then_some(i)
                })
                .unwrap_or(raw.len());
            let line = raw[..end].trim();
            if line.is_empty() {
                continue;
            }
            let (name, rest) = line
                .split_once('(')
                .ok_or_else(|| error("invalid command syntax"))?;
            let rest = rest.trim_end_matches(';').trim();
            let args = rest
                .strip_suffix(')')
                .ok_or_else(|| error("invalid command syntax"))?
                .trim();
            let name = name.trim();
            match name {
                "IMAGE_CORNER" => {
                    let value = args.strip_prefix('"').and_then(|v|v.strip_suffix('"'))
                        .ok_or_else(||error("IMAGE_CORNER requires a quoted PNG filename"))?;
                    if value.is_empty() {return Err(error("IMAGE_CORNER filename must not be empty"));}
                    if value.contains('"') || value.chars().any(char::is_control) {
                        return Err(error("invalid IMAGE_CORNER syntax"));
                    }
                    image_name = Some(value.to_owned());
                },
                "IMAGE_SCALE" | "IMAGE_CORNER_PAD" | "BORDER_LINES" | "BORDER_PAD" | "BORDER_END_OFFSETS" => {
                    let (start,count) = match name {
                        "IMAGE_SCALE" => (0,1),
                        "IMAGE_CORNER_PAD" => (1,2),
                        "BORDER_LINES" => (3,4),
                        "BORDER_PAD" => (7,2),
                        _ => (9,2),
                    };
                    let numbers = args.split_whitespace().map(str::parse::<f64>)
                        .collect::<std::result::Result<Vec<_>,_>>()
                        .map_err(|_|error("invalid numeric arguments"))?;
                    if numbers.len()!=count {return Err(error("wrong number of numeric arguments"));}
                    policy.values[start..start+count].copy_from_slice(&numbers);
                    policy = DecorativeBorderPolicy::parse(&policy.values,policy.colour)
                        .map_err(|message|error(&message))?;
                },
                _ => return Err(error("unsupported decorative-border command (edge ornaments are not implemented); remove it or use a supported definition")),
            }
        }
        let name = image_name.ok_or(
            "Decorative border requires IMAGE_CORNER; edge-only definitions are unsupported",
        )?;
        let image_path = path.parent().unwrap_or_else(|| Path::new("")).join(name);
        let bytes = fs::read(&image_path).map_err(|error| {
            format!(
                "Cannot read decorative IMAGE_CORNER {}: {error}",
                image_path.display()
            )
        })?;
        let decoded = image::ImageReader::with_format(Cursor::new(&bytes), image::ImageFormat::Png)
            .decode()
            .map_err(|_| {
                format!(
                    "Invalid or oversized decorative IMAGE_CORNER PNG {}",
                    image_path.display()
                )
            })?;
        if !decoded.color().has_alpha() {
            return Err(format!(
                "Decorative IMAGE_CORNER PNG {} requires an alpha channel",
                image_path.display()
            ));
        }
        let image = Image::from_png_data(bytes).map_err(|_| {
            format!(
                "Cannot decode decorative IMAGE_CORNER PNG {}",
                image_path.display()
            )
        })?;
        Ok(Self {
            policy,
            image: Arc::new(image),
        })
    }

    pub fn layout(&self, margin: Bounds) -> Result<DecorativeBorderLayout> {
        margin.inset(0.0, 0.0)?;
        let [scale, ch, cv, outer, inner1, inner2, gap, ph, pv, eh, ev] = self.policy.values;
        let w = points_to_mm(f64::from(self.image.width()) * scale);
        let h = points_to_mm(f64::from(self.image.height()) * scale);
        let left = margin.left - ch;
        let right = margin.right + ch;
        let top = margin.top + cv;
        let bottom = margin.bottom - cv;
        let corners = [
            [left, top - h, w, h],
            [right, top - h, -w, h],
            [left, bottom + h, w, -h],
            [right, bottom + h, -w, -h],
        ];
        let mut lines = Vec::new();
        for (slot, width_mm) in [outer, inner1, inner2].into_iter().enumerate() {
            if width_mm == 0.0 {
                continue;
            }
            let inset = slot as f64 * gap;
            let l = margin.left - ph + inset;
            let r = margin.right + ph - inset;
            let b = margin.bottom - pv + inset;
            let t = margin.top + pv - inset;
            let x0 = margin.left + eh;
            let x1 = margin.right - eh;
            let y0 = margin.bottom + ev;
            let y1 = margin.top - ev;
            if l >= r || b >= t || x0 >= x1 || y0 >= y1 {
                return Err("native_pdf.decorative_border padding/spacing/end offsets leave no usable connecting line; correct TOML or definition commands".into());
            }
            for points in [
                [[x0, t], [x1, t]],
                [[r, y1], [r, y0]],
                [[x1, b], [x0, b]],
                [[l, y0], [l, y1]],
            ] {
                lines.push(DecorativeLine { points, width_mm });
            }
        }
        if w <= 0.0
            || h <= 0.0
            || !corners
                .iter()
                .flatten()
                .all(|v| v.is_finite() && mm_to_points(*v).is_finite())
        {
            return Err(
                "native_pdf.decorative_border image scale/padding overflow PDF coordinates".into(),
            );
        }
        for corner in corners {
            let [x, y, w, h] = corner;
            if ![x + w, y + h]
                .iter()
                .all(|v| v.is_finite() && mm_to_points(*v).is_finite())
            {
                return Err(
                    "native_pdf.decorative_border image bounds overflow PDF coordinates".into(),
                );
            }
        }
        for line in &lines {
            for [x, y] in line.points {
                if ![
                    x,
                    y,
                    x - line.width_mm / 2.0,
                    x + line.width_mm / 2.0,
                    y - line.width_mm / 2.0,
                    y + line.width_mm / 2.0,
                ]
                .iter()
                .all(|v| v.is_finite() && mm_to_points(*v).is_finite())
                {
                    return Err("native_pdf.decorative_border connecting line bounds overflow PDF coordinates".into());
                }
            }
        }
        Ok(DecorativeBorderLayout {
            corners,
            image: Arc::clone(&self.image),
            lines,
            colour: self.policy.colour,
        })
    }
}
