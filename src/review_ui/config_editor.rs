use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;

// use crate::album_globals::pdf_dx_logging_enabled;
use crate::config;

use super::{ConfigEditorField, ConfigEditorSection};

pub(super) fn update_default_catalog_code_for_album(
    album_dir: &std::path::Path,
    album_name: &str,
    global_config_path: &str,
    pdf_path: &str,
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
        pdf_path,
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
        add_unique(
            "AlbumEasy font code. Common values: HN=Helvetica Normal, HB=Helvetica Bold, TN=Times Normal, TB=Times Bold, TR=Times Roman, TI=Times Italic, BI=Bold Italic, CN=Courier Normal, CO=Courier Oblique.",
        );
        add_unique(
            "Tip: keep heading/body font families consistent across related fields to avoid visual mismatch.",
        );
    }

    if key_only.ends_with("_size") {
        add_unique(
            "Font size used by AlbumEasy text rendering. Decimal values are allowed (for example 8.0, 10.5). Larger sizes increase layout pressure and can cause wrapping.",
        );
    }

    if key_only.ends_with("_vspace") || key_only.contains("padding") || key_only.contains("margin")
    {
        add_unique(
            "Spacing value in millimeters (mm). Positive values increase vertical/horizontal whitespace; overly large values may reduce stamps-per-page.",
        );
    }

    if key_only == "page_text_format"
        || key_only == "page_text_centre_format"
        || key_only == "page_text_right_format"
    {
        add_unique(
            "Template must include exactly one '{}' placeholder where the text value is injected.",
        );
        add_unique(
            "Expected command shapes: PAGE_TEXT (<FONT> <SIZE> \"{}\" <VSPACE>), PAGE_TEXT_CENTRE (...), PAGE_TEXT_RIGHT (...).",
        );
        add_unique("Example: PAGE_TEXT (HN 10 \"{}\" 1.0)");
    }

    if key_only == "stamp_heading_font"
        || key_only == "stamp_heading_size"
        || key_only == "stamp_heading_label"
    {
        add_unique(
            "Emitted heading uses STAMP_HEADING(<FONT> <SIZE> \"<LABEL> : <CATALOG_NUMBER>\").",
        );
        add_unique(
            "stamp_heading_label='auto' uses catalog display name from default_catalog_code; empty string hides the label.",
        );
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
        add_unique(
            "false hides perforation details above the image and from the per-stamp heading.",
        );
    }

    if key_only == "include_catalog_numbers" {
        add_unique("Controls the STAMP_HEADING block above each image.");
        add_unique("false hides the catalog label/number heading entirely.");
    }

    if key_only == "stamp_detail_layout" {
        add_unique(
            "inline: details inside stamp text blocks; separate_rows: details in dedicated rows; minimal: reduced metadata output.",
        );
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
        add_unique(
            "Controls ROW_START_ES marker output used to anchor row layout in generated AlbumEasy text.",
        );
        add_unique(
            "Disable row_start_es_enabled only if your output pipeline does not rely on row-start markers.",
        );
    }

    if key_only == "albumeasy_cli_path" {
        add_unique(
            "Absolute path to AlbumEasy CLI script/binary used for PDF generation from generated TXT.",
        );
        add_unique("Linux example: /home/<user>/work/AlbumEasy/AlbumEasy.sh");
    }

    if key_only == "albumeasy_auto_generate_pdf" {
        add_unique(
            "When true, tool runs '<albumeasy_cli_path> -c <generated_txt>' after generation.",
        );
        add_unique("Requires albumeasy_cli_path to be configured.");
    }

    if key_only == "expand_grouped_ranges" {
        add_unique(
            "placeholders: split grouped catalog ranges into placeholder rows quickly; detail_fetch: attempt richer detail fetch per item (slower).",
        );
    }

    if key_only.starts_with("flag_cover_") || full_id_key.starts_with("flag_cover.") {
        add_unique(
            "Flag cover values control cover-page image/text layout and are interpreted in millimeters for size/spacing fields.",
        );
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
    toml::Value::String(input.to_string()).to_string()
}

