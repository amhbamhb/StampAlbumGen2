//! Standalone, bounded AlbumEasy compatibility input for the native renderer.

use super::{
    album::{
        integer, number, parse_command, restore_stamp_settings, save_stamp_settings, strip_comment,
        Argument,
    },
    colour::NamedColour,
    commands::{
        colour::colour_album_border::AlbumBorderColour,
        page::{
            album_pages_border::PageBorder,
            album_pages_footer::AlbumPagesFooter,
            album_pages_footer_number::AlbumPagesFooterNumber,
            album_pages_footer_pad::FooterPadding,
            album_pages_header::{AlbumPagesHeader, HeaderPosition},
            album_pages_header_pad::HeaderPadding,
            album_pages_margin_txt::{
                AlbumPagesMarginText, MarginAlignment, MarginDirection, MarginPosition,
            },
            album_pages_margins::Margins,
            album_pages_size::PageSize,
            album_pages_spacing::Spacing,
            album_pages_title::AlbumPagesTitle,
        },
    },
    document::{Bounds, PageSettings},
    renderer::{write_imported_album_pdf, ImportedPdfPage},
    stamp_heading::{self, HeadingMode, HeadingRowItem, StampHeading, VerticalAlignment},
    stamp_image::{ImageAspectRatio, ImagePaddingUnit, StampImageOptions, StampImageSettings},
    stamp_inner_border::{BorderStyleState, InnerBorderStyle, StampInnerBorder},
    stamp_text::{
        position_footer_caption, FooterTextAlignment, FooterTextSide, PositionedStampText,
    },
    text::{MeasuredText, PositionedTextLine},
    Result,
};
use std::{fs, path::Path};

#[derive(Debug)]
struct Command {
    name: String,
    args: Vec<Argument>,
    line: usize,
}

/// An unsupported command is never omitted during rendering. A caller can
/// explicitly prepare a commented test copy and retain this loss inventory.
#[derive(Clone, Debug)]
pub struct UnsupportedCommand {
    pub line: usize,
    pub name: String,
    pub source: String,
}

pub fn commented_test_source(source: &str) -> Result<(String, Vec<UnsupportedCommand>)> {
    let mut unsupported = Vec::new();
    let mut lines = Vec::new();
    for (index, raw) in source.lines().enumerate() {
        let line = strip_comment(raw).trim().trim_end_matches(';').trim();
        if line.is_empty() {
            lines.push(raw.to_string());
            continue;
        }
        let command = command(line, index + 1)?;
        if supported(&command.name) {
            lines.push(raw.trim_end().to_string());
        } else {
            unsupported.push(UnsupportedCommand {
                line: index + 1,
                name: command.name,
                source: raw.to_string(),
            });
            lines.push(format!("# NATIVE-TODO: {raw}"));
        }
    }
    Ok((format!("{}\n", lines.join("\n")), unsupported))
}

fn supported(name: &str) -> bool {
    matches!(
        name,
        "ALBUM_PAGES_SIZE"
            | "ALBUM_PAGES_MARGINS"
            | "ALBUM_PAGES_SPACING"
            | "ALBUM_PAGES_BORDER"
            | "ALBUM_PAGES_TITLE"
            | "ALBUM_PAGES_HEADER"
            | "ALBUM_PAGES_FOOTER_NUMBER"
            | "COLOR_ALBUM_FOOTER"
            | "COLOUR_ALBUM_FOOTER"
            | "ALBUM_PAGES_MARGIN_TXT"
            | "ALBUM_PAGES_MARGIN_TXT_PAD"
            | "COLOR_ALBUM_MARGIN_TXT"
            | "COLOUR_ALBUM_MARGIN_TXT"
            | "COLOR_ALBUM_BORDER"
            | "COLOUR_ALBUM_BORDER"
            | "COLOR_ALBUM_HEADER"
            | "COLOR_ALBUM_TITLE"
            | "COLOR_STAMP_BORDER"
            | "COLOR_STAMP_TEXT"
            | "COLOR_STAMP_HEADING"
            | "COLOUR_STAMP_HEADING"
            | "COLOR_STAMP_BACKGROUND"
            | "COLOUR_STAMP_BACKGROUND"
            | "STAMP_BOXES_SIZE_ADJUST"
            | "STAMP_BORDER_STYLE"
            | "STAMP_INNER_BORDER"
            | "STAMP_INNER_BORDER_STYLE"
            | "COLOR_STAMP_INNER_BORDER"
            | "COLOUR_STAMP_INNER_BORDER"
            | "ALBUM_STAMP_IMG_NEW_SETTING"
            | "ALBUM_STAMP_IMG_NEW_SETTING_PCNT"
            | "ALBUM_STAMP_IMG_ASPECT_RATIO"
            | "ALBUM_STAMP_IMG_GREYSCALE_ON"
            | "ALBUM_STAMP_IMG_GRAYSCALE_OFF"
            | "ALBUM_STAMP_IMG_GREYSCALE_OFF"
            | "ALBUM_STAMP_IMG_GRAYSCALE_ON"
            | "ALBUM_STAMP_IMG_HIDE"
            | "ALBUM_STAMP_IMG_SHOW"
            | "ALBUM_STAMP_SETTINGS_SAVE"
            | "ALBUM_STAMP_SETTINGS_RESTORE"
            | "PAGE_START"
            | "PAGE_VSPACE"
            | "PAGE_TEXT_CENTRE"
            | "ROW_START_ES"
            | "ROW_START_FS"
            | "ROW_ALIGN_TOP"
            | "ROW_ALIGN_MIDDLE"
            | "ROW_ALIGN_BOTTOM"
            | "STAMP_ADD_IMG"
            | "STAMP_HEADING_VA"
            | "STAMP_HEADING"
            | "STAMP_HEADING_PADDING"
            | "STAMP_FOOTER_TXT_ALIGN"
            | "STAMP_FOOTER_TXT_PAD"
            | "TEXT_LINE_LEADING"
    )
}

