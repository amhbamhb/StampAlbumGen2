//! Ordered page layout with whole frames and line-boundary text continuation.

use super::{
    document::{Bounds, PageLayout, PageSettings},
    stamp_box::StampBoxGeometry,
    stamp_rows::{row_prefix_len, RowAlignment},
    stamp_text::{
        fit_text_rows, height, position, PositionedFooterText, PositionedStampText, StampTextItem,
        StampTextPolicy,
    },
    text::MeasuredText,
    Result,
};

#[derive(Clone, Debug)]
pub struct StampPageFragment {
    pub source_index: usize,
    pub footprint: Bounds,
    pub geometry: Option<StampBoxGeometry>,
    pub heading: Option<PositionedStampText>,
    pub caption: Option<PositionedStampText>,
    pub footer_text: Option<PositionedFooterText>,
}

#[derive(Clone, Debug)]
pub struct StampItemPage {
    pub layout: PageLayout,
    pub items: Vec<StampPageFragment>,
}

/// Consecutive items sharing one alignment. A section starts a new row, not
/// necessarily a new page. Counts must exactly cover the input item list.
#[derive(Clone, Copy, Debug)]
pub struct StampRowSection {
    pub item_count: usize,
    pub alignment: RowAlignment,
}

/// Final-count-sensitive layout is rerun until stable. Frames are never split,
/// scaled or repeated; oversized text continues before/after its frame in order.
pub fn paginate_stamp_items(
    settings: &PageSettings,
    items: &[StampTextItem],
    policy: &StampTextPolicy,
    generation_date: Option<chrono::NaiveDate>,
) -> Result<Vec<StampItemPage>> {
    let sections = if items.is_empty() {
        Vec::new()
    } else {
        vec![StampRowSection {
            item_count: items.len(),
            alignment: settings.row_alignment(),
        }]
    };
    paginate_stamp_sections(settings, items, &sections, policy, generation_date)
}

pub fn paginate_stamp_sections(
    settings: &PageSettings,
    items: &[StampTextItem],
    sections: &[StampRowSection],
    policy: &StampTextPolicy,
    generation_date: Option<chrono::NaiveDate>,
) -> Result<Vec<StampItemPage>> {
    paginate_stamp_sections_with_page_breaks(
        settings,
        items,
        sections,
        policy,
        generation_date,
        &[],
    )
}

pub(crate) fn paginate_stamp_sections_with_page_breaks(
    settings: &PageSettings,
    items: &[StampTextItem],
    sections: &[StampRowSection],
    policy: &StampTextPolicy,
    generation_date: Option<chrono::NaiveDate>,
    page_breaks: &[usize],
) -> Result<Vec<StampItemPage>> {
    let mut end = 0usize;
    let mut boundaries = Vec::new();
    for section in sections {
        if section.item_count == 0 {
            return Err("native_pdf.rows sections must contain at least one item".into());
        }
        end = end
            .checked_add(section.item_count)
            .ok_or("native_pdf.rows section counts overflow")?;
        boundaries.push((end, section.alignment));
    }
    if end != items.len() {
        return Err("native_pdf.rows section counts must exactly cover the input items".into());
    }
    validate_page_breaks(sections, page_breaks)?;
    let mut total = 1;
    let mut seen = std::collections::HashSet::new();
    for _ in 0..32 {
        if !seen.insert(total) {
            return Err("native_pdf.pagination final-count layout does not converge".into());
        }
        let pages = paginate_pass(
            settings,
            items,
            &boundaries,
            policy,
            generation_date,
            total,
            page_breaks,
        )?;
        if pages.len() == total {
            return Ok(pages);
        }
        total = pages.len();
    }
    Err("native_pdf.pagination final-count layout exceeds 32 resolution passes".into())
}

pub(crate) fn validate_page_breaks(
    sections: &[StampRowSection],
    page_breaks: &[usize],
) -> Result<()> {
    let mut boundaries = std::collections::HashSet::from([0]);
    let mut end = 0usize;
    for section in sections {
        end = end
            .checked_add(section.item_count)
            .ok_or("Native page-break section counts overflow")?;
        boundaries.insert(end);
    }
    if page_breaks.windows(2).any(|pair| pair[0] > pair[1])
        || page_breaks.iter().any(|index| !boundaries.contains(index))
    {
        return Err("Native page breaks must be ordered row-section boundaries".into());
    }
    Ok(())
}

pub(crate) fn advance_page(
    settings: &PageSettings,
    pages: &mut Vec<StampItemPage>,
    page: &mut StampItemPage,
    total: usize,
    date: Option<chrono::NaiveDate>,
) -> Result<()> {
    let next = new_page(settings, pages.len() + 2, total, date)?;
    pages.push(std::mem::replace(page, next));
    Ok(())
}