fn normalize_value_for_template(template_value: Option<&String>, raw_value: &str) -> String {
    let trimmed = raw_value.trim();

    if trimmed.is_empty() {
        if let Some(template) = template_value
            && template_literal_kind(template) == "string" {
                return "\"\"".to_string();
            }
        return String::new();
    }

    if trimmed.len() >= 2
        && ((trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\'')))
    {
        if matches!(
            toml::from_str::<toml::Value>(&format!("value = {trimmed}")),
            Ok(value) if value.get("value").and_then(toml::Value::as_str).is_some()
        ) {
            return trimmed.to_string();
        }
        // Pasted paths may include shell quotes rather than TOML escaping.
        return quote_toml_basic_string(&trimmed[1..trimmed.len() - 1]);
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

fn is_sensitive_config_field(id: &str) -> bool {
    let key = id.rsplit('.').next().unwrap_or(id).to_ascii_lowercase();
    [
        "password",
        "passwd",
        "secret",
        "token",
        "api_key",
        "access_key",
    ]
    .iter()
    .any(|part| key.contains(part))
}

pub(super) fn build_config_editor_data(
    config_path: &str,
    template_path: &str,
    pdf_path: &str,
) -> Result<Vec<ConfigEditorSection>, Box<dyn std::error::Error>> {
    config::ensure_config_and_template_files(config_path, template_path, pdf_path)?;
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

    for section in &mut template_sections {
        section
            .fields
            .retain(|field| !is_sensitive_config_field(&field.id));
    }
    template_sections.retain(|section| !section.fields.is_empty());
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
    pdf_path: &str,
    values: &HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    if values.is_empty() {
        return Err("no config values were provided".into());
    }

    config::ensure_config_and_template_files(config_path, template_path, pdf_path)?;
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
        if is_sensitive_config_field(field_id) {
            return Err(format!("Sensitive setting {field_id} must be managed separately").into());
        }
        let normalized = normalize_value_for_template(template_defaults_by_id.get(field_id), value);
        normalized_values.insert(field_id.clone(), normalized);
    }
    for (field_id, value) in load_current_config_values(config_path)? {
        if is_sensitive_config_field(&field_id) {
            normalized_values.insert(field_id, value);
        }
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
    pdf_path: &str,
    album_dir: &std::path::Path,
) -> Result<(Vec<ConfigEditorSection>, bool), Box<dyn std::error::Error>> {
    // Base: load sections with global config values.
    let mut sections = build_config_editor_data(global_config_path, template_path, pdf_path)?;

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
    pdf_path: &str,
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
        if is_sensitive_config_field(field_id) {
            return Err(format!("Sensitive setting {field_id} must be managed separately").into());
        }
        let n = normalize_value_for_template(template_defaults_by_id.get(field_id), raw_value);
        normalized.insert(field_id.clone(), n);
    }
    let existing_override = album_dir.join("config.toml");
    if existing_override.exists() {
        let path = existing_override
            .to_str()
            .ok_or("Album config path is not valid UTF-8")?;
        for (field_id, value) in load_current_config_values(path)? {
            if is_sensitive_config_field(&field_id) {
                normalized.insert(field_id, value);
            }
        }
    }

    // Load the global config current values for comparison.
    let global_values: HashMap<String, String> =
        config::ensure_config_and_template_files(global_config_path, template_path, pdf_path)
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
        if new_trimmed == global_val && !is_sensitive_config_field(field_id) {
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
    let css = include_str!("config_editor.css");
    let html = include_str!("config_editor.html");
    let javascript = include_str!("config_editor.js");

    format!(
        "<style>{css}</style>{html}<script>{javascript}</script>",
        css = css,
        html = html,
        javascript = javascript
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_hides_and_preserves_sensitive_values_in_global_and_album_configs() {
        let temp = tempfile::tempdir().expect("temporary config directory");
        let global = temp.path().join("config.toml");
        let template = temp.path().join("template.toml");
        let pdf = temp.path().join("pdf.toml");
        let album = temp.path().join("album");
        fs::create_dir(&album).expect("album directory");
        let defaults = "[stampworld]\nusername = \"\"\npassword = \"\"\n[colnect]\npassword = \"\"\n[runtime]\nname = \"\"\n";
        fs::write(&template, defaults).expect("template");
        fs::write(&pdf, "").expect("pdf");
        fs::write(&global, "[stampworld]\nusername = \"user\"\npassword = \"global-one\"\n[colnect]\npassword = \"global-two\"\n[runtime]\nname = \"original\"\n[extra]\napi_key = \"extra-secret\"\n").expect("global");
        fs::write(
            album.join("config.toml"),
            "[stampworld]\npassword = \"album-secret\"\n[colnect]\npassword = \"global-two\"\n[runtime]\nname = \"album\"\n",
        )
        .expect("album override");
        let global_path = global.to_str().expect("global path");
        let template_path = template.to_str().expect("template path");
        let pdf_path = pdf.to_str().expect("pdf path");
        let sections =
            build_config_editor_data(global_path, template_path, pdf_path).expect("editor");
        let serialized = serde_json::to_string(&sections).expect("serialize");
        for secret in ["global-one", "global-two", "extra-secret"] {
            assert!(!serialized.contains(secret));
        }
        let (sections, _) =
            build_config_editor_data_for_album(global_path, template_path, pdf_path, &album)
                .expect("album editor");
        assert!(
            !serde_json::to_string(&sections)
                .expect("serialize")
                .contains("album-secret")
        );
        let mut values: HashMap<String, String> = sections
            .iter()
            .flat_map(|section| section.fields.iter())
            .map(|field| (field.id.clone(), field.current_value.clone()))
            .collect();
        values.insert("runtime.name".to_string(), "\"changed\"".to_string());
        save_full_config_values(global_path, template_path, pdf_path, &values)
            .expect("save global");
        let saved = load_current_config_values(global_path).expect("saved config");
        assert_eq!(saved["stampworld.password"], "\"global-one\"");
        assert_eq!(saved["colnect.password"], "\"global-two\"");
        assert_eq!(saved["extra.api_key"], "\"extra-secret\"");
        save_album_config_diff(
            &album,
            "Test",
            global_path,
            pdf_path,
            template_path,
            &values,
        )
        .expect("save album");
        let saved =
            load_current_config_values(album.join("config.toml").to_str().expect("album path"))
                .expect("saved album");
        assert_eq!(saved["stampworld.password"], "\"album-secret\"");
        assert_eq!(saved["colnect.password"], "\"global-two\"");
        let injected =
            HashMap::from([("stampworld.password".to_string(), "\"replace\"".to_string())]);
        assert!(save_full_config_values(global_path, template_path, pdf_path, &injected).is_err());
    }

    #[test]
    fn native_row_alignment_help_reaches_editor_and_matches_example() {
        let template = parse_template_config_sections(include_str!("../../config_template.toml"));
        let example =
            parse_template_config_sections(include_str!("../../config_full_example.toml"));
        let field = |sections: Vec<ConfigEditorSection>| {
            sections
                .into_iter()
                .find(|s| s.id == "native_pdf.rows")
                .expect("rows section")
                .fields
                .into_iter()
                .find(|f| f.key == "alignment")
                .expect("alignment field")
        };
        let a = field(template);
        let b = field(example);
        assert_eq!(a.comments, b.comments);
        assert_eq!(a.template_value, b.template_value);
        for required in [
            "NEW NATIVE DEFAULT",
            "including the last row",
            "minimum",
            "singleton",
            "TXT EXPORT CONTRACT",
            "not implemented",
        ] {
            assert!(
                a.comments.iter().any(|c| c.contains(required)),
                "missing help: {required}"
            );
        }
    }

    #[test]
    fn native_stamp_text_template_help_reaches_each_editor_field() {
        let sections = parse_template_config_sections(include_str!("../../config_template.toml"));
        let section = sections
            .iter()
            .find(|section| section.id == "native_pdf.stamp_text")
            .expect("native stamp text section");
        for (key, required) in [
            ("enabled", "production album generator"),
            ("width_policy", "expand_footprint"),
            ("width_policy", "TXT EXPORT CONTRACT"),
            ("expanded_width_limit", "No fixed line-count limit"),
            ("expanded_width_value", "33.6 mm text cap"),
            ("horizontal_padding_mm", "PER SIDE"),
            ("vertical_padding_mm", "PER EDGE"),
            ("text_gap_mm", "absent text adds no gap"),
        ] {
            let field = section
                .fields
                .iter()
                .find(|field| field.key == key)
                .expect("field");
            assert!(
                field
                    .comments
                    .iter()
                    .any(|comment| comment.contains(required)),
                "missing help for {key}: {required}"
            );
        }
        let example =
            parse_template_config_sections(include_str!("../../config_full_example.toml"));
        let example = example
            .iter()
            .find(|section| section.id == "native_pdf.stamp_text")
            .expect("example section");
        for field in &section.fields {
            let other = example
                .fields
                .iter()
                .find(|other| other.key == field.key)
                .expect("example field");
            assert_eq!(field.comments, other.comments);
            assert_eq!(field.template_value, other.template_value);
        }
    }

    #[test]
    fn normalizes_windows_paths_and_preserves_valid_toml_strings() {
        let template = "\"\"".to_string();
        let windows = r"C:\Program Files (x86)\AlbumEasy\AlbumEasy.exe";
        for (input, expected) in [
            (windows.to_string(), windows),
            (format!("\"{windows}\""), windows),
            (format!("'{windows}'"), windows),
            (
                r#""C:\\Program Files (x86)\\AlbumEasy\\AlbumEasy.exe""#.to_string(),
                windows,
            ),
            (
                r#""\\server\share\AlbumEasy.exe""#.to_string(),
                r"\\server\share\AlbumEasy.exe",
            ),
            (
                "/opt/Album Easy/AlbumEasy".to_string(),
                "/opt/Album Easy/AlbumEasy",
            ),
            ("line\nwith\ttabs".to_string(), "line\nwith\ttabs"),
        ] {
            let normalized = normalize_value_for_template(Some(&template), &input);
            let parsed: toml::Value = toml::from_str(&format!("value = {normalized}"))
                .expect("normalized string must be valid TOML");
            assert_eq!(parsed["value"].as_str(), Some(expected), "{input}");
            assert_eq!(
                normalize_value_for_template(Some(&template), &normalized),
                normalized,
                "reloading and saving must not double-escape"
            );
        }
    }

    #[test]
    fn renders_global_and_saves_album_config_with_quoted_windows_path() {
        let temp = tempfile::tempdir().expect("temporary config directory");
        let global = temp.path().join("config.toml");
        let template = temp.path().join("template.toml");
        let pdf = temp.path().join("pdf.toml");
        let album = temp.path().join("album");
        let defaults = "[runtime]\nalbumeasy_cli_path = \"\"\n";
        fs::write(&global, defaults).expect("global config");
        fs::write(&template, defaults).expect("template config");
        fs::write(&pdf, "").expect("pdf config");
        let windows = r"C:\Program Files (x86)\AlbumEasy\AlbumEasy.exe";
        let values = HashMap::from([(
            "runtime.albumeasy_cli_path".to_string(),
            format!("\"{windows}\""),
        )]);

        assert_eq!(
            save_album_config_diff(
                &album,
                "Test",
                global.to_str().expect("global path"),
                pdf.to_str().expect("pdf path"),
                template.to_str().expect("template path"),
                &values,
            )
            .expect("save album config"),
            1
        );
        let normalized = values
            .iter()
            .map(|(key, value)| {
                (
                    key.clone(),
                    normalize_value_for_template(Some(&"\"\"".to_string()), value),
                )
            })
            .collect();
        let global_content = render_config_from_template_values(defaults, &normalized);
        let album_content =
            fs::read_to_string(album.join("config.toml")).expect("saved album config");
        for content in [global_content, album_content] {
            let parsed: toml::Value = toml::from_str(&content).expect("valid saved TOML");
            assert_eq!(
                parsed["runtime"]["albumeasy_cli_path"].as_str(),
                Some(windows)
            );
        }
    }
}