fn command(line: &str, source_line: usize) -> Result<Command> {
    let parsed = if line.contains('(') {
        parse_command(line)
    } else {
        parse_command(&format!("{line} ()"))
    };
    let (name, args) = parsed.map_err(|e| format!("Line {source_line}: {e}"))?;
    Ok(Command {
        name,
        args,
        line: source_line,
    })
}

fn count(c: &Command, min: usize, max: usize) -> Result<()> {
    if !(min..=max).contains(&c.args.len()) {
        return Err(format!("{} expects {min}..{max} arguments", c.name));
    }
    Ok(())
}

fn n(c: &Command, index: usize) -> Result<f64> {
    number(
        c.args.get(index).ok_or("Missing numeric argument")?,
        &c.name,
        index + 1,
    )
}

fn text(c: &Command, index: usize) -> Result<&str> {
    let arg = c.args.get(index).ok_or("Missing text argument")?;
    if !arg.quoted {
        return Err(format!("{} argument {} must be quoted", c.name, index + 1));
    }
    Ok(&arg.value)
}

fn flag(c: &Command, index: usize) -> Result<bool> {
    let arg = c.args.get(index).ok_or("Missing boolean argument")?;
    if arg.quoted {
        return Err("Boolean arguments must not be quoted".into());
    }
    match arg.value.to_ascii_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err("Expected True or False".into()),
    }
}

struct Item {
    width: f64,
    height: f64,
    image: std::path::PathBuf,
    options: StampImageOptions,
    captions: Vec<Option<MeasuredText>>,
    footer_alignment: FooterTextAlignment,
    footer_padding_mm: f64,
    heading: Option<StampHeading>,
    frame: bool,
    outer_style: BorderStyleState,
    background: Option<NamedColour>,
    border_colour: NamedColour,
    text_colour: NamedColour,
    inner_border: Option<StampInnerBorder>,
}

fn height(t: &MeasuredText) -> f64 {
    t.ascent_mm + t.descent_mm + (t.lines.len() - 1) as f64 * t.line_advance_mm
}

fn positioned(
    t: MeasuredText,
    left: f64,
    top: f64,
    width: f64,
    align: usize,
    colour: NamedColour,
) -> PositionedStampText {
    let lines = t
        .lines
        .iter()
        .enumerate()
        .map(|(index, line)| PositionedTextLine {
            text: line.text.clone(),
            left_mm: match align {
                0 => left,
                2 => left + width - line.width_mm,
                _ => left + (width - line.width_mm) / 2.0,
            },
            baseline_mm: top - t.ascent_mm - index as f64 * t.line_advance_mm,
            width_mm: line.width_mm,
        })
        .collect();
    let bounds = Bounds {
        left,
        right: left + width,
        top,
        bottom: top - height(&t),
    };
    PositionedStampText {
        measured: t,
        colour,
        lines,
        bounds,
    }
}

struct Row {
    font: String,
    size: f64,
    border: f64,
    gap: Option<f64>,
    items: Vec<Item>,
    vertical: VerticalAlignment,
}

