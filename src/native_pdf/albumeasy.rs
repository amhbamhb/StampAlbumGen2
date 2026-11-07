//! Standalone, bounded AlbumEasy compatibility input for the native renderer.

use super::{
    album::{number, parse_command, strip_comment, Argument},
    colour::NamedColour,
    commands::{
        colour::colour_album_border::AlbumBorderColour,
        page::{
            album_pages_border::PageBorder,
            album_pages_header::{AlbumPagesHeader, HeaderPosition},
            album_pages_header_pad::HeaderPadding,
            album_pages_margins::Margins,
            album_pages_size::PageSize,
            album_pages_spacing::Spacing,
            album_pages_title::AlbumPagesTitle,
        },
    },
    document::{Bounds, PageSettings},
    renderer::{write_imported_album_pdf, ImportedPdfPage},
    stamp_image::{ImageAspectRatio, ImagePaddingUnit, StampImageOptions, StampImageSettings},
    stamp_text::PositionedStampText,
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
            | "COLOR_ALBUM_BORDER"
            | "COLOUR_ALBUM_BORDER"
            | "COLOR_ALBUM_HEADER"
            | "COLOR_ALBUM_TITLE"
            | "COLOR_STAMP_BORDER"
            | "COLOR_STAMP_TEXT"
            | "COLOR_STAMP_HEADING"
            | "STAMP_BOXES_SIZE_ADJUST"
            | "STAMP_BORDER_STYLE"
            | "ALBUM_STAMP_IMG_NEW_SETTING"
            | "ALBUM_STAMP_IMG_NEW_SETTING_PCNT"
            | "ALBUM_STAMP_IMG_ASPECT_RATIO"
            | "ALBUM_STAMP_IMG_GREYSCALE_ON"
            | "ALBUM_STAMP_IMG_GRAYSCALE_OFF"
            | "ALBUM_STAMP_IMG_GREYSCALE_OFF"
            | "ALBUM_STAMP_IMG_GRAYSCALE_ON"
            | "ALBUM_STAMP_IMG_HIDE"
            | "ALBUM_STAMP_IMG_SHOW"
            | "PAGE_START"
            | "PAGE_VSPACE"
            | "PAGE_TEXT_CENTRE"
            | "ROW_START_ES"
            | "ROW_START_FS"
            | "ROW_ALIGN_TOP"
            | "STAMP_ADD_IMG"
            | "STAMP_HEADING_VA"
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
    heading: Option<MeasuredText>,
    frame: bool,
    border_colour: NamedColour,
    text_colour: NamedColour,
    heading_colour: NamedColour,
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
    let above = row
        .items
        .iter()
        .filter_map(|i| i.heading.as_ref())
        .map(height)
        .fold(0.0, f64::max);
    let below_frame = row
        .items
        .iter()
        .map(|item| {
            let caption_height = item
                .captions
                .iter()
                .flatten()
                .map(height)
                .fold(0.0, f64::max);
            item.height
                + if caption_height > 0.0 {
                    0.5 + caption_height
                } else {
                    0.0
                }
        })
        .fold(0.0, f64::max);
    let heading_gap = if above > 0.0 { 1.0 } else { 0.0 };
    let bottom = *top - above - heading_gap - below_frame;
    if bottom < page.layout.content_bounds.bottom {
        return Err(
            "Imported AlbumEasy row exceeds the page height; add PAGE_START before this row".into(),
        );
    }
    let mut left = page.layout.content_bounds.left + (available - used) / 2.0;
    let frame_top = *top - above - heading_gap;
    for item in row.items {
        let bounds = Bounds {
            left,
            right: left + item.width,
            top: frame_top,
            bottom: frame_top - item.height,
        };
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
        });
        if item.frame && row.border > 0.0 {
            page.frames.push((bounds, row.border, item.border_colour));
        }
        if let Some(heading) = item.heading {
            page.text.push(positioned(
                heading,
                left,
                *top,
                item.width,
                1,
                item.heading_colour,
            ));
        }
        for (align, caption) in item.captions.into_iter().enumerate() {
            if let Some(caption) = caption {
                page.text.push(positioned(
                    caption,
                    left,
                    bounds.bottom - 0.5,
                    item.width,
                    align,
                    item.text_colour,
                ));
            }
        }
        left += item.width + gap;
    }
    *top = bottom - spacing;
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
    let mut border_colour = NamedColour::parse("black")?;
    let mut border_colour_name = "black".to_string();
    let mut header_colour = border_colour;
    let mut title_colour = border_colour;
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
    let mut stamp_colour = NamedColour::parse("black")?;
    let mut text_colour = stamp_colour;
    let mut heading_colour = stamp_colour;
    for c in &commands {
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
                    });
                }
                "ROW_ALIGN_TOP" => {
                    count(c, 0, 0)?;
                }
                "COLOR_STAMP_BORDER" | "COLOR_STAMP_TEXT" | "COLOR_STAMP_HEADING" => {
                    count(c, 1, 1)?;
                    let colour = NamedColour::parse(&c.args[0].value)?;
                    match c.name.as_str() {
                        "COLOR_STAMP_BORDER" => stamp_colour = colour,
                        "COLOR_STAMP_TEXT" => text_colour = colour,
                        _ => heading_colour = colour,
                    }
                }
                "STAMP_BOXES_SIZE_ADJUST" => {
                    count(c, 1, 1)?;
                    adjust = n(c, 0)?;
                }
                "STAMP_BORDER_STYLE" => {
                    count(c, 1, 1)?;
                    frame = match c.args[0].value.as_str() {
                        "SOLID" => true,
                        "BLANK" => false,
                        _ => {
                            return Err(
                                "Only SOLID and BLANK stamp borders are currently implemented"
                                    .into(),
                            )
                        }
                    };
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
                        heading: None,
                        frame: frame && c.args.len() != 7,
                        border_colour: stamp_colour,
                        text_colour,
                        heading_colour,
                    });
                }
                "STAMP_HEADING_VA" => {
                    count(c, 3, 3)?;
                    let item = row
                        .as_mut()
                        .and_then(|r| r.items.last_mut())
                        .ok_or("STAMP_HEADING_VA must follow a stamp")?;
                    item.heading =
                        Some(MeasuredText::new(&c.args[0].value, n(c, 1)?, text(c, 2)?)?);
                    item.heading_colour = heading_colour;
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
                    || c.name == "COLOUR_ALBUM_BORDER" =>
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
