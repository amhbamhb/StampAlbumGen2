use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

use crate::config;

use super::{ConfigEditorField, ConfigEditorSection};

pub(super) fn update_default_catalog_code_for_album(
    album_dir: &std::path::Path,
    album_name: &str,
    global_config_path: &str,
    template_path: &str,
    requested_code: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let normalized = crate::catalog::normalize_catalog_type(requested_code);
    if normalized.is_empty() {
        return Err("default catalog code cannot be empty".into());
    }
    // Reject codes that look like injection attempts (only allow alphanumeric + underscore).
    if normalized.len() > 32
        || !normalized
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(format!("unsupported catalog code: {}", normalized).into());
    }

    // Load any existing per-album overrides so they are preserved.
    let album_override_path = album_dir.join("config.toml");
    let mut merged: HashMap<String, String> = if album_override_path.exists() {
        load_current_config_values(album_override_path.to_str().unwrap_or("")).unwrap_or_default()
    } else {
        HashMap::new()
    };

    // Insert the new code (unquoted – save_album_config_diff normalises it).
    merged.insert(
        "stamps.default_catalog_code".to_string(),
        normalized.clone(),
    );

    // Re-write the album override, keeping only fields that differ from global.
    save_album_config_diff(
        album_dir,
        album_name,
        global_config_path,
        template_path,
        &merged,
    )?;

    Ok(normalized)
}

fn config_field_id(section_id: &str, key: &str) -> String {
    if section_id.trim().is_empty() {
        key.trim().to_string()
    } else {
        format!("{}.{}", section_id.trim(), key.trim())
    }
}

fn friendly_config_section_title(section_id: &str) -> String {
    if section_id.trim().is_empty() {
        return "General".to_string();
    }

    section_id
        .split('.')
        .map(|part| {
            part.split('_')
                .filter(|segment| !segment.is_empty())
                .map(|segment| {
                    let mut chars = segment.chars();
                    match chars.next() {
                        Some(first) => {
                            format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
                        }
                        None => String::new(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" / ")
}

fn parse_config_key_value_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    let (key, value) = trimmed.split_once('=')?;
    let key = key.trim();
    if key.is_empty() || key.starts_with('[') {
        return None;
    }
    Some((key.to_string(), value.trim().to_string()))
}

fn load_example_config_help_map(path: &str) -> HashMap<String, Vec<String>> {
    let Ok(content) = fs::read_to_string(path) else {
        return HashMap::new();
    };

    let mut out: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_section_id = String::new();
    let mut pending_comments: Vec<String> = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let comment = trimmed.trim_start_matches('#').trim();
            if !comment.is_empty() {
                pending_comments.push(comment.to_string());
            }
            continue;
        }

        if trimmed.is_empty() {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section_id = trimmed
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            pending_comments.clear();
            continue;
        }

        if let Some((key, _)) = parse_config_key_value_line(trimmed) {
            if !pending_comments.is_empty() {
                let full_id = config_field_id(&current_section_id, &key).to_ascii_lowercase();
                let key_only = key.to_ascii_lowercase();
                out.entry(full_id)
                    .or_default()
                    .extend(pending_comments.clone());
                out.entry(key_only)
                    .or_default()
                    .extend(pending_comments.clone());
            }
            pending_comments.clear();
            continue;
        }

        pending_comments.clear();
    }

    out
}

fn enrich_field_help(field: &mut ConfigEditorField, help_map: &HashMap<String, Vec<String>>) {
    let full_id_key = field.id.to_ascii_lowercase();
    let key_only = field.key.to_ascii_lowercase();

    if let Some(extra_help) = help_map
        .get(&full_id_key)
        .or_else(|| help_map.get(&key_only))
    {
        for line in extra_help {
            if !field.comments.iter().any(|existing| existing == line) {
                field.comments.push(line.clone());
            }
        }
    }

    let mut add_unique = |line: &str| {
        if !field.comments.iter().any(|existing| existing == line) {
            field.comments.push(line.to_string());
        }
    };

    if key_only.ends_with("_font") {
        add_unique("AlbumEasy font code. Common values: HN=Helvetica Normal, HB=Helvetica Bold, TN=Times Normal, TB=Times Bold, TR=Times Roman, TI=Times Italic, BI=Bold Italic, CN=Courier Normal, CO=Courier Oblique.");
        add_unique("Tip: keep heading/body font families consistent across related fields to avoid visual mismatch.");
    }

    if key_only.ends_with("_size") {
        add_unique("Font size used by AlbumEasy text rendering. Decimal values are allowed (for example 8.0, 10.5). Larger sizes increase layout pressure and can cause wrapping.");
    }

    if key_only.ends_with("_vspace") || key_only.contains("padding") || key_only.contains("margin")
    {
        add_unique("Spacing value in millimeters (mm). Positive values increase vertical/horizontal whitespace; overly large values may reduce stamps-per-page.");
    }

    if key_only == "page_text_format"
        || key_only == "page_text_centre_format"
        || key_only == "page_text_right_format"
    {
        add_unique(
            "Template must include exactly one '{}' placeholder where the text value is injected.",
        );
        add_unique("Expected command shapes: PAGE_TEXT (<FONT> <SIZE> \"{}\" <VSPACE>), PAGE_TEXT_CENTRE (...), PAGE_TEXT_RIGHT (...).");
        add_unique("Example: PAGE_TEXT (HN 10 \"{}\" 1.0)");
    }

    if key_only == "stamp_heading_font"
        || key_only == "stamp_heading_size"
        || key_only == "stamp_heading_label"
    {
        add_unique(
            "Emitted heading uses STAMP_HEADING(<FONT> <SIZE> \"<LABEL> : <CATALOG_NUMBER>\").",
        );
        add_unique("stamp_heading_label='auto' uses catalog display name from default_catalog_code; empty string hides the label.");
    }

    if key_only == "default_catalog_code" {
        add_unique("Catalog family used for stamp heading and matching preference.");
        add_unique(
            "Common values: sn=Scott, sg=Stanley Gibbons, sw=StampWorld, mi=Michel, yt=Yvert, col=Colnect.",
        );
    }

    if key_only == "include_series" {
        add_unique("Controls the series title line shown above each stamp row.");
        add_unique("false hides the series heading such as '1970 New Civil Law'.");
    }

    if key_only == "include_date" {
        add_unique("Controls the date line shown above each stamp row.");
        add_unique("false hides issue dates such as 'January 13, 1970'.");
    }

    if key_only == "include_watermark" {
        add_unique("Controls the watermark line shown above each stamp row.");
        add_unique("false hides watermark text in the header area.");
    }

    if key_only == "include_engraving" {
        add_unique("Controls the engraving line shown above each stamp row.");
        add_unique("false hides engraving text in the header area.");
    }

    if key_only == "include_design" {
        add_unique("Controls the design line shown above each stamp row when present.");
        add_unique("false hides design text in the header area.");
    }

    if key_only == "include_perforation" {
        add_unique("Controls perforation text in the header area and in STAMP_HEADING.");
        add_unique("false hides perforation details above the image and from the per-stamp heading.");
    }

    if key_only == "include_catalog_numbers" {
        add_unique("Controls the STAMP_HEADING block above each image.");
        add_unique("false hides the catalog label/number heading entirely.");
    }

    if key_only == "stamp_detail_layout" {
        add_unique("inline: details inside stamp text blocks; separate_rows: details in dedicated rows; minimal: reduced metadata output.");
    }

    if key_only == "include_denomination" {
        add_unique("Controls the left value under each stamp image.");
        add_unique("false hides denomination/face-value fallback text in the stamp row.");
    }

    if key_only == "include_stamps_issued" {
        add_unique("Controls the centre-top quantity value under each stamp image.");
        add_unique("false hides stamps-issued / print-run values in the stamp row.");
    }

    if key_only == "include_description" {
        add_unique("Controls the centre description text under each stamp image.");
        add_unique("false hides the main stamp description/body text in the stamp row.");
    }

    if key_only == "include_color" {
        add_unique("Controls the colour line under each stamp image.");
        add_unique("false hides color text such as multicoloured in the stamp row.");
    }

    if key_only == "include_mint_unused" {
        add_unique("Controls the first right-side value under each stamp image.");
        add_unique("false hides mint (or unused fallback) in the stamp row.");
    }

    if key_only == "include_used" {
        add_unique("Controls the second right-side value under each stamp image.");
        add_unique("false hides used value/condition in the stamp row.");
    }

    if key_only == "row_start_es_enabled"
        || key_only == "row_start_es_font"
        || key_only == "row_start_es_size"
        || key_only == "row_start_es_spacing"
        || key_only == "row_start_es_vspace"
        || key_only == "row_start_es_marker"
    {
        add_unique("Controls ROW_START_ES marker output used to anchor row layout in generated AlbumEasy text.");
        add_unique("Disable row_start_es_enabled only if your output pipeline does not rely on row-start markers.");
    }

    if key_only == "albumeasy_cli_path" {
        add_unique("Absolute path to AlbumEasy CLI script/binary used for PDF generation from generated TXT.");
        add_unique("Linux example: /home/<user>/work/AlbumEasy/AlbumEasy.sh");
    }

    if key_only == "albumeasy_auto_generate_pdf" {
        add_unique(
            "When true, tool runs '<albumeasy_cli_path> -c <generated_txt>' after generation.",
        );
        add_unique("Requires albumeasy_cli_path to be configured.");
    }

    if key_only == "expand_grouped_ranges" {
        add_unique("placeholders: split grouped catalog ranges into placeholder rows quickly; detail_fetch: attempt richer detail fetch per item (slower)." );
    }

    if key_only.starts_with("flag_cover_") || full_id_key.starts_with("flag_cover.") {
        add_unique("Flag cover values control cover-page image/text layout and are interpreted in millimeters for size/spacing fields.");
    }
}

fn template_literal_kind(template_value: &str) -> &'static str {
    let t = template_value.trim();
    if t.eq_ignore_ascii_case("true") || t.eq_ignore_ascii_case("false") {
        return "bool";
    }
    if t.parse::<f64>().is_ok() {
        return "number";
    }
    if (t.starts_with('"') && t.ends_with('"')) || (t.starts_with('\'') && t.ends_with('\'')) {
        return "string";
    }
    if t.starts_with('[') && t.ends_with(']') {
        return "array";
    }
    if t.starts_with('{') && t.ends_with('}') {
        return "inline_table";
    }
    "unknown"
}

fn quote_toml_basic_string(input: &str) -> String {
    let escaped = input.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

fn normalize_value_for_template(template_value: Option<&String>, raw_value: &str) -> String {
    let trimmed = raw_value.trim();

    if trimmed.is_empty() {
        if let Some(template) = template_value {
            if template_literal_kind(template) == "string" {
                return "\"\"".to_string();
            }
        }
        return String::new();
    }

    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        return trimmed.to_string();
    }

    if let Some(template) = template_value {
        match template_literal_kind(template) {
            "bool" => {
                if trimmed.eq_ignore_ascii_case("true") {
                    return "true".to_string();
                }
                if trimmed.eq_ignore_ascii_case("false") {
                    return "false".to_string();
                }
                return trimmed.to_string();
            }
            "number" => {
                return trimmed.to_string();
            }
            "string" => {
                return quote_toml_basic_string(trimmed);
            }
            _ => {}
        }
    }

    if trimmed.eq_ignore_ascii_case("true") || trimmed.eq_ignore_ascii_case("false") {
        return trimmed.to_ascii_lowercase();
    }
    if trimmed.parse::<f64>().is_ok() {
        return trimmed.to_string();
    }
    if (trimmed.starts_with('[') && trimmed.ends_with(']'))
        || (trimmed.starts_with('{') && trimmed.ends_with('}'))
    {
        return trimmed.to_string();
    }

    quote_toml_basic_string(trimmed)
}

fn ensure_config_section<'a>(
    sections: &'a mut Vec<ConfigEditorSection>,
    section_id: &str,
    section_comments: Vec<String>,
) -> &'a mut ConfigEditorSection {
    if let Some(pos) = sections.iter().position(|section| section.id == section_id) {
        return &mut sections[pos];
    }

    sections.push(ConfigEditorSection {
        id: section_id.to_string(),
        title: friendly_config_section_title(section_id),
        comments: section_comments,
        fields: Vec::new(),
    });
    sections.last_mut().expect("section just pushed")
}