fn flush(
    row: &mut Option<Row>,
    page: &mut ImportedPdfPage,
    top: &mut f64,
    spacing: f64,
) -> Result<()> {
    let Some(row) = row.take() else {
        return Ok(());
    };
    if row.items.is_empty() {
        return Ok(());
    }
    let available = page.layout.content_bounds.right - page.layout.content_bounds.left;
    let sum: f64 = row.items.iter().map(|i| i.width).sum();
    let count = row.items.len();
    let gap = row.gap.unwrap_or((available - sum) / (count + 1) as f64);
    let used = sum + gap * (count - 1) as f64;
    if gap < 0.0 || used > available {
        return Err(
            "Imported AlbumEasy row exceeds the page width; split the row explicitly".into(),
        );
    }
    let row_items = row
        .items
        .iter()
        .map(|item| {
            let caption_below = item
                .captions
                .iter()
                .flatten()
                .map(|caption| (height(caption) + 0.5 + item.footer_padding_mm).max(0.0))
                .fold(0.0, f64::max);
            HeadingRowItem {
                width_mm: item.width,
                height_mm: item.height,
                heading: item.heading.as_ref(),
                below_mm: caption_below,
            }
        })
        .collect::<Vec<_>>();
    let mut left = page.layout.content_bounds.left + (available - used) / 2.0;
    let lefts = row
        .items
        .iter()
        .map(|item| {
            let position = left;
            left += item.width + gap;
            position
        })
        .collect::<Vec<_>>();
    let layout = stamp_heading::layout_row(
        page.layout.content_bounds,
        *top,
        &row_items,
        &lefts,
        row.vertical,
    )
    .map_err(|error| {
        format!("Imported AlbumEasy row: {error}; add PAGE_START or correct heading/row settings")
    })?;
    for (item, placed) in row.items.into_iter().zip(layout.items) {
        let bounds = placed.frame;
        page.images.push(StampImageSettings {
            image_path: item.image,
            box_bounds: bounds,
            image_scale_pt_per_pixel: 1.0,
            options: item.options,
            image_padding_horizontal_mm: 0.0,
            image_padding_vertical_mm: 0.0,
            text_gap_mm: 0.0,
            heading: None,
            caption: None,
            labels: None,
        });
        if item.frame && row.border > 0.0 {
            if item.outer_style.uses_legacy_rectangle() {
                page.frames.push((bounds, row.border, item.border_colour));
            } else {
                page.inner_borders.push(item.outer_style.outer_paths(
                    bounds,
                    row.border,
                    item.border_colour,
                )?);
            }
        }
        if let Some(inner) = item.inner_border {
            if let Some(paths) = inner.layout(bounds, row.border)? {
                page.inner_borders.push(paths);
            }
        }
        if let Some(colour) = item.background {
            page.backgrounds.push((bounds, colour));
        }
        if let Some(heading) = placed.heading {
            page.text.push(heading);
        }
        for (align, caption) in item.captions.into_iter().enumerate() {
            if let Some(caption) = caption {
                let side = match align {
                    0 => FooterTextSide::Left,
                    1 => FooterTextSide::Centre,
                    _ => FooterTextSide::Right,
                };
                page.text.push(position_footer_caption(
                    caption,
                    bounds,
                    super::stamp_text::footer_caption_top(
                        bounds.bottom,
                        0.5,
                        item.footer_padding_mm,
                    )?,
                    side,
                    item.footer_alignment,
                    item.text_colour,
                )?);
            }
        }
    }
    *top = layout.bottom_mm - spacing;
    Ok(())
}

/// Generates a PDF without invoking AlbumEasy. Unsupported commands are errors;
/// comments must be applied explicitly to a separate test copy beforehand.
pub fn render_file(input: &Path, output: &Path) -> Result<usize> {
    if !matches!(
        input.extension().and_then(|e| e.to_str()),
        Some("txt" | "stampalbum")
    ) {
        return Err("AlbumEasy input must use .txt or .stampalbum".into());
    }
    let source =
        fs::read_to_string(input).map_err(|e| format!("Cannot read {}: {e}", input.display()))?;
    render(
        &source,
        input.parent().unwrap_or_else(|| Path::new("")),
        output,
    )
}

