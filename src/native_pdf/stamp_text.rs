//! Native text footprints, separate from physical stamps and mounting frames.

use super::{
    commands::page::album_pages_spacing::Spacing,
    document::Bounds,
    stamp_box::StampBoxGeometry,
    stamp_image::StampTextProofSettings,
    stamp_rows::{fit_known_size_rows_with_alignment, RowAlignment},
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FooterTextAlignment {
    Align,
    Centre,
    CornerCentre,
}

impl FooterTextAlignment {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "ALIGN" => Ok(Self::Align),
            "CENTRE" | "CENTER" => Ok(Self::Centre),
            "CNR_CENTRE" | "CNR_CENTER" => Ok(Self::CornerCentre),
            _ => Err(
                "Footer text alignment must be ALIGN, CENTRE, CNR_CENTRE, CENTER or CNR_CENTER"
                    .into(),
            ),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FooterTextSide {
    Left,
    Centre,
    Right,
}

#[derive(Clone, Debug)]
pub struct StampFooterText {
    pub left: Option<StampTextProofSettings>,
    pub centre: Option<StampTextProofSettings>,
    pub right: Option<StampTextProofSettings>,
    pub alignment: FooterTextAlignment,
    pub padding_mm: f64,
}

#[derive(Clone, Debug)]
pub struct MeasuredFooterText {
    pub left: Option<MeasuredText>,
    pub centre: Option<MeasuredText>,
    pub right: Option<MeasuredText>,
}

impl MeasuredFooterText {
    pub(crate) fn height(&self) -> f64 {
        [&self.left, &self.centre, &self.right]
            .into_iter()
            .flatten()
            .map(height)
            .fold(0.0, f64::max)
    }

    pub(crate) fn width(&self) -> f64 {
        [&self.left, &self.centre, &self.right]
            .into_iter()
            .flatten()
            .flat_map(|text| text.lines.iter())
            .map(|line| line.width_mm)
            .fold(0.0, f64::max)
    }
}

#[derive(Clone, Debug)]
pub struct PositionedFooterText {
    pub left: Option<PositionedStampText>,
    pub centre: Option<PositionedStampText>,
    pub right: Option<PositionedStampText>,
}

impl PositionedFooterText {
    pub(crate) fn bounds(&self, frame: Bounds) -> Bounds {
        [&self.left, &self.centre, &self.right]
            .into_iter()
            .flatten()
            .fold(frame, |bounds, text| Bounds {
                left: bounds.left.min(text.bounds.left),
                bottom: bounds.bottom.min(text.bounds.bottom),
                right: bounds.right.max(text.bounds.right),
                top: bounds.top.max(text.bounds.top),
            })
    }
}

pub(crate) fn position_footer_caption(
    measured: MeasuredText,
    frame: Bounds,
    top: f64,
    side: FooterTextSide,
    alignment: FooterTextAlignment,
    colour: super::colour::NamedColour,
) -> Result<PositionedStampText> {
    let first = measured
        .lines
        .first()
        .ok_or("Footer caption has no measured lines")?;
    let frame_centre = (frame.left + frame.right) / 2.0;
    let first_left = match side {
        FooterTextSide::Left => frame.left,
        FooterTextSide::Centre => frame_centre - first.width_mm / 2.0,
        FooterTextSide::Right => frame.right - first.width_mm,
    };
    let first_centre = first_left + first.width_mm / 2.0;
    let corner_centre = match side {
        FooterTextSide::Left => frame.left,
        FooterTextSide::Centre => frame_centre,
        FooterTextSide::Right => frame.right,
    };
    let lines = measured
        .lines
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let left_mm = match side {
                FooterTextSide::Centre => frame_centre - line.width_mm / 2.0,
                FooterTextSide::Left if alignment == FooterTextAlignment::Align => frame.left,
                FooterTextSide::Right if alignment == FooterTextAlignment::Align => {
                    frame.right - line.width_mm
                }
                _ if index == 0 => first_left,
                _ if alignment == FooterTextAlignment::Centre => first_centre - line.width_mm / 2.0,
                _ => corner_centre - line.width_mm / 2.0,
            };
            PositionedTextLine {
                text: line.text.clone(),
                left_mm,
                baseline_mm: top - measured.ascent_mm - index as f64 * measured.line_advance_mm,
                width_mm: line.width_mm,
            }
        })
        .collect::<Vec<_>>();
    let left = lines
        .iter()
        .map(|line| line.left_mm)
        .fold(f64::INFINITY, f64::min);
    let right = lines
        .iter()
        .map(|line| line.left_mm + line.width_mm)
        .fold(f64::NEG_INFINITY, f64::max);
    let bounds = Bounds {
        left,
        right,
        top,
        bottom: top - height(&measured),
    };
    for value in [bounds.left, bounds.bottom, bounds.right, bounds.top] {
        if !value.is_finite() || !mm_to_points(value).is_finite() {
            return Err("Footer caption positioning overflows PDF coordinates".into());
        }
    }
    Ok(PositionedStampText {
        measured,
        colour,
        lines,
        bounds,
    })
}

