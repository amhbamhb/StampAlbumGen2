//! Measured heading/frame row geometry shared by compatibility and native input.

use super::{
    Result,
    colour::NamedColour,
    document::Bounds,
    stamp_text::{PositionedStampText, StampFooterText},
    text::{MeasuredText, PositionedTextLine},
    units::mm_to_points,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

impl VerticalAlignment {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "TOP" => Ok(Self::Top),
            "MIDDLE" => Ok(Self::Middle),
            "BOTTOM" => Ok(Self::Bottom),
            _ => Err("Row vertical alignment must be TOP, MIDDLE or BOTTOM".into()),
        }
    }

    fn factor(self) -> f64 {
        match self {
            Self::Top => 0.0,
            Self::Middle => 0.5,
            Self::Bottom => 1.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeadingMode {
    Local,
    Aligned,
}

impl HeadingMode {
    pub fn parse(value: &str) -> Result<Self> {
        match value {
            "LOCAL" => Ok(Self::Local),
            "VA" => Ok(Self::Aligned),
            _ => Err("Heading mode must be LOCAL or VA".into()),
        }
    }
}

pub fn signed_mm(value: f64, name: &str) -> Result<f64> {
    if !value.is_finite() || !mm_to_points(value).is_finite() {
        return Err(format!(
            "{name} requires finite signed millimetres without PDF overflow"
        ));
    }
    Ok(value)
}

#[derive(Clone, Debug)]
pub struct StampHeading {
    pub measured: MeasuredText,
    pub mode: HeadingMode,
    pub offset_mm: f64,
    pub gap_mm: f64,
    pub colour: NamedColour,
}

impl StampHeading {
    pub fn new(
        measured: MeasuredText,
        mode: HeadingMode,
        offset_mm: f64,
        gap_mm: f64,
        colour: NamedColour,
    ) -> Result<Self> {
        signed_mm(offset_mm, "Heading offset")?;
        signed_mm(gap_mm, "Heading gap")?;
        if measured
            .lines
            .iter()
            .all(|line| line.text.trim().is_empty())
        {
            return Err("Stamp heading must contain visible text".into());
        }
        Ok(Self {
            measured,
            mode,
            offset_mm,
            gap_mm,
            colour,
        })
    }

    fn above_mm(&self) -> f64 {
        self.measured.ascent_mm
            + (self.measured.lines.len() - 1) as f64 * self.measured.line_advance_mm
            + self.gap_mm
    }
}

pub struct HeadingRowItem<'a> {
    pub width_mm: f64,
    pub height_mm: f64,
    pub below_mm: f64,
    pub left_overhang_mm: f64,
    pub right_overhang_mm: f64,
    pub top_overhang_mm: f64,
    pub bottom_overhang_mm: f64,
    pub heading: Option<&'a StampHeading>,
}

pub struct HeadingPlacement {
    pub frame: Bounds,
    pub heading: Option<PositionedStampText>,
}

pub struct HeadingRowLayout {
    pub items: Vec<HeadingPlacement>,
    pub bottom_mm: f64,
}

#[derive(Clone, Debug)]
pub struct NativeHeadingItem {
    pub heading: Option<StampHeading>,
    pub footer_text: Option<StampFooterText>,
    pub vertical: VerticalAlignment,
}

/// `lefts` come from the owning row's horizontal alignment policy.
pub fn layout_row(
    content: Bounds,
    top: f64,
    items: &[HeadingRowItem<'_>],
    lefts: &[f64],
    alignment: VerticalAlignment,
) -> Result<HeadingRowLayout> {
    if items.is_empty() || items.len() != lefts.len() {
        return Err("Heading row requires matching nonempty items and positions".into());
    }
    let max_height = items
        .iter()
        .map(|item| item.height_mm + item.top_overhang_mm + item.bottom_overhang_mm)
        .fold(0.0, f64::max);
    let max_above = items
        .iter()
        .filter_map(|item| item.heading)
        .map(StampHeading::above_mm)
        .fold(0.0, f64::max);
    let mut result = Vec::new();
    let mut bottom = top;
    for (item, &left) in items.iter().zip(lefts) {
        for value in [
            item.width_mm,
            item.height_mm,
            item.below_mm,
            item.left_overhang_mm,
            item.right_overhang_mm,
            item.top_overhang_mm,
            item.bottom_overhang_mm,
            left,
            top,
        ] {
            signed_mm(value, "Heading row geometry")?;
        }
        if item.width_mm <= 0.0
            || item.height_mm <= 0.0
            || item.below_mm < 0.0
            || item.left_overhang_mm < 0.0
            || item.right_overhang_mm < 0.0
            || item.top_overhang_mm < 0.0
            || item.bottom_overhang_mm < 0.0
        {
            return Err(
                "Heading row dimensions must be positive with nonnegative caption height".into(),
            );
        }
        let item_height = item.height_mm + item.top_overhang_mm + item.bottom_overhang_mm;
        let shift = (max_height - item_height) * alignment.factor();
        let above = match item.heading {
            Some(heading) if heading.mode == HeadingMode::Local => heading.above_mm(),
            _ => max_above,
        };
        let frame = Bounds {
            left,
            right: left + item.width_mm,
            top: top - above - shift - item.top_overhang_mm,
            bottom: top - above - shift - item.top_overhang_mm - item.height_mm,
        };
        let heading = item.heading.map(|heading| {
            let measured = heading.measured.clone();
            let heading_top = top
                - if heading.mode == HeadingMode::Local {
                    shift
                } else {
                    0.0
                };
            let centre = left + item.width_mm / 2.0 + heading.offset_mm;
            let lines = measured
                .lines
                .iter()
                .enumerate()
                .map(|(i, line)| PositionedTextLine {
                    text: line.text.clone(),
                    left_mm: centre - line.width_mm / 2.0,
                    baseline_mm: heading_top
                        - measured.ascent_mm
                        - i as f64 * measured.line_advance_mm,
                    width_mm: line.width_mm,
                })
                .collect::<Vec<_>>();
            let text_width = measured
                .lines
                .iter()
                .map(|line| line.width_mm)
                .fold(0.0, f64::max);
            let bounds = Bounds {
                left: centre - text_width / 2.0,
                right: centre + text_width / 2.0,
                top: heading_top,
                bottom: heading_top
                    - measured.ascent_mm
                    - measured.descent_mm
                    - (measured.lines.len() - 1) as f64 * measured.line_advance_mm,
            };
            PositionedStampText {
                measured,
                colour: heading.colour,
                lines,
                bounds,
            }
        });
        frame.inset(0.0, 0.0)?;
        if frame.left - item.left_overhang_mm < content.left
            || frame.right + item.right_overhang_mm > content.right
            || frame.top + item.top_overhang_mm > content.top
        {
            return Err("Heading/frame row exceeds usable page bounds".into());
        }
        if let Some(text) = &heading {
            if text.bounds.left < content.left
                || text.bounds.right > content.right
                || text.bounds.top > content.top
                || text.bounds.bottom < content.bottom
            {
                return Err("Stamp heading offset/text exceeds usable page bounds".into());
            }
            bottom = bottom.min(text.bounds.bottom);
        }
        bottom = bottom.min(frame.bottom - item.bottom_overhang_mm - item.below_mm);
        result.push(HeadingPlacement { frame, heading });
    }
    if !bottom.is_finite() || bottom < content.bottom {
        return Err("Heading/frame row exceeds page height".into());
    }
    Ok(HeadingRowLayout {
        items: result,
        bottom_mm: bottom,
    })
}

pub fn paginate_native(
    settings: &super::document::PageSettings,
    items: &[super::stamp_image::StampImageRowItem],
    headings: &[NativeHeadingItem],
    sections: &[super::pagination::StampRowSection],
    policy: &super::stamp_text::StampTextPolicy,
) -> Result<Vec<super::pagination::StampItemPage>> {
    paginate_native_with_page_breaks(settings, items, headings, sections, policy, &[])
}

pub(crate) fn paginate_native_with_page_breaks(
    settings: &super::document::PageSettings,
    items: &[super::stamp_image::StampImageRowItem],
    headings: &[NativeHeadingItem],
    sections: &[super::pagination::StampRowSection],
    policy: &super::stamp_text::StampTextPolicy,
    page_breaks: &[super::pagination::NativePageBreak],
) -> Result<Vec<super::pagination::StampItemPage>> {
    use super::{
        pagination::{StampItemPage, StampPageFragment},
        stamp_text,
    };
    if items.len() != headings.len()
        || sections.iter().map(|s| s.item_count).sum::<usize>() != items.len()
        || sections.iter().any(|s| s.item_count == 0)
    {
        return Err("Native heading sections must exactly cover nonempty stamp rows".into());
    }
    super::pagination::validate_page_breaks(sections, page_breaks)?;
    let date = settings.capture_generation_date();
    let mut total = 1;
    let mut seen = std::collections::HashSet::new();
    for _ in 0..32 {
        if !seen.insert(total) {
            return Err("Native heading pagination does not converge".into());
        }
        let mut pages = Vec::new();
        let mut page = StampItemPage {
            layout: settings.resolve_for_pagination(1, total, date)?,
            items: Vec::new(),
        };
        let mut top = page.layout.content_bounds.top;
        let mut start = 0;
        let mut active_settings = settings.clone();
        let mut breaks = page_breaks.iter().copied().peekable();
        for section in sections {
            while let Some(entry) = breaks.next_if(|entry| entry.item_index == start) {
                active_settings.set_page_spacing(entry.spacing);
                super::pagination::advance_page(
                    &active_settings,
                    &mut pages,
                    &mut page,
                    total,
                    date,
                )?;
                top = page.layout.content_bounds.top;
            }
            let settings = &active_settings;
            let end = start + section.item_count;
            let metadata = &headings[start..end];
            if metadata
                .iter()
                .any(|item| item.vertical != metadata[0].vertical)
            {
                return Err("Native vertical alignment changes require NATIVE_ROW_BREAK".into());
            }
            let measured = items[start..end]
                .iter()
                .map(|item| policy.measure_item(&item.stamp))
                .collect::<Result<Vec<_>>>()?;
            let measured_footer = metadata
                .iter()
                .zip(items[start..end].iter())
                .map(|(metadata, item)| {
                    metadata
                        .footer_text
                        .as_ref()
                        .map(|footer| {
                            let frame = item.stamp.geometry.frame_bounds();
                            footer.measure(frame.right - frame.left, policy)
                        })
                        .transpose()
                })
                .collect::<Result<Vec<_>>>()?;
            let side_overhangs = items[start..end]
                .iter()
                .map(|item| {
                    let measure =
                        |label: Option<&super::stamp_image::StampSideLabelSettings>| -> Result<_> {
                            label
                                .map(|label| {
                                    let size_pt = label.size_pt.ok_or(
                                        "native side-label font size was not resolved from TOML",
                                    )?;
                                    MeasuredText::new(&label.font, size_pt, &label.text).map(
                                        |text| {
                                            let width = text
                                                .lines
                                                .iter()
                                                .map(|line| line.width_mm)
                                                .fold(0.0, f64::max);
                                            let height = stamp_text::height(&text);
                                            (width, height)
                                        },
                                    )
                                })
                                .transpose()
                        };
                    let left =
                        measure(item.labels.as_ref().and_then(|labels| labels.left.as_ref()))?;
                    let right = measure(
                        item.labels
                            .as_ref()
                            .and_then(|labels| labels.right.as_ref()),
                    )?;
                    let frame = item.stamp.geometry.frame_bounds();
                    let frame_height = frame.top - frame.bottom;
                    let label_height = left
                        .map(|(_, height)| height)
                        .into_iter()
                        .chain(right.map(|(_, height)| height))
                        .fold(frame_height, f64::max);
                    let vertical_overhang = ((label_height - frame_height) / 2.0).max(0.0);
                    Ok::<_, String>((
                        left.map_or(0.0, |(width, _)| {
                            width + super::stamp_image::STAMP_IMAGE_LABEL_GAP_MM
                        }),
                        right.map_or(0.0, |(width, _)| {
                            width + super::stamp_image::STAMP_IMAGE_LABEL_GAP_MM
                        }),
                        vertical_overhang,
                        vertical_overhang,
                    ))
                })
                .collect::<Result<Vec<_>>>()?;
            let (pad_x, pad_y) = policy.padding();
            let row_items = items[start..end]
                .iter()
                .zip(metadata)
                .zip(measured.iter().zip(&measured_footer).zip(&side_overhangs))
                .map(
                    |((item, metadata), ((measurement, footer), &side_overhangs))| {
                        let (side_left, side_right, side_top, side_bottom) = side_overhangs;
                        let frame = item.stamp.geometry.frame_bounds();
                        let frame_width = frame.right - frame.left;
                        let caption_below = measurement
                            .1
                            .as_ref()
                            .map_or(0.0, |text| stamp_text::height(text) + policy.gap());
                        let footer_below = metadata
                            .footer_text
                            .as_ref()
                            .zip(footer.as_ref())
                            .map_or(0.0, |(settings, measured)| {
                                settings.below_mm(measured, policy.gap())
                            });
                        let text_overhang = ((measurement.2 - frame_width) / 2.0).max(0.0);
                        let footer_overhang = footer
                            .as_ref()
                            .map_or(0.0, |text| (text.width() + pad_x - frame_width).max(0.0));
                        HeadingRowItem {
                            width_mm: frame_width,
                            height_mm: frame.top - frame.bottom,
                            below_mm: caption_below.max(footer_below) + pad_y,
                            left_overhang_mm: side_left.max(text_overhang).max(footer_overhang),
                            right_overhang_mm: side_right.max(text_overhang).max(footer_overhang),
                            top_overhang_mm: side_top,
                            bottom_overhang_mm: side_bottom,
                            heading: metadata.heading.as_ref(),
                        }
                    },
                )
                .collect::<Vec<_>>();
            let widths = row_items
                .iter()
                .map(|item| item.width_mm + item.left_overhang_mm + item.right_overhang_mm)
                .collect::<Vec<_>>();
            let place = |page: &StampItemPage, top: f64| -> Result<HeadingRowLayout> {
                let content = page.layout.content_bounds;
                let footprint_lefts = section.alignment.positions(
                    content,
                    &widths,
                    settings.spacing().horizontal_mm(),
                )?;
                let lefts = footprint_lefts
                    .iter()
                    .zip(&row_items)
                    .map(|(&left, item)| left + item.left_overhang_mm)
                    .collect::<Vec<_>>();
                layout_row(
                    content,
                    top - pad_y,
                    &row_items,
                    &lefts,
                    metadata[0].vertical,
                )
            };
            let layout = match place(&page, top) {
                Ok(layout) => layout,
                Err(error) if !page.items.is_empty() => {
                    pages.push(page);
                    page = StampItemPage {
                        layout: settings.resolve_for_pagination(pages.len() + 1, total, date)?,
                        items: Vec::new(),
                    };
                    top = page.layout.content_bounds.top;
                    place(&page,top).map_err(|fresh|format!("Native heading row starting at stamp {} cannot fit a fresh page: {fresh}; previous page: {error}",start+1))?
                }
                Err(error) => {
                    return Err(format!(
                        "Native heading row starting at stamp {}: {error}",
                        start + 1
                    ));
                }
            };
            for (offset, placed) in layout.items.into_iter().enumerate() {
                let item = &items[start + offset];
                let side_bottom = row_items[offset].bottom_overhang_mm;
                let caption = stamp_text::position(
                    measured[offset].1.clone(),
                    &item.stamp.caption,
                    (placed.frame.left + placed.frame.right) / 2.0,
                    placed.frame.bottom - side_bottom - policy.gap(),
                );
                let footer_text = match (
                    metadata[offset].footer_text.as_ref(),
                    measured_footer[offset].clone(),
                ) {
                    (Some(footer), Some(measured)) => Some(footer.position(
                        measured,
                        placed.frame,
                        stamp_text::footer_caption_top(
                            placed.frame.bottom - side_bottom,
                            policy.gap(),
                            footer.padding_mm,
                        )?,
                    )?),
                    _ => None,
                };
                let geometry = item
                    .stamp
                    .geometry
                    .at_frame_top_left(placed.frame.left, placed.frame.top)?;
                let mut footprint = Bounds {
                    left: placed.frame.left,
                    right: placed.frame.right,
                    top: placed.heading.as_ref().map_or(
                        placed.frame.top + row_items[offset].top_overhang_mm,
                        |text| text.bounds.top,
                    ),
                    bottom: caption
                        .as_ref()
                        .map_or(placed.frame.bottom - side_bottom, |text| text.bounds.bottom),
                };
                let (side_left, side_right, _, _) = side_overhangs[offset];
                footprint.left -= side_left;
                footprint.right += side_right;
                for text in footer_text.iter() {
                    let bounds = text.bounds(placed.frame);
                    footprint.left = footprint.left.min(bounds.left);
                    footprint.right = footprint.right.max(bounds.right);
                    footprint.top = footprint.top.max(bounds.top);
                    footprint.bottom = footprint.bottom.min(bounds.bottom);
                    if bounds.left < page.layout.content_bounds.left
                        || bounds.right > page.layout.content_bounds.right
                        || bounds.top > page.layout.content_bounds.top
                        || bounds.bottom < page.layout.content_bounds.bottom
                    {
                        return Err(format!(
                            "Native footer text for stamp {} exceeds usable page bounds",
                            start + offset + 1
                        ));
                    }
                }
                page.items.push(StampPageFragment {
                    source_index: start + offset,
                    geometry: Some(geometry),
                    heading: placed.heading,
                    caption,
                    footer_text,
                    footprint,
                });
            }
            top = settings.spacing().next_top(layout.bottom_mm)?;
            start = end;
        }
        while let Some(entry) = breaks.next_if(|entry| entry.item_index == items.len()) {
            active_settings.set_page_spacing(entry.spacing);
            super::pagination::advance_page(&active_settings, &mut pages, &mut page, total, date)?;
        }
        if !page.items.is_empty() || !page_breaks.is_empty() {
            pages.push(page);
        }
        if pages.len() == total {
            return Ok(pages);
        }
        total = pages.len();
    }
    Err("Native heading pagination exceeded 32 passes".into())
}