fn parse_template_config_sections(template_content: &str) -> Vec<ConfigEditorSection> {
    let mut sections: Vec<ConfigEditorSection> = Vec::new();
    let mut current_section_id = String::new();
    let mut pending_comments: Vec<String> = Vec::new();

    for line in template_content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('#') {
            let comment = trimmed.trim_start_matches('#').trim();
            if !comment.is_empty() {
                pending_comments.push(comment.to_string());
            }
            continue;
        }

        if trimmed.is_empty() {
            pending_comments.clear();
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section_id = trimmed
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            let section_comments = std::mem::take(&mut pending_comments);
            let _ = ensure_config_section(&mut sections, &current_section_id, section_comments);
            continue;
        }

        if let Some((key, value)) = parse_config_key_value_line(trimmed) {
            let field_comments = std::mem::take(&mut pending_comments);
            let section = ensure_config_section(&mut sections, &current_section_id, Vec::new());
            section.fields.push(ConfigEditorField {
                id: config_field_id(&current_section_id, &key),
                section_id: current_section_id.clone(),
                section_title: section.title.clone(),
                key,
                template_value: value.clone(),
                current_value: value.clone(),
                global_value: value,
                is_album_override: false,
                comments: field_comments,
                from_template: true,
            });
            continue;
        }

        pending_comments.clear();
    }

    sections.retain(|section| !section.fields.is_empty());
    sections
}

fn flatten_toml_value_into_map(
    prefix: &str,
    value: &toml::Value,
    out: &mut HashMap<String, String>,
) {
    match value {
        toml::Value::Table(table) => {
            for (key, entry) in table {
                let next_prefix = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", prefix, key)
                };
                flatten_toml_value_into_map(&next_prefix, entry, out);
            }
        }
        _ => {
            out.insert(prefix.to_string(), value.to_string());
        }
    }
}

pub(super) fn load_current_config_values(
    config_path: &str,
) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(config_path)
        .map_err(|e| format!("failed to read {}: {}", config_path, e))?;
    let parsed: toml::Value =
        toml::from_str(&content).map_err(|e| format!("failed to parse {}: {}", config_path, e))?;
    let mut values = HashMap::new();
    flatten_toml_value_into_map("", &parsed, &mut values);
    Ok(values)
}