pub(crate) fn footer_caption_top(frame_bottom: f64, gap_mm: f64, padding_mm: f64) -> Result<f64> {
    let top = frame_bottom - gap_mm - padding_mm;
    if !top.is_finite() || !mm_to_points(top).is_finite() {
        return Err("Footer text padding overflows PDF coordinates".into());
    }
    Ok(top)
}

impl StampFooterText {
    pub(crate) fn measure(
        &self,
        width: f64,
        policy: &StampTextPolicy,
    ) -> Result<MeasuredFooterText> {
        Ok(MeasuredFooterText {
            left: measure(&self.left, width, policy)?,
            centre: measure(&self.centre, width, policy)?,
            right: measure(&self.right, width, policy)?,
        })
    }

    pub(crate) fn position(
        &self,
        measured: MeasuredFooterText,
        frame: Bounds,
        top: f64,
    ) -> Result<PositionedFooterText> {
        let position = |measured: Option<MeasuredText>,
                        settings: &Option<StampTextProofSettings>,
                        side: FooterTextSide| {
            measured
                .zip(settings.as_ref())
                .map(|(measured, settings)| {
                    position_footer_caption(
                        measured,
                        frame,
                        top,
                        side,
                        self.alignment,
                        settings.colour,
                    )
                })
                .transpose()
        };
        Ok(PositionedFooterText {
            left: position(measured.left, &self.left, FooterTextSide::Left)?,
            centre: position(measured.centre, &self.centre, FooterTextSide::Centre)?,
            right: position(measured.right, &self.right, FooterTextSide::Right)?,
        })
    }

    pub(crate) fn below_mm(&self, measured: &MeasuredFooterText, gap_mm: f64) -> f64 {
        (measured.height() + gap_mm + self.padding_mm).max(0.0)
    }
}

#[derive(Clone, Debug)]
pub struct PositionedStampItem {
    pub geometry: StampBoxGeometry,
    pub footprint: Bounds,
    pub heading: Option<PositionedStampText>,
    pub caption: Option<PositionedStampText>,
    pub footer_text: Option<PositionedFooterText>,
}

