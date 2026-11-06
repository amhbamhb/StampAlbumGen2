use super::{
    colour::NamedColour,
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder,
        album_pages_crop_marks::AlbumPagesCropMarks,
        album_pages_decorative_border::{AlbumPagesDecorativeBorder, DecorativeBorderPolicy},
        album_pages_footer::AlbumPagesFooter,
        album_pages_footer_date::AlbumPagesFooterDate,
        album_pages_footer_number::AlbumPagesFooterNumber,
        album_pages_footer_pad::FooterPadding,
        album_pages_header::{AlbumPagesHeader, HeaderPosition},
        album_pages_header_date::AlbumPagesHeaderDate,
        album_pages_header_number::AlbumPagesHeaderNumber,
        album_pages_header_pad::HeaderPadding,
        album_pages_margins::Margins,
        album_pages_marginse::EvenMargins,
        album_pages_size::PageSize,
        album_pages_spacing::Spacing,
        album_pages_title::AlbumPagesTitle,
    },
    date::{DateFormat, DateSource},
    document::PageSettings,
    proof::GeometryProofSettings,
    stamp_text::{ExpandedWidthLimit, StampTextPolicy, TextWidthPolicy},
    text::TextPosition,
    Result,
};
use std::{fs, path::Path};
use toml::{Table, Value};

const SIZE: [&str; 2] = ["album_page_width_mm", "album_page_height_mm"];
const MARGINS: [&str; 4] = [
    "page_left_margin_mm",
    "page_right_margin_mm",
    "page_top_margin_mm",
    "page_bottom_margin_mm",
];
const SPACING: [&str; 2] = ["album_pages_spacing_h", "album_pages_spacing_v"];
const BORDER: [&str; 4] = [
    "page_border_outer",
    "page_border_inner1",
    "page_border_inner2",
    "page_border_spacing",
];

/// An explicit, validated snapshot for implemented native commands only.
/// Other template settings are reported by key, never interpreted as supported.
pub struct NativePdfConfig {
    pub page_settings: PageSettings,
    pub proof: GeometryProofSettings,
    pub stamp_text: Option<StampTextPolicy>,
    pub unapplied_keys: Vec<String>,
    pub overridden_keys: Vec<String>,
}

impl NativePdfConfig {
    /// Reads exactly this path; no global config, upward search or default fallback.
    pub fn load(path: &Path) -> Result<Self> {
        let source = fs::read_to_string(path).map_err(|error| {
            format!("Cannot read native PDF config {}: {error}", path.display())
        })?;
        Self::from_toml_in(&source, path.parent().unwrap_or_else(|| Path::new("")))
            .map_err(|error| format!("Native PDF config {}: {error}", path.display()))
    }

    pub fn from_toml(source: &str) -> Result<Self> {
        Self::parse(source, None)
    }

    /// Explicit asset base for in-memory settings; never searches global/CWD directories.
    pub fn from_toml_in(source: &str, base: &Path) -> Result<Self> {
        Self::parse(source, Some(base))
    }

