use crate::native_pdf::{
    Result,
    colour::NamedColour,
    document::Bounds,
    units::{mm_to_points, points_to_mm},
};
use oxidize_pdf::graphics::Image;
use std::{fs, io::Cursor, path::Path, sync::Arc};

/// Explicit TOML policy for commands omitted from a definition file.
#[derive(Clone, Copy, Debug)]
pub struct DecorativeBorderPolicy {
    values: [f64; 13],
    colour: NamedColour,
}

impl DecorativeBorderPolicy {
    pub(crate) const KEYS: [&'static str; 13] = [
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
        "edge_padding_h_mm",
        "edge_padding_v_mm",
    ];

    pub fn parse(values: &[f64], colour: NamedColour) -> Result<Self> {
        let values: [f64; 13] = values.try_into().map_err(
            |_| "native_pdf.decorative_border requires thirteen explicit numeric settings",
        )?;
        for (index, key) in Self::KEYS.into_iter().enumerate() {
            let value = values[index];
            let signed = matches!(index, 1 | 2 | 7 | 8 | 11 | 12);
            if !value.is_finite()
                || !mm_to_points(value).is_finite()
                || (!signed && value < 0.0)
                || (index == 0 && value == 0.0)
            {
                return Err(format!(
                    "native_pdf.decorative_border.{key} must be finite and fit PDF points; scale must be positive, widths/spacing/end offsets nonnegative"
                ));
            }
        }
        Ok(Self { values, colour })
    }
}

#[derive(Clone, Debug)]
pub struct AlbumPagesDecorativeBorder {
    policy: DecorativeBorderPolicy,
    corner: Option<Arc<Image>>,
    edges: [Option<Arc<Image>>; 4],
}

#[derive(Clone, Debug)]
pub struct DecorativeImageLayout {
    pub resource_name: &'static str,
    pub image: Arc<Image>,
    /// x/y/width/height in mm; signed dimensions mirror the original PNG.
    pub rect_mm: [f64; 4],
}

#[derive(Clone, Debug)]
pub struct DecorativeLine {
    pub points: [[f64; 2]; 2],
    pub width_mm: f64,
}

#[derive(Clone, Debug)]
pub struct DecorativeBorderLayout {
    pub corners: Vec<DecorativeImageLayout>,
    /// AlbumEasy emits vertical edges before bottom and top edges.
    pub edges: Vec<DecorativeImageLayout>,
    pub lines: Vec<DecorativeLine>,
    pub colour: NamedColour,
    page_height_mm: f64,
}

impl DecorativeBorderLayout {
    /// Conservative rectangular image boxes plus exact butt-capped line footprints.
    /// Alpha-transparent pixels are not treated as a certified collision mask.
    pub fn footprints(&self) -> Vec<Bounds> {
        let mut bounds: Vec<_> = self
            .corners
            .iter()
            .chain(&self.edges)
            .map(|image| {
                let [x, y, w, h] = image.rect_mm;
                let bottom = self.page_height_mm - y - h;
                Bounds {
                    left: x.min(x + w),
                    right: x.max(x + w),
                    bottom: bottom.min(bottom + h),
                    top: bottom.max(bottom + h),
                }
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
        let mut corner_name = None;
        let mut edge_names: [Option<String>; 4] = [None, None, None, None];
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
                "IMAGE_CORNER" | "IMAGE_EDGE_LEFT" | "IMAGE_EDGE_RIGHT" | "IMAGE_EDGE_BOTTOM"
                | "IMAGE_EDGE_TOP" => {
                    let value = args
                        .strip_prefix('"')
                        .and_then(|value| value.strip_suffix('"'))
                        .ok_or_else(|| error("image command requires a quoted PNG filename"))?;
                    if value.is_empty() {
                        return Err(error("image filename must not be empty"));
                    }
                    if value.contains('"') || value.chars().any(char::is_control) {
                        return Err(error("invalid image command syntax"));
                    }
                    match name {
                        "IMAGE_CORNER" => corner_name = Some(value.to_owned()),
                        "IMAGE_EDGE_LEFT" => edge_names[0] = Some(value.to_owned()),
                        "IMAGE_EDGE_RIGHT" => edge_names[1] = Some(value.to_owned()),
                        "IMAGE_EDGE_BOTTOM" => edge_names[2] = Some(value.to_owned()),
                        _ => edge_names[3] = Some(value.to_owned()),
                    }
                }
                "IMAGE_SCALE" | "IMAGE_CORNER_PAD" | "IMAGE_EDGE_PAD" | "BORDER_LINES"
                | "BORDER_PAD" | "BORDER_END_OFFSETS" => {
                    let (start, count) = match name {
                        "IMAGE_SCALE" => (0, 1),
                        "IMAGE_CORNER_PAD" => (1, 2),
                        "BORDER_LINES" => (3, 4),
                        "BORDER_PAD" => (7, 2),
                        "BORDER_END_OFFSETS" => (9, 2),
                        _ => (11, 2),
                    };
                    let numbers = args
                        .split_whitespace()
                        .map(str::parse::<f64>)
                        .collect::<std::result::Result<Vec<_>, _>>()
                        .map_err(|_| error("invalid numeric arguments"))?;
                    if numbers.len() != count {
                        return Err(error("wrong number of numeric arguments"));
                    }
                    policy.values[start..start + count].copy_from_slice(&numbers);
                    policy = DecorativeBorderPolicy::parse(&policy.values, policy.colour)
                        .map_err(|message| error(&message))?;
                }
                _ => {
                    return Err(error(
                        "unsupported decorative-border command; remove it or use a supported definition",
                    ));
                }
            }
        }
        if corner_name.is_none() && edge_names.iter().all(Option::is_none) {
            return Err(
                "Decorative border requires at least one image command: IMAGE_CORNER or IMAGE_EDGE_*"
                    .into(),
            );
        }
        let base = path.parent().unwrap_or_else(|| Path::new(""));
        let corner = corner_name
            .as_deref()
            .map(|name| load_decorative_png(&base.join(name), "IMAGE_CORNER"))
            .transpose()?;
        let mut edges = [None, None, None, None];
        for (index, (name, resource)) in edge_names
            .iter()
            .zip([
                "NativeDecorativeEdgeLeft",
                "NativeDecorativeEdgeRight",
                "NativeDecorativeEdgeBottom",
                "NativeDecorativeEdgeTop",
            ])
            .enumerate()
        {
            if let Some(name) = name {
                edges[index] = Some(load_decorative_png(&base.join(name), resource)?);
            }
        }
        Ok(Self {
            policy,
            corner,
            edges,
        })
    }