pub(super) fn build_config_editor_data(
    config_path: &str,
    template_path: &str,
) -> Result<Vec<ConfigEditorSection>, Box<dyn std::error::Error>> {
    config::ensure_config_and_template_files(config_path, template_path)?;
    let template_content = config::load_template_content_with_fallback(template_path)?;
    let mut template_sections = parse_template_config_sections(&template_content);
    let example_help_map = load_example_config_help_map("config_full_example.toml");
    let mut current_values = load_current_config_values(config_path)?;
    // Normalize legacy root-level keys into their sectioned template IDs.
    // This prevents these fields from showing as "Preserved" and ensures
    // save operations keep them in the documented [stamps] section.
    let legacy_stamp_aliases = [
        ("include_denomination", "stamps.include_denomination"),
        ("include_stamps_issued", "stamps.include_stamps_issued"),
        ("include_description", "stamps.include_description"),
        ("include_color", "stamps.include_color"),
        ("include_mint_unused", "stamps.include_mint_unused"),
        ("include_used", "stamps.include_used"),
    ];
    for (legacy_key, sectioned_key) in legacy_stamp_aliases {
        if let Some(v) = current_values.remove(legacy_key) {
            current_values.entry(sectioned_key.to_string()).or_insert(v);
        }
    }
    let mut used_field_ids = HashSet::new();

    for section in &mut template_sections {
        for field in &mut section.fields {
            if let Some(current_value) = current_values.get(&field.id) {
                field.current_value = current_value.clone();
            }
            enrich_field_help(field, &example_help_map);
            used_field_ids.insert(field.id.clone());
        }
    }

    let mut extra_sections: BTreeMap<String, Vec<ConfigEditorField>> = BTreeMap::new();
    for (field_id, current_value) in current_values {
        if used_field_ids.contains(&field_id) {
            continue;
        }

        let mut parts = field_id.rsplitn(2, '.');
        let key = parts.next().unwrap_or_default().to_string();
        let section_id = parts.next().unwrap_or_default().to_string();
        extra_sections
            .entry(section_id.clone())
            .or_default()
            .push(ConfigEditorField {
                id: field_id.clone(),
                section_id: section_id.clone(),
                section_title: friendly_config_section_title(&section_id),
                key,
                template_value: current_value.clone(),
                current_value: current_value.clone(),
                global_value: current_value,
                is_album_override: false,
                comments: vec!["Preserved from the current config file. No template comment is available for this key.".to_string()],
                from_template: false,
            });
    }

    for (section_id, mut fields) in extra_sections {
        fields.sort_by(|left, right| left.key.cmp(&right.key));
        if let Some(existing_idx) = template_sections
            .iter()
            .position(|section| section.id == section_id)
        {
            for field in &mut fields {
                enrich_field_help(field, &example_help_map);
            }
            template_sections[existing_idx].fields.extend(fields);
            if !template_sections[existing_idx]
                .comments
                .iter()
                .any(|c| c.contains("not documented in config_template.toml"))
            {
                template_sections[existing_idx].comments.push(
                    "Some settings in this section exist in config.toml but are not documented in config_template.toml. They will be preserved on save.".to_string(),
                );
            }
        } else {
            template_sections.push(ConfigEditorSection {
                id: section_id.clone(),
                title: if section_id.is_empty() {
                    "Additional".to_string()
                } else {
                    format!("{} (Preserved)", friendly_config_section_title(&section_id))
                },
                comments: vec!["These settings exist in the active config.toml but are not documented in config_template.toml. They will be preserved on save.".to_string()],
                fields,
            });
        }
    }

    Ok(template_sections)
}

fn render_config_from_template_values(
    template_content: &str,
    values: &HashMap<String, String>,
) -> String {
    fn is_multiline_array_start(template_value: &str) -> bool {
        let t = template_value.trim();
        t == "[" || (t.starts_with('[') && !t.contains(']'))
    }

    let mut rendered_lines: Vec<String> = Vec::new();
    let mut current_section_id = String::new();
    let mut template_field_ids = HashSet::new();
    let mut template_section_ids = HashSet::new();

    let template_lines: Vec<&str> = template_content.lines().collect();
    let mut idx = 0usize;
    while idx < template_lines.len() {
        let line = template_lines[idx];
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section_id = trimmed
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            template_section_ids.insert(current_section_id.clone());
            rendered_lines.push(line.to_string());
            idx += 1;
            continue;
        }

        if let Some((key, original_value)) = parse_config_key_value_line(line) {
            let field_id = config_field_id(&current_section_id, &key);
            template_field_ids.insert(field_id.clone());
            let value = values
                .get(&field_id)
                .cloned()
                .unwrap_or_else(|| original_value.clone())
                .trim()
                .to_string();
            rendered_lines.push(format!("{} = {}", key, value));

            if is_multiline_array_start(&original_value) {
                idx += 1;
                while idx < template_lines.len() {
                    let next_trimmed = template_lines[idx].trim();
                    if next_trimmed.starts_with(']') {
                        break;
                    }
                    idx += 1;
                }
            }

            idx += 1;
            continue;
        }

        rendered_lines.push(line.to_string());
        idx += 1;
    }

    let mut extra_sections_new_tables: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    let mut extra_dotted_keys: Vec<(String, String)> = Vec::new();
    for (field_id, value) in values {
        if template_field_ids.contains(field_id) {
            continue;
        }

        let mut parts = field_id.rsplitn(2, '.');
        let key = parts.next().unwrap_or_default().to_string();
        let section_id = parts.next().unwrap_or_default().to_string();
        let trimmed_value = value.trim().to_string();
        if section_id.is_empty() {
            extra_dotted_keys.push((key, trimmed_value));
        } else if template_section_ids.contains(&section_id) {
            extra_dotted_keys.push((format!("{}.{}", section_id, key), trimmed_value));
        } else {
            extra_sections_new_tables
                .entry(section_id)
                .or_default()
                .push((key, trimmed_value));
        }
    }

    if !extra_dotted_keys.is_empty() || !extra_sections_new_tables.is_empty() {
        if rendered_lines
            .last()
            .map(|line| !line.is_empty())
            .unwrap_or(false)
        {
            rendered_lines.push(String::new());
        }
        rendered_lines.push("###################################".to_string());
        rendered_lines.push("# Preserved Extra Keys".to_string());
        rendered_lines.push("###################################".to_string());

        extra_dotted_keys.sort_by(|left, right| left.0.cmp(&right.0));
        for (full_key, value) in extra_dotted_keys {
            rendered_lines.push(format!("{} = {}", full_key, value));
        }

        for (section_id, mut fields) in extra_sections_new_tables {
            fields.sort_by(|left, right| left.0.cmp(&right.0));
            rendered_lines.push(String::new());
            if !section_id.trim().is_empty() {
                rendered_lines.push(format!("[{}]", section_id));
            }
            for (key, value) in fields {
                rendered_lines.push(format!("{} = {}", key, value));
            }
        }
    }

    let mut output = rendered_lines.join("\n");
    output.push('\n');
    output
}

pub(super) fn save_full_config_values(
    config_path: &str,
    template_path: &str,
    values: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if values.is_empty() {
        return Err("no config values were provided".into());
    }

    config::ensure_config_and_template_files(config_path, template_path)?;
    let template_content = config::load_template_content_with_fallback(template_path)?;
    let template_sections = parse_template_config_sections(&template_content);
    let mut template_defaults_by_id: HashMap<String, String> = HashMap::new();
    for section in &template_sections {
        for field in &section.fields {
            template_defaults_by_id.insert(field.id.clone(), field.template_value.clone());
        }
    }

    let mut normalized_values: HashMap<String, String> = HashMap::new();
    for (field_id, value) in values {
        let normalized = normalize_value_for_template(template_defaults_by_id.get(field_id), value);
        normalized_values.insert(field_id.clone(), normalized);
    }

    let rendered = render_config_from_template_values(&template_content, &normalized_values);

    toml::from_str::<toml::Value>(&rendered)
        .map_err(|e| format!("generated config is not valid TOML: {}", e))?;

    fs::write(config_path, rendered)
        .map_err(|e| format!("failed to write {}: {}", config_path, e))?;
    Ok(())
}

/// Build config editor data for a country review UI session.
/// Returns `(sections, has_album_override)` where sections have `current_value`
/// set to the merged (effective) value and `global_value` / `is_album_override`
/// populated correctly.
pub(super) fn build_config_editor_data_for_album(
    global_config_path: &str,
    template_path: &str,
    album_dir: &std::path::Path,
) -> Result<(Vec<ConfigEditorSection>, bool), Box<dyn std::error::Error>> {
    // Base: load sections with global config values.
    let mut sections = build_config_editor_data(global_config_path, template_path)?;

    // Set global_value = current_value (baseline before album overlay).
    for section in &mut sections {
        for field in &mut section.fields {
            field.global_value = field.current_value.clone();
        }
    }

    // Check whether an album override file exists.
    let override_path = album_dir.join("config.toml");
    if !override_path.exists() {
        return Ok((sections, false));
    }

    // Load album override raw key/value pairs.
    let override_values = match load_current_config_values(override_path.to_str().unwrap_or("")) {
        Ok(v) => v,
        Err(e) => {
            log::warn!(
                "Could not read album config override '{}': {}",
                override_path.display(),
                e
            );
            return Ok((sections, true));
        }
    };

    // Overlay: for each field whose id appears in the album override, update
    // current_value and mark as album_override.
    for section in &mut sections {
        for field in &mut section.fields {
            if let Some(album_val) = override_values.get(&field.id) {
                field.current_value = album_val.clone();
                field.is_album_override = true;
            }
        }
    }

    Ok((sections, true))
}