    fn parse(source: &str, base: Option<&Path>) -> Result<Self> {
        // TODO(native-pdf-ui): Surface missing/empty/invalid setting errors in the
        // settings UI, identify the key and ask the user to correct their TOML.
        // Keep shipped defaults in TOML; never silently replace a user's value.
        let mut root: Table = toml::from_str(source).map_err(|error: toml::de::Error| {
            // TOML Display includes source snippets, which may contain credentials.
            match error.span() {
                Some(span) => format!("Invalid TOML syntax at byte offset {}", span.start),
                None => "Invalid TOML syntax".into(),
            }
        })?;
        let mut page = match root.remove("page") {
            Some(Value::Table(table)) => table,
            Some(_) => return Err("Native PDF setting page must be a table".into()),
            None => Table::new(),
        };
        let mut overridden_keys = Vec::new();
        for key in SIZE.iter().chain(&MARGINS).chain(&SPACING).chain(&BORDER) {
            if let Some(value) = page.remove(*key) {
                if root.insert((*key).into(), value).is_some() {
                    overridden_keys.push((*key).into());
                }
            }
        }
        let mut colors = match root.remove("colors") {
            Some(Value::Table(table)) => table,
            Some(_) => return Err("Native PDF setting colors must be a table".into()),
            None => Table::new(),
        };
        if let Some(value) = colors.remove("color_album_border") {
            if root.insert("color_album_border".into(), value).is_some() {
                overridden_keys.push("color_album_border".into());
            }
        }
        overridden_keys.sort();

        let size = PageSize::parse(&numbers(&mut root, &SIZE, "")?)?;
        let margins = Margins::parse(&numbers(&mut root, &MARGINS, "")?)?;
        let spacing = Spacing::parse(&numbers(&mut root, &SPACING, "")?)?;
        let border = PageBorder::parse(&numbers(&mut root, &BORDER, "")?)?;
        let colour = string(&mut root, "color_album_border", "")?;
        let border_colour = AlbumBorderColour::parse(&[&colour])
            .map_err(|error| format!("color_album_border: {error}"))?;
        let mut page_settings = PageSettings::new(size, margins, spacing, border, border_colour);
        let mut native = table(&mut root, "native_pdf", "")?;
        let mut stamp_text = None;
        if native.contains_key("stamp_text") {
            let prefix = "native_pdf.stamp_text.";
            let mut settings = table(&mut native, "stamp_text", "native_pdf.")?;
            let enabled = boolean(&mut settings, "enabled", prefix)?;
            let mode = TextWidthPolicy::parse(&string(&mut settings, "width_policy", prefix)?)?;
            let policy = StampTextPolicy::new(
                mode,
                ExpandedWidthLimit::new(
                    &string(&mut settings, "expanded_width_limit", prefix)?,
                    number(
                        &mut settings,
                        "expanded_width_value",
                        prefix,
                        "percent or millimetres",
                    )?,
                )?,
                number(
                    &mut settings,
                    "horizontal_padding_mm",
                    prefix,
                    "millimetres",
                )?,
                number(&mut settings, "vertical_padding_mm", prefix, "millimetres")?,
                number(&mut settings, "text_gap_mm", prefix, "millimetres")?,
            )?;
            if enabled {
                stamp_text = Some(policy);
            }
            if !settings.is_empty() {
                native.insert("stamp_text".into(), Value::Table(settings));
            }
        }
        if native.contains_key("decorative_border") {
            let prefix = "native_pdf.decorative_border.";
            let mut settings = table(&mut native, "decorative_border", "native_pdf.")?;
            let enabled = boolean(&mut settings, "enabled", prefix)?;
            let filename = string(&mut settings, "definition_file", prefix)?;
            let values = DecorativeBorderPolicy::KEYS
                .iter()
                .map(|key| {
                    number(
                        &mut settings,
                        key,
                        prefix,
                        if *key == "image_scale" {
                            "scale factor"
                        } else {
                            "millimetres"
                        },
                    )
                })
                .collect::<Result<Vec<_>>>()?;
            let colour = NamedColour::parse(&string(&mut settings, "colour", prefix)?)
                .map_err(|error| format!("{prefix}colour: {error}"))?;
            let policy = DecorativeBorderPolicy::parse(&values, colour)?;
            if enabled {
                if filename.trim().is_empty() {
                    return Err(format!(
                        "{prefix}definition_file must not be empty when enabled"
                    ));
                }
                let path = Path::new(&filename);
                let path = if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    base.ok_or("native_pdf.decorative_border.definition_file is relative; use load or from_toml_in with an explicit config directory")?.join(path)
                };
                page_settings
                    .set_decorative_border(AlbumPagesDecorativeBorder::load(&path, policy)?);
            }
            if !settings.is_empty() {
                native.insert("decorative_border".into(), Value::Table(settings));
            }
        }
        if native.contains_key("crop_marks") {
            let mut settings = table(&mut native, "crop_marks", "native_pdf.")?;
            let prefix = "native_pdf.crop_marks.";
            let enabled = boolean(&mut settings, "enabled", prefix)?;
            let values = numbers(
                &mut settings,
                &[
                    "line_width_mm",
                    "horizontal_padding_mm",
                    "vertical_padding_mm",
                    "arms_mm",
                ],
                prefix,
            )?;
            let colour = NamedColour::parse(&string(&mut settings, "colour", prefix)?)
                .map_err(|error| format!("{prefix}colour: {error}"))?;
            let command = AlbumPagesCropMarks::parse(&values, colour)?;
            // Validate disabled settings too; never hide an unusable override.
            command.layout(size)?;
            if enabled {
                page_settings.set_crop_marks(command);
            }
            if !settings.is_empty() {
                native.insert("crop_marks".into(), Value::Table(settings));
            }
        }
        let mut footer_policy = None;
        if native.contains_key("footer") {
            let mut settings = table(&mut native, "footer", "native_pdf.")?;
            let enabled = match settings.remove("enabled") {
                Some(Value::Boolean(enabled)) => enabled,
                _ => {
                    return Err(
                        "Missing or invalid native_pdf.footer.enabled; expected a boolean".into(),
                    )
                }
            };
            if let Some(colour) = colors.remove("color_album_footer") {
                if root.insert("color_album_footer".into(), colour).is_some() {
                    overridden_keys.push("color_album_footer".into());
                }
            }
            let colour = NamedColour::parse(&string(&mut root, "color_album_footer", "")?)
                .map_err(|error| format!("color_album_footer: {error}"))?;
            let prefix = "native_pdf.footer.";
            let font = string(&mut settings, "font", prefix)?;
            let size = number(&mut settings, "size_pt", prefix, "points")?;
            let position = TextPosition::parse_for(
                &string(&mut settings, "position", prefix)?,
                "native_pdf.footer.position",
            )?;
            let text = string(&mut settings, "text", prefix)?;
            let padding = FooterPadding::parse(&[number(
                &mut settings,
                "padding_mm",
                prefix,
                "millimetres",
            )?])?;
            let minimum = number(&mut settings, "minimum_height_mm", prefix, "millimetres")?;
            footer_policy = Some((padding, minimum, colour));
            let command =
                AlbumPagesFooter::new(&font, size, position, &text, padding, minimum, colour)?;
            if enabled {
                page_settings.set_footer(command);
            }
            if !settings.is_empty() {
                native.insert("footer".into(), Value::Table(settings));
            }
        }
        if native.contains_key("footer_date") {
            let mut settings = table(&mut native, "footer_date", "native_pdf.")?;
            let prefix = "native_pdf.footer_date.";
            let enabled = boolean(&mut settings, "enabled", prefix)?;
            let font = string(&mut settings, "font", prefix)?;
            let size = number(&mut settings, "size_pt", prefix, "points")?;
            let position = TextPosition::parse_for(
                &string(&mut settings, "position", prefix)?,
                "native_pdf.footer_date.position",
            )?;
            let format = DateFormat::parse_for(
                &string(&mut settings, "format", prefix)?,
                "native_pdf.footer_date",
            )?;
            let separator = string(&mut settings, "separator", prefix)?;
            let before = string(&mut settings, "before", prefix)?;
            let after = string(&mut settings, "after", prefix)?;
            let source_token = string(&mut settings, "date_source", prefix)?;
            let fixed_date = string(&mut settings, "fixed_date", prefix)?;
            let source =
                DateSource::parse_for(&source_token, &fixed_date, "native_pdf.footer_date")?;
            let (padding, minimum, colour) = footer_policy
                .ok_or("native_pdf.footer_date requires explicit native_pdf.footer padding/height/colour settings")?;
            let style =
                AlbumPagesFooter::new(&font, size, position, &fixed_date, padding, minimum, colour)
                    .map_err(|error| {
                        format!("native_pdf.footer_date.font/size_pt/position: {error}")
                    })?;
            let command =
                AlbumPagesFooterDate::new(style, format, &separator, &before, &after, source)?;
            if enabled {
                page_settings.set_footer_date(command);
            }
            if !settings.is_empty() {
                native.insert("footer_date".into(), Value::Table(settings));
            }
        }
        let mut numbering_enabled = false;
        if native.contains_key("footer_number") {
            let mut settings = table(&mut native, "footer_number", "native_pdf.")?;
            let opt_in = boolean(&mut settings, "enabled", "native_pdf.footer_number.")?;
            let mut footer = match root.remove("footer") {
                Some(Value::Table(table)) => table,
                Some(_) => return Err("Native PDF setting footer must be a table".into()),
                None => Table::new(),
            };
            for key in [
                "album_pages_footer_number_enabled",
                "album_pages_footer_number_font",
                "album_pages_footer_number_size",
                "album_pages_footer_number_position",
                "album_pages_footer_number_start",
                "album_pages_footer_number_before",
                "album_pages_footer_number_after",
            ] {
                if let Some(value) = footer.remove(key) {
                    if root.insert(key.into(), value).is_some() {
                        overridden_keys.push(key.into());
                    }
                }
            }
            let enabled = boolean(&mut root, "album_pages_footer_number_enabled", "")?;
            let font = string(&mut root, "album_pages_footer_number_font", "")?;
            let size = number(&mut root, "album_pages_footer_number_size", "", "points")?;
            let position = TextPosition::parse_for(
                &string(&mut root, "album_pages_footer_number_position", "")?,
                "album_pages_footer_number_position",
            )?;
            let start = match root.remove("album_pages_footer_number_start") {
                    Some(Value::Integer(value)) => i32::try_from(value)
                        .map_err(|_| "album_pages_footer_number_start must be a signed 32-bit integer")?,
                    _ => return Err("Missing or invalid album_pages_footer_number_start; expected a signed 32-bit integer".into()),
                };
            let before = string(&mut root, "album_pages_footer_number_before", "")?;
            let after = string(&mut root, "album_pages_footer_number_after", "")?;
            let (padding, minimum, colour) = footer_policy
                    .ok_or("native_pdf.footer_number requires explicit native_pdf.footer padding/height/colour settings")?;
            let style = AlbumPagesFooter::new(
                &font,
                size,
                position,
                &start.to_string(),
                padding,
                minimum,
                colour,
            )
            .map_err(|error| format!("album_pages_footer_number_font/size/position: {error}"))?;
            let command = AlbumPagesFooterNumber::new(style, start, &before, &after)?;
            numbering_enabled = opt_in && enabled;
            if numbering_enabled {
                page_settings.set_footer_number(command);
            }
            if !footer.is_empty() {
                root.insert("footer".into(), Value::Table(footer));
            }
            if !settings.is_empty() {
                native.insert("footer_number".into(), Value::Table(settings));
            }
        }
        let mut header_policy = None;
        if native.contains_key("header") {
            let mut settings = table(&mut native, "header", "native_pdf.")?;
            let enabled = match settings.remove("enabled") {
                Some(Value::Boolean(enabled)) => enabled,
                _ => {
                    return Err(
                        "Missing or invalid native_pdf.header.enabled; expected a boolean".into(),
                    )
                }
            };
            let mut header = match root.remove("header") {
                Some(Value::Table(table)) => table,
                Some(_) => return Err("Native PDF setting header must be a table".into()),
                None => Table::new(),
            };
            if let Some(position) = header.remove("album_pages_header_position") {
                if root
                    .insert("album_pages_header_position".into(), position)
                    .is_some()
                {
                    overridden_keys.push("album_pages_header_position".into());
                }
            }
            if let Some(colour) = colors.remove("color_album_header") {
                if root.insert("color_album_header".into(), colour).is_some() {
                    overridden_keys.push("color_album_header".into());
                }
            }
            let position =
                HeaderPosition::parse(&string(&mut root, "album_pages_header_position", "")?)?;
            let colour = NamedColour::parse(&string(&mut root, "color_album_header", "")?)
                .map_err(|error| format!("color_album_header: {error}"))?;
            let prefix = "native_pdf.header.";
            let font = string(&mut settings, "font", prefix)?;
            let size = number(&mut settings, "size_pt", prefix, "points")?;
            let text = string(&mut settings, "text", prefix)?;
            let padding = HeaderPadding::parse(&[number(
                &mut settings,
                "padding_mm",
                prefix,
                "millimetres",
            )?])?;
            let minimum = number(&mut settings, "minimum_height_mm", prefix, "millimetres")?;
            header_policy = Some((padding, minimum, colour));
            let command =
                AlbumPagesHeader::new(&font, size, position, &text, padding, minimum, colour)?;
            if enabled {
                page_settings.set_header(command);
            }
            if !header.is_empty() {
                root.insert("header".into(), Value::Table(header));
            }
            if !settings.is_empty() {
                native.insert("header".into(), Value::Table(settings));
            }
        }
        if native.contains_key("header_number") {
            let mut settings = table(&mut native, "header_number", "native_pdf.")?;
            let prefix = "native_pdf.header_number.";
            let enabled = boolean(&mut settings, "enabled", prefix)?;
            let font = string(&mut settings, "font", prefix)?;
            let size = number(&mut settings, "size_pt", prefix, "points")?;
            let position = TextPosition::parse_for(
                &string(&mut settings, "position", prefix)?,
                "native_pdf.header_number.position",
            )?;
            let start = match settings.remove("start") {
                Some(Value::Integer(value)) => i32::try_from(value)
                    .map_err(|_| "native_pdf.header_number.start must be a signed 32-bit integer")?,
                _ => return Err("Missing or invalid native_pdf.header_number.start; expected a signed 32-bit integer".into()),
            };
            let before = string(&mut settings, "before", prefix)?;
            let after = string(&mut settings, "after", prefix)?;
            let (padding, minimum, colour) = header_policy
                .ok_or("native_pdf.header_number requires explicit native_pdf.header padding/height/colour settings")?;
            let style = AlbumPagesHeader::new(
                &font,
                size,
                position,
                &start.to_string(),
                padding,
                minimum,
                colour,
            )
            .map_err(|error| format!("native_pdf.header_number.font/size_pt/position: {error}"))?;
            let command = AlbumPagesHeaderNumber::new(style, start, &before, &after)?;
            numbering_enabled |= enabled;
            if enabled {
                page_settings.set_header_number(command);
            }
            if !settings.is_empty() {
                native.insert("header_number".into(), Value::Table(settings));
            }
        }
        if native.contains_key("header_date") {
            let mut settings = table(&mut native, "header_date", "native_pdf.")?;
            let prefix = "native_pdf.header_date.";
            let enabled = boolean(&mut settings, "enabled", prefix)?;
            let font = string(&mut settings, "font", prefix)?;
            let size = number(&mut settings, "size_pt", prefix, "points")?;
            let position = TextPosition::parse_for(
                &string(&mut settings, "position", prefix)?,
                "native_pdf.header_date.position",
            )?;
            let format = DateFormat::parse(&string(&mut settings, "format", prefix)?)?;
            let separator = string(&mut settings, "separator", prefix)?;
            let before = string(&mut settings, "before", prefix)?;
            let after = string(&mut settings, "after", prefix)?;
            let source_token = string(&mut settings, "date_source", prefix)?;
            let fixed_date = string(&mut settings, "fixed_date", prefix)?;
            let source = DateSource::parse(&source_token, &fixed_date)?;
            let (padding, minimum, colour) = header_policy
                .ok_or("native_pdf.header_date requires explicit native_pdf.header padding/height/colour settings")?;
            let style =
                AlbumPagesHeader::new(&font, size, position, &fixed_date, padding, minimum, colour)
                    .map_err(|error| {
                        format!(
                            "native_pdf.header_date.font/size_pt/position/before/after: {error}"
                        )
                    })?;
            let command =
                AlbumPagesHeaderDate::new(style, format, &separator, &before, &after, source)?;
            if enabled {
                page_settings.set_header_date(command);
            }
            if !settings.is_empty() {
                native.insert("header_date".into(), Value::Table(settings));
            }
        }
        if native.contains_key("title") {
            let mut title = table(&mut native, "title", "native_pdf.")?;
            let enabled = match title.remove("enabled") {
                Some(Value::Boolean(enabled)) => enabled,
                _ => {
                    return Err(
                        "Missing or invalid native_pdf.title.enabled; expected a boolean".into(),
                    )
                }
            };
            let mut header = match root.remove("header") {
                Some(Value::Table(table)) => table,
                Some(_) => return Err("Native PDF setting header must be a table".into()),
                None => Table::new(),
            };
            for key in ["album_pages_title_font", "album_pages_title_size"] {
                if let Some(value) = header.remove(key) {
                    if root.insert(key.into(), value).is_some() {
                        overridden_keys.push(key.into());
                    }
                }
            }
            if let Some(value) = colors.remove("color_album_title") {
                if root.insert("color_album_title".into(), value).is_some() {
                    overridden_keys.push("color_album_title".into());
                }
            }
            let font = string(&mut root, "album_pages_title_font", "")?;
            let size = number(&mut root, "album_pages_title_size", "", "points")?;
            let colour = NamedColour::parse(&string(&mut root, "color_album_title", "")?)
                .map_err(|error| format!("color_album_title: {error}"))?;
            let text = string(&mut title, "text", "native_pdf.title.")?;
            let vspace = number(
                &mut title,
                "vspace_after_mm",
                "native_pdf.title.",
                "millimetres",
            )?;
            let minimum = number(
                &mut title,
                "minimum_height_mm",
                "native_pdf.title.",
                "millimetres",
            )?;
            let command = AlbumPagesTitle::new(&font, size, &text, vspace, minimum, colour)?;
            if enabled {
                page_settings.set_title(command);
            }
            if !header.is_empty() {
                root.insert("header".into(), Value::Table(header));
            }
            if !title.is_empty() {
                native.insert("title".into(), Value::Table(title));
            }
        }
        if native.contains_key("border_colours") {
            let mut colours = table(&mut native, "border_colours", "native_pdf.")?;
            let mut parse_colour = |key| -> Result<AlbumBorderColour> {
                let name = string(&mut colours, key, "native_pdf.border_colours.")?;
                AlbumBorderColour::parse(&[&name])
                    .map_err(|error| format!("native_pdf.border_colours.{key}: {error}"))
            };
            page_settings.set_border_line_colours([
                parse_colour("outer")?,
                parse_colour("inner1")?,
                parse_colour("inner2")?,
            ]);
            if !colours.is_empty() {
                native.insert("border_colours".into(), Value::Table(colours));
            }
        }
        let mut proof_table = table(&mut native, "proof", "native_pdf.")?;
        let prefix = "native_pdf.proof.";
        let page_count = match proof_table.remove("page_count") {
            Some(Value::Integer(count)) => usize::try_from(count).map_err(|_| {
                "native_pdf.proof.page_count must be a positive PDF page count".to_string()
            })?,
            _ => {
                return Err(
                    "Missing or invalid native_pdf.proof.page_count; expected an integer".into(),
                )
            }
        };
        let guide_enabled = match proof_table.remove("guide_enabled") {
            Some(Value::Boolean(enabled)) => enabled,
            _ => {
                return Err(
                    "Missing or invalid native_pdf.proof.guide_enabled; expected a boolean".into(),
                )
            }
        };
        let guide_width_mm = numbers(&mut proof_table, &["guide_width_mm"], prefix)?[0];
        let guide_colour = NamedColour::parse(&string(&mut proof_table, "guide_colour", prefix)?)
            .map_err(|error| format!("{prefix}guide_colour: {error}"))?;
        let proof = GeometryProofSettings {
            page_count,
            guide_enabled,
            guide_width_mm,
            guide_colour,
        };
        proof.validate()?;
        if !proof_table.is_empty() {
            native.insert("proof".into(), Value::Table(proof_table));
        }
        if !native.is_empty() {
            root.insert("native_pdf".into(), Value::Table(native));
        }
        if let Some(value) = page.remove("even_margins") {
            let Value::Table(mut table) = value else {
                return Err("Native PDF setting page.even_margins must be a table".into());
            };
            page_settings.set_even_margins(EvenMargins::parse(&numbers(
                &mut table,
                &["left_mm", "right_mm", "top_mm", "bottom_mm"],
                "page.even_margins.",
            )?)?);
            if !table.is_empty() {
                page.insert("even_margins".into(), Value::Table(table));
            }
        }
        let pages = if numbering_enabled {
            vec![1, 2.min(proof.page_count), proof.page_count]
        } else {
            vec![1, 2]
        };
        let generation_date = page_settings.capture_generation_date();
        for number in pages {
            let layout = if numbering_enabled || generation_date.is_some() {
                // Legacy validation still checks both base/even geometry when
                // proof count is one; do not remove that existing safety gate.
                page_settings.resolve_with_context(
                    number,
                    proof.page_count.max(number),
                    generation_date,
                )
            } else {
                page_settings.resolve(number)
            }
            .map_err(|error| format!("Native PDF page {number}: {error}"))?;
            proof.validate_layout(&layout)?;
        }
        if !page.is_empty() {
            root.insert("page".into(), Value::Table(page));
        }
        if !colors.is_empty() {
            root.insert("colors".into(), Value::Table(colors));
        }
        let mut unapplied_keys = Vec::new();
        collect_keys(&root, "", &mut unapplied_keys);
        unapplied_keys.sort();
        overridden_keys.sort();
        Ok(Self {
            stamp_text,
            page_settings,
            proof,
            unapplied_keys,
            overridden_keys,
        })
    }
}