fn new_page(
    settings: &PageSettings,
    number: usize,
    total: usize,
    date: Option<chrono::NaiveDate>,
) -> Result<StampItemPage> {
    Ok(StampItemPage {
        layout: settings
            .resolve_for_pagination(number, total, date)
            .map_err(|e| format!("native_pdf.pagination page {number}: {e}"))?,
        items: Vec::new(),
    })
}

fn paginate_pass(
    settings: &PageSettings,
    items: &[StampTextItem],
    boundaries: &[(usize, RowAlignment)],
    policy: &StampTextPolicy,
    date: Option<chrono::NaiveDate>,
    total: usize,
    page_breaks: &[usize],
) -> Result<Vec<StampItemPage>> {
    let measured = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            policy
                .measure_item(item)
                .map_err(|e| format!("native_pdf.pagination item {}: {e}", i + 1))
        })
        .collect::<Result<Vec<_>>>()?;
    let mut pages = Vec::new();
    let mut page = new_page(settings, 1, total, date)?;
    let mut top = page.layout.content_bounds.top;
    let mut index = 0;
    let mut section_index = 0;
    let mut breaks = page_breaks.iter().copied().peekable();
    loop {
        while breaks.next_if_eq(&index).is_some() {
            advance_page(settings, &mut pages, &mut page, total, date)?;
            top = page.layout.content_bounds.top;
        }
        if index == items.len() {
            break;
        }
        while index >= boundaries[section_index].0 {
            section_index += 1;
        }
        let (section_end, alignment) = boundaries[section_index];
        let content = page.layout.content_bounds;
        let (_, _, width, item_height) = &measured[index];
        if *width > content.right - content.left {
            return Err(format!("native_pdf.pagination item {} footprint width {width} mm exceeds page {} content width", index + 1, pages.len() + 1));
        }
        let fresh_height = content.top - content.bottom;
        if *item_height > fresh_height {
            if !page.items.is_empty() {
                pages.push(page);
                page = new_page(settings, pages.len() + 1, total, date)?;
                top = page.layout.content_bounds.top;
                continue;
            }
            let (heading, caption, width, _) = &measured[index];
            let mut cursor = SplitCursor {
                heading: 0,
                caption: 0,
                frame_pending: true,
            };
            loop {
                let content = page.layout.content_bounds;
                if *width > content.right - content.left {
                    return Err(format!("native_pdf.pagination item {} continuation width exceeds page {} content width", index + 1, pages.len() + 1));
                }
                let mut fragment = split_fragment(
                    index,
                    &items[index],
                    heading,
                    caption,
                    *width,
                    content,
                    policy,
                    &mut cursor,
                )?;
                align_fragments(
                    std::slice::from_mut(&mut fragment),
                    content,
                    alignment,
                    settings.spacing().horizontal_mm(),
                )?;
                top = settings.spacing().next_top(fragment.footprint.bottom)?;
                page.items.push(fragment);
                if cursor.finished(heading, caption) {
                    break;
                }
                pages.push(page);
                page = new_page(settings, pages.len() + 1, total, date)?;
            }
            index += 1;
            continue;
        }
        let candidates = measured[index..section_end]
            .iter()
            .take_while(|m| m.3 <= fresh_height);
        let count = row_prefix_len(
            content.right - content.left,
            candidates.map(|m| m.2),
            settings.spacing(),
        );
        if count == 0 {
            return Err(format!(
                "native_pdf.pagination item {} cannot start a row",
                index + 1
            ));
        }
        let row_height = measured[index..index + count]
            .iter()
            .map(|m| m.3)
            .fold(0.0, f64::max);
        if row_height > top - content.bottom {
            pages.push(page);
            page = new_page(settings, pages.len() + 1, total, date)?;
            top = page.layout.content_bounds.top;
            continue;
        }
        let row_bounds = Bounds { top, ..content };
        let rows = fit_text_rows(
            row_bounds,
            &items[index..index + count],
            settings.spacing(),
            policy,
        )?;
        let mut fragments = Vec::new();
        for (offset, placed) in rows.into_iter().flatten().enumerate() {
            fragments.push(StampPageFragment {
                source_index: index + offset,
                footprint: placed.footprint,
                geometry: Some(placed.geometry),
                heading: placed.heading,
                caption: placed.caption,
                footer_text: None,
            });
        }
        align_fragments(
            &mut fragments,
            content,
            alignment,
            settings.spacing().horizontal_mm(),
        )?;
        page.items.extend(fragments);
        top = settings.spacing().next_top(top - row_height)?;
        index += count;
    }

    fn align_fragments(
        fragments: &mut [StampPageFragment],
        content: Bounds,
        alignment: RowAlignment,
        minimum_gap: f64,
    ) -> Result<()> {
        let widths: Vec<_> = fragments
            .iter()
            .map(|f| f.footprint.right - f.footprint.left)
            .collect();
        let lefts = alignment.positions(content, &widths, minimum_gap)?;
        for (fragment, left) in fragments.iter_mut().zip(lefts) {
            let delta = left - fragment.footprint.left;
            fragment.footprint.left += delta;
            fragment.footprint.right += delta;
            if let Some(geometry) = fragment.geometry {
                let frame = geometry.frame_bounds();
                fragment.geometry =
                    Some(geometry.at_frame_top_left(frame.left + delta, frame.top)?);
            }
            for text in fragment
                .heading
                .iter_mut()
                .chain(fragment.caption.iter_mut())
            {
                text.bounds.left += delta;
                text.bounds.right += delta;
                for line in &mut text.lines {
                    line.left_mm += delta;
                }
            }
        }
        Ok(())
    }
    pages.push(page);
    Ok(pages)
}