/// Write a minimal album-local config.toml that stores only the fields that
/// differ from the global config.  The file is written into `album_dir`.
/// Returns the number of fields written (0 = nothing changed vs global).
pub(super) fn save_album_config_diff(
    album_dir: &std::path::Path,
    album_name: &str,
    global_config_path: &str,
    template_path: &str,
    all_ui_values: &HashMap<String, String>,
) -> Result<usize, Box<dyn std::error::Error>> {
    // Get template section data for normalization.
    let template_content = config::load_template_content_with_fallback(template_path)?;
    let template_sections = parse_template_config_sections(&template_content);
    let mut template_defaults_by_id: HashMap<String, String> = HashMap::new();
    for section in &template_sections {
        for field in &section.fields {
            template_defaults_by_id.insert(field.id.clone(), field.template_value.clone());
        }
    }

    // Normalize all submitted values.
    let mut normalized: HashMap<String, String> = HashMap::new();
    for (field_id, raw_value) in all_ui_values {
        let n = normalize_value_for_template(template_defaults_by_id.get(field_id), raw_value);
        normalized.insert(field_id.clone(), n);
    }

    // Load the global config current values for comparison.
    let global_values: HashMap<String, String> =
        config::ensure_config_and_template_files(global_config_path, template_path)
            .ok()
            .and_then(|_| load_current_config_values(global_config_path).ok())
            .unwrap_or_default();

    // Collect only the fields that differ from global.
    // Group by template section_id so we write proper section headers.
    let mut diff_by_section: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for (field_id, new_val) in &normalized {
        let global_val = global_values
            .get(field_id)
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let new_trimmed = new_val.trim().to_string();
        if new_trimmed == global_val {
            continue; // unchanged vs global — skip
        }
        // field_id is like "stamps.default_catalog_code"
        let mut parts = field_id.rsplitn(2, '.');
        let key = parts.next().unwrap_or_default().to_string();
        let section_id = parts.next().unwrap_or_default().to_string();
        diff_by_section
            .entry(section_id)
            .or_default()
            .push((key, new_trimmed));
    }

    let total: usize = diff_by_section.values().map(|v| v.len()).sum();
    if total == 0 {
        // Delete the override file if it now matches global exactly.
        let override_path = album_dir.join("config.toml");
        if override_path.exists() {
            let _ = fs::remove_file(&override_path);
        }
        return Ok(0);
    }

    // Build TOML content with section headers matching the template.
    let mut lines = vec![
        format!(
            "# Stamp Album Generator 2 album config override: {}",
            album_name
        ),
        "# Only values that differ from the global config.toml are stored here.".to_string(),
        "# Settings not listed here fall back to the global config.toml.".to_string(),
        String::new(),
    ];

    for (section_id, mut fields) in diff_by_section {
        fields.sort_by(|a, b| a.0.cmp(&b.0));
        if !section_id.is_empty() {
            lines.push(format!("[{}]", section_id));
        }
        for (key, val) in fields {
            lines.push(format!("{} = {}", key, val));
        }
        lines.push(String::new());
    }

    let content = lines.join("\n");
    // Validate before writing.
    toml::from_str::<toml::Value>(&content)
        .map_err(|e| format!("generated album override is not valid TOML: {}", e))?;

    fs::create_dir_all(album_dir).map_err(|e| {
        format!(
            "failed to create album dir '{}': {}",
            album_dir.display(),
            e
        )
    })?;
    let override_path = album_dir.join("config.toml");
    fs::write(&override_path, content).map_err(|e| {
        format!(
            "failed to write album config override '{}': {}",
            override_path.display(),
            e
        )
    })?;

    Ok(total)
}

