use super::{
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
    config::{NativeFooterNumberingConfig, NativePageNumberingScope, NativePdfConfig},
    document::{Bounds, PageSettings},
    numbering::NumberingContext,
    pagination::{StampItemPage, StampRowSection},
    renderer,
    stamp_box::StampBoxGeometry,
    stamp_heading::{self, HeadingMode, NativeHeadingItem, StampHeading, VerticalAlignment},
    stamp_image::{
        ImageAspectRatio, ImagePaddingUnit, NativePageBackgroundImage, StampFrameStyle,
        StampImageLabels, StampImageOptions, StampImageRowItem, StampSideLabelSettings,
        StampTextSettings,
    },
    stamp_inner_border::{InnerBorderStyle, StampInnerBorder},
    stamp_rows::RowAlignment,
    stamp_text::{
        ExpandedWidthLimit, FooterTextAlignment, StampFooterText, StampTextItem, StampTextPolicy,
        TextWidthPolicy,
    },
    text::MeasuredText,
    units::mm_to_points,
    Result,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

type NativeImageSpec = (
    PathBuf,
    StampBoxGeometry,
    f64,
    StampImageOptions,
    Option<StampTextSettings>,
    Option<StampFrameStyle>,
    Option<StampImageLabels>,
);

/// A parsed native album. Settings and asset paths come only from the file.
#[derive(Debug)]
pub struct NativeAlbum {
    page_settings: PageSettings,
    stamp_text_policy: StampTextPolicy,
    frame_width_mm: f64,
    frame_colour: NamedColour,
    page_background: Option<NativePageBackgroundImage>,
    items: Vec<StampImageRowItem>,
    sections: Vec<StampRowSection>,
    numbering_section_starts: Vec<usize>,
    headings: Vec<NativeHeadingItem>,
}

impl NativeAlbum {
    /// Loads a `.stampalbum` document and resolves its assets relative to the file.
    pub fn load(path: &Path) -> Result<Self> {
        if path.extension().and_then(|value| value.to_str()) != Some("stampalbum") {
            return Err(format!(
                "Native album file must use the .stampalbum extension: {}",
                path.display()
            ));
        }
        let source = fs::read_to_string(path)
            .map_err(|error| format!("Cannot read native album {}: {error}", path.display()))?;
        let base = path.parent().unwrap_or_else(|| Path::new(""));
        Self::parse(&source, base)
            .map_err(|error| format!("Native album {}: {error}", path.display()))
    }

    /// Parses the version-1 command text; relative image paths use `asset_root`.
    pub fn parse(source: &str, asset_root: &Path) -> Result<Self> {
        let mut header_seen = false;
        let mut page_size = None;
        let mut margins = None;
        let mut spacing = None;
        let mut border = None;
        let mut border_colour = None;
        let mut page_title = None;
        let mut page_header = None;
        let mut footer_number = None;
        let mut margin_text = Vec::new();
        let mut alignment = None;
        let mut stamp_frame = None;
        let mut frame_addition_mm = 0.0;
        let mut text_policy = None;
        let mut image_options = StampImageOptions::default();
        let mut frame_style = None;
        let mut saved_stamp_settings = None;
        let mut page_background = None;
        let mut image_specs: Vec<NativeImageSpec> = Vec::new();
        let mut section_start = 0;
        let mut section_counts = Vec::new();
        let mut numbering_section_starts = Vec::new();
        let mut headings: Vec<NativeHeadingItem> = Vec::new();
        let mut vertical = VerticalAlignment::Top;
        let mut heading_padding = None;
        let mut footer_text_alignment = None;
        let mut footer_text_padding = None;
        let mut previous_command = String::new();

        for (index, line) in source.lines().enumerate() {
            let line_number = index + 1;
            let line = strip_comment(line).trim();
            if line.is_empty() {
                continue;
            }
            if !header_seen {
                if line != "STAMPALBUM 1" {
                    return Err(format!(
                        "Line {line_number}: expected the version header `STAMPALBUM 1`"
                    ));
                }
                header_seen = true;
                continue;
            }

            let (name, args) =
                parse_command(line).map_err(|error| format!("Line {line_number}: {error}"))?;
            let fail = |message: String| format!("Line {line_number}: {message}");
            match name.as_str() {
                "ALBUM_PAGES_SIZE" => {
                    set_once(
                        &mut page_size,
                        PageSize::parse(&numbers(&name, &args, 2).map_err(&fail)?)
                            .map_err(&fail)?,
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "ALBUM_PAGES_MARGINS" => {
                    set_once(
                        &mut margins,
                        Margins::parse(&numbers(&name, &args, 4).map_err(&fail)?).map_err(&fail)?,
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "ALBUM_PAGES_SPACING" => {
                    set_once(
                        &mut spacing,
                        Spacing::parse(&numbers(&name, &args, 2).map_err(&fail)?).map_err(&fail)?,
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "ALBUM_PAGES_BORDER" => {
                    if border.is_some() {
                        return Err(fail(format!("Duplicate {name} command")));
                    }
                    if !matches!(args.len(), 3 | 4) {
                        return Err(fail(
                            "ALBUM_PAGES_BORDER requires three or four numeric arguments".into(),
                        ));
                    }
                    border = Some(
                        PageBorder::parse(&numbers(&name, &args, args.len()).map_err(&fail)?)
                            .map_err(&fail)?,
                    );
                }
                "COLOUR_ALBUM_BORDER" => {
                    let colour = words(&name, &args, 1)
                        .map_err(&fail)?
                        .into_iter()
                        .next()
                        .ok_or_else(|| fail(format!("{name} requires one named colour")))?;
                    set_once(
                        &mut border_colour,
                        AlbumBorderColour::parse(&[&colour]).map_err(&fail)?,
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_PAGE_TITLE" => {
                    if args.len() != 4 {
                        return Err(fail(format!(
                            "{name} requires font, size in points, quoted text, and following space in millimetres"
                        )));
                    }
                    let font = unquoted(&args[0], &name).map_err(&fail)?;
                    let size = number(&args[1], &name, 2).map_err(&fail)?;
                    let text = quoted(&args[2], &name).map_err(&fail)?;
                    let following_space = number(&args[3], &name, 4).map_err(&fail)?;
                    set_once(
                        &mut page_title,
                        AlbumPagesTitle::new(
                            &font,
                            size,
                            &text,
                            following_space,
                            0.0,
                            NamedColour::parse("black").map_err(&fail)?,
                        )
                        .map_err(&fail)?,
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_PAGE_HEADER" => {
                    if args.len() != 6 {
                        return Err(fail(format!(
                            "{name} requires font, size in points, position, quoted text, margin padding, and minimum height"
                        )));
                    }
                    let font = unquoted(&args[0], &name).map_err(&fail)?;
                    let size = number(&args[1], &name, 2).map_err(&fail)?;
                    let position = HeaderPosition::parse_for(
                        &unquoted(&args[2], &name).map_err(&fail)?,
                        "native page header position",
                    )
                    .map_err(&fail)?;
                    let text = quoted(&args[3], &name).map_err(&fail)?;
                    let padding_mm = number(&args[4], &name, 5).map_err(&fail)?;
                    let minimum_height_mm = number(&args[5], &name, 6).map_err(&fail)?;
                    set_once(
                        &mut page_header,
                        AlbumPagesHeader::new(
                            &font,
                            size,
                            position,
                            &text,
                            HeaderPadding::parse(&[padding_mm]).map_err(&fail)?,
                            minimum_height_mm,
                            NamedColour::parse("black").map_err(&fail)?,
                        )
                        .map_err(&fail)?,
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_PAGE_FOOTER_NUMBER" => {
                    if args.len() != 9 {
                        return Err(fail(format!(
                            "{name} requires font, point size, position, integer start, quoted prefix, quoted suffix, padding mm, minimum height mm, and colour"
                        )));
                    }
                    let style = AlbumPagesFooter::new(
                        &unquoted(&args[0], &name).map_err(&fail)?,
                        number(&args[1], &name, 2).map_err(&fail)?,
                        HeaderPosition::parse(&unquoted(&args[2], &name).map_err(&fail)?)
                            .map_err(&fail)?,
                        "1",
                        FooterPadding::parse(&[number(&args[6], &name, 7).map_err(&fail)?])
                            .map_err(&fail)?,
                        number(&args[7], &name, 8).map_err(&fail)?,
                        NamedColour::parse(&unquoted(&args[8], &name).map_err(&fail)?)
                            .map_err(&fail)?,
                    )
                    .map_err(&fail)?;
                    set_once(
                        &mut footer_number,
                        AlbumPagesFooterNumber::new(
                            style,
                            integer(&args[3], &name, 4).map_err(&fail)?,
                            &quoted(&args[4], &name).map_err(&fail)?,
                            &quoted(&args[5], &name).map_err(&fail)?,
                        )
                        .map_err(&fail)?,
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_PAGE_MARGIN_TEXT" => {
                    if args.len() != 8 {
                        return Err(fail(format!("{name} requires font, size, quoted text, position, alignment, direction, explicit border gap mm and colour")));
                    }
                    for index in [0, 3, 4, 5, 7] {
                        if args[index].quoted {
                            return Err(fail(format!(
                                "{name} argument {} must be unquoted",
                                index + 1
                            )));
                        }
                    }
                    if margin_text.len() == 2 {
                        return Err(fail(format!("{name} supports at most two definitions")));
                    }
                    margin_text.push(
                        AlbumPagesMarginText::new(
                            MeasuredText::new(
                                &args[0].value,
                                number(&args[1], &name, 2).map_err(&fail)?,
                                &quoted(&args[2], &name).map_err(&fail)?,
                            )
                            .map_err(&fail)?,
                            MarginPosition::parse(&args[3].value).map_err(&fail)?,
                            MarginAlignment::parse(&args[4].value).map_err(&fail)?,
                            MarginDirection::parse(&args[5].value).map_err(&fail)?,
                            number(&args[6], &name, 7).map_err(&fail)?,
                            NamedColour::parse(&args[7].value).map_err(&fail)?,
                        )
                        .map_err(&fail)?,
                    );
                }
                "NATIVE_ROW_ALIGNMENT" => {
                    let value = words(&name, &args, 1)
                        .map_err(&fail)?
                        .into_iter()
                        .next()
                        .ok_or_else(|| fail(format!("{name} requires one alignment")))?;
                    set_once(
                        &mut alignment,
                        RowAlignment::parse(&value).map_err(&fail)?,
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_STAMP_FRAME" => {
                    if !matches!(args.len(), 2 | 3) {
                        return Err(fail(format!(
                            "{name} requires line width, named colour, and optional total frame additions in millimetres"
                        )));
                    }
                    let width = number(&args[0], &name, 1).map_err(&fail)?;
                    if width <= 0.0 || !mm_to_points(width).is_finite() {
                        return Err(fail(format!(
                            "{name} line width must be finite and positive"
                        )));
                    }
                    let colour = words(&name, std::slice::from_ref(&args[1]), 1)
                        .map_err(&fail)?
                        .into_iter()
                        .next()
                        .ok_or_else(|| fail(format!("{name} requires a named colour")))?;
                    if let Some(addition) = args.get(2) {
                        frame_addition_mm = number(addition, &name, 3).map_err(&fail)?;
                        if frame_addition_mm < 0.0 || !mm_to_points(frame_addition_mm).is_finite() {
                            return Err(fail(format!(
                                "{name} total frame additions must be finite and nonnegative"
                            )));
                        }
                    }
                    set_once(
                        &mut stamp_frame,
                        (width, NamedColour::parse(&colour).map_err(&fail)?),
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_STAMP_TEXT_POLICY" => {
                    let [mode, limit_unit, limit_value, horizontal, vertical, gap] = &args[..]
                    else {
                        return Err(fail(format!(
                                "{name} requires six arguments: mode, limit unit, limit value, horizontal padding, vertical padding, text gap"
                            )));
                    };
                    let mode = unquoted(mode, &name).map_err(&fail)?;
                    let limit_unit = unquoted(limit_unit, &name).map_err(&fail)?;
                    let limit_value = number(limit_value, &name, 3).map_err(&fail)?;
                    let horizontal = number(horizontal, &name, 4).map_err(&fail)?;
                    let vertical = number(vertical, &name, 5).map_err(&fail)?;
                    let gap = number(gap, &name, 6).map_err(&fail)?;
                    let policy = StampTextPolicy::new(
                        TextWidthPolicy::parse(&mode).map_err(&fail)?,
                        ExpandedWidthLimit::new(&limit_unit, limit_value).map_err(&fail)?,
                        horizontal,
                        vertical,
                        gap,
                    )
                    .map_err(&fail)?;
                    set_once(&mut text_policy, policy, &name).map_err(&fail)?;
                }
                "NATIVE_STAMP_SETTINGS_SAVE" => {
                    words(&name, &args, 0).map_err(&fail)?;
                    save_stamp_settings(
                        &mut saved_stamp_settings,
                        (
                            image_options,
                            frame_style,
                            heading_padding,
                            footer_text_alignment,
                            footer_text_padding,
                        ),
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_STAMP_SETTINGS_RESTORE" => {
                    words(&name, &args, 0).map_err(&fail)?;
                    (
                        image_options,
                        frame_style,
                        heading_padding,
                        footer_text_alignment,
                        footer_text_padding,
                    ) = restore_stamp_settings(&mut saved_stamp_settings, &name).map_err(&fail)?;
                }
                "NATIVE_STAMP_BACKGROUND" => {
                    let colour = words(&name, &args, 1).map_err(&fail)?;
                    let background = if colour[0] == "none" {
                        None
                    } else {
                        Some(NamedColour::parse(&colour[0]).map_err(&fail)?)
                    };
                    let style = frame_style.get_or_insert(StampFrameStyle {
                        outline: true,
                        background: None,
                        inner_border: None,
                        outer_style: None,
                        inner_style: None,
                    });
                    style.background = background;
                }
                "NATIVE_STAMP_OUTLINE" => {
                    let values = words(&name, &args, 1).map_err(&fail)?;
                    let outline = match values[0].as_str() {
                        "show" => true,
                        "hide" => false,
                        _ => return Err(fail(format!("{name} requires show or hide"))),
                    };
                    let style = frame_style.get_or_insert(StampFrameStyle {
                        outline: true,
                        background: None,
                        inner_border: None,
                        outer_style: None,
                        inner_style: None,
                    });
                    style.outline = outline;
                }
                "NATIVE_STAMP_INNER_BORDER" => {
                    if args.len() != 4 {
                        return Err(fail(format!(
                            "{name} requires padding mm, thickness mm, style and colour"
                        )));
                    }
                    let inner = StampInnerBorder::new(
                        number(&args[0], &name, 1).map_err(&fail)?,
                        number(&args[1], &name, 2).map_err(&fail)?,
                        InnerBorderStyle::parse(&unquoted(&args[2], &name).map_err(&fail)?)
                            .map_err(|error| fail(format!("{name}: {error}")))?,
                        NamedColour::parse(&unquoted(&args[3], &name).map_err(&fail)?)
                            .map_err(&fail)?,
                    )
                    .map_err(&fail)?;
                    let style = frame_style.get_or_insert(StampFrameStyle {
                        outline: true,
                        background: None,
                        inner_border: None,
                        outer_style: None,
                        inner_style: None,
                    });
                    if let Some(state) = &mut style.inner_style {
                        state.style = inner.state.style;
                    }
                    style.inner_border = Some(inner);
                }
                "NATIVE_STAMP_BORDER_STYLE" | "NATIVE_STAMP_INNER_BORDER_STYLE" => {
                    if args.iter().any(|arg| arg.quoted) {
                        return Err(fail(format!("{name} requires unquoted styles/selectors")));
                    }
                    let values = args
                        .iter()
                        .map(|arg| unquoted(arg, &name))
                        .collect::<Result<Vec<_>>>()
                        .map_err(&fail)?;
                    let style = frame_style.get_or_insert(StampFrameStyle {
                        outline: true,
                        background: None,
                        inner_border: None,
                        outer_style: None,
                        inner_style: None,
                    });
                    let initial =
                        super::stamp_inner_border::BorderStyleState::new(InnerBorderStyle::Solid);
                    let (target, inherited) = if name == "NATIVE_STAMP_BORDER_STYLE" {
                        (&mut style.outer_style, initial)
                    } else {
                        (
                            &mut style.inner_style,
                            style
                                .inner_border
                                .map(|inner| inner.state)
                                .unwrap_or(initial),
                        )
                    };
                    target
                        .get_or_insert(inherited)
                        .apply(&values.iter().map(String::as_str).collect::<Vec<_>>())
                        .map_err(&fail)?;
                }
                "NATIVE_STAMP_IMAGE_POLICY" => {
                    if args.len() != 7 {
                        return Err(fail(format!(
                            "{name} requires seven arguments: padding unit, horizontal padding, vertical padding, stretch-small, aspect ratio, colour mode, and visibility"
                        )));
                    }
                    let padding_unit = unquoted(&args[0], &name).map_err(&fail)?;
                    let padding_unit = match padding_unit.as_str() {
                        "millimetres" => ImagePaddingUnit::Millimetres,
                        "percent" => ImagePaddingUnit::Percent,
                        _ => {
                            return Err(fail(format!(
                                "{name} padding unit must be millimetres or percent"
                            )))
                        }
                    };
                    let padding_horizontal = number(&args[1], &name, 2).map_err(&fail)?;
                    let padding_vertical = number(&args[2], &name, 3).map_err(&fail)?;
                    if padding_horizontal < 0.0 || padding_vertical < 0.0 {
                        return Err(fail(format!("{name} padding values must be nonnegative")));
                    }
                    let stretch_small = boolean(&args[3], &name, 4).map_err(&fail)?;
                    let aspect_ratio = match unquoted(&args[4], &name).map_err(&fail)?.as_str() {
                        "preserve" => ImageAspectRatio::Preserve,
                        "fit" => ImageAspectRatio::Fit,
                        _ => {
                            return Err(fail(format!(
                                "{name} aspect ratio must be preserve or fit"
                            )))
                        }
                    };
                    let grayscale = match unquoted(&args[5], &name).map_err(&fail)?.as_str() {
                        "colour" => false,
                        "grayscale" => true,
                        _ => {
                            return Err(fail(format!(
                                "{name} colour mode must be colour or grayscale"
                            )))
                        }
                    };
                    let visible = match unquoted(&args[6], &name).map_err(&fail)?.as_str() {
                        "show" => true,
                        "hide" => false,
                        _ => return Err(fail(format!("{name} visibility must be show or hide"))),
                    };
                    image_options = StampImageOptions {
                        padding_unit,
                        padding_horizontal,
                        padding_vertical,
                        stretch_small,
                        aspect_ratio,
                        grayscale,
                        visible,
                    };
                }
                "NATIVE_PAGE_BACKGROUND_IMAGE" => {
                    if args.len() != 5 {
                        return Err(fail(format!(
                            "{name} requires a quoted image path and x, y, width, and height in millimetres"
                        )));
                    }
                    let image_path = PathBuf::from(quoted(&args[0], &name).map_err(&fail)?);
                    let [x, y, width, height] = numbers(&name, &args[1..], 4)
                        .map_err(&fail)?
                        .try_into()
                        .map_err(|_| fail(format!("{name} requires four numeric bounds")))?;
                    if width <= 0.0 || height <= 0.0 {
                        return Err(fail(format!("{name} width and height must be positive")));
                    }
                    let bounds = Bounds {
                        left: x,
                        bottom: y,
                        right: x + width,
                        top: y + height,
                    };
                    bounds
                        .inset(0.0, 0.0)
                        .map_err(|error| fail(format!("{name} bounds: {error}")))?;
                    set_once(
                        &mut page_background,
                        NativePageBackgroundImage {
                            image_path: asset_root.join(image_path),
                            bounds,
                        },
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_ROW_BREAK" => {
                    if !args.is_empty() {
                        return Err(fail(format!("{name} takes no arguments")));
                    }
                    let count = image_specs.len().saturating_sub(section_start);
                    if count == 0 {
                        return Err(fail(format!(
                            "{name} must follow a row containing at least one STAMP_IMAGE"
                        )));
                    }
                    section_counts.push(count);
                    section_start = image_specs.len();
                }
                "NATIVE_PAGE_NUMBER_SECTION" => {
                    if !args.is_empty() {
                        return Err(fail(format!("{name} takes no arguments")));
                    }
                    let item_count = image_specs.len();
                    if item_count == 0
                        || numbering_section_starts.last().copied() == Some(item_count)
                    {
                        return Err(fail(format!(
                            "{name} must follow at least one stamp and not duplicate a section boundary"
                        )));
                    }
                    numbering_section_starts.push(item_count);
                }
                "NATIVE_ROW_VERTICAL_ALIGNMENT" => {
                    let values = words(&name, &args, 1).map_err(&fail)?;
                    if args[0].quoted {
                        return Err(fail(format!("{name} requires an unquoted alignment")));
                    }
                    vertical = VerticalAlignment::parse(&values[0]).map_err(&fail)?;
                }
                "NATIVE_STAMP_HEADING_PADDING" => {
                    let values = numbers(&name, &args, 1).map_err(&fail)?;
                    heading_padding =
                        Some(stamp_heading::signed_mm(values[0], &name).map_err(&fail)?);
                }
                "NATIVE_STAMP_FOOTER_TEXT_ALIGN" => {
                    if args.len() != 1 || args[0].quoted {
                        return Err(fail(format!("{name} requires one unquoted alignment")));
                    }
                    footer_text_alignment =
                        Some(FooterTextAlignment::parse(&args[0].value).map_err(&fail)?);
                }
                "NATIVE_STAMP_FOOTER_TEXT_PAD" => {
                    let values = numbers(&name, &args, 1).map_err(&fail)?;
                    footer_text_padding =
                        Some(stamp_heading::signed_mm(values[0], &name).map_err(&fail)?);
                }
                "NATIVE_STAMP_HEADING" => {
                    if args.len() != 7 {
                        return Err(fail(format!("{name} requires mode, font, size, quoted text, offset mm, baseline gap mm and colour")));
                    }
                    if !matches!(
                        previous_command.as_str(),
                        "STAMP_IMAGE" | "NATIVE_STAMP_FOOTER_TEXT"
                    ) {
                        return Err(fail(format!("{name} must immediately follow STAMP_IMAGE")));
                    }
                    if [0, 1, 6].iter().any(|&index| args[index].quoted) {
                        return Err(fail(format!("{name} mode/font/colour must be unquoted")));
                    }
                    let padding = heading_padding.ok_or_else(|| {
                        fail("Missing explicit NATIVE_STAMP_HEADING_PADDING".into())
                    })?;
                    let measured = MeasuredText::new(
                        &args[1].value,
                        number(&args[2], &name, 3).map_err(&fail)?,
                        &quoted(&args[3], &name).map_err(&fail)?,
                    )
                    .map_err(&fail)?;
                    let heading = StampHeading::new(
                        measured,
                        HeadingMode::parse(&args[0].value).map_err(&fail)?,
                        number(&args[4], &name, 5).map_err(&fail)?,
                        number(&args[5], &name, 6).map_err(&fail)? + padding,
                        NamedColour::parse(&args[6].value).map_err(&fail)?,
                    )
                    .map_err(&fail)?;
                    headings
                        .last_mut()
                        .ok_or_else(|| fail("Heading has no preceding stamp".into()))?
                        .heading = Some(heading);
                }
                "NATIVE_STAMP_FOOTER_TEXT" => {
                    if args.len() != 6 {
                        return Err(fail(format!(
                            "{name} requires font, size, colour, and quoted left, centre, and right text"
                        )));
                    }
                    if !matches!(
                        previous_command.as_str(),
                        "STAMP_IMAGE" | "NATIVE_STAMP_HEADING"
                    ) {
                        return Err(fail(format!(
                            "{name} must immediately follow STAMP_IMAGE or NATIVE_STAMP_HEADING"
                        )));
                    }
                    if args[0].quoted || args[2].quoted {
                        return Err(fail(format!("{name} font and colour must be unquoted")));
                    }
                    let alignment = footer_text_alignment.ok_or_else(|| {
                        fail("Missing explicit NATIVE_STAMP_FOOTER_TEXT_ALIGN".into())
                    })?;
                    let padding_mm = footer_text_padding.ok_or_else(|| {
                        fail("Missing explicit NATIVE_STAMP_FOOTER_TEXT_PAD".into())
                    })?;
                    if image_specs.last().is_some_and(|image| image.4.is_some()) {
                        return Err(fail(format!(
                            "{name} cannot be combined with STAMP_IMAGE's single caption"
                        )));
                    }
                    let font = args[0].value.clone();
                    let size = number(&args[1], &name, 2).map_err(&fail)?;
                    let colour = NamedColour::parse(&args[2].value).map_err(&fail)?;
                    let make_text = |argument: &Argument| -> Result<_> {
                        let text = quoted_text(argument, &name)?;
                        if text.trim().is_empty() {
                            Ok(None)
                        } else {
                            Ok(Some(StampTextSettings::new(text, &font, size, colour)))
                        }
                    };
                    let left = make_text(&args[3]).map_err(&fail)?;
                    let centre = make_text(&args[4]).map_err(&fail)?;
                    let right = make_text(&args[5]).map_err(&fail)?;
                    if left.is_none() && centre.is_none() && right.is_none() {
                        return Err(fail(format!(
                            "{name} requires at least one visible caption"
                        )));
                    }
                    let item = headings
                        .last_mut()
                        .ok_or_else(|| fail("Footer text has no preceding stamp".into()))?;
                    if item.footer_text.is_some() {
                        return Err(fail(format!("{name} may be attached only once per stamp")));
                    }
                    item.footer_text = Some(StampFooterText {
                        left,
                        centre,
                        right,
                        alignment,
                        padding_mm,
                    });
                }
                "NATIVE_STAMP_IMAGE_LABEL" => {
                    if args.len() != 5 {
                        return Err(fail(format!(
                            "{name} requires font, size, colour, and quoted above/below text"
                        )));
                    }
                    if !matches!(
                        previous_command.as_str(),
                        "STAMP_IMAGE" | "NATIVE_STAMP_IMAGE_SIDE_LABEL"
                    ) {
                        return Err(fail(format!("{name} must immediately follow STAMP_IMAGE")));
                    }
                    if args[0].quoted || args[2].quoted {
                        return Err(fail(format!("{name} font and colour must be unquoted")));
                    }
                    if !args[3].quoted || !args[4].quoted {
                        return Err(fail(format!("{name} above and below text must be quoted")));
                    }
                    let font = args[0].value.clone();
                    let size = number(&args[1], &name, 2).map_err(&fail)?;
                    let colour = NamedColour::parse(&args[2].value).map_err(&fail)?;
                    let make_text = |argument: &Argument| -> Result<_> {
                        let text = quoted_text(argument, &name)?;
                        if text.trim().is_empty() {
                            Ok(None)
                        } else {
                            MeasuredText::new(&font, size, &text)?;
                            Ok(Some(StampTextSettings::new(text, &font, size, colour)))
                        }
                    };
                    let above = make_text(&args[3]).map_err(&fail)?;
                    let below = make_text(&args[4]).map_err(&fail)?;
                    if above.is_none() && below.is_none() {
                        return Err(fail(format!("{name} requires at least one visible label")));
                    }
                    let labels = image_specs
                        .last_mut()
                        .ok_or_else(|| fail("Image label has no preceding stamp".into()))?;
                    if labels
                        .6
                        .as_ref()
                        .is_some_and(|labels| labels.above.is_some() || labels.below.is_some())
                    {
                        return Err(fail(format!("{name} may be attached only once per stamp")));
                    }
                    let labels = labels.6.get_or_insert_with(StampImageLabels::default);
                    labels.above = above;
                    labels.below = below;
                }
                "NATIVE_STAMP_IMAGE_SIDE_LABEL" => {
                    if !matches!(args.len(), 4..=7) {
                        return Err(fail(format!(
                            "{name} requires font, optional size, colour, optional wrap width and maximum lines, and quoted left/right text"
                        )));
                    }
                    if !matches!(
                        previous_command.as_str(),
                        "STAMP_IMAGE" | "NATIVE_STAMP_IMAGE_LABEL"
                    ) {
                        return Err(fail(format!(
                            "{name} must immediately follow STAMP_IMAGE or NATIVE_STAMP_IMAGE_LABEL"
                        )));
                    }
                    let has_size = matches!(args.len(), 5 | 7);
                    let has_wrap_settings = matches!(args.len(), 6 | 7);
                    let colour_index = if has_size { 2 } else { 1 };
                    if args[0].quoted || args[colour_index].quoted {
                        return Err(fail(format!("{name} font and colour must be unquoted")));
                    }
                    let (wrap_width_mm, max_lines, left_index, right_index) = if has_wrap_settings {
                        let width_index = colour_index + 1;
                        let lines_index = colour_index + 2;
                        let width =
                            number(&args[width_index], &name, width_index + 1).map_err(&fail)?;
                        let lines =
                            integer(&args[lines_index], &name, lines_index + 1).map_err(&fail)?;
                        if width <= 0.0 || lines <= 0 {
                            return Err(fail(format!(
                                "{name} wrap width and maximum lines must be positive"
                            )));
                        }
                        (
                            Some(width),
                            Some(usize::try_from(lines).map_err(|_| {
                                fail(format!("{name} maximum lines is out of range"))
                            })?),
                            lines_index + 1,
                            lines_index + 2,
                        )
                    } else {
                        (None, None, colour_index + 1, colour_index + 2)
                    };
                    if !args[left_index].quoted || !args[right_index].quoted {
                        return Err(fail(format!("{name} left and right text must be quoted")));
                    }
                    let font = args[0].value.clone();
                    let size = if has_size {
                        Some(number(&args[1], &name, 2).map_err(&fail)?)
                    } else {
                        None
                    };
                    let colour = NamedColour::parse(&args[colour_index].value).map_err(&fail)?;
                    let make_text = |argument: &Argument| -> Result<_> {
                        let text = quoted_text(argument, &name)?;
                        if text.trim().is_empty() {
                            Ok(None)
                        } else {
                            if let Some(size) = size {
                                MeasuredText::new(&font, size, &text)?;
                            }
                            Ok(Some(StampSideLabelSettings::new(text, &font, size, colour)))
                        }
                    };
                    let left = make_text(&args[left_index]).map_err(&fail)?;
                    let right = make_text(&args[right_index]).map_err(&fail)?;
                    if left.is_none() && right.is_none() {
                        return Err(fail(format!(
                            "{name} requires at least one visible side label"
                        )));
                    }
                    let labels = image_specs
                        .last_mut()
                        .ok_or_else(|| fail("Side label has no preceding stamp".into()))?;
                    if labels
                        .6
                        .as_ref()
                        .is_some_and(|labels| labels.left.is_some() || labels.right.is_some())
                    {
                        return Err(fail(format!("{name} may be attached only once per stamp")));
                    }
                    let labels = labels.6.get_or_insert_with(StampImageLabels::default);
                    labels.left = left;
                    labels.right = right;
                    labels.wrap_width_mm = wrap_width_mm;
                    labels.max_lines = max_lines;
                }
                "STAMP_IMAGE" => {
                    if !matches!(args.len(), 4 | 7) {
                        return Err(fail(format!(
                            "{name} requires image path, physical width, physical height, image scale, and optional caption font, size, and text"
                        )));
                    }
                    let image_path = PathBuf::from(quoted(&args[0], &name).map_err(&fail)?);
                    let width = number(&args[1], &name, 2).map_err(&fail)?;
                    let height = number(&args[2], &name, 3).map_err(&fail)?;
                    let scale_mm_per_pixel = number(&args[3], &name, 4).map_err(&fail)?;
                    if width <= 0.0 || height <= 0.0 || scale_mm_per_pixel <= 0.0 {
                        return Err(fail(format!(
                            "{name} physical dimensions and image scale must be positive"
                        )));
                    }
                    let caption = if args.len() == 7 {
                        Some(StampTextSettings::new(
                            quoted(&args[6], &name).map_err(&fail)?,
                            unquoted(&args[4], &name).map_err(&fail)?,
                            number(&args[5], &name, 6).map_err(&fail)?,
                            NamedColour::parse("black").map_err(&fail)?,
                        ))
                    } else {
                        None
                    };
                    let geometry = StampBoxGeometry::from_total_additions(
                        Bounds {
                            left: 0.0,
                            bottom: 0.0,
                            right: width,
                            top: height,
                        },
                        frame_addition_mm,
                        frame_addition_mm,
                        0.0,
                    )
                    .map_err(&fail)?;
                    image_specs.push((
                        asset_root.join(image_path),
                        geometry,
                        mm_to_points(scale_mm_per_pixel),
                        image_options,
                        caption,
                        frame_style,
                        None,
                    ));
                    headings.push(NativeHeadingItem {
                        heading: None,
                        footer_text: None,
                        vertical,
                    });
                }
                _ => return Err(fail(format!("Unsupported native command {name}"))),
            }
            previous_command = name;
        }

        if !header_seen {
            return Err("Missing version header `STAMPALBUM 1`".into());
        }
        let page_size = page_size.ok_or("Missing required ALBUM_PAGES_SIZE command")?;
        let margins = margins.ok_or("Missing required ALBUM_PAGES_MARGINS command")?;
        let spacing = spacing.ok_or("Missing required ALBUM_PAGES_SPACING command")?;
        let border = border.ok_or("Missing required ALBUM_PAGES_BORDER command")?;
        let border_colour = border_colour.ok_or("Missing required COLOUR_ALBUM_BORDER command")?;
        let alignment = alignment.ok_or("Missing required NATIVE_ROW_ALIGNMENT command")?;
        let (frame_width_mm, frame_colour) =
            stamp_frame.ok_or("Missing required NATIVE_STAMP_FRAME command")?;
        let stamp_text_policy =
            text_policy.ok_or("Missing required NATIVE_STAMP_TEXT_POLICY command")?;
        if image_specs.is_empty() {
            return Err("At least one STAMP_IMAGE command is required".into());
        }
        if numbering_section_starts
            .last()
            .is_some_and(|start| *start >= image_specs.len())
        {
            return Err("NATIVE_PAGE_NUMBER_SECTION must be followed by a STAMP_IMAGE".into());
        }
        let remaining_items = image_specs.len().saturating_sub(section_start);
        if remaining_items == 0 {
            return Err("NATIVE_ROW_BREAK must not follow the final STAMP_IMAGE".into());
        }
        section_counts.push(remaining_items);

        let mut page_settings =
            PageSettings::new(page_size, margins, spacing, border, border_colour);
        page_settings.set_row_alignment(alignment);
        if let Some(title) = page_title {
            page_settings.set_title(title);
        }
        if let Some(header) = page_header {
            page_settings.set_header(header);
        }
        if let Some(number) = footer_number {
            page_settings.set_footer_number(number);
        }
        for text in margin_text {
            page_settings.add_margin_text(text)?;
        }
        let sections = section_counts
            .into_iter()
            .map(|item_count| StampRowSection {
                item_count,
                alignment,
            })
            .collect();
        let items = image_specs
            .into_iter()
            .map(
                |(
                    image_path,
                    geometry,
                    image_scale_pt_per_pixel,
                    options,
                    caption,
                    frame_style,
                    labels,
                )| {
                    StampImageRowItem {
                        stamp: StampTextItem {
                            geometry,
                            heading: None,
                            caption,
                        },
                        image_path,
                        image_scale_pt_per_pixel,
                        options,
                        frame_style,
                        labels,
                    }
                },
            )
            .collect();

        Ok(Self {
            page_settings,
            stamp_text_policy,
            frame_width_mm,
            frame_colour,
            page_background,
            items,
            sections,
            numbering_section_starts,
            headings,
        })
    }

    /// Generates a native PDF directly without launching AlbumEasy or exporting TXT.
    pub fn write_pdf(&self, output: &Path) -> Result<Vec<StampItemPage>> {
        self.write_pdf_with_numbering(None, None, output)
    }

    /// Generates a native PDF using explicitly supplied TOML layout policies.
    pub fn write_pdf_with_config(
        &self,
        config: &NativePdfConfig,
        output: &Path,
    ) -> Result<Vec<StampItemPage>> {
        self.write_pdf_with_numbering(
            config.footer_numbering,
            config.stamp_image_side_labels,
            output,
        )
    }

    fn write_pdf_with_numbering(
        &self,
        numbering: Option<NativeFooterNumberingConfig>,
        side_label_config: Option<super::config::StampImageSideLabelConfig>,
        output: &Path,
    ) -> Result<Vec<StampItemPage>> {
        let numbering = numbering.filter(|policy| policy.enabled);
        match (self.numbering_section_starts.is_empty(), numbering) {
            (false, Some(policy)) if policy.scope == NativePageNumberingScope::Section => {}
            (false, _) => {
                return Err("NATIVE_PAGE_NUMBER_SECTION requires enabled native_pdf.footer_number.scope = \"section\"".into());
            }
            (true, Some(policy)) if policy.scope == NativePageNumberingScope::Section => {}
            _ => {}
        }
        if numbering.is_some() && !self.page_settings.has_footer_number() {
            return Err(
                "Native footer numbering configuration requires NATIVE_PAGE_FOOTER_NUMBER".into(),
            );
        }

        let mut items = self.items.clone();
        for (index, item) in items.iter_mut().enumerate() {
            let Some(labels) = item.labels.as_mut() else {
                continue;
            };
            if labels.left.is_none() && labels.right.is_none() {
                continue;
            }
            let config = side_label_config.ok_or(
                "NATIVE_STAMP_IMAGE_SIDE_LABEL requires native_pdf.stamp_image_side_labels in the supplied TOML config",
            )?;
            let (wrap_width_mm, max_lines) =
                config.resolve(labels.wrap_width_mm, labels.max_lines)?;
            for (side, label) in [
                ("left", labels.left.as_mut()),
                ("right", labels.right.as_mut()),
            ] {
                let Some(label) = label else {
                    continue;
                };
                let size_pt = config.resolve_font_size(label.size_pt)?;
                label.size_pt = Some(size_pt);
                let measured =
                    MeasuredText::new(&label.font, size_pt, &label.text).map_err(|error| {
                        format!(
                            "Side label for native stamp {} ({side}): {error}",
                            index + 1
                        )
                    })?;
                let wrapped = measured.wrapped_to_width(wrap_width_mm).map_err(|error| {
                    format!(
                        "Side label for native stamp {} ({side}): {error}",
                        index + 1
                    )
                })?;
                if wrapped.lines.len() > max_lines {
                    return Err(format!(
                        "Side label for native stamp {} ({side}) uses {} lines; configured limit is {max_lines}",
                        index + 1,
                        wrapped.lines.len()
                    ));
                }
                label.text = wrapped
                    .lines
                    .iter()
                    .map(|line| line.text.as_str())
                    .collect::<Vec<_>>()
                    .join("\n");
            }
            labels.wrap_width_mm = Some(wrap_width_mm);
            labels.max_lines = Some(max_lines);
        }

        let mut page_settings = self.page_settings.clone();
        if numbering.is_some() {
            page_settings.enable_native_numbering_tokens();
        }
        let mut pages = if self.headings.iter().any(|item| {
            item.heading.is_some()
                || item.footer_text.is_some()
                || item.vertical != VerticalAlignment::Top
        }) || items.iter().any(|item| {
            item.labels
                .as_ref()
                .is_some_and(|labels| labels.left.is_some() || labels.right.is_some())
        }) {
            stamp_heading::paginate_native(
                &page_settings,
                &items,
                &self.headings,
                &self.sections,
                &self.stamp_text_policy,
            )?
        } else {
            let stamps = self
                .items
                .iter()
                .map(|item| item.stamp.clone())
                .collect::<Vec<_>>();
            super::pagination::paginate_stamp_sections(
                &page_settings,
                &stamps,
                &self.sections,
                &self.stamp_text_policy,
                page_settings.capture_generation_date(),
            )?
        };
        if let Some(policy) = numbering {
            apply_footer_numbering(
                &page_settings,
                &mut pages,
                &self.numbering_section_starts,
                policy,
            )?;
        }
        renderer::write_native_heading_album_pdf(
            &page_settings,
            &items,
            pages,
            (self.frame_width_mm, self.frame_colour),
            self.page_background.as_ref(),
            output,
        )
    }
}

#[derive(Debug)]
pub(super) struct Argument {
    pub value: String,
    pub quoted: bool,
}

pub(super) fn strip_comment(line: &str) -> &str {
    let mut quoted = false;
    let mut escaped = false;
    for (index, character) in line.char_indices() {
        if character == '"' && !escaped {
            quoted = !quoted;
        }
        if character == '#' && !quoted {
            return &line[..index];
        }
        escaped = character == '\\' && !escaped;
        if character != '\\' {
            escaped = false;
        }
    }
    line
}

pub(super) fn parse_command(line: &str) -> Result<(String, Vec<Argument>)> {
    let open = line
        .find('(')
        .ok_or("Expected a command followed by parenthesized arguments")?;
    let name = line[..open].trim();
    if name.is_empty()
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte == b'_')
    {
        return Err("Command names must contain uppercase ASCII letters and underscores".into());
    }
    let remainder = line[open + 1..].trim();
    let body = remainder
        .strip_suffix(')')
        .ok_or("Command arguments must end with `)`")?;
    parse_arguments(name, body).map(|args| (name.to_string(), args))
}

fn parse_arguments(name: &str, body: &str) -> Result<Vec<Argument>> {
    let mut arguments = Vec::new();
    let mut chars = body.chars().peekable();
    while let Some(character) = chars.peek().copied() {
        if character.is_whitespace() {
            chars.next();
            continue;
        }
        if character == '"' {
            chars.next();
            let mut value = String::new();
            let mut closed = false;
            while let Some(character) = chars.next() {
                match character {
                    '"' => {
                        closed = true;
                        break;
                    }
                    '\\' => match chars.next() {
                        Some('"') => value.push('"'),
                        Some('\\') => value.push('\\'),
                        Some('n') => value.push('\n'),
                        Some(other) => {
                            return Err(format!(
                                "{name} contains unsupported string escape `\\{other}`"
                            ))
                        }
                        None => return Err(format!("{name} has an incomplete string escape")),
                    },
                    other => value.push(other),
                }
            }
            if !closed {
                return Err(format!("{name} has an unterminated quoted argument"));
            }
            if chars.peek().is_some_and(|next| !next.is_whitespace()) {
                return Err(format!("{name} arguments must be separated by whitespace"));
            }
            arguments.push(Argument {
                value,
                quoted: true,
            });
        } else {
            let mut value = String::new();
            while let Some(character) = chars.peek().copied() {
                if character.is_whitespace() {
                    break;
                }
                if matches!(character, '(' | ')' | '"' | '#') {
                    return Err(format!("{name} has an invalid unquoted argument"));
                }
                value.push(character);
                chars.next();
            }
            arguments.push(Argument {
                value,
                quoted: false,
            });
        }
    }
    Ok(arguments)
}

fn numbers(name: &str, arguments: &[Argument], count: usize) -> Result<Vec<f64>> {
    if arguments.len() != count {
        return Err(format!(
            "{name} requires exactly {count} numeric arguments, received {}",
            arguments.len()
        ));
    }
    arguments
        .iter()
        .enumerate()
        .map(|(index, argument)| number(argument, name, index + 1))
        .collect()
}

pub(super) fn save_stamp_settings<T>(saved: &mut Option<T>, settings: T, name: &str) -> Result<()> {
    if saved.is_some() {
        return Err(format!(
            "{name}: stamp settings have already been saved; restore before saving again"
        ));
    }
    *saved = Some(settings);
    Ok(())
}

pub(super) fn restore_stamp_settings<T>(saved: &mut Option<T>, name: &str) -> Result<T> {
    saved
        .take()
        .ok_or_else(|| format!("{name}: stamp settings have not been saved; save before restoring"))
}

pub(super) fn integer(argument: &Argument, name: &str, index: usize) -> Result<i32> {
    if argument.quoted {
        return Err(format!(
            "{name} argument {index} must be an unquoted integer"
        ));
    }
    argument
        .value
        .parse::<i32>()
        .map_err(|_| format!("{name} argument {index} must be a signed 32-bit integer"))
}

pub(super) fn number(argument: &Argument, name: &str, index: usize) -> Result<f64> {
    if argument.quoted {
        return Err(format!(
            "{name} argument {index} must be numeric, not quoted"
        ));
    }
    let value = argument
        .value
        .parse::<f64>()
        .map_err(|_| format!("{name} argument {index} must be a finite numeric value"))?;
    if !value.is_finite() {
        return Err(format!("{name} argument {index} must be finite"));
    }
    Ok(value)
}

fn boolean(argument: &Argument, name: &str, index: usize) -> Result<bool> {
    if argument.quoted {
        return Err(format!("{name} argument {index} must be true or false"));
    }
    match argument.value.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("{name} argument {index} must be true or false")),
    }
}

fn words(name: &str, arguments: &[Argument], count: usize) -> Result<Vec<String>> {
    if arguments.len() != count {
        return Err(format!(
            "{name} requires exactly {count} text arguments, received {}",
            arguments.len()
        ));
    }
    arguments
        .iter()
        .map(|argument| unquoted(argument, name))
        .collect()
}

fn unquoted(argument: &Argument, name: &str) -> Result<String> {
    if argument.value.is_empty() {
        return Err(format!("{name} text arguments must not be empty"));
    }
    Ok(argument.value.clone())
}

fn quoted(argument: &Argument, name: &str) -> Result<String> {
    if !argument.quoted || argument.value.is_empty() {
        return Err(format!(
            "{name} image path must be a non-empty quoted string"
        ));
    }
    Ok(argument.value.clone())
}

fn quoted_text(argument: &Argument, name: &str) -> Result<String> {
    if !argument.quoted {
        return Err(format!("{name} caption text must be quoted"));
    }
    Ok(argument.value.clone())
}

fn set_once<T>(slot: &mut Option<T>, value: T, command: &str) -> Result<()> {
    if slot.is_some() {
        return Err(format!("Duplicate {command} command"));
    }
    *slot = Some(value);
    Ok(())
}

fn apply_footer_numbering(
    settings: &PageSettings,
    pages: &mut [StampItemPage],
    section_starts: &[usize],
    policy: NativeFooterNumberingConfig,
) -> Result<()> {
    let document_pages = pages.len();
    if document_pages == 0 {
        return Err("Native page numbering requires at least one rendered page".into());
    }

    let mut page_sections = Vec::with_capacity(document_pages);
    let mut current_section = 0usize;
    for page in pages.iter() {
        if let Some(first_item) = page.items.iter().map(|item| item.source_index).min() {
            current_section = section_starts.partition_point(|start| *start <= first_item);
        }
        page_sections.push(current_section);
    }

    let mut section_pages = vec![0usize; section_starts.len() + 1];
    for section in &page_sections {
        section_pages[*section] = section_pages[*section]
            .checked_add(1)
            .ok_or("Native numbering section page count overflow")?;
    }

    let generation_date = settings.capture_generation_date();
    let mut section_ordinals = vec![0usize; section_pages.len()];
    for (index, (page, section)) in pages.iter_mut().zip(page_sections).enumerate() {
        let ordinal = section_ordinals[section]
            .checked_add(1)
            .ok_or("Native numbering section ordinal overflow")?;
        section_ordinals[section] = ordinal;
        let section_scope = policy.scope == NativePageNumberingScope::Section;
        let context = NumberingContext {
            section_ordinal: if section_scope { ordinal } else { index + 1 },
            section_pages: if section_scope {
                section_pages[section]
            } else {
                document_pages
            },
            document_pages,
            start: if section_scope {
                policy.section_start
            } else {
                None
            },
        };
        let layout = settings
            .resolve_with_numbering_context(index + 1, document_pages, generation_date, context)
            .map_err(|error| format!("Native PDF page {}: {error}", index + 1))?;
        if !same_bounds(layout.content_bounds, page.layout.content_bounds) {
            return Err(format!(
                "Native PDF page {}: section numbering changed content bounds after pagination",
                index + 1
            ));
        }
        page.layout = layout;
    }
    Ok(())
}

fn same_bounds(left: super::document::Bounds, right: super::document::Bounds) -> bool {
    [
        (left.left, right.left),
        (left.bottom, right.bottom),
        (left.right, right.right),
        (left.top, right.top),
    ]
    .iter()
    .all(|(left, right)| (left - right).abs() <= 1e-9)
}

#[cfg(test)]
mod stamp_settings_tests {
    use super::*;

    #[test]
    fn native_snapshot_captures_options_and_style_per_item() -> Result<()> {
        let source = include_str!(
            "../../tests/fixtures/albumeasy/stamp-settings-save-restore-v1/album.stampalbum"
        );
        let album = NativeAlbum::parse(source, Path::new("."))?;
        assert_eq!(album.items.len(), 12);
        for (first, restored) in album.items[..2].iter().zip(&album.items[4..6]) {
            assert_eq!(first.options, restored.options);
            assert_eq!(
                first
                    .frame_style
                    .ok_or("Missing baseline frame style")?
                    .background
                    .map(NamedColour::rgb),
                restored
                    .frame_style
                    .ok_or("Missing restored frame style")?
                    .background
                    .map(NamedColour::rgb)
            );
        }
        assert!(album.items[0].options.grayscale);
        assert!(!album.items[2].options.grayscale);
        assert!(!album.items[6].options.visible);
        assert!(album.items[8].options.visible);
        assert!(!album.items[10].options.visible);
        assert!(
            !album.items[6]
                .frame_style
                .ok_or("Missing hidden outline")?
                .outline
        );
        assert!(
            album.items[8]
                .frame_style
                .ok_or("Missing temporary outline")?
                .outline
        );
        assert!(
            !album.items[10]
                .frame_style
                .ok_or("Missing restored outline")?
                .outline
        );
        Ok(())
    }
}