fn table(parent: &mut Table, key: &str, prefix: &str) -> Result<Table> {
    match parent.remove(key) {
        Some(Value::Table(table)) => Ok(table),
        _ => Err(format!("Missing or invalid native PDF table {prefix}{key}")),
    }
}

fn string(table: &mut Table, key: &str, prefix: &str) -> Result<String> {
    match table.remove(key) {
        Some(Value::String(value)) => Ok(value),
        _ => Err(format!(
            "Missing or invalid native PDF setting {prefix}{key}; expected a string"
        )),
    }
}

fn boolean(table: &mut Table, key: &str, prefix: &str) -> Result<bool> {
    match table.remove(key) {
        Some(Value::Boolean(value)) => Ok(value),
        _ => Err(format!(
            "Missing or invalid {prefix}{key}; expected a boolean"
        )),
    }
}

fn numbers(table: &mut Table, keys: &[&str], prefix: &str) -> Result<Vec<f64>> {
    keys.iter()
        .map(|key| number(table, key, prefix, "millimetres"))
        .collect()
}

fn number(table: &mut Table, key: &str, prefix: &str, unit: &str) -> Result<f64> {
    let value = table
        .remove(key)
        .ok_or_else(|| format!("Missing native PDF setting {prefix}{key}"))?;
    match value {
        Value::Float(value) => Ok(value),
        Value::Integer(value) => Ok(value as f64),
        _ => Err(format!(
            "Native PDF setting {prefix}{key} must be a number in {unit}"
        )),
    }
}

fn collect_keys(table: &Table, prefix: &str, keys: &mut Vec<String>) {
    for (name, value) in table {
        let path = format!("{prefix}{name}");
        if let Value::Table(children) = value {
            if children.is_empty() {
                keys.push(path);
            } else {
                collect_keys(children, &format!("{path}."), keys);
            }
        } else {
            keys.push(path);
        }
    }
}
