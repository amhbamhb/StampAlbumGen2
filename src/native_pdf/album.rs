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
    document::{Bounds, PageSettings},
    pagination::{StampItemPage, StampRowSection},
    renderer,
    stamp_box::StampBoxGeometry,
    stamp_image::{
        ImageAspectRatio, ImagePaddingUnit, NativePageBackgroundImage, StampFrameStyle,
        StampImageOptions, StampImageRowItem, StampTextSettings,
    },
    stamp_inner_border::{InnerBorderStyle, StampInnerBorder},
    stamp_rows::RowAlignment,
    stamp_text::{ExpandedWidthLimit, StampTextItem, StampTextPolicy, TextWidthPolicy},
    text::MeasuredText,
    units::mm_to_points,
    Result,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

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
        let mut image_specs = Vec::new();
        let mut section_start = 0;
        let mut section_counts = Vec::new();

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
                        (image_options, frame_style),
                        &name,
                    )
                    .map_err(&fail)?;
                }
                "NATIVE_STAMP_SETTINGS_RESTORE" => {
                    words(&name, &args, 0).map_err(&fail)?;
                    (image_options, frame_style) =
                        restore_stamp_settings(&mut saved_stamp_settings, &name).map_err(&fail)?;
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
                    ));
                }
                _ => return Err(fail(format!("Unsupported native command {name}"))),
            }
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
        })
    }

    /// Generates a native PDF directly without launching AlbumEasy or exporting TXT.
    pub fn write_pdf(&self, output: &Path) -> Result<Vec<StampItemPage>> {
        renderer::write_native_album_pdf_with_sections(
            &self.page_settings,
            &self.items,
            &self.sections,
            &self.stamp_text_policy,
            self.frame_width_mm,
            self.frame_colour,
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

fn set_once<T>(slot: &mut Option<T>, value: T, command: &str) -> Result<()> {
    if slot.is_some() {
        return Err(format!("Duplicate {command} command"));
    }
    *slot = Some(value);
    Ok(())
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