pub(super) fn render_config_editor_assets() -> String {
    r#"
<style>
.cfgBackdrop { position: fixed; inset: 0; background: radial-gradient(circle at 20% 20%, rgba(12, 90, 123, 0.25) 0%, rgba(4, 20, 30, 0.7) 48%, rgba(4, 16, 25, 0.86) 100%); display: none; align-items: flex-start; justify-content: center; z-index: 2000; padding: 16px; padding-top: 24px; backdrop-filter: blur(2px); }
.cfgDialog { --cfgPrimary: #0b6a87; --cfgPrimaryHover: #09576f; --cfgTextStrong: #14384a; --cfgTextSoft: #3b5f70; --cfgPanel: #f4f9fc; --cfgPanelDeep: #e8f1f7; width: min(1280px, 96vw); max-height: 92vh; display: flex; flex-direction: column; background: linear-gradient(180deg, #ffffff 0%, #f7fbfe 100%); border: 1px solid #bfd0dc; border-radius: 16px; box-shadow: 0 26px 62px rgba(10, 28, 41, 0.45); overflow: hidden; animation: cfgDialogIn 180ms ease-out; }
.cfgHeader, .cfgFooter { display: flex; align-items: center; justify-content: space-between; gap: 10px; padding: 12px 16px; background: linear-gradient(180deg, #f4fafd 0%, #e9f3f9 100%); border-bottom: 1px solid #d3e0ea; }
.cfgFooter { border-top: 1px solid #e0e8ef; border-bottom: none; }
.cfgTitleBlock h3 { margin: 0; font-size: 20px; letter-spacing: 0.2px; color: var(--cfgTextStrong); }
.cfgTitleBlock p { margin: 4px 0 0 0; font-size: 13px; color: var(--cfgTextSoft); }
.cfgActionRow { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.cfgToolbar { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; padding: 12px 16px; border-bottom: 1px solid #d8e4ec; background: linear-gradient(180deg, #ffffff 0%, #f3f8fb 100%); }
.cfgToolbar input { min-width: 260px; border: 1px solid #bfd0dd; border-radius: 999px; padding: 8px 12px; background: #ffffff; color: #1f4358; }
.cfgToolbar input:focus { outline: 2px solid rgba(11, 106, 135, 0.18); border-color: #0b6a87; }
.cfgBody { overflow: auto; padding: 14px 16px 18px; background: linear-gradient(180deg, #f7fbfe 0%, #eef5fa 100%); }
.cfgLoadState { padding: 24px 10px; color: #36546a; font-size: 14px; }
.cfgSection { border: 1px solid #ccdae4; border-radius: 12px; background: #ffffff; box-shadow: 0 8px 20px rgba(18, 46, 64, 0.09); margin-bottom: 14px; overflow: hidden; }
.cfgSectionHead { display: flex; align-items: center; justify-content: space-between; gap: 10px; padding: 12px 14px; background: linear-gradient(90deg, var(--cfgPanel) 0%, var(--cfgPanelDeep) 100%); border-bottom: 1px solid #d4e0e9; }
.cfgSectionHead h4 { margin: 0; font-size: 15px; color: #163a4d; }
.cfgSectionHead p { margin: 4px 0 0 0; font-size: 12px; color: #406072; }
.cfgFieldRow { display: grid; grid-template-columns: minmax(340px, 42%) minmax(320px, 1fr); gap: 14px; padding: 14px; border-top: 1px solid #edf2f6; }
.cfgFieldRow:first-child { border-top: none; }
.cfgStaticPane { display: flex; flex-direction: column; gap: 8px; }
.cfgEditPane { display: flex; flex-direction: column; gap: 6px; }
.cfgKeyRow { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
.cfgKeyRow strong { font-size: 14px; color: #173b4e; }
.cfgTemplateBox { border: 1px solid #d2dee8; border-radius: 10px; background: #f6fbff; padding: 10px; font-size: 12px; color: #355267; }
.cfgTemplateBox code { display: block; white-space: pre-wrap; word-break: break-word; background: #ffffff; border: 1px solid #d7e2eb; border-radius: 8px; padding: 8px; margin-top: 6px; }
.cfgInfoDetail { border: 1px solid #d7e3ec; border-radius: 10px; background: #fbfdff; }
.cfgInfoDetail summary { cursor: pointer; padding: 8px 10px; font-size: 12px; font-weight: 600; color: #284b64; }
.cfgInfoBody { padding: 0 10px 10px; font-size: 12px; color: #46657a; white-space: pre-wrap; }
.cfgValueLabel { font-size: 12px; font-weight: 600; color: #284b64; }
.cfgValueInput { width: 100%; min-height: 40px; padding: 8px 10px; border: 1px solid #cfdbe6; border-radius: 10px; font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; font-size: 12px; line-height: 1.45; background: #ffffff; box-sizing: border-box; }
.cfgValueInput[type='number'] { font-family: ui-sans-serif, system-ui, sans-serif; }
.cfgValueInput:focus { outline: 2px solid rgba(11, 106, 135, 0.2); border-color: #0b6a87; }
.cfgHelpText { margin-top: 6px; font-size: 11px; color: #46657a; line-height: 1.35; }
.cfgTypeTag { background: #f0f7fb; color: #1f4d63; border: 1px solid #c8dce9; border-radius: 999px; font-size: 11px; padding: 2px 8px; }
.cfgDialog .badge { background: #e8f2f9; border: 1px solid #cbdae5; color: #1f4d63; }
.cfgAlbumBadge { background: #fff0dd !important; border-color: #e8c08b !important; color: #7a4a0d !important; }
.cfgAlbumBanner { display: none; border: 1px solid #d9c59e; background: linear-gradient(180deg, #fff8eb 0%, #fff2dd 100%); color: #6d470f; border-radius: 999px; padding: 6px 10px; font-size: 12px; }
.cfgSaveTarget { display: none; align-items: center; gap: 8px; font-size: 12px; color: #35556a; }
.cfgSaveTarget select { border: 1px solid #bfd0dd; border-radius: 8px; padding: 6px 8px; color: #1f4358; background: #ffffff; }
.cfgPrimaryBtn, .cfgSecondaryBtn { border: none; border-radius: 10px; padding: 8px 13px; cursor: pointer; font-weight: 600; font-size: 13px; transition: transform 120ms ease, filter 120ms ease, box-shadow 120ms ease; }
.cfgPrimaryBtn { background: linear-gradient(180deg, var(--cfgPrimary, #0b6a87) 0%, #0a5f7a 100%); color: #ffffff; box-shadow: 0 4px 12px rgba(11, 106, 135, 0.25); }
.cfgPrimaryBtn:hover { filter: brightness(1.04); transform: translateY(-1px); }
.cfgSecondaryBtn { background: #e9f1f6; color: #214356; border: 1px solid #c7d7e2; }
.cfgSecondaryBtn:hover { background: #dbe8f1; }
.cfgFooterNote { font-size: 12px; color: #3f6071; max-width: 820px; }
.cfgConfirmDialog { padding: 18px 20px; border-radius: 14px; border: 1px solid #bfd1de; min-width: 340px; max-width: 530px; box-shadow: 0 18px 42px rgba(10, 28, 41, 0.35); background: linear-gradient(180deg, #ffffff 0%, #f4f9fc 100%); }
.cfgConfirmDialog::backdrop { background: rgba(4, 20, 30, 0.55); }
.cfgConfirmTitle { margin: 0 0 10px; font-size: 17px; color: #12364a; }
.cfgConfirmBody { margin: 0 0 14px; color: #406072; font-size: 13px; line-height: 1.45; }
.cfgConfirmActions { display: flex; gap: 8px; justify-content: flex-end; flex-wrap: wrap; }
.cfgDangerBtn { border: none; border-radius: 10px; padding: 8px 12px; cursor: pointer; font-weight: 600; color: #ffffff; background: linear-gradient(180deg, #b45342 0%, #943828 100%); }
.cfgHidden { display: none !important; }
.cfgSectionBody { display: block; }
.cfgSection.collapsed .cfgSectionBody { display: none; }
.cfgSectionToggle { display: inline-flex; align-items: center; justify-content: center; width: 22px; height: 22px; border-radius: 6px; background: none; border: none; cursor: pointer; color: #284b64; font-size: 13px; transition: transform 180ms ease; flex-shrink: 0; }
.cfgSection.collapsed .cfgSectionToggle { transform: rotate(-90deg); }
.cfgSectionHead { cursor: pointer; }
.cfgSectionHead:hover { background: linear-gradient(90deg, #e8f2f8 0%, #ddeaf3 100%); }
@keyframes cfgDialogIn {
    from { opacity: 0; transform: translateY(7px) scale(0.985); }
    to { opacity: 1; transform: translateY(0) scale(1); }
}
@media (max-width: 960px) {
  .cfgFieldRow { grid-template-columns: 1fr; }
  .cfgDialog { width: min(96vw, 96vw); }
  .cfgToolbar input { min-width: 180px; width: 100%; }
    .cfgHeader, .cfgFooter { align-items: flex-start; }
}
</style>

<div id='configEditorBackdrop' class='cfgBackdrop'>
    <div class='cfgDialog'>
        <div class='cfgHeader'>
            <div class='cfgTitleBlock'>
                <h3 id='cfgEditorTitle'>Edit config.toml</h3>
                <p>Static template defaults and comments are on the left. Editable current values are on the right.</p>
            </div>
            <div class='cfgActionRow'>
                <button class='cfgPrimaryBtn' onclick='saveConfigEditorChanges()'>OK / Save</button>
                <button class='cfgSecondaryBtn' onclick='requestCloseConfigEditor()'>Cancel</button>
            </div>
        </div>
        <div class='cfgToolbar'>
            <span id='cfgEditorStatus' class='badge'>Ready</span>
            <span id='cfgEditorFieldCount' class='badge'>Fields: 0</span>
            <span id='cfgAlbumBanner' class='cfgAlbumBanner'></span>
            <button id='cfgDeleteAlbumBtn' class='cfgSecondaryBtn cfgHidden' onclick='deleteAlbumConfigOverride()'>Delete Album Override</button>
            <input id='cfgSearchInput' type='text' placeholder='Filter by section, key, or comment' oninput='filterConfigEditorFields()' />
        </div>
        <div id='cfgEditorBody' class='cfgBody'>
            <div id='cfgEditorLoadState' class='cfgLoadState'>Load config details to start editing.</div>
            <div id='cfgEditorSections'></div>
        </div>
        <div class='cfgFooter'>
            <div class='cfgFooterNote'>Saving validates the generated config and reloads the review UI so new values take effect.</div>
            <div class='cfgActionRow'>
                <div id='cfgSaveTargetBlock' class='cfgSaveTarget'>
                    <label for='cfgSaveTarget'>Save target:</label>
                    <select id='cfgSaveTarget' onchange='onConfigSaveTargetChanged()'>
                        <option value='album'>Album override</option>
                        <option value='global'>Global config.toml</option>
                    </select>
                </div>
                <button class='cfgPrimaryBtn' onclick='saveConfigEditorChanges()'>OK / Save</button>
                <button class='cfgSecondaryBtn' onclick='requestCloseConfigEditor()'>Cancel</button>
            </div>
        </div>
    </div>
</div>

<script>
const configEditorState = { open: false, loading: false, saving: false, sections: [], originalValues: {}, albumName: '', hasAlbumOverride: false, saveTarget: 'global' };
function cfgHtml(v) { return String(v).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;').replaceAll('"','&quot;'); }
function configEditorBackdrop() { return document.getElementById('configEditorBackdrop'); }
function configEditorSectionsHost() { return document.getElementById('cfgEditorSections'); }
function configEditorLoadState() { return document.getElementById('cfgEditorLoadState'); }
function configEditorStatusBadge() { return document.getElementById('cfgEditorStatus'); }
function configEditorFieldCountBadge() { return document.getElementById('cfgEditorFieldCount'); }
function configEditorSearchInput() { return document.getElementById('cfgSearchInput'); }
function configEditorTitle() { return document.getElementById('cfgEditorTitle'); }
function configEditorAlbumBanner() { return document.getElementById('cfgAlbumBanner'); }
function configEditorSaveTargetBlock() { return document.getElementById('cfgSaveTargetBlock'); }
function configEditorSaveTargetSelect() { return document.getElementById('cfgSaveTarget'); }
function configEditorDeleteAlbumBtn() { return document.getElementById('cfgDeleteAlbumBtn'); }
function setConfigEditorStatus(message) {
    const badge = configEditorStatusBadge();
    if (badge) badge.innerText = message;
    if (typeof window.setConfigStatus === 'function') {
        window.setConfigStatus(message);
        return;
    }
    const statusEl = document.getElementById('status');
    if (statusEl) statusEl.innerText = message;
}
function onConfigSaveTargetChanged() { const el = configEditorSaveTargetSelect(); if (!el) return; configEditorState.saveTarget = el.value === 'album' ? 'album' : 'global'; const targetName = configEditorState.saveTarget === 'album' ? `Album override (${configEditorState.albumName || 'current'})` : 'global config.toml'; setConfigEditorStatus(`Save target: ${targetName}`); }
function refreshConfigEditorModeUi() { const albumName = String(configEditorState.albumName || '').trim(); const inAlbumMode = albumName.length > 0; const titleEl = configEditorTitle(); if (titleEl) titleEl.innerText = inAlbumMode ? `Edit config for ${albumName}` : 'Edit config.toml'; const banner = configEditorAlbumBanner(); if (banner) { if (inAlbumMode) { banner.style.display = 'inline-flex'; banner.innerText = configEditorState.hasAlbumOverride ? `Album override active: ${albumName}` : `Using global config for ${albumName}`; } else { banner.style.display = 'none'; banner.innerText = ''; } } const saveTargetBlock = configEditorSaveTargetBlock(); if (saveTargetBlock) saveTargetBlock.style.display = inAlbumMode ? 'inline-flex' : 'none'; const saveTargetSelect = configEditorSaveTargetSelect(); if (saveTargetSelect) { if (!inAlbumMode) { configEditorState.saveTarget = 'global'; saveTargetSelect.value = 'global'; } else if (configEditorState.saveTarget !== 'global' && configEditorState.saveTarget !== 'album') { configEditorState.saveTarget = 'album'; saveTargetSelect.value = 'album'; } else { saveTargetSelect.value = configEditorState.saveTarget; } const albumOption = saveTargetSelect.querySelector("option[value='album']"); if (albumOption) albumOption.textContent = `Album override - ${albumName || 'current'}`; } const deleteBtn = configEditorDeleteAlbumBtn(); if (deleteBtn) deleteBtn.classList.toggle('cfgHidden', !(inAlbumMode && configEditorState.hasAlbumOverride)); }
function configFieldInputId(fieldId) { return 'cfg-input-' + String(fieldId || '').replaceAll('.', '__').replaceAll('/', '_'); }
function isBooleanValue(v) { const s = String(v ?? '').trim().toLowerCase(); return s === 'true' || s === 'false'; }
function isNumericValue(v) { return /^-?\d+(\.\d+)?$/.test(String(v ?? '').trim()); }
function isFontField(field) { const key = String(field?.key || '').toLowerCase(); const id = String(field?.id || '').toLowerCase(); return key.endsWith('_font') || id.endsWith('_font'); }
function configFieldKnownOptions(fieldId, key) {
    const id = String(fieldId || '').toLowerCase();
    const k = String(key || '').toLowerCase();
    if (id === 'stamps.stamp_detail_layout' || k === 'stamp_detail_layout') return ['inline', 'separate_rows', 'minimal'];
    if (id === 'header.album_pages_header_position' || id === 'footer.album_pages_footer_number_position' || k.endsWith('_position')) return ['L', 'R', 'C', 'LR', 'RL'];
    if (id === 'data_sources.expand_grouped_ranges' || k === 'expand_grouped_ranges') return ['placeholders', 'detail_fetch'];
    if (id === 'wikipedia.wikipedia_paragraph_alignment' || k.endsWith('_alignment')) return ['JUSTIFIED', 'LEFT', 'CENTER', 'RIGHT'];
    if (id === 'wikipedia.wikipedia_pagination_strategy' || k.endsWith('_pagination_strategy')) return ['balanced', 'conservative', 'strict'];
    return null;
}
function configFieldType(field) {
    const current = String(field?.current_value ?? '').trim();
    const template = String(field?.template_value ?? '').trim();
    if (isBooleanValue(current) || isBooleanValue(template)) return 'bool';
    if (isFontField(field)) return 'font';
    if (configFieldKnownOptions(field?.id, field?.key)) return 'enum';
    if (isNumericValue(current) || isNumericValue(template)) return 'number';
    return 'text';
}
function configFieldHelpHtml(field) {
    const comments = Array.isArray(field?.comments) ? field.comments : [];
    const notes = [];
    if (comments.length) notes.push(...comments);
    if (isFontField(field)) notes.push('AlbumEasy font codes often used: HN, HB, TN, TB, TR, TI, BI, CN, CO.');
    const type = configFieldType(field);
    if (type === 'bool') notes.push('Boolean field: choose true or false.');
    if (type === 'number') notes.push('Numeric field: enter a valid integer or decimal number.');
    if (!notes.length) return '';
    const text = notes.map(line => cfgHtml(line)).join('\n');
    return `<details class='cfgInfoDetail'><summary>Help</summary><div class='cfgInfoBody'>${text}</div></details>`;
}
function renderConfigValueInput(field) {
    const fieldId = String(field?.id || '');
    const inputId = configFieldInputId(fieldId);
    const current = String(field?.current_value ?? '');
    const type = configFieldType(field);
    const knownOptions = configFieldKnownOptions(field?.id, field?.key);
    const fontOptions = ['HN','HB','TN','TB','TR','TI','BI','CN','CO'];
    const normBool = String(current).trim().toLowerCase() === 'true' ? 'true' : 'false';

    if (type === 'bool') {
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'><option value='true' ${normBool === 'true' ? 'selected' : ''}>true</option><option value='false' ${normBool === 'false' ? 'selected' : ''}>false</option></select><div class='cfgHelpText'>Type: boolean</div>`;
    }

    if (type === 'font') {
        const options = fontOptions.map(opt => `<option value='${cfgHtml(opt)}' ${String(current).trim() === opt ? 'selected' : ''}>${cfgHtml(opt)}</option>`).join('');
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'>${options}</select><div class='cfgHelpText'>Type: font code</div>`;
    }

    if (type === 'enum' && Array.isArray(knownOptions) && knownOptions.length) {
        const fallbackOption = knownOptions.includes(current) ? '' : `<option value='${cfgHtml(current)}' selected>${cfgHtml(current || '(empty)')}</option>`;
        const options = knownOptions.map(opt => `<option value='${cfgHtml(opt)}' ${String(current).trim() === opt ? 'selected' : ''}>${cfgHtml(opt)}</option>`).join('');
        return `<select id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' oninput='markConfigEditorDirty()'>${fallbackOption}${options}</select><div class='cfgHelpText'>Type: option list</div>`;
    }

    if (type === 'number') {
        return `<input id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' type='number' step='any' value='${cfgHtml(current)}' oninput='markConfigEditorDirty()'/><div class='cfgHelpText'>Type: number</div>`;
    }

    return `<input id='${cfgHtml(inputId)}' class='cfgValueInput' data-field-id='${cfgHtml(fieldId)}' type='text' value='${cfgHtml(current)}' oninput='markConfigEditorDirty()'/>`;
}
function showConfigEditorLoading(message) { const loadState = configEditorLoadState(); const host = configEditorSectionsHost(); if (host) host.innerHTML = ''; if (loadState) { loadState.innerText = message; loadState.classList.remove('cfgHidden'); } }
function hideConfigEditorLoading() { const loadState = configEditorLoadState(); if (loadState) loadState.classList.add('cfgHidden'); }
function renderConfigEditorSections() {
    const host = configEditorSectionsHost();
    if (!host) return;

    const rawSections = Array.isArray(configEditorState.sections) ? configEditorState.sections : [];
    const startupFieldKeys = new Set(['albumeasy_cli_path']);
    const startupFields = [];
    const normalizedSections = [];

    rawSections.forEach(section => {
        const fields = Array.isArray(section?.fields) ? section.fields : [];
        const keepFields = [];
        fields.forEach(field => {
            const key = String(field?.key || '').trim();
            const id = String(field?.id || '').trim();
            const idTail = id.includes('.') ? id.split('.').pop() : id;
            if (startupFieldKeys.has(key) || startupFieldKeys.has(idTail)) {
                startupFields.push({ ...field, __startup_section_title: 'Startup Quick Settings' });
            } else {
                keepFields.push(field);
            }
        });
        if (keepFields.length) normalizedSections.push({ ...section, fields: keepFields });
    });

    const sections = [];
    if (startupFields.length) {
        sections.push({
            id: 'startup.quick',
            title: 'Startup Quick Settings',
            comments: ['Frequently-used startup values. This section is expanded by default.'],
            fields: startupFields,
            __defaultOpen: true,
        });
    }
    sections.push(...normalizedSections);

    let fieldCount = 0;
    host.innerHTML = sections
        .map(section => {
            const fields = Array.isArray(section.fields) ? section.fields : [];
            fieldCount += fields.length;
            const sectionId = cfgHtml(section.id || section.title || 'section');
            const defaultOpen = !!section.__defaultOpen;
            const collapsedClass = defaultOpen ? '' : ' collapsed';
            const sectionComments =
                Array.isArray(section.comments) && section.comments.length
                    ? `<p>${section.comments.map(line => cfgHtml(line)).join(' ')}</p>`
                    : '';
            const rows = fields
                .map(field => {
                    const searchCorpus = [
                        section.title || '',
                        field.key || '',
                        field.id || '',
                        ...(Array.isArray(field.comments) ? field.comments : []),
                    ]
                        .join(' ')
                        .toLowerCase();
                    const fieldType = configFieldType(field);
                    const albumBadge = field?.is_album_override
                        ? "<span class='badge cfgAlbumBadge'>album override</span>"
                        : '';
                    return `<div class='cfgFieldRow' data-config-search='${cfgHtml(searchCorpus)}'><div class='cfgStaticPane'><div class='cfgKeyRow'><strong>${cfgHtml(field.key || '')}</strong><span class='badge'>${cfgHtml(field.from_template === false ? 'Preserved key' : 'Template key')}</span>${albumBadge}<span class='cfgTypeTag'>${cfgHtml(fieldType)}</span></div><div class='cfgTemplateBox'><div>Template value</div><code>${cfgHtml(field.template_value || '')}</code></div>${configFieldHelpHtml(field)}</div><div class='cfgEditPane'><label class='cfgValueLabel' for='${cfgHtml(configFieldInputId(field.id))}'>Current value</label>${renderConfigValueInput(field)}</div></div>`;
                })
                .join('');
            return `<section class='cfgSection${collapsedClass}' data-section-id='${sectionId}' data-default-open='${defaultOpen ? '1' : '0'}'><div class='cfgSectionHead' onclick='toggleConfigSection(this.closest(".cfgSection"))'><div style='display:flex;align-items:center;gap:8px;'><button class='cfgSectionToggle' tabindex='-1' aria-label='Toggle section'>▼</button><div><h4>${cfgHtml(section.title || 'Section')}</h4>${sectionComments}</div></div><span class='badge'>Fields: ${fields.length}</span></div><div class='cfgSectionBody'>${rows}</div></section>`;
        })
        .join('');

    const badge = configEditorFieldCountBadge();
    if (badge) badge.innerText = `Fields: ${fieldCount}`;
    filterConfigEditorFields();
}
function toggleConfigSection(sectionEl) { if (!sectionEl) return; sectionEl.classList.toggle('collapsed'); }
function collectConfigEditorValues() { const values = {}; document.querySelectorAll('.cfgValueInput[data-field-id]').forEach(el => { values[el.getAttribute('data-field-id')] = String(el.value ?? ''); }); return values; }
function configEditorIsDirty() { const currentValues = collectConfigEditorValues(); const originalValues = configEditorState.originalValues || {}; const currentKeys = Object.keys(currentValues).sort(); const originalKeys = Object.keys(originalValues).sort(); if (currentKeys.length !== originalKeys.length) return true; for (let idx = 0; idx < currentKeys.length; idx += 1) { if (currentKeys[idx] !== originalKeys[idx]) return true; if (String(currentValues[currentKeys[idx]]) !== String(originalValues[currentKeys[idx]])) return true; } return false; }
function markConfigEditorDirty() { if (!configEditorState.open) return; setConfigEditorStatus(configEditorIsDirty() ? 'Config changes pending' : 'Config unchanged'); }
function openConfigEditorBackdrop() { const backdrop = configEditorBackdrop(); if (backdrop) backdrop.style.display = 'flex'; configEditorState.open = true; }
function closeConfigEditorBackdrop() { const backdrop = configEditorBackdrop(); if (backdrop) backdrop.style.display = 'none'; configEditorState.open = false; }
async function openConfigEditor() { if (configEditorState.loading || configEditorState.saving) return; configEditorState.loading = true; openConfigEditorBackdrop(); showConfigEditorLoading('Loading config template, comments, and current values...'); setConfigEditorStatus('Loading config editor...'); try { const response = await fetch('/api/config-editor'); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to load config editor data.'); configEditorState.sections = payload.sections || []; configEditorState.originalValues = {}; configEditorState.albumName = String(payload.album_name || ''); configEditorState.hasAlbumOverride = !!payload.has_album_override; configEditorState.saveTarget = configEditorState.albumName ? 'album' : 'global'; (payload.sections || []).forEach(section => { (section.fields || []).forEach(field => { configEditorState.originalValues[field.id] = String(field.current_value ?? ''); }); }); hideConfigEditorLoading(); renderConfigEditorSections(); refreshConfigEditorModeUi(); const searchInput = configEditorSearchInput(); if (searchInput) searchInput.value = ''; setConfigEditorStatus('Config editor ready'); } catch (error) { showConfigEditorLoading('Failed to load config editor: ' + error); setConfigEditorStatus('Config editor failed to load'); } finally { configEditorState.loading = false; } }
function filterConfigEditorFields() {
    const query = String(configEditorSearchInput()?.value || '').trim().toLowerCase();
    const sections = document.querySelectorAll('.cfgSection');
    sections.forEach(section => {
        const rows = section.querySelectorAll('.cfgFieldRow[data-config-search]');
        if (!query) {
            rows.forEach(row => row.classList.remove('cfgHidden'));
            const isDefaultOpen = section.getAttribute('data-default-open') === '1';
            section.classList.toggle('collapsed', !isDefaultOpen);
            return;
        }
        let anyVisible = false;
        rows.forEach(row => {
            const corpus = String(row.getAttribute('data-config-search') || '');
            const hidden = !corpus.includes(query);
            row.classList.toggle('cfgHidden', hidden);
            if (!hidden) anyVisible = true;
        });
        if (anyVisible) section.classList.remove('collapsed');
        else section.classList.add('collapsed');
    });
}
function configEditorSaveTargetLabel() { return configEditorState.saveTarget === 'album' ? `album override (${configEditorState.albumName || 'current'})` : 'global config.toml'; }
function configEditorFieldMetaMap() { const fieldMeta = {}; (Array.isArray(configEditorState.sections) ? configEditorState.sections : []).forEach(section => { (Array.isArray(section.fields) ? section.fields : []).forEach(field => { fieldMeta[field.id] = { key: field.key || field.id || 'field', comments: Array.isArray(field.comments) ? field.comments : [], section: section.title || 'Section' }; }); }); return fieldMeta; }
function configEditorChangeList(values = collectConfigEditorValues()) { const originalValues = configEditorState.originalValues || {}; const metaMap = configEditorFieldMetaMap(); const changes = []; Object.keys(values).forEach(fieldId => { const before = String(originalValues[fieldId] ?? ''); const after = String(values[fieldId] ?? ''); if (before === after) return; const meta = metaMap[fieldId] || { key: fieldId, comments: [], section: 'Section' }; let kind = 'updated'; if (!before && after) kind = 'added'; else if (before && !after) kind = 'removed'; const reason = meta.comments.length ? meta.comments[0] : ''; changes.push({ fieldId, key: meta.key, section: meta.section, before, after, kind, reason }); }); return changes; }
function configEditorChangeSummaryHtml(values = collectConfigEditorValues()) { const changes = configEditorChangeList(values); if (!changes.length) return '<p class="cfgConfirmBody">No field values have changed yet.</p>'; const buckets = { added: [], removed: [], updated: [] }; changes.forEach(change => { buckets[change.kind].push(change); }); const renderItem = change => {
        const beforeText = cfgHtml(change.before || '(empty)');
        const afterText = cfgHtml(change.after || '(empty)');
        const reasonText = change.reason ? `<div class='cfgConfirmReason'>Reason: ${cfgHtml(change.reason)}</div>` : '';
        return `<li><strong>${cfgHtml(change.key)}</strong> <span class='cfgChangeKind'>${cfgHtml(change.kind)}</span> <span class='cfgChangeValue'>${beforeText} → ${afterText}</span>${reasonText}</li>`;
    };
    const sections = [];
    if (buckets.added.length) sections.push(`<div class='cfgConfirmGroup'><div class='cfgConfirmGroupTitle'>Added</div><ul>${buckets.added.map(renderItem).join('')}</ul></div>`);
    if (buckets.removed.length) sections.push(`<div class='cfgConfirmGroup'><div class='cfgConfirmGroupTitle'>Removed</div><ul>${buckets.removed.map(renderItem).join('')}</ul></div>`);
    if (buckets.updated.length) sections.push(`<div class='cfgConfirmGroup'><div class='cfgConfirmGroupTitle'>Updated</div><ul>${buckets.updated.map(renderItem).join('')}</ul></div>`);
    return `<div class='cfgConfirmBody'>${sections.join('')}</div>`;
}
function configEditorSaveConfirmChoice(values = collectConfigEditorValues()) { return new Promise(resolve => { const changes = configEditorChangeList(values); const targetLabel = configEditorSaveTargetLabel(); const changeSummary = configEditorChangeSummaryHtml(values); const changeCountText = changes.length ? `Changed fields (${changes.length})` : 'No field values have changed yet.'; const dlg = document.createElement('dialog'); dlg.className = 'cfgConfirmDialog'; dlg.innerHTML = `<h4 class='cfgConfirmTitle'>Save config changes?</h4>` + `<p class='cfgConfirmBody'>This will write to ${cfgHtml(targetLabel)} and reload the review UI.</p>` + `<p class='cfgConfirmBody'><strong>${cfgHtml(changeCountText)}</strong></p>` + changeSummary + "<div class='cfgConfirmActions'>" + "<button id='_cfg_cancel' class='cfgSecondaryBtn'>Keep Editing</button>" + "<button id='_cfg_save' class='cfgPrimaryBtn'>Save</button>" + "</div>"; document.body.appendChild(dlg); dlg.showModal(); const finish = choice => { dlg.close(); dlg.remove(); resolve(choice); }; dlg.querySelector('#_cfg_save').onclick = () => finish('save'); dlg.querySelector('#_cfg_cancel').onclick = () => finish('cancel'); dlg.addEventListener('cancel', () => finish('cancel')); }); }
function configEditorExitChoice(message) { return new Promise(resolve => { const dlg = document.createElement('dialog'); dlg.className = 'cfgConfirmDialog'; const target = configEditorState.saveTarget === 'album' ? `album override (${configEditorState.albumName || 'current'})` : 'global config.toml'; dlg.innerHTML = `<h4 class='cfgConfirmTitle'>${cfgHtml(message || 'Save config changes?')}</h4>` + `<p class='cfgConfirmBody'>Save applies changes to ${cfgHtml(target)} and reloads the review UI. Discard closes the editor without saving.</p>` + "<div class='cfgConfirmActions'>" + "<button id='_cfg_cancel' class='cfgSecondaryBtn'>Continue Editing</button>" + "<button id='_cfg_discard' class='cfgDangerBtn'>Discard</button>" + "<button id='_cfg_save' class='cfgPrimaryBtn'>Save</button>" + "</div>"; document.body.appendChild(dlg); dlg.showModal(); const finish = choice => { dlg.close(); dlg.remove(); resolve(choice); }; dlg.querySelector('#_cfg_save').onclick = () => finish('save'); dlg.querySelector('#_cfg_discard').onclick = () => finish('discard'); dlg.querySelector('#_cfg_cancel').onclick = () => finish('cancel'); dlg.addEventListener('cancel', () => finish('cancel')); }); }
async function requestCloseConfigEditor() { if (!configEditorState.open || configEditorState.loading || configEditorState.saving) { closeConfigEditorBackdrop(); return; } if (!configEditorIsDirty()) { closeConfigEditorBackdrop(); setConfigEditorStatus('Config editor closed'); return; } const choice = await configEditorExitChoice('Save config changes before closing?'); if (choice === 'save') { await saveConfigEditorChanges(true); return; } if (choice === 'discard') { closeConfigEditorBackdrop(); setConfigEditorStatus('Config changes discarded'); } }
async function saveConfigEditorChanges(skipConfirm = false) { if (configEditorState.saving) return false; const values = collectConfigEditorValues(); if (!skipConfirm) { const confirmChoice = await configEditorSaveConfirmChoice(values); if (confirmChoice !== 'save') return false; } configEditorState.saving = true; const saveTarget = configEditorState.saveTarget === 'album' ? 'album' : 'global'; const endpoint = saveTarget === 'album' ? '/api/config-editor-save-album' : '/api/config-editor-save'; const savingLabel = saveTarget === 'album' ? `Saving album override (${configEditorState.albumName || 'current'})...` : 'Saving global config.toml...'; setConfigEditorStatus(savingLabel); if (typeof setBusy === 'function') setBusy(true, savingLabel); try { const response = await fetch(endpoint, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ values }) }); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to save config values.'); configEditorState.originalValues = { ...values }; closeConfigEditorBackdrop(); setConfigEditorStatus(payload.message || 'Config saved.'); if (typeof setBusy === 'function') setBusy(false, ''); if (payload.reload_required) { if (typeof setBusy === 'function') setBusy(true, 'Config saved. Reloading review UI...'); setTimeout(() => { window.location.reload(); }, 1600); } return true; } catch (error) { if (typeof setBusy === 'function') setBusy(false, ''); setConfigEditorStatus('Config save failed'); alert('Config save failed: ' + error); return false; } finally { configEditorState.saving = false; } }
async function deleteAlbumConfigOverride() { if (configEditorState.saving || configEditorState.loading) return; if (!configEditorState.albumName) return; const ok = confirm(`Delete album override config for ${configEditorState.albumName}?\n\nThis removes Albums/${configEditorState.albumName}/config.toml and falls back to global config.toml.`); if (!ok) return; configEditorState.saving = true; setConfigEditorStatus(`Deleting album override for ${configEditorState.albumName}...`); if (typeof setBusy === 'function') setBusy(true, `Deleting album override for ${configEditorState.albumName}...`); try { const response = await fetch('/api/config-editor-delete-album-override', { method: 'POST' }); const payload = await response.json(); if (!payload.ok) throw new Error(payload.message || 'Failed to delete album override.'); closeConfigEditorBackdrop(); setConfigEditorStatus(payload.message || 'Album override deleted.'); if (typeof setBusy === 'function') setBusy(false, ''); if (payload.reload_required) { if (typeof setBusy === 'function') setBusy(true, 'Album override deleted. Reloading review UI...'); setTimeout(() => { window.location.reload(); }, 1600); } } catch (error) { if (typeof setBusy === 'function') setBusy(false, ''); setConfigEditorStatus('Delete override failed'); alert('Delete override failed: ' + error); } finally { configEditorState.saving = false; } }
window.addEventListener('beforeunload', event => { if (!configEditorState.open || !configEditorIsDirty()) return; event.preventDefault(); event.returnValue = ''; });
configEditorBackdrop()?.addEventListener('click', event => { if (event.target === configEditorBackdrop()) requestCloseConfigEditor(); });
</script>
"#.to_string()
}