pub(crate) fn measure(
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

pub(crate) fn height(text: &MeasuredText) -> f64 {
    text.ascent_mm + text.descent_mm + (text.lines.len() - 1) as f64 * text.line_advance_mm
}

pub(crate) fn position(
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

#[cfg(test)]
mod tests {
    use super::*;

    fn caption(
        alignment: FooterTextAlignment,
        side: FooterTextSide,
    ) -> Result<PositionedStampText> {
        let frame = Bounds {
            left: 10.0,
            bottom: 10.0,
            right: 30.0,
            top: 30.0,
        };
        position_footer_caption(
            MeasuredText::new("HN", 10.0, "Anchor\nShort")?,
            frame,
            9.0,
            side,
            alignment,
            super::super::colour::NamedColour::parse("black")?,
        )
    }

    #[test]
    fn footer_alignment_modes_keep_first_line_anchors() -> Result<()> {
        let align = caption(FooterTextAlignment::Align, FooterTextSide::Left)?;
        let centre = caption(FooterTextAlignment::Centre, FooterTextSide::Left)?;
        let corner = caption(FooterTextAlignment::CornerCentre, FooterTextSide::Left)?;
        let anchor_width = align.lines[0].width_mm;
        let short_width = align.lines[1].width_mm;

        assert!((align.lines[0].left_mm - 10.0).abs() < 0.001);
        assert!((centre.lines[0].left_mm - 10.0).abs() < 0.001);
        assert!((corner.lines[0].left_mm - 10.0).abs() < 0.001);
        assert!(
            (centre.lines[1].left_mm - (10.0 + anchor_width / 2.0 - short_width / 2.0)).abs()
                < 0.001
        );
        assert!((corner.lines[1].left_mm - (10.0 - short_width / 2.0)).abs() < 0.001);

        let right = caption(FooterTextAlignment::Align, FooterTextSide::Right)?;
        assert!((right.lines[0].left_mm - (30.0 - anchor_width)).abs() < 0.001);
        assert!((right.lines[1].left_mm - (30.0 - short_width)).abs() < 0.001);
        Ok(())
    }

    #[test]
    fn footer_padding_shifts_the_caption_block_without_changing_leading() -> Result<()> {
        let frame = Bounds {
            left: 10.0,
            bottom: 10.0,
            right: 30.0,
            top: 30.0,
        };
        let measured = MeasuredText::new("HN", 10.0, "Top\nBottom")?;
        let position = |padding| {
            position_footer_caption(
                measured.clone(),
                frame,
                footer_caption_top(10.0, 0.5, padding)?,
                FooterTextSide::Left,
                FooterTextAlignment::Align,
                super::super::colour::NamedColour::parse("black")?,
            )
        };
        let baseline = position(0.0)?;
        let positive = position(6.0)?;
        let negative = position(-1.0)?;
        assert!(
            (positive.lines[0].baseline_mm - baseline.lines[0].baseline_mm + 6.0).abs() < 0.001
        );
        assert!(
            (negative.lines[0].baseline_mm - baseline.lines[0].baseline_mm - 1.0).abs() < 0.001
        );
        let baseline_leading = baseline.lines[0].baseline_mm - baseline.lines[1].baseline_mm;
        assert!(
            (baseline_leading - (positive.lines[0].baseline_mm - positive.lines[1].baseline_mm))
                .abs()
                < 0.001
        );
        assert!(
            (baseline_leading - (negative.lines[0].baseline_mm - negative.lines[1].baseline_mm))
                .abs()
                < 0.001
        );
        Ok(())
    }
}

impl StampTextPolicy {
    pub(crate) fn padding(&self) -> (f64, f64) {
        (self.horizontal_padding_mm, self.vertical_padding_mm)
    }

    pub(crate) fn gap(&self) -> f64 {
        self.text_gap_mm
    }

    pub(crate) fn measure_item(
        &self,
        item: &StampTextItem,
    ) -> Result<(Option<MeasuredText>, Option<MeasuredText>, f64, f64)> {
        let frame = item.geometry.frame_bounds();
        let frame_width = frame.right - frame.left;
        if self.width_policy == TextWidthPolicy::ExpandFootprint {
            self.expanded_width.text_width(frame_width)?;
        }
        let heading = measure(&item.heading, frame_width, self)?;
        let caption = measure(&item.caption, frame_width, self)?;
        let width = heading
            .iter()
            .chain(caption.iter())
            .flat_map(|text| text.lines.iter())
            .map(|line| line.width_mm)
            .fold(frame_width, f64::max)
            + 2.0 * self.horizontal_padding_mm;
        let total_height = frame.top - frame.bottom
            + heading
                .as_ref()
                .map_or(0.0, |t| height(t) + self.text_gap_mm)
            + caption
                .as_ref()
                .map_or(0.0, |t| height(t) + self.text_gap_mm)
            + 2.0 * self.vertical_padding_mm;
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
        )?;
        Ok((heading, caption, width, total_height))
    }
}

/// Fit complete text/frame footprints; mounting and physical sizes never change.
pub fn fit_text_rows(
    content: Bounds,
    items: &[StampTextItem],
    spacing: Spacing,
    policy: &StampTextPolicy,
) -> Result<Vec<Vec<PositionedStampItem>>> {
    fit_text_rows_with_alignment(content, items, spacing, policy, RowAlignment::Left)
}

pub fn fit_text_rows_with_alignment(
    content: Bounds,
    items: &[StampTextItem],
    spacing: Spacing,
    policy: &StampTextPolicy,
    alignment: RowAlignment,
) -> Result<Vec<Vec<PositionedStampItem>>> {
    let mut measurements = Vec::new();
    let mut footprints = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let (heading, caption, width, total_height) = policy
            .measure_item(item)
            .map_err(|error| format!("native_pdf.stamp_text item {}: {error}", index + 1))?;
        let above = heading
            .as_ref()
            .map_or(0.0, |text| height(text) + policy.text_gap_mm);
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
    let rows = fit_known_size_rows_with_alignment(content, &footprints, spacing, alignment)?;
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
                    let frame_top = footprint.top - policy.vertical_padding_mm - above;
                    let frame_height =
                        item.geometry.frame_bounds().top - item.geometry.frame_bounds().bottom;
                    let frame = item.geometry.frame_bounds();
                    let geometry = item
                        .geometry
                        .at_frame_top_left(centre - (frame.right - frame.left) / 2.0, frame_top)?;
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
                        footer_text: None,
                    })
                })
                .collect()
        })
        .collect()
}