    pub fn layout(&self, margin: Bounds, page_height_mm: f64) -> Result<DecorativeBorderLayout> {
        margin.inset(0.0, 0.0)?;
        if !page_height_mm.is_finite()
            || page_height_mm <= 0.0
            || !mm_to_points(page_height_mm).is_finite()
        {
            return Err(
                "native_pdf.decorative_border page height must be finite and positive".into(),
            );
        }
        let [
            scale,
            ch,
            cv,
            outer,
            inner1,
            inner2,
            gap,
            ph,
            pv,
            eh,
            ev,
            edge_h,
            edge_v,
        ] = self.policy.values;
        let mut corners = Vec::new();
        if let Some(image) = &self.corner {
            let (w, h) = image_dimensions_mm(image, scale)?;
            let left = margin.left - ch;
            let right = margin.right + ch;
            let top = margin.top + cv;
            let bottom = margin.bottom - cv;
            for rect_mm in [
                [left, top - h, w, h],
                [right, top - h, -w, h],
                [left, bottom + h, w, -h],
                [right, bottom + h, -w, -h],
            ] {
                validate_image_rect(rect_mm)?;
                corners.push(DecorativeImageLayout {
                    resource_name: "NativeDecorativeCorner",
                    image: Arc::clone(image),
                    rect_mm,
                });
            }
        }

        let horizontal_center = (margin.left + margin.right) / 2.0;
        let vertical_center = (margin.top + margin.bottom) / 2.0;
        let mut edges = Vec::new();
        for (index, image) in self.edges.iter().enumerate() {
            let Some(image) = image else {
                continue;
            };
            let (w, h) = image_dimensions_mm(image, scale)?;
            let rect_mm = match index {
                0 => [margin.left - edge_h - w, vertical_center - h / 2.0, w, h],
                1 => [margin.right + edge_h + w, vertical_center - h / 2.0, -w, h],
                2 => [horizontal_center - w / 2.0, margin.top + edge_v, w, h],
                _ => [horizontal_center - w / 2.0, margin.bottom - edge_v, w, -h],
            };
            validate_image_rect(rect_mm)?;
            let resource_name = [
                "NativeDecorativeEdgeLeft",
                "NativeDecorativeEdgeRight",
                "NativeDecorativeEdgeBottom",
                "NativeDecorativeEdgeTop",
            ][index];
            edges.push(DecorativeImageLayout {
                resource_name,
                image: Arc::clone(image),
                rect_mm,
            });
        }

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
            edges,
            lines,
            colour: self.policy.colour,
            page_height_mm,
        })
    }
}

fn load_decorative_png(path: &Path, name: &str) -> Result<Arc<Image>> {
    let bytes = fs::read(path).map_err(|error| {
        format!(
            "Cannot read decorative {name} PNG {}: {error}",
            path.display()
        )
    })?;
    let decoded = image::ImageReader::with_format(Cursor::new(&bytes), image::ImageFormat::Png)
        .decode()
        .map_err(|_| {
            format!(
                "Invalid or oversized decorative {name} PNG {}",
                path.display()
            )
        })?;
    if !decoded.color().has_alpha() {
        return Err(format!(
            "Decorative {name} PNG {} requires an alpha channel",
            path.display()
        ));
    }
    let image = Image::from_png_data(bytes)
        .map_err(|_| format!("Cannot decode decorative {name} PNG {}", path.display()))?;
    Ok(Arc::new(image))
}

fn image_dimensions_mm(image: &Image, scale: f64) -> Result<(f64, f64)> {
    let width_points = f64::from(image.width()) * scale;
    let height_points = f64::from(image.height()) * scale;
    let width = points_to_mm(width_points);
    let height = points_to_mm(height_points);
    if !width_points.is_finite()
        || !height_points.is_finite()
        || width_points <= 0.0
        || height_points <= 0.0
        || !width.is_finite()
        || !height.is_finite()
        || !mm_to_points(width).is_finite()
        || !mm_to_points(height).is_finite()
        || width <= 0.0
        || height <= 0.0
    {
        return Err("native_pdf.decorative_border image scale overflowed PDF coordinates".into());
    }
    Ok((width, height))
}

fn validate_image_rect([x, y, width, height]: [f64; 4]) -> Result<()> {
    if ![x, y, width, height, x + width, y + height]
        .iter()
        .all(|value| value.is_finite() && mm_to_points(*value).is_finite())
        || width == 0.0
        || height == 0.0
    {
        return Err("native_pdf.decorative_border image bounds overflow PDF coordinates".into());
    }
    Ok(())
}