struct SplitCursor {
    heading: usize,
    caption: usize,
    frame_pending: bool,
}

impl SplitCursor {
    fn finished(&self, heading: &Option<MeasuredText>, caption: &Option<MeasuredText>) -> bool {
        !self.frame_pending
            && self.heading == heading.as_ref().map_or(0, |t| t.lines.len())
            && self.caption == caption.as_ref().map_or(0, |t| t.lines.len())
    }
}

fn take_lines(text: &MeasuredText, offset: &mut usize, available: f64) -> Option<MeasuredText> {
    let remaining = text.lines.len() - *offset;
    let mut count = 0;
    while count < remaining {
        let required = text.ascent_mm + text.descent_mm + count as f64 * text.line_advance_mm;
        if required > available {
            break;
        }
        count += 1;
    }
    if count == 0 {
        return None;
    }
    // Slice measured lines directly: a continuation consisting only of explicit
    // blank lines is valid, although a standalone all-blank input is not.
    let mut part = text.clone();
    part.lines = text.lines[*offset..*offset + count].to_vec();
    *offset += count;
    Some(part)
}

#[allow(clippy::too_many_arguments)]
fn split_fragment(
    index: usize,
    item: &StampTextItem,
    heading: &Option<MeasuredText>,
    caption: &Option<MeasuredText>,
    width: f64,
    content: Bounds,
    policy: &StampTextPolicy,
    cursor: &mut SplitCursor,
) -> Result<StampPageFragment> {
    let (_, pad_y) = policy.padding();
    let available = content.top - content.bottom - 2.0 * pad_y;
    let frame = item.geometry.frame_bounds();
    let frame_height = frame.top - frame.bottom;
    if frame_height > available {
        return Err(format!("native_pdf.pagination item {} indivisible frame height {frame_height} mm exceeds fresh page height", index + 1));
    }
    let centre = content.left + width / 2.0;
    let mut used = 0.0;
    let mut heading_part = None;
    let mut caption_part = None;
    let mut geometry = None;
    if let Some(text) = heading {
        if cursor.heading < text.lines.len() {
            heading_part = take_lines(text, &mut cursor.heading, available);
            if let Some(part) = &heading_part {
                used += height(part);
            }
        }
    }
    let heading_done = cursor.heading == heading.as_ref().map_or(0, |t| t.lines.len());
    if heading_done && cursor.frame_pending {
        let gap = if heading_part.is_some() {
            policy.gap()
        } else {
            0.0
        };
        if used + gap + frame_height <= available {
            used += gap;
            geometry = Some(item.geometry.at_frame_top_left(
                centre - (frame.right - frame.left) / 2.0,
                content.top - pad_y - used,
            )?);
            used += frame_height;
            cursor.frame_pending = false;
        }
    }
    let caption_top = content.top
        - pad_y
        - used
        - if geometry.is_some() {
            policy.gap()
        } else {
            0.0
        };
    if !cursor.frame_pending {
        if let Some(text) = caption {
            let gap = if geometry.is_some() {
                policy.gap()
            } else {
                0.0
            };
            caption_part = take_lines(text, &mut cursor.caption, available - used - gap);
            if let Some(part) = &caption_part {
                used += gap + height(part);
            }
        }
    }
    if used == 0.0 {
        return Err(format!(
            "native_pdf.pagination item {} cannot fit a complete text line on a fresh page",
            index + 1
        ));
    }
    let footprint = Bounds {
        left: content.left,
        right: content.left + width,
        top: content.top,
        bottom: content.top - used - 2.0 * pad_y,
    };
    Ok(StampPageFragment {
        source_index: index,
        footprint,
        geometry,
        heading: position(heading_part, &item.heading, centre, content.top - pad_y),
        caption: position(caption_part, &item.caption, centre, caption_top),
        footer_text: None,
    })
}
