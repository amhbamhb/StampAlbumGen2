//! Native text footprints, separate from physical stamps and mounting frames.

use super::{
    commands::page::album_pages_spacing::Spacing,
    document::Bounds,
    stamp_box::StampBoxGeometry,
    stamp_image::StampTextProofSettings,
    stamp_rows::fit_known_size_rows,
    text::{MeasuredText, PositionedTextLine},
    units::mm_to_points,
    Result,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextWidthPolicy {
    WrapToFrame,
    ExpandFootprint,
}

impl TextWidthPolicy {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "wrap_to_frame" => Ok(Self::WrapToFrame),
            "expand_footprint" => Ok(Self::ExpandFootprint),
            _ => Err(
                "native_pdf.stamp_text.width_policy must be wrap_to_frame or expand_footprint"
                    .into(),
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StampTextPolicy {
    pub width_policy: TextWidthPolicy,
    expanded_width: ExpandedWidthLimit,
    horizontal_padding_mm: f64,
    vertical_padding_mm: f64,
    text_gap_mm: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct ExpandedWidthLimit {
    percent: bool,
    value: f64,
}

impl ExpandedWidthLimit {
    pub fn new(unit: &str, value: f64) -> Result<Self> {
        let percent = match unit {
            "percent" => true,
            "mm" => false,
            _ => {
                return Err(
                    "native_pdf.stamp_text.expanded_width_limit must be percent or mm".into(),
                )
            }
        };
        if !value.is_finite()
            || value <= 0.0
            || (percent && value < 100.0)
            || (!percent && !mm_to_points(value).is_finite())
        {
            return Err("native_pdf.stamp_text.expanded_width_value must be finite: percent >= 100, or positive mm without PDF overflow".into());
        }
        Ok(Self { percent, value })
    }

    fn text_width(self, frame_width: f64) -> Result<f64> {
        let width = if self.percent {
            frame_width * (self.value / 100.0)
        } else {
            self.value
        };
        if !width.is_finite() || !mm_to_points(width).is_finite() || width < frame_width {
            return Err("native_pdf.stamp_text expanded text cap must be finite and at least the mounting frame width".into());
        }
        Ok(width)
    }
}

impl StampTextPolicy {
    pub fn new(
        width_policy: TextWidthPolicy,
        expanded_width: ExpandedWidthLimit,
        horizontal_padding_mm: f64,
        vertical_padding_mm: f64,
        text_gap_mm: f64,
    ) -> Result<Self> {
        for (key, value) in [
            ("horizontal_padding_mm", horizontal_padding_mm),
            ("vertical_padding_mm", vertical_padding_mm),
            ("text_gap_mm", text_gap_mm),
        ] {
            if !value.is_finite() || value < 0.0 || !mm_to_points(value * 2.0).is_finite() {
                return Err(format!("native_pdf.stamp_text.{key} must be finite nonnegative millimetres without PDF overflow"));
            }
        }
        Ok(Self {
            width_policy,
            expanded_width,
            horizontal_padding_mm,
            vertical_padding_mm,
            text_gap_mm,
        })
    }
}

#[derive(Clone, Debug)]
pub struct StampTextItem {
    pub geometry: StampBoxGeometry,
    pub heading: Option<StampTextProofSettings>,
    pub caption: Option<StampTextProofSettings>,
}

#[derive(Clone, Debug)]
pub struct PositionedStampText {
    pub measured: MeasuredText,
    pub colour: super::colour::NamedColour,
    pub lines: Vec<PositionedTextLine>,
    pub bounds: Bounds,
}

#[derive(Clone, Debug)]
pub struct PositionedStampItem {
    pub geometry: StampBoxGeometry,
    pub footprint: Bounds,
    pub heading: Option<PositionedStampText>,
    pub caption: Option<PositionedStampText>,
}

fn measure(
    settings: &Option<StampTextProofSettings>,
    width: f64,
    policy: &StampTextPolicy,
) -> Result<Option<MeasuredText>> {
    settings
        .as_ref()
        .map(|settings| {
            let text = MeasuredText::new(&settings.font, settings.size_pt, &settings.text)?;
            match policy.width_policy {
                TextWidthPolicy::WrapToFrame => text.wrapped_to_width(width),
                TextWidthPolicy::ExpandFootprint => {
                    let cap = policy.expanded_width.text_width(width)?;
                    if text.lines.iter().any(|line| line.width_mm > cap) {
                        text.wrapped_to_width(cap)
                    } else {
                        Ok(text)
                    }
                }
            }
        })
        .transpose()
        .map_err(|error| format!("native_pdf.stamp_text: {error}"))
}

fn height(text: &MeasuredText) -> f64 {
    text.ascent_mm + text.descent_mm + (text.lines.len() - 1) as f64 * text.line_advance_mm
}

fn position(
    measured: Option<MeasuredText>,
    settings: &Option<StampTextProofSettings>,
    centre: f64,
    top: f64,
) -> Option<PositionedStampText> {
    measured.zip(settings.as_ref()).map(|(measured, settings)| {
        let width = measured
            .lines
            .iter()
            .map(|line| line.width_mm)
            .fold(0.0, f64::max);
        let bounds = Bounds {
            left: centre - width / 2.0,
            right: centre + width / 2.0,
            top,
            bottom: top - height(&measured),
        };
        let lines = measured
            .lines
            .iter()
            .enumerate()
            .map(|(index, line)| PositionedTextLine {
                text: line.text.clone(),
                left_mm: centre - line.width_mm / 2.0,
                baseline_mm: top - measured.ascent_mm - index as f64 * measured.line_advance_mm,
                width_mm: line.width_mm,
            })
            .collect();
        PositionedStampText {
            measured,
            colour: settings.colour,
            lines,
            bounds,
        }
    })
}

/// Fit complete text/frame footprints; mounting and physical sizes never change.
pub fn fit_text_rows(
    content: Bounds,
    items: &[StampTextItem],
    spacing: Spacing,
    policy: &StampTextPolicy,
) -> Result<Vec<Vec<PositionedStampItem>>> {
    let mut measurements = Vec::new();
    let mut footprints = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let frame = item.geometry.frame_bounds();
        let frame_width = frame.right - frame.left;
        if policy.width_policy == TextWidthPolicy::ExpandFootprint {
            policy.expanded_width.text_width(frame_width)?;
        }
        let heading = measure(&item.heading, frame_width, policy)?;
        let caption = measure(&item.caption, frame_width, policy)?;
        let above = heading
            .as_ref()
            .map_or(0.0, |text| height(text) + policy.text_gap_mm);
        let below = caption
            .as_ref()
            .map_or(0.0, |text| height(text) + policy.text_gap_mm);
        let width = heading
            .iter()
            .chain(caption.iter())
            .flat_map(|text| text.lines.iter())
            .map(|line| line.width_mm)
            .fold(frame_width, f64::max)
            + 2.0 * policy.horizontal_padding_mm;
        let total_height =
            frame.top - frame.bottom + above + below + 2.0 * policy.vertical_padding_mm;
        footprints.push(
            StampBoxGeometry::from_total_additions(
                Bounds {
                    left: 0.0,
                    bottom: 0.0,
                    right: width,
                    top: total_height,
                },
                0.0,
                0.0,
                0.0,
            )
            .map_err(|error| {
                format!(
                    "native_pdf.stamp_text item {} footprint: {error}",
                    index + 1
                )
            })?,
        );
        measurements.push((heading, caption, above));
    }
    let rows = fit_known_size_rows(content, &footprints, spacing)?;
    let mut source = items.iter().zip(measurements);
    rows.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|placed| {
                    let (item, (heading, caption, above)) = source
                        .next()
                        .ok_or("native_pdf.stamp_text internal item count mismatch")?;
                    let footprint = placed.frame_bounds();
                    let centre = (footprint.left + footprint.right) / 2.0;
                    let physical = item.geometry.physical_bounds();
                    let frame_top = footprint.top - policy.vertical_padding_mm - above;
                    let frame_height =
                        item.geometry.frame_bounds().top - item.geometry.frame_bounds().bottom;
                    let physical_width = physical.right - physical.left;
                    let physical_height = physical.top - physical.bottom;
                    let y = item.geometry.vertical_clearance_mm();
                    let geometry = StampBoxGeometry::from_total_additions(
                        Bounds {
                            left: centre - physical_width / 2.0,
                            right: centre + physical_width / 2.0,
                            top: frame_top - y,
                            bottom: frame_top - y - physical_height,
                        },
                        item.geometry.horizontal_clearance_mm() * 2.0,
                        y * 2.0,
                        0.0,
                    )?;
                    let heading = position(
                        heading,
                        &item.heading,
                        centre,
                        footprint.top - policy.vertical_padding_mm,
                    );
                    let caption = position(
                        caption,
                        &item.caption,
                        centre,
                        frame_top - frame_height - policy.text_gap_mm,
                    );
                    Ok(PositionedStampItem {
                        geometry,
                        footprint,
                        heading,
                        caption,
                    })
                })
                .collect()
        })
        .collect()
}