pub fn render(source: &str, assets: &Path, output: &Path) -> Result<usize> {
    let mut commands = Vec::new();
    for (index, raw) in source.lines().enumerate() {
        let line = strip_comment(raw).trim().trim_end_matches(';').trim();
        if line.is_empty() {
            continue;
        }
        let c = command(line, index + 1)?;
        if !supported(&c.name) {
            return Err(format!("Line {}: Unsupported AlbumEasy command {}; record it in the plan and comment it out in a test copy", c.line, c.name));
        }
        commands.push(c);
    }
    let mut size = None;
    let mut margins = None;
    let mut spacing = None;
    let mut border = None;
    let mut title = None;
    let mut header = None;
    let mut footer_number = None;
    let mut margin_text = Vec::new();
    let mut margin_padding = 0.0;
    let mut border_colour = NamedColour::parse("black")?;
    let mut border_colour_name = "black".to_string();
    let mut header_colour = border_colour;
    let mut title_colour = border_colour;
    let mut footer_colour = border_colour;
    let mut margin_colour = border_colour;
    for c in commands.iter().take_while(|c| c.name != "PAGE_START") {
        let result = (|| -> Result<()> {
            match c.name.as_str() {
                "ALBUM_PAGES_SIZE" => {
                    count(c, 2, 2)?;
                    size = Some(PageSize::parse(&[n(c, 0)?, n(c, 1)?])?);
                }
                "ALBUM_PAGES_MARGINS" => {
                    count(c, 4, 4)?;
                    margins = Some(Margins::parse(&[n(c, 0)?, n(c, 1)?, n(c, 2)?, n(c, 3)?])?);
                }
                "ALBUM_PAGES_SPACING" => {
                    count(c, 2, 2)?;
                    spacing = Some(Spacing::parse(&[n(c, 0)?, n(c, 1)?])?);
                }
                "ALBUM_PAGES_BORDER" => {
                    count(c, 3, 4)?;
                    border = Some(PageBorder::parse(
                        &c.args
                            .iter()
                            .enumerate()
                            .map(|(i, _)| n(c, i))
                            .collect::<Result<Vec<_>>>()?,
                    )?);
                }
                "COLOR_ALBUM_BORDER" | "COLOUR_ALBUM_BORDER" => {
                    count(c, 1, 1)?;
                    border_colour = NamedColour::parse(&c.args[0].value)?;
                    border_colour_name = c.args[0].value.clone();
                }
                "COLOR_ALBUM_HEADER" => {
                    count(c, 1, 1)?;
                    header_colour = NamedColour::parse(&c.args[0].value)?;
                }
                "COLOR_ALBUM_TITLE" => {
                    count(c, 1, 1)?;
                    title_colour = NamedColour::parse(&c.args[0].value)?;
                }
                "COLOR_ALBUM_FOOTER" | "COLOUR_ALBUM_FOOTER" => {
                    count(c, 1, 1)?;
                    if c.args[0].quoted {
                        return Err("Footer colour requires an unquoted named colour".into());
                    }
                    footer_colour = NamedColour::parse(&c.args[0].value)?;
                }
                "COLOR_ALBUM_MARGIN_TXT" | "COLOUR_ALBUM_MARGIN_TXT" => {
                    count(c, 1, 1)?;
                    if c.args[0].quoted {
                        return Err("Margin colour must be an unquoted named colour".into());
                    }
                    margin_colour = NamedColour::parse(&c.args[0].value)?;
                }
                "ALBUM_PAGES_MARGIN_TXT_PAD" => {
                    count(c, 1, 1)?;
                    margin_padding = n(c, 0)?;
                }
                "ALBUM_PAGES_MARGIN_TXT" => {
                    count(c, 4, 6)?;
                    if [0, 3, 4, 5]
                        .iter()
                        .any(|index| c.args.get(*index).is_some_and(|a| a.quoted))
                    {
                        return Err(
                            "Margin font, position, alignment and direction must be unquoted"
                                .into(),
                        );
                    }
                    if margin_text.len() == 2 {
                        return Err("ALBUM_PAGES_MARGIN_TXT supports at most two definitions; extra definitions are not silently ignored".into());
                    }
                    margin_text.push((
                        c.line,
                        MeasuredText::new(&c.args[0].value, n(c, 1)?, text(c, 2)?)?,
                        MarginPosition::parse(&c.args[3].value)?,
                        MarginAlignment::parse(
                            c.args.get(4).map_or("CENTRE", |a| a.value.as_str()),
                        )?,
                        MarginDirection::parse(c.args.get(5).map_or("UP", |a| a.value.as_str()))?,
                    ));
                }
                "ALBUM_PAGES_FOOTER_NUMBER" => {
                    count(c, 3, 6)?;
                    if c.args[0].quoted || c.args[2].quoted {
                        return Err("Footer font and position must be unquoted".into());
                    }
                    footer_number = Some((
                        c.line,
                        c.args[0].value.clone(),
                        n(c, 1)?,
                        HeaderPosition::parse(&c.args[2].value)?,
                        if c.args.len() > 3 {
                            integer(&c.args[3], &c.name, 4)?
                        } else {
                            1
                        },
                        if c.args.len() > 4 {
                            text(c, 4)?.to_string()
                        } else {
                            String::new()
                        },
                        if c.args.len() > 5 {
                            text(c, 5)?.to_string()
                        } else {
                            String::new()
                        },
                    ));
                }
                "ALBUM_PAGES_TITLE" => {
                    count(c, 3, 4)?;
                    title = Some((
                        c.args[0].value.clone(),
                        n(c, 1)?,
                        text(c, 2)?.to_string(),
                        if c.args.len() == 4 { n(c, 3)? } else { 0.0 },
                    ));
                }
                "ALBUM_PAGES_HEADER" => {
                    count(c, 4, 4)?;
                    header = Some((
                        c.args[0].value.clone(),
                        n(c, 1)?,
                        HeaderPosition::parse(&c.args[2].value)?,
                        text(c, 3)?.to_string(),
                    ));
                }
                _ => {}
            }
            Ok(())
        })();
        result.map_err(|e| format!("Line {}: {e}", c.line))?;
    }
    let mut settings = PageSettings::new(
        size.ok_or("Missing ALBUM_PAGES_SIZE")?,
        margins.ok_or("Missing ALBUM_PAGES_MARGINS")?,
        spacing.ok_or("Missing ALBUM_PAGES_SPACING")?,
        border.ok_or("Missing ALBUM_PAGES_BORDER")?,
        AlbumBorderColour::parse(&[&border_colour_name])?,
    );
    if let Some((font, size, text, gap)) = title {
        settings.set_title(AlbumPagesTitle::new(
            &font,
            size,
            &text,
            gap,
            0.0,
            title_colour,
        )?);
    }
    if let Some((font, size, position, text)) = header {
        settings.set_header(AlbumPagesHeader::new(
            &font,
            size,
            position,
            &text,
            HeaderPadding::parse(&[0.0])?,
            0.0,
            header_colour,
        )?);
    }
    if let Some((line, font, size, position, start, before, after)) = footer_number {
        let footer = (|| -> Result<AlbumPagesFooterNumber> {
            let style = AlbumPagesFooter::new(
                &font,
                size,
                position,
                "1",
                FooterPadding::parse(&[0.0])?,
                0.0,
                footer_colour,
            )?;
            AlbumPagesFooterNumber::new(style, start, &before, &after)
        })()
        .map_err(|error| format!("Line {line}: ALBUM_PAGES_FOOTER_NUMBER: {error}"))?;
        settings.set_footer_number(footer);
    }
    for (line, text, position, alignment, direction) in margin_text {
        settings
            .add_margin_text(
                AlbumPagesMarginText::new(
                    text,
                    position,
                    alignment,
                    direction,
                    2.0 + margin_padding,
                    margin_colour,
                )
                .map_err(|e| format!("Line {line}: ALBUM_PAGES_MARGIN_TXT: {e}"))?,
            )
            .map_err(|e| format!("Line {line}: {e}"))?;
    }
    let total = commands.iter().filter(|c| c.name == "PAGE_START").count();
    if total == 0 {
        return Err("AlbumEasy input requires PAGE_START".into());
    }
    let mut pages: Vec<ImportedPdfPage> = Vec::new();
    let mut row = None;
    let mut top = 0.0;
    let mut adjust = 0.0;
    let mut options = StampImageOptions {
        padding_unit: ImagePaddingUnit::Millimetres,
        padding_horizontal: 0.7,
        padding_vertical: 0.7,
        stretch_small: false,
        aspect_ratio: ImageAspectRatio::Fit,
        grayscale: false,
        visible: true,
    };
    let mut frame = true;
    let mut background = None;
    let mut stamp_colour = NamedColour::parse("black")?;
    let mut text_colour = stamp_colour;
    let mut heading_colour = stamp_colour;
    let mut saved_stamp_settings = None;
    let mut inner_dimensions = (0.0, 0.0);
    let mut inner_style = BorderStyleState::new(InnerBorderStyle::Solid);
    let mut outer_style = BorderStyleState::new(InnerBorderStyle::Solid);
    let mut inner_colour = stamp_colour;
    let mut heading_padding = 0.0;
    let mut footer_alignment = FooterTextAlignment::Align;
    let mut footer_padding_mm = 0.0;
    let mut vertical = VerticalAlignment::Top;
    for (command_index, c) in commands.iter().enumerate() {
        let result = (|| -> Result<()> {
            match c.name.as_str() {
                "PAGE_START" => {
                    count(c, 0, 0)?;
                    if let Some(page) = pages.last_mut() {
                        flush(&mut row, page, &mut top, settings.spacing().vertical_mm())?;
                    }
                    let layout = settings.resolve_with_context(pages.len() + 1, total, None)?;
                    top = layout.content_bounds.top;
                    pages.push(ImportedPdfPage {
                        layout,
                        images: Vec::new(),
                        text: Vec::new(),
                        frames: Vec::new(),
                        backgrounds: Vec::new(),
                        inner_borders: Vec::new(),
                    });
                }
                "ROW_ALIGN_TOP" | "ROW_ALIGN_MIDDLE" | "ROW_ALIGN_BOTTOM" => {
                    count(c, 0, 0)?;
                    if let Some(page) = pages.last_mut() {
                        flush(&mut row, page, &mut top, settings.spacing().vertical_mm())?;
                    }
                    vertical = VerticalAlignment::parse(c.name.trim_start_matches("ROW_ALIGN_"))?;
                }
                "ALBUM_STAMP_SETTINGS_SAVE" => {
                    count(c, 0, 0)?;
                    save_stamp_settings(
                        &mut saved_stamp_settings,
                        (
                            adjust,
                            options,
                            frame,
                            background,
                            stamp_colour,
                            text_colour,
                            heading_colour,
                            inner_dimensions,
                            inner_style,
                            inner_colour,
                            outer_style,
                            heading_padding,
                            footer_alignment,
                            footer_padding_mm,
                        ),
                        &c.name,
                    )?;
                }
                "ALBUM_STAMP_SETTINGS_RESTORE" => {
                    count(c, 0, 0)?;
                    (
                        adjust,
                        options,
                        frame,
                        background,
                        stamp_colour,
                        text_colour,
                        heading_colour,
                        inner_dimensions,
                        inner_style,
                        inner_colour,
                        outer_style,
                        heading_padding,
                        footer_alignment,
                        footer_padding_mm,
                    ) = restore_stamp_settings(&mut saved_stamp_settings, &c.name)?;
                }
                "COLOR_STAMP_BORDER"
                | "COLOR_STAMP_TEXT"
                | "COLOR_STAMP_HEADING"
                | "COLOUR_STAMP_HEADING" => {
                    count(c, 1, 1)?;
                    let colour = NamedColour::parse(&c.args[0].value)?;
                    match c.name.as_str() {
                        "COLOR_STAMP_BORDER" => stamp_colour = colour,
                        "COLOR_STAMP_TEXT" => text_colour = colour,
                        _ => heading_colour = colour,
                    }
                }
                "COLOR_STAMP_BACKGROUND" | "COLOUR_STAMP_BACKGROUND" => {
                    count(c, 1, 1)?;
                    if c.args[0].quoted {
                        return Err("Stamp background requires an unquoted named colour".into());
                    }
                    background = Some(NamedColour::parse(&c.args[0].value)?);
                }
                "STAMP_BOXES_SIZE_ADJUST" => {
                    count(c, 1, 1)?;
                    adjust = n(c, 0)?;
                }
                "STAMP_BORDER_STYLE" | "STAMP_INNER_BORDER_STYLE" => {
                    if c.args.iter().any(|arg| arg.quoted) {
                        return Err("Border styles/selectors must be unquoted".into());
                    }
                    let values = c
                        .args
                        .iter()
                        .map(|arg| arg.value.as_str())
                        .collect::<Vec<_>>();
                    if c.name == "STAMP_BORDER_STYLE" {
                        outer_style.apply(&values)?;
                        frame = !matches!(outer_style.style, InnerBorderStyle::Blank);
                    } else {
                        inner_style.apply(&values)?;
                    }
                }
                "STAMP_INNER_BORDER" => {
                    count(c, 2, 2)?;
                    let dimensions = (n(c, 0)?, n(c, 1)?);
                    StampInnerBorder::new(
                        dimensions.0,
                        dimensions.1,
                        inner_style.style,
                        inner_colour,
                    )?;
                    inner_dimensions = dimensions;
                }
                "COLOR_STAMP_INNER_BORDER" | "COLOUR_STAMP_INNER_BORDER" => {
                    count(c, 1, 1)?;
                    if c.args[0].quoted {
                        return Err("Inner colour must be unquoted".into());
                    }
                    inner_colour = NamedColour::parse(&c.args[0].value)?;
                }
                "ALBUM_STAMP_IMG_NEW_SETTING" | "ALBUM_STAMP_IMG_NEW_SETTING_PCNT" => {
                    count(c, 1, 3)?;
                    options.padding_unit = if c.name.ends_with("PCNT") {
                        ImagePaddingUnit::Percent
                    } else {
                        ImagePaddingUnit::Millimetres
                    };
                    options.padding_horizontal = n(c, 0)?;
                    options.padding_vertical = if c.args.len() > 1 {
                        n(c, 1)?
                    } else {
                        options.padding_horizontal
                    };
                    options.stretch_small = if c.args.len() > 2 { flag(c, 2)? } else { false };
                    if options.padding_horizontal < 0.0 || options.padding_vertical < 0.0 {
                        return Err("Image padding must be nonnegative".into());
                    }
                }
                "ALBUM_STAMP_IMG_ASPECT_RATIO" => {
                    count(c, 1, 1)?;
                    options.aspect_ratio = if flag(c, 0)? {
                        ImageAspectRatio::Preserve
                    } else {
                        ImageAspectRatio::Fit
                    };
                }
                "ALBUM_STAMP_IMG_GREYSCALE_ON" | "ALBUM_STAMP_IMG_GRAYSCALE_ON" => {
                    count(c, 0, 0)?;
                    options.grayscale = true;
                }
                "ALBUM_STAMP_IMG_GREYSCALE_OFF" | "ALBUM_STAMP_IMG_GRAYSCALE_OFF" => {
                    count(c, 0, 0)?;
                    options.grayscale = false;
                }
                "ALBUM_STAMP_IMG_HIDE" => {
                    count(c, 0, 0)?;
                    options.visible = false;
                }
                "ALBUM_STAMP_IMG_SHOW" => {
                    count(c, 0, 0)?;
                    options.visible = true;
                }
                "ROW_START_ES" | "ROW_START_FS" => {
                    count(
                        c,
                        if c.name.ends_with("FS") { 4 } else { 3 },
                        if c.name.ends_with("FS") { 4 } else { 3 },
                    )?;
                    let page = pages.last_mut().ok_or("Row requires PAGE_START")?;
                    flush(&mut row, page, &mut top, settings.spacing().vertical_mm())?;
                    let font = c.args[0].value.clone();
                    let size = n(c, 1)?;
                    MeasuredText::new(&font, size, "Validate font")?;
                    let border = n(c, 2)?;
                    let gap = if c.name.ends_with("FS") {
                        Some(n(c, 3)?)
                    } else {
                        None
                    };
                    if border < 0.0 || gap.is_some_and(|g| g < 0.0) {
                        return Err("Row border and gap must be nonnegative".into());
                    }
                    row = Some(Row {
                        font,
                        size,
                        border,
                        gap,
                        items: Vec::new(),
                        vertical,
                    });
                }
                "STAMP_ADD_IMG" => {
                    count(c, 6, 7)?;
                    let row = row.as_mut().ok_or("STAMP_ADD_IMG requires ROW_START")?;
                    let width = n(c, 0)? + adjust;
                    let height = n(c, 1)? + adjust;
                    if width <= 0.0 || height <= 0.0 {
                        return Err("Adjusted stamp dimensions must be positive".into());
                    }
                    let path = text(c, 2)?;
                    if path.is_empty() {
                        return Err("Stamp image path must not be empty".into());
                    }
                    if c.args.len() == 7 && c.args[6].value != "NO_FRAME" {
                        return Err(
                            "Only the basic STAMP_ADD_IMG and NO_FRAME overloads are implemented"
                                .into(),
                        );
                    }
                    let mut captions = Vec::new();
                    for i in 3..6 {
                        let value = text(c, i)?;
                        let measured = if value.trim().is_empty() {
                            None
                        } else {
                            Some(MeasuredText::new(&row.font, row.size, value)?)
                        };
                        captions.push(measured);
                    }
                    row.items.push(Item {
                        width,
                        height,
                        image: assets.join(path),
                        options,
                        captions,
                        footer_alignment,
                        footer_padding_mm,
                        heading: None,
                        frame: frame && c.args.len() != 7,
                        outer_style,
                        background: if c.args.len() == 7 { None } else { background },
                        border_colour: stamp_colour,
                        text_colour,
                        inner_border: if c.args.len() == 7 {
                            None
                        } else {
                            let mut inner = StampInnerBorder::new(
                                inner_dimensions.0,
                                inner_dimensions.1,
                                inner_style.style,
                                inner_colour,
                            )?;
                            inner.state = inner_style;
                            Some(inner)
                        },
                    });
                }
                "STAMP_HEADING_PADDING" => {
                    count(c, 1, 1)?;
                    heading_padding = stamp_heading::signed_mm(n(c, 0)?, &c.name)?;
                }
                "STAMP_FOOTER_TXT_ALIGN" => {
                    count(c, 1, 1)?;
                    if c.args[0].quoted {
                        return Err("Footer text alignment must be unquoted".into());
                    }
                    footer_alignment = FooterTextAlignment::parse(&c.args[0].value)?;
                }
                "STAMP_FOOTER_TXT_PAD" => {
                    count(c, 1, 1)?;
                    footer_padding_mm = stamp_heading::signed_mm(n(c, 0)?, &c.name)?;
                }
                "TEXT_LINE_LEADING" => {
                    count(c, 1, 1)?;
                    if n(c, 0)? != 0.0 {
                        return Err(
                            "Only TEXT_LINE_LEADING (0) is currently supported; nonzero leading is not ignored"
                                .into(),
                        );
                    }
                }
                "STAMP_HEADING" | "STAMP_HEADING_VA" => {
                    count(c, 3, 4)?;
                    if command_index == 0 || commands[command_index - 1].name != "STAMP_ADD_IMG" {
                        return Err(format!("{} must immediately follow STAMP_ADD_IMG", c.name));
                    }
                    let item = row
                        .as_mut()
                        .and_then(|r| r.items.last_mut())
                        .ok_or("STAMP_HEADING_VA must follow a stamp")?;
                    if c.args[0].quoted {
                        return Err("Heading font must be unquoted".into());
                    }
                    let measured = MeasuredText::new(&c.args[0].value, n(c, 1)?, text(c, 2)?)?;
                    // AlbumEasy's heading gap is one font-size unit to the last baseline.
                    let gap = super::units::points_to_mm(measured.size_pt) + heading_padding;
                    item.heading = Some(StampHeading::new(
                        measured,
                        if c.name == "STAMP_HEADING" {
                            HeadingMode::Local
                        } else {
                            HeadingMode::Aligned
                        },
                        if c.args.len() == 4 { n(c, 3)? } else { 0.0 },
                        gap,
                        heading_colour,
                    )?);
                }
                "PAGE_VSPACE" | "PAGE_TEXT_CENTRE" => {
                    let page = pages.last_mut().ok_or("Page content requires PAGE_START")?;
                    flush(&mut row, page, &mut top, settings.spacing().vertical_mm())?;
                    if c.name == "PAGE_VSPACE" {
                        count(c, 1, 1)?;
                        top -= n(c, 0)?;
                    } else {
                        count(c, 3, 4)?;
                        let measured = MeasuredText::new(&c.args[0].value, n(c, 1)?, text(c, 2)?)?;
                        let width =
                            page.layout.content_bounds.right - page.layout.content_bounds.left;
                        if measured.lines.iter().any(|line| line.width_mm > width) {
                            return Err("PAGE_TEXT_CENTRE exceeds content width".into());
                        }
                        let block = positioned(
                            measured,
                            page.layout.content_bounds.left,
                            top,
                            width,
                            1,
                            text_colour,
                        );
                        top = block.bounds.bottom - if c.args.len() == 4 { n(c, 3)? } else { 0.0 };
                        page.text.push(block);
                    }
                    if !top.is_finite()
                        || top < page.layout.content_bounds.bottom
                        || top > page.layout.content_bounds.top
                    {
                        return Err("Page text/space exceeds content bounds".into());
                    }
                }
                _ if c.name.starts_with("ALBUM_PAGES_")
                    || c.name.starts_with("COLOR_ALBUM_")
                    || matches!(
                        c.name.as_str(),
                        "COLOUR_ALBUM_BORDER" | "COLOUR_ALBUM_FOOTER" | "COLOUR_ALBUM_MARGIN_TXT"
                    ) =>
                {
                    if !pages.is_empty() {
                        return Err(
                            "Changing page settings after PAGE_START is not implemented".into()
                        );
                    }
                }
                _ => return Err(format!("Unsupported AlbumEasy command {}", c.name)),
            }
            Ok(())
        })();
        result.map_err(|e| format!("Line {} ({}): {e}", c.line, c.name))?;
    }
    flush(
        &mut row,
        pages.last_mut().ok_or("No pages")?,
        &mut top,
        settings.spacing().vertical_mm(),
    )?;
    let count = pages.len();
    write_imported_album_pdf(&settings, pages, output)?;
    Ok(count)
}
