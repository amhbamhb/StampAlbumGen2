use super::{
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder, album_pages_margins::Margins,
        album_pages_marginse::EvenMargins, album_pages_size::PageSize,
        album_pages_spacing::Spacing,
    },
    document::PageSettings,
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
    pub unapplied_keys: Vec<String>,
    pub overridden_keys: Vec<String>,
}

impl NativePdfConfig {
    /// Reads exactly this path; no global config, upward search or default fallback.
    pub fn load(path: &Path) -> Result<Self> {
        let source = fs::read_to_string(path).map_err(|error| {
            format!("Cannot read native PDF config {}: {error}", path.display())
        })?;
        Self::from_toml(&source)
            .map_err(|error| format!("Native PDF config {}: {error}", path.display()))
    }

    pub fn from_toml(source: &str) -> Result<Self> {
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
        let mut page_settings = PageSettings::new(size, margins, spacing);
        if BORDER.iter().any(|key| root.contains_key(*key)) {
            page_settings.set_border(PageBorder::parse(&numbers(&mut root, &BORDER, "")?)?);
        }
        if let Some(value) = root.remove("color_album_border") {
            let Value::String(name) = value else {
                return Err(
                    "Native PDF setting color_album_border must be a named colour string".into(),
                );
            };
            page_settings.set_border_colour(AlbumBorderColour::parse(&[&name])?);
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
        for number in [1, 2] {
            page_settings
                .resolve(number)
                .map_err(|error| format!("Native PDF page {number}: {error}"))?;
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
        Ok(Self {
            page_settings,
            unapplied_keys,
            overridden_keys,
        })
    }
}

fn numbers(table: &mut Table, keys: &[&str], prefix: &str) -> Result<Vec<f64>> {
    keys.iter()
        .map(|key| {
            let value = table
                .remove(*key)
                .ok_or_else(|| format!("Missing native PDF setting {prefix}{key}"))?;
            match value {
                Value::Float(value) => Ok(value),
                Value::Integer(value) => Ok(value as f64),
                _ => Err(format!(
                    "Native PDF setting {prefix}{key} must be a number in millimetres"
                )),
            }
        })
        .collect()
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
