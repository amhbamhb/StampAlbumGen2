use super::{
    colour::NamedColour,
    commands::{
        colour::colour_album_border::AlbumBorderColour,
        page::{
            album_pages_border::PageBorder, album_pages_margins::Margins,
            album_pages_size::PageSize, album_pages_spacing::Spacing,
        },
    },
    document::{Bounds, PageSettings},
    pagination::StampItemPage,
    renderer,
    stamp_box::StampBoxGeometry,
    stamp_image::StampImageRowItem,
    stamp_rows::RowAlignment,
    stamp_text::{ExpandedWidthLimit, StampTextItem, StampTextPolicy, TextWidthPolicy},
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
    items: Vec<StampImageRowItem>,
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
        let mut alignment = None;
        let mut stamp_frame = None;
        let mut text_policy = None;
        let mut image_specs = Vec::new();

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
                    if args.len() != 2 {
                        return Err(fail(format!(
                            "{name} requires a positive line width in millimetres and a named colour"
                        )));
                    }
                    let width = number(&args[0], &name, 1).map_err(&fail)?;
                    if width <= 0.0 || !mm_to_points(width).is_finite() {
                        return Err(fail(format!(
                            "{name} line width must be finite and positive"
                        )));
                    }
                    let colour = words(&name, &args[1..], 1)
                        .map_err(&fail)?
                        .into_iter()
                        .next()
                        .ok_or_else(|| fail(format!("{name} requires a named colour")))?;
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
                "STAMP_IMAGE" => {
                    if args.len() != 4 {
                        return Err(fail(format!(
                            "{name} requires image path, physical width, physical height, and image scale in millimetres per pixel"
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
                    let geometry = StampBoxGeometry::from_total_additions(
                        Bounds {
                            left: 0.0,
                            bottom: 0.0,
                            right: width,
                            top: height,
                        },
                        0.0,
                        0.0,
                        0.0,
                    )
                    .map_err(&fail)?;
                    image_specs.push((
                        asset_root.join(image_path),
                        geometry,
                        mm_to_points(scale_mm_per_pixel),
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

        let mut page_settings =
            PageSettings::new(page_size, margins, spacing, border, border_colour);
        page_settings.set_row_alignment(alignment);
        let items = image_specs
            .into_iter()
            .map(
                |(image_path, geometry, image_scale_pt_per_pixel)| StampImageRowItem {
                    stamp: StampTextItem {
                        geometry,
                        heading: None,
                        caption: None,
                    },
                    image_path,
                    image_scale_pt_per_pixel,
                },
            )
            .collect();

        Ok(Self {
            page_settings,
            stamp_text_policy,
            frame_width_mm,
            frame_colour,
            items,
        })
    }

    /// Generates a native PDF directly without launching AlbumEasy or exporting TXT.
    pub fn write_pdf(&self, output: &Path) -> Result<Vec<StampItemPage>> {
        renderer::write_native_album_pdf(
            &self.page_settings,
            &self.items,
            &self.stamp_text_policy,
            self.frame_width_mm,
            self.frame_colour,
            output,
        )
    }
}

#[derive(Debug)]
struct Argument {
    value: String,
    quoted: bool,
}

fn strip_comment(line: &str) -> &str {
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

fn parse_command(line: &str) -> Result<(String, Vec<Argument>)> {
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

fn number(argument: &Argument, name: &str, index: usize) -> Result<f64> {
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
