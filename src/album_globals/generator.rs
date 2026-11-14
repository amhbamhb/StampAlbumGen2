use super::constants::MONTH_NAMES;
use super::globals::{compute_usable_height, write_globals};
use super::helpers::{
    build_date_page_text_centre_template, build_engraving_page_text_centre_template,
    build_page_text_template, build_series_page_text_centre_template,
    build_watermark_page_text_centre_template,
};
use super::structs::Stamp;
use crate::album_globals::generator_new::helpers::RowSpacingMode;
use crate::album_globals::generator_new::layout;
use crate::album_utils::clean_text;
use crate::config::load_effective_config;
use crate::db_utils::{get_all_series, init_db};
use crate::image_utils::{calc_mm_per_pixel_embedded, estimate_real_dimensions};
use log::{error, info, warn};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Buffered header: (lines with widths, vspace_before, vspace_after).
type PendingHeader = (Vec<(String, f64)>, f64, f64);

fn json_escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn normalize_denom(v: &str) -> String {
    v.to_ascii_lowercase()
        .replace([' ', '\t', '\n', '\r'], "")
        .replace("\u{a0}", "")
}

fn normalize_digits(v: &str) -> String {
    v.chars().filter(|c| c.is_ascii_digit()).collect()
}

fn parse_sort_day(v: Option<&String>) -> i32 {
    let Some(raw) = v else {
        return 0;
    };

    let trimmed = raw.trim();
    let digits: String = trimmed.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse::<i32>().unwrap_or(0)
}

fn stamp_date_key(stamp: &Stamp) -> String {
    let year = stamp
        .attrs
        .get("year")
        .map(|s| normalize_digits(s))
        .unwrap_or_default();
    let month = stamp
        .attrs
        .get("month_num")
        .map(|s| normalize_digits(s))
        .unwrap_or_default();
    let day = stamp
        .attrs
        .get("day")
        .map(|s| normalize_digits(s))
        .unwrap_or_default();
    if !year.is_empty() {
        format!(
            "{}-{:0>2}-{:0>2}",
            year,
            if month.is_empty() { "0" } else { &month },
            if day.is_empty() { "0" } else { &day }
        )
    } else {
        String::new()
    }
}

fn stamp_dedupe_fingerprints(stamp: &Stamp) -> Vec<String> {
    let year = stamp
        .attrs
        .get("year")
        .map(|s| normalize_digits(s))
        .unwrap_or_default();
    let date = stamp_date_key(stamp);
    let denom = normalize_denom(
        stamp
            .attrs
            .get("denomination")
            .map(|s| s.as_str())
            .unwrap_or(""),
    );
    let issued = normalize_digits(
        stamp
            .attrs
            .get("stamps_issued")
            .map(|s| s.as_str())
            .unwrap_or(""),
    );

    let mut keys = Vec::new();
    if !year.is_empty() && !date.is_empty() && !denom.is_empty() && !issued.is_empty() {
        keys.push(format!(
            "y:{}|d:{}|den:{}|iss:{}",
            year, date, denom, issued
        ));
    }
    if !year.is_empty() && !denom.is_empty() && !issued.is_empty() {
        keys.push(format!("y:{}|den:{}|iss:{}", year, denom, issued));
    }
    keys
}

fn strip_bracket_tags(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_tag = false;
    for ch in input.chars() {
        if ch == '[' {
            in_tag = true;
            continue;
        }
        if in_tag {
            if ch == ']' {
                in_tag = false;
            }
            continue;
        }
        out.push(ch);
    }
    out
}

fn normalize_for_text_dedupe(input: &str) -> String {
    let cleaned = clean_text(&strip_bracket_tags(input)).to_ascii_lowercase();
    let mut out = String::with_capacity(cleaned.len());
    let mut prev_space = false;
    for ch in cleaned.chars() {
        let keep = ch.is_ascii_alphanumeric() || ch.is_ascii_whitespace();
        let normalized = if keep { ch } else { ' ' };
        if normalized.is_ascii_whitespace() {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(normalized);
            prev_space = false;
        }
    }
    out.trim().to_string()
}

fn is_effectively_duplicate_text(base_text: &str, candidate_text: &str) -> bool {
    let base = normalize_for_text_dedupe(base_text);
    let candidate = normalize_for_text_dedupe(candidate_text);
    if base.is_empty() || candidate.is_empty() {
        return false;
    }
    if base == candidate {
        return true;
    }
    // Avoid false positives on short labels/phrases.
    if base.len() < 32 || candidate.len() < 32 {
        return false;
    }
    base.contains(&candidate) || candidate.contains(&base)
}

fn is_real_image_path(image_path: &str) -> bool {
    let p = image_path.trim().to_ascii_lowercase();
    !p.is_empty() && !p.contains("no_image")
}

/// Format a number string with thousand separators (commas).
/// Input: "1000000" Output: "1,000,000"
/// Also normalizes malformed grouped values (e.g. "4,545,4320" -> "45,454,320").
/// Leaves non-numeric parts unchanged.
fn format_number_with_commas(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    // Accept digits plus common grouping separators and normalize from raw digits.
    // If any other character appears, treat as non-numeric and preserve as-is.
    let mut digits = String::new();
    for ch in trimmed.chars() {
        if ch.is_ascii_digit() {
            digits.push(ch);
        } else if matches!(ch, ',' | '.' | ' ' | '_' | '\'') {
            continue;
        } else {
            return trimmed.to_string();
        }
    }

    if digits.is_empty() {
        return trimmed.to_string();
    }

    let normalized = digits.trim_start_matches('0');
    let digits = if normalized.is_empty() {
        "0"
    } else {
        normalized
    };

    if digits.len() <= 3 {
        return digits.to_string();
    }

    let first_group_len = digits.len() % 3;
    let mut out = String::with_capacity(digits.len() + (digits.len() / 3));

    if first_group_len > 0 {
        out.push_str(&digits[..first_group_len]);
    }

    let mut i = first_group_len;
    while i < digits.len() {
        if !out.is_empty() {
            out.push(',');
        }
        out.push_str(&digits[i..i + 3]);
        i += 3;
    }

    out
}

fn count_albumeasy_text_lines(s: &str) -> usize {
    // AlbumEasy line breaks can be represented as real newlines or escaped "\\n".
    // Count both so page-fit estimates match rendered PDF height.
    1 + s.matches('\n').count() + s.matches("\\n").count()
}

fn count_nonempty_albumeasy_text_lines(s: &str) -> usize {
    if s.trim().is_empty() {
        return 0;
    }

    s.replace("\\n", "\n")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

fn pad_centre_body_for_side_columns(
    top_block: &str,
    body: &str,
    left: &str,
    right_first_line: &str,
    right_second_line: &str,
) -> String {
    if top_block.trim().is_empty() && body.trim().is_empty() {
        return String::new();
    }

    let left_lines = count_nonempty_albumeasy_text_lines(left);
    let right = format!("{}\n{}", right_first_line, right_second_line);
    let right_lines = count_nonempty_albumeasy_text_lines(&right);
    let side_lines = left_lines.max(right_lines);
    // Only add padding if side columns wrap to multiple lines
    // Add +1 safety margin only when there's actual wrapping to prevent overflow
    let pad_lines = if side_lines > 1 {
        side_lines.saturating_sub(1) + 1 // Base padding + 1 safety line
    } else {
        0 // No padding for single-line sides
    };

    if top_block.trim().is_empty() {
        if pad_lines == 0 {
            body.to_string()
        } else {
            format!("{}{}", "\\n".repeat(pad_lines), body)
        }
    } else if body.trim().is_empty() {
        top_block.to_string()
    } else if pad_lines == 0 {
        format!("{}\\n{}", top_block, body)
    } else {
        format!("{}\\n{}{}", top_block, "\\n".repeat(pad_lines), body)
    }
}

struct StampText {
    left: String,
    centre: String,
    right: String,
    raw_left: String,
    raw_stamps_issued: String,
    wrapped_desc: String,
    raw_color: String,
}

fn build_stamp_text(
    stamp: &Stamp,
    config: &crate::config::Config,
    emit_width: f64,
    emit_height: f64,
) -> StampText {
    let include_denomination = config.include_denomination.unwrap_or(true);
    let include_stamps_issued = config.include_stamps_issued.unwrap_or(true);
    let include_description = config.include_description.unwrap_or(true);
    let include_color = config.include_color.unwrap_or(true);
    let include_mint_unused = config.include_mint_unused.unwrap_or(true);
    let include_used = config.include_used.unwrap_or(true);
    // Left: denomination (may include surcharge like "20+10 (C)").
    // Wrap/split the denomination so a surcharge does not force the
    // centre/right columns to be overwritten. Prefer splitting at
    // a '+' when present (keep the '+' with the second line),
    // otherwise perform a small greedy word-wrap.
    let mut raw_left = clean_text(stamp.attrs.get("denomination").unwrap_or(&"".to_string()));
    if raw_left.trim().is_empty() {
        let face = clean_text(
            stamp
                .attrs
                .get("colnect_face_value")
                .unwrap_or(&"".to_string()),
        );
        let cur = clean_text(
            stamp
                .attrs
                .get("colnect_currency")
                .unwrap_or(&"".to_string()),
        );
        if !face.trim().is_empty() {
            raw_left = if cur.trim().is_empty() {
                face
            } else {
                format!("{} {}", face, cur)
            };
        }
    }
    if !include_denomination {
        raw_left.clear();
    }
    // Compute a conservative max chars for the left column from emit_width
    let approx_char_width_mm_left = 2.6_f64;
    let padding_mm_left = 4.0_f64;
    let avail_width_mm_left = if emit_width > padding_mm_left {
        emit_width - padding_mm_left
    } else {
        emit_width
    };
    let mut chars_per_line_left =
        (avail_width_mm_left / approx_char_width_mm_left / 2.5).floor() as usize; // left column is smaller portion
    if chars_per_line_left < 4 {
        chars_per_line_left = 4;
    }

    let wrap_left = |s: &str, max_chars: usize| -> String {
        if s.trim().is_empty() {
            return String::new();
        }
        // If string contains a '+' and not whitespace around it (common surcharge form),
        // split at the first '+' and keep the '+' on the second line.
        if s.contains('+') && !s.contains(' ') {
            if let Some(pos) = s.find('+') {
                let first = s[..pos].to_string();
                let second = format!("+{}", s[pos + 1..].trim());
                return format!("{}\\n{}", first, second);
            }
        }
        // Greedy word-wrap preserving words
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for word in s.split_whitespace() {
            if current.is_empty() {
                current.push_str(word);
            } else if current.len() + 1 + word.len() <= max_chars {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current);
                current = word.to_string();
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        lines.join("\\n")
    };

    let left = wrap_left(&raw_left, chars_per_line_left);
    // let sgnum = clean_text(stamp.attrs.get("sg_number").unwrap_or(&"".to_string()));
    // Centre: color, description, and stamps_issued (counts are shown here when present)
    // Wrap long descriptions to multiple lines based on stamp width so they
    // don't overflow adjacent fields in the AlbumEasy output. We approximate
    // average character width and greedily build lines that fit the
    // estimated emit_width (mm).
    let mut raw_stamps_issued = format_number_with_commas(&clean_text(
        stamp.attrs.get("stamps_issued").unwrap_or(&"".to_string()),
    ));
    if raw_stamps_issued.trim().is_empty() {
        raw_stamps_issued = format_number_with_commas(&clean_text(
            stamp
                .attrs
                .get("colnect_print_run")
                .unwrap_or(&"".to_string()),
        ));
    }
    if !include_stamps_issued {
        raw_stamps_issued.clear();
    }
    let raw_description = if include_description {
        clean_text(stamp.attrs.get("description").unwrap_or(&"".to_string()))
    } else {
        String::new()
    };
    let raw_color = if include_color {
        clean_text(stamp.attrs.get("color").unwrap_or(&"".to_string()))
    } else {
        String::new()
    };

    // Simple wrapping: estimate characters per line from emit_width.
    // Use an approximate character width (mm). This is heuristic but
    // prevents very long single-line descriptions from overwriting
    // neighboring fields. Leave short descriptions unchanged.
    let approx_char_width_mm = 2.2_f64; // base heuristic average char width
    let padding_mm = 4.0_f64; // horizontal padding to avoid tight wrap
    let avail_width_mm = if emit_width > padding_mm {
        emit_width - padding_mm
    } else {
        emit_width
    };
    // Wider-than-tall stamps can carry longer metadata lines safely,
    // while tall/narrow stamps should keep shorter lines.
    let aspect_ratio = if emit_height > 0.0 {
        emit_width / emit_height
    } else {
        1.0
    };
    let width_bias = if aspect_ratio >= 1.20 {
        1.35
    } else if aspect_ratio >= 1.0 {
        1.15
    } else {
        0.95
    };
    let mut chars_per_line =
        ((avail_width_mm / approx_char_width_mm) * width_bias).floor() as usize;
    if chars_per_line < 10 {
        chars_per_line = 10;
    } // minimum line length

    // Description text can be sentence-like and reads poorly with
    // very tight wrapping. Use a looser target and soft overflow
    // so lines look closer to other metadata fields.
    let wrap_description_words = |s: &str, max_chars: usize| -> String {
        if s.trim().is_empty() {
            return String::new();
        }
        let target = max_chars.max(18);
        let soft_limit = target + 8;
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for word in s.split_whitespace() {
            if current.is_empty() {
                current.push_str(word);
            } else if current.len() + 1 + word.len() <= soft_limit {
                current.push(' ');
                current.push_str(word);
            } else {
                lines.push(current);
                current = word.to_string();
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        lines.join("\\n")
    };

    // Wrap a metadata field while keeping `Label: value` together.
    // If value is empty, drop the field entirely.
    let wrap_label_value = |s: &str, max_chars: usize| -> Option<String> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return None;
        }

        if let Some((label, value_raw)) = trimmed.split_once(':') {
            let value = value_raw.trim();
            if value.is_empty() {
                return None;
            }

            let label = label.trim();
            let value_words: Vec<&str> = value.split_whitespace().collect();
            if value_words.is_empty() {
                return None;
            }

            let mut lines: Vec<String> = Vec::new();
            // Keep at least two value words on the label line when available
            // so fields like `Designers: Otto Treumann` do not fragment.
            let first_take = value_words.len().min(2);
            let mut current = format!("{}: {}", label, value_words[..first_take].join(" "));
            for word in value_words.iter().skip(first_take) {
                // Soft overflow tolerance reduces orphan single-word lines.
                let soft_limit = max_chars + 10;
                if current.len() + 1 + word.len() <= soft_limit {
                    current.push(' ');
                    current.push_str(word);
                } else {
                    lines.push(current);
                    current = (*word).to_string();
                }
            }
            if !current.is_empty() {
                lines.push(current);
            }
            return Some(lines.join("\\n"));
        }

        // Unlabeled metadata entries are typically descriptions;
        // keep phrase-level readability similar to other fields.
        let wrapped = wrap_description_words(trimmed, max_chars);
        if wrapped.is_empty() {
            None
        } else {
            Some(wrapped)
        }
    };

    let wrapped_desc = wrap_description_words(&raw_description, (chars_per_line / 3).max(10));

    // Get layout mode from config
    let detail_layout = config.stamp_detail_layout.as_deref().unwrap_or("inline");

    // Build Colnect data based on layout mode and config
    let mut colnect_parts = Vec::new();
    let colnect_enabled = config.colnect_details_enabled();
    let enabled_fields = if colnect_enabled {
        config.effective_colnect_detail_field_set()
    } else {
        std::collections::HashSet::new()
    };

    // Only collect Colnect data if not in minimal mode
    if detail_layout != "minimal" && colnect_enabled {
        if enabled_fields.contains("name") {
            if let Some(v) = stamp.attrs.get("colnect_name") {
                colnect_parts.push(format!("Name: {}", v));
            }
        }
        if enabled_fields.contains("catalog_codes") {
            if let Some(v) = stamp.attrs.get("colnect_catalog_codes") {
                colnect_parts.push(format!("Catalog: {}", v));
            }
        }
        if enabled_fields.contains("country") {
            if let Some(v) = stamp.attrs.get("colnect_country") {
                colnect_parts.push(format!("Country: {}", v));
            }
        }
        if enabled_fields.contains("colnect_link") {
            if let Some(v) = stamp.attrs.get("colnect_page_url") {
                colnect_parts.push(format!("Colnect link: {}", v));
            }
        }
        if enabled_fields.contains("description") {
            if let Some(v) = stamp.attrs.get("colnect_description") {
                let cleaned_colnect_desc = clean_text(&strip_bracket_tags(v));
                if !cleaned_colnect_desc.trim().is_empty()
                    && !is_effectively_duplicate_text(&raw_description, &cleaned_colnect_desc)
                {
                    colnect_parts.push(cleaned_colnect_desc);
                }
            }
        }
        if enabled_fields.contains("themes") {
            if let Some(v) = stamp.attrs.get("colnect_themes") {
                colnect_parts.push(format!("Themes: {}", v));
            }
        }
        if enabled_fields.contains("printing") || enabled_fields.contains("printing_type") {
            if let Some(v) = stamp.attrs.get("colnect_printing") {
                colnect_parts.push(format!("Printing type: {}", v));
            }
        }
        if enabled_fields.contains("format") {
            if let Some(v) = stamp.attrs.get("colnect_format") {
                colnect_parts.push(format!("Format: {}", v));
            }
        }
        if enabled_fields.contains("emission") {
            if let Some(v) = stamp.attrs.get("colnect_emission") {
                colnect_parts.push(format!("Emission: {}", v));
            }
        }
        if enabled_fields.contains("paper") {
            if let Some(v) = stamp.attrs.get("colnect_paper") {
                colnect_parts.push(format!("Paper: {}", v));
            }
        }
        if enabled_fields.contains("gum") {
            if let Some(v) = stamp.attrs.get("colnect_gum") {
                colnect_parts.push(format!("Gum: {}", v));
            }
        }
        if enabled_fields.contains("designers") {
            if let Some(v) = stamp.attrs.get("colnect_designers") {
                colnect_parts.push(format!("Designers: {}", v));
            }
        }
        if enabled_fields.contains("printers") {
            if let Some(v) = stamp.attrs.get("colnect_printers") {
                colnect_parts.push(format!("Printers: {}", v));
            }
        }
        if enabled_fields.contains("colors") {
            if let Some(v) = stamp.attrs.get("colnect_colors") {
                colnect_parts.push(format!("Colors: {}", v));
            }
        }
        if enabled_fields.contains("watermark") {
            if let Some(v) = stamp.attrs.get("colnect_watermark") {
                colnect_parts.push(format!("Watermark: {}", v));
            }
        }
        // Perforation is shown in STAMP_HEADING; keep it out of stamp body text to avoid duplicate clutter.
        if enabled_fields.contains("issued_on") {
            if let Some(v) = stamp.attrs.get("colnect_issued_on") {
                colnect_parts.push(format!("Issued: {}", v));
            }
        }
        if enabled_fields.contains("expiry_date") {
            if let Some(v) = stamp.attrs.get("colnect_expiry_date") {
                colnect_parts.push(format!("Expiry: {}", v));
            }
        }
        if enabled_fields.contains("size") {
            if let (Some(w), Some(h)) = (
                stamp.attrs.get("colnect_width_mm"),
                stamp.attrs.get("colnect_height_mm"),
            ) {
                colnect_parts.push(format!("Size: {} x {} mm", w, h));
            }
        }
        if enabled_fields.contains("width") {
            if let Some(v) = stamp.attrs.get("colnect_width_mm") {
                colnect_parts.push(format!("Width: {} mm", v));
            }
        }
        if enabled_fields.contains("height") {
            if let Some(v) = stamp.attrs.get("colnect_height_mm") {
                colnect_parts.push(format!("Height: {} mm", v));
            }
        }
        if enabled_fields.contains("face_value") {
            if let Some(v) = stamp.attrs.get("colnect_face_value") {
                if let Some(cur) = stamp.attrs.get("colnect_currency") {
                    colnect_parts.push(format!("Face value: {} {}", v, cur));
                } else {
                    colnect_parts.push(format!("Face value: {}", v));
                }
            }
        }
        if enabled_fields.contains("currency") {
            if let Some(v) = stamp.attrs.get("colnect_currency") {
                colnect_parts.push(format!("Currency: {}", v));
            }
        }
        if enabled_fields.contains("score") {
            if let Some(v) = stamp.attrs.get("colnect_score") {
                if let Some(acc) = stamp.attrs.get("colnect_accuracy") {
                    colnect_parts.push(format!("Score: {} (Accuracy: {})", v, acc));
                } else {
                    colnect_parts.push(format!("Score: {}", v));
                }
            }
        }
        if enabled_fields.contains("accuracy") {
            if let Some(v) = stamp.attrs.get("colnect_accuracy") {
                colnect_parts.push(format!("Accuracy: {}", v));
            }
        }
        if enabled_fields.contains("ord_mi") {
            if let Some(v) = stamp.attrs.get("colnect_ord_mi") {
                colnect_parts.push(format!("MI order: {}", v));
            }
        }
        if enabled_fields.contains("ord_sn") {
            if let Some(v) = stamp.attrs.get("colnect_ord_sn") {
                colnect_parts.push(format!("SN order: {}", v));
            }
        }
        if enabled_fields.contains("ord_yt") {
            if let Some(v) = stamp.attrs.get("colnect_ord_yt") {
                colnect_parts.push(format!("YT order: {}", v));
            }
        }
        if enabled_fields.contains("ord_sg") {
            if let Some(v) = stamp.attrs.get("colnect_ord_sg") {
                colnect_parts.push(format!("SG order: {}", v));
            }
        }
        if enabled_fields.contains("parent_item") {
            if let Some(v) = stamp.attrs.get("colnect_parent_item") {
                colnect_parts.push(format!("Parent item: {}", v));
            }
        }
        if enabled_fields.contains("variant") {
            if let Some(v) = stamp.attrs.get("colnect_variant") {
                colnect_parts.push(format!("Variant: {}", v));
            }
        }
        if enabled_fields.contains("overprint_variant") {
            if let Some(v) = stamp.attrs.get("colnect_overprint_variant") {
                colnect_parts.push(format!("Overprint variant: {}", v));
            }
        }
        if enabled_fields.contains("list") {
            if let Some(v) = stamp.attrs.get("colnect_list") {
                colnect_parts.push(format!("List: {}", v));
            }
        }
        if enabled_fields.contains("quantity") {
            if let Some(v) = stamp.attrs.get("colnect_quantity") {
                colnect_parts.push(format!("Quantity: {}", v));
            }
        }
        if enabled_fields.contains("condition") {
            if let Some(v) = stamp.attrs.get("colnect_condition") {
                colnect_parts.push(format!("Condition: {}", v));
            }
        }
        if enabled_fields.contains("public_note") {
            if let Some(v) = stamp.attrs.get("colnect_public_note") {
                colnect_parts.push(format!("Public note: {}", v));
            }
        }
        if enabled_fields.contains("private_note") {
            if let Some(v) = stamp.attrs.get("colnect_private_note") {
                colnect_parts.push(format!("Private note: {}", v));
            }
        }
    }

    // Remove empty fields (or empty `Label:` entries) before formatting.
    colnect_parts.retain(|entry| {
        let t = entry.trim();
        if t.is_empty() {
            return false;
        }
        if let Some((_, rhs)) = t.split_once(':') {
            return !rhs.trim().is_empty();
        }
        true
    });

    let wrapped_colnect = {
        let mut wrapped_entries: Vec<String> = Vec::new();
        for (idx, entry) in colnect_parts.iter().enumerate() {
            if let Some(mut wrapped) = wrap_label_value(entry, chars_per_line) {
                if idx + 1 < colnect_parts.len() {
                    wrapped.push(';');
                }
                wrapped_entries.push(wrapped);
            }
        }
        wrapped_entries.join("\\n")
    };

    // Build centre text based on layout mode, keeping stamps_issued at the top.
    let centre_body = match detail_layout {
        "minimal" => {
            // Minimal: only face value, no clutter
            String::new()
        }
        _ => {
            // Inline / separate_rows (now merged): detail text goes in the centre
            // parameter of STAMP_ADD_IMG, below the issued count.
            let mut blocks: Vec<String> = Vec::new();
            if !wrapped_desc.is_empty() {
                blocks.push(wrapped_desc.clone());
            }
            if !wrapped_colnect.is_empty() {
                if !wrapped_desc.is_empty() {
                    blocks.push(wrapped_colnect.clone());
                } else if !raw_stamps_issued.is_empty() {
                    blocks.push(format!("\\n{}", wrapped_colnect));
                } else {
                    blocks.push(wrapped_colnect.clone());
                }
            }
            if !raw_color.is_empty() {
                blocks.push(raw_color.clone());
            }
            blocks.join("\\n")
        }
    };

    // Helper: consider common placeholders as "empty"
    let get_meaningful = |key: &str| -> Option<String> {
        let v = clean_text(stamp.attrs.get(key).unwrap_or(&"".to_string()))
            .trim()
            .to_string();
        let low = v.to_lowercase();
        if v.is_empty() || low == "0" || low == "-" || low == "n/a" || low == "—" {
            None
        } else {
            Some(v)
        }
    };

    // Fetch cleaned values for all relevant keys
    let v_mint = get_meaningful("mint_condition");
    let v_unused = get_meaningful("unused");
    let v_used = get_meaningful("used");

    // Right-hand field rule: per user preference the right column must be
    //   - first line: mint_condition (if meaningful) else fallback to unused
    //   - second line: used
    // Do NOT shift used/unused into a separate third-right line here. The centre
    // field continues to hold `stamps_issued` when present.
    let first_line = if include_mint_unused {
        v_mint
            .or(v_unused.clone())
            .unwrap_or_else(|| "".to_string())
    } else {
        String::new()
    };
    let second_line = if include_used {
        v_used.clone().unwrap_or_else(|| "".to_string())
    } else {
        String::new()
    };
    let centre = pad_centre_body_for_side_columns(
        &raw_stamps_issued,
        &centre_body,
        &left,
        &first_line,
        &second_line,
    );
    let right = format!("{}\\n{}", first_line, second_line);

    StampText {
        left,
        centre,
        right,
        raw_left,
        raw_stamps_issued,
        wrapped_desc,
        raw_color,
    }
}

fn stamp_content_height(
    stamp: &Stamp,
    config: &crate::config::Config,
    emit_width: f64,
    emit_height: f64,
) -> f64 {
    let text = build_stamp_text(stamp, config, emit_width, emit_height);
    let line_count = [&text.left, &text.centre, &text.right]
        .into_iter()
        .filter(|text| !text.replace("\\n", "").trim().is_empty())
        .map(|text| count_albumeasy_text_lines(text))
        .max()
        .unwrap_or(0);
    let label = if config.include_catalog_numbers.unwrap_or(true) {
        crate::album_globals::generator_new::emit::resolve_stamp_heading_label(
            config,
            stamp.attrs.get("catalog_code").map(String::as_str),
        )
    } else {
        String::new()
    };
    let number = stamp
        .attrs
        .get("catalog_number")
        .or_else(|| stamp.attrs.get("scott_number"))
        .or_else(|| stamp.attrs.get("sg_number"))
        .map(String::as_str)
        .unwrap_or("");
    let heading_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
    let heading_lines =
        crate::album_globals::generator_new::emit::estimate_stamp_heading_line_count(
            &label,
            number,
            stamp.attrs.get("colnect_name").map(String::as_str),
            stamp
                .attrs
                .get("colnect_perforation")
                .or_else(|| stamp.attrs.get("perforation"))
                .map(String::as_str),
            config.include_perforation.unwrap_or(true),
            emit_width,
            heading_size,
        );
    let text_line_height = (config.text_line_height_mm.unwrap_or(2.6) as f64)
        .max(config.row_start_es_size.unwrap_or(6.0) as f64 * 25.4 / 72.0);
    let heading_line_height = (heading_size.max(1) as f64) * 25.4 / 72.0
        + config.row_start_es_spacing.unwrap_or(0.1) as f64;
    emit_height
        + config.image_spacing_mm.unwrap_or(5.0) as f64
        + line_count as f64 * text_line_height
        + heading_lines as f64 * heading_line_height
}

/// Main entry point for AlbumEasy output generation.
/// Queries the database, formats output, and writes to a .txt file for AlbumEasy import.
/// Returns the path to the generated output file.
pub async fn generate_album_output_conn<F>(
    conn: &Connection,
    album_name: &str,
    page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> std::path::PathBuf
where
    F: FnMut(usize),
{
    generate_album_output_conn_prepared(conn, album_name, page_callback, start_year, end_year, None)
        .await
}

pub(crate) async fn generate_album_output_conn_prepared<F>(
    conn: &Connection,
    album_name: &str,
    mut page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
    prepared: Option<super::selection::PreparedPdfStamps>,
) -> std::path::PathBuf
where
    F: FnMut(usize),
{
    // Use the start_year/end_year passed to this function (may be None)
    let passed_start_year = start_year;
    let passed_end_year = end_year;
    // Load config values from config.toml
    let config = load_effective_config("config.toml");
    // Check if this is a US1847 sourced album
    let is_us1847 = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='us1847_series'",
            [],
            |r| r.get::<_, i64>(0),
        )
        .unwrap_or(0)
        > 0
        && conn
            .query_row("SELECT COUNT(*) FROM us1847_series", [], |r| {
                r.get::<_, i64>(0)
            })
            .unwrap_or(0)
            > 0;

    // The commemorative summary page includes US-specific collecting notes.
    // Keep it limited to US albums so non-US albums (for example Switzerland)
    // do not get legacy US summary content injected.
    let normalized_album_name = album_name
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase();
    let album_name_suggests_us = matches!(
        normalized_album_name.as_str(),
        "us" | "usa" | "unitedstates" | "unitedstatesofamerica"
    );
    let country_hint_suggests_us = conn
        .query_row(
            "SELECT LOWER(TRIM(country)) FROM stamps WHERE TRIM(COALESCE(country, '')) != '' LIMIT 1",
            [],
            |r| r.get::<_, String>(0),
        )
        .map(|country| {
            country == "us" || country == "usa" || country.contains("united states")
        })
        .unwrap_or(false);
    let allow_legacy_us_commemorative_summary = album_name_suggests_us || country_hint_suggests_us;
    // Also obtain canonical defaults so we always fall back to the same
    // values the `Config::with_defaults()` helper exposes. This ensures
    // generator math always uses config-driven values (either user-set or
    // project defaults) rather than ad-hoc literals sprinkled through code.
    let cfg_defaults = crate::config::Config::with_defaults();
    let target_stamp_height_mm = config
        .target_stamp_height_mm
        .unwrap_or(cfg_defaults.target_stamp_height_mm.unwrap())
        as f64;
    // extra space applied per-side around each stamp when emitted
    let _stamp_box_adjust = config
        .stamp_box_adjust
        .unwrap_or(cfg_defaults.stamp_box_adjust.unwrap()) as f64;
    let row_font_spacing = config
        .row_font_spacing
        .unwrap_or(cfg_defaults.row_font_spacing.unwrap()) as f64;
    let _header_font_spacing = config
        .header_font_spacing
        .unwrap_or(cfg_defaults.header_font_spacing.unwrap()) as f64;
    let reference_width_mm = config
        .reference_width_mm
        .unwrap_or(cfg_defaults.reference_width_mm.unwrap()) as f64;
    let reference_height_mm = config
        .reference_height_mm
        .unwrap_or(cfg_defaults.reference_height_mm.unwrap()) as f64;
    // Reference image path and measured size (dynamic by country)
    let _country_dir = format!("Albums/{}/images", album_name.replace(' ', "-"));
    let _reference_image_path = format!("{}/BG1-s.jpg", _country_dir);
    // Use embedded reference ratio as a fallback (no external reference file required)
    // If the configured reference size is implausibly small (e.g. 24mm), assume it's
    // a misconfiguration and fall back to sensible defaults used by the project
    // (200 x 240 mm). This prevents stamp sizes from being calculated as ~1-3mm.
    let (cfg_ref_w, cfg_ref_h) = (reference_width_mm, reference_height_mm);
    // Accept smaller reference sizes (stamp-size references like 24x27mm are valid).
    // Only treat sizes under 10mm as implausibly small and trigger fallback.
    let (ref_w_mm, ref_h_mm) = if cfg_ref_w < 10.0 || cfg_ref_h < 10.0 {
        warn!(
            "Configured reference size {:.1}x{:.1}mm looks too small, falling back to 25x30mm",
            cfg_ref_w, cfg_ref_h
        );
        (25.0_f64, 30.0_f64)
    } else {
        (cfg_ref_w, cfg_ref_h)
    };
    // Compute mm-per-pixel. Allow an explicit DPI override from config
    // (useful when the page provides pixels and you want a DPI-based mapping).
    let (mm_per_pixel_w, mm_per_pixel_h) = if let Some(dpi) = config.force_dpi {
        // mm per pixel = 25.4 mm / dpi
        let mmpp = 25.4_f32 / dpi;
        (mmpp, mmpp)
    } else {
        match calc_mm_per_pixel_embedded(ref_w_mm as f32, ref_h_mm as f32) {
            Some(ratio) => ratio,
            None => {
                warn!("Embedded reference image failed to load. Using default ratio (1.0, 1.0)");
                (1.0, 1.0)
            }
        }
    };
    // sizing_mode controls whether stamp heights are clamped to target_stamp_height_mm.
    // "fit_box": scale images down when taller than target_stamp_height_mm.
    // "real_world": emit actual DPI-derived physical dimensions as-is.
    let fit_box = config.sizing_mode.as_deref().unwrap_or("real_world") != "real_world";

    // Canonical output filename: Albums/{Album-Name}/{Album-Name}.txt
    let album_dir = crate::file_utils::resolve_album_dir(album_name);
    if let Err(e) = std::fs::create_dir_all(&album_dir) {
        warn!(
            "Failed to create album directory {}: {}",
            album_dir.display(),
            e
        );
    }
    // Build output filename. Use semi-intelligent year inference:
    // - If both start and end provided and sensible, use them.
    // - If only start provided, infer end from DB MAX(year) or current year.
    // - If only end provided, infer start from DB MIN(year) or current year.
    // - If neither provided, infer both from DB MIN/MAX(year) when available.
    let db_min_year = conn
        .query_row("SELECT MIN(year) FROM stamps", [], |r| {
            r.get::<_, Option<i32>>(0)
        })
        .ok()
        .flatten();
    let db_max_year = conn
        .query_row("SELECT MAX(year) FROM stamps", [], |r| {
            r.get::<_, Option<i32>>(0)
        })
        .ok()
        .flatten();
    // Bring Datelike into scope for `.year()`
    use chrono::Datelike;
    let current_year = chrono::Utc::now().year();

    // Determine effective start/end to use for filename (Option<i32>)
    let effective_start: Option<i32> = if let Some(s) = passed_start_year {
        if s > 0 { Some(s) } else { None }
    } else {
        // not provided: try DB min
        db_min_year
    };
    let effective_end: Option<i32> = if let Some(e) = passed_end_year {
        if e < 9999 { Some(e) } else { None }
    } else {
        // not provided: try DB max
        db_max_year
    };

    // If we have at least one bound, fill missing with sensible fallback
    let final_start = match (effective_start, effective_end) {
        (Some(s), Some(e)) => Some((s, e)),
        (Some(s), None) => Some((s, db_max_year.unwrap_or(current_year))),
        (None, Some(e)) => Some((db_min_year.unwrap_or(current_year), e)),
        (None, None) => None,
    };

    let filename = if let Some((s, e)) = final_start {
        // Always include both start and end in the filename when we have bounds.
        if s > 0 && e < 9999 && s <= e {
            format!("{}-{}-{}.txt", album_name.replace(' ', "-"), s, e)
        } else {
            format!("{}.txt", album_name.replace(' ', "-"))
        }
    } else {
        format!("{}.txt", album_name.replace(' ', "-"))
    };
    let output_path = album_dir.join(filename);
    // Write into a temporary file first to avoid truncating the canonical
    // output file if generation yields no stamp lines. We will promote the
    // temporary file into place only when it contains STAMP lines.
    let tmp_name = format!(
        "{}.tmp",
        output_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("out")
    );
    let tmp_path = output_path.with_file_name(tmp_name);
    let mut output = BufWriter::new(
        File::create(&tmp_path)
            .unwrap_or_else(|e| panic!("Failed to create temp output file: {}", e)),
    );

    // Write global album settings (pass DB connection and CLI year range so the header can include years)
    let pages_from_globals = write_globals(
        &mut output,
        album_name,
        Some(conn),
        passed_start_year,
        passed_end_year,
    )
    .await;

    // Start page counter after the globals section
    let starting_page_number = pages_from_globals + 1;

    // Emit page header for first stamp page
    let _ = crate::album_globals::generator_new::layout::write_stamp_page_header(
        &mut output,
        starting_page_number,
        &config,
    );
    // Compute usable height via shared helper (centralizes margins/borders/padding)
    let usable_height = compute_usable_height(&config);
    // Allow a configurable conservative buffer (mm) to reserve footer/decoration space.
    // Always prefer the value from `config.toml`; if absent, fall back to the
    // canonical defaults from `Config::with_defaults()` so behavior is consistent.
    let conservative_buffer = config
        .page_fit_conservative_buffer_mm
        .unwrap_or(cfg_defaults.page_fit_conservative_buffer_mm.unwrap())
        as f64;
    // Effective usable height after reserving the conservative buffer
    let effective_usable = usable_height - conservative_buffer;
    // Recreate a few layout vars for diagnostics and width calculations
    let _page_height = config.album_page_height_mm.unwrap_or(279.4) as f64;
    // Use `effective_usable` (usable_height minus conservative buffer)
    // when computing and inserting PAGE_VSPACE so the post-process
    // respects the same reserved footer/buffer used during page-fit
    let _top_margin = config.page_top_margin_mm.unwrap_or(10.0) as f64;
    let _bottom_margin = config.page_bottom_margin_mm.unwrap_or(10.0) as f64;
    let border_outer = config.page_border_outer.unwrap_or(0.5) as f64;
    let border_inner1 = config.page_border_inner1.unwrap_or(0.1) as f64;
    let border_inner2 = config.page_border_inner2.unwrap_or(0.0) as f64;
    let border_spacing = config.page_border_spacing.unwrap_or(3.0) as f64;
    let total_border = border_outer + border_inner1 + border_inner2 + border_spacing;
    // Compute a suggested `computed_stamps_per_row` value using simple math
    // so we can write it into `config.toml` for user convenience. The value is
    // based on the usable page width and the average stamp emit width computed
    // from the DB (or fallback reference size).
    let page_width = config
        .album_page_width_mm
        .unwrap_or(cfg_defaults.album_page_width_mm.unwrap()) as f64;
    let left_margin = config
        .page_left_margin_mm
        .unwrap_or(cfg_defaults.page_left_margin_mm.unwrap()) as f64;
    let right_margin = config
        .page_right_margin_mm
        .unwrap_or(cfg_defaults.page_right_margin_mm.unwrap()) as f64;
    let pages_spacing_h = config
        .album_pages_spacing_h
        .unwrap_or(cfg_defaults.album_pages_spacing_h.unwrap()) as f64;
    let usable_width_global =
        page_width - left_margin - right_margin - pages_spacing_h - total_border;
    // Compute average image width from DB (px -> mm) when available
    let avg_w_mm: f64 = match conn.query_row("SELECT AVG(image_width_px) FROM stamps", [], |r| {
        r.get::<_, Option<f64>>(0)
    }) {
        Ok(opt_px) => opt_px
            .map(|px| px * (mm_per_pixel_w as f64))
            .unwrap_or(reference_width_mm),
        Err(_) => reference_width_mm,
    };
    let extra_width_mm = config
        .extra_width_mm
        .unwrap_or(cfg_defaults.extra_width_mm.unwrap()) as f64;
    let extra_height_mm = config
        .extra_height_mm
        .unwrap_or(cfg_defaults.extra_height_mm.unwrap()) as f64;
    let stamp_box_adjust = config
        .stamp_box_adjust
        .unwrap_or(cfg_defaults.stamp_box_adjust.unwrap()) as f64;
    let avg_emit_w_mm = avg_w_mm + extra_width_mm + stamp_box_adjust;
    let packing_h = config
        .packing_horizontal_safety_mm
        .unwrap_or(cfg_defaults.packing_horizontal_safety_mm.unwrap()) as f64;
    let mut computed_stamps = (usable_width_global / (avg_emit_w_mm + packing_h)).floor() as i32;
    if computed_stamps < 1 {
        computed_stamps = 1;
    }
    // Respect configured hard cap if present
    if let Some(cap) = config.max_stamps_per_row {
        let cap_i = cap as i32;
        if computed_stamps > cap_i {
            computed_stamps = cap_i;
        }
    }
    // Attempt to update `config.toml` with a computed value so users can see it
    // Note: best-effort — do not fail generation if writing the file fails.
    // Write computed values to an album-specific advisory file so we do not
    // mutate the user's main `config.toml`. This is non-destructive and makes
    // it easy to inspect recommended values per-album.
    if std::fs::create_dir_all(&album_dir).is_ok() {
        let computed_path = album_dir.join("computed_values.toml");
        let computed_txt = format!(
            "# Auto-generated computed values for album\ncomputed_stamps_per_row = {}\naverage_emit_width_mm = {:.2}\n# usable_width_mm = {:.2}\n",
            computed_stamps, avg_emit_w_mm, usable_width_global
        );
        if let Err(e) = std::fs::write(&computed_path, computed_txt) {
            warn!(
                "Failed to write computed values to {}: {}",
                computed_path.display(),
                e
            );
        } else {
            info!("Wrote computed values to {}", computed_path.display());
        }
    }
    // debug!("[PAGE LAYOUT] page_height={:.2}, top_margin={:.2}, bottom_margin={:.2}, total_border={:.2}, usable_height={:.2}",
    // page_height, top_margin, bottom_margin, total_border, usable_height);
    // Font and spacing from AlbumEasy settings
    // Local layout estimates (we use configured values above where available)
    let _stamp_box_height = 24.0; // Estimate: typical stamp box height (unused default)
    let _header_lines = 4; // Year, name, desc, perforation (unused default)
    // let header_height = (header_lines as f64) * header_font_spacing;
    // let row_height = stamp_box_height + row_font_spacing;

    let series_order = config.series_order.clone().map(|s| s.as_str().to_string());
    let series_order_slice = series_order.as_deref();
    let series_list = get_all_series(conn, series_order_slice).expect("Failed to get series");
    // Initialize page counter from the starting page (after globals pages)
    let mut page_counter = starting_page_number;
    let mut current_height = 0.0f64;
    let mut rows_on_page = 0;
    let mut seen_stamp_fingerprints: std::collections::HashMap<String, bool> =
        std::collections::HashMap::new();
    let mut dedupe_audit: Vec<String> = Vec::new();
    let mut dedupe_dropped: usize = 0;
    // Treat series_list as Vec<String> (series names). We don't filter by year here.
    let filtered_series: Vec<String> = series_list.to_vec();
    let _series_count = filtered_series.len();
    crate::album_globals::generator_new::for_each_series(conn, series_order_slice, | _series_idx, series | {
        if let Some(cb) = page_callback.as_mut() {
            cb(page_counter);
        }
        // Fetch stamps + year-filter + conversion into Stamp objects via helper.
        let stamps_iter = if let Some(selected) = prepared.as_ref() {
            selected.get(series).cloned().unwrap_or_default()
        } else {
            crate::album_globals::generator_new::helpers::
                fetch_and_filter_stamps(conn, series, passed_start_year, passed_end_year)
                .expect("Failed to get stamps")
        };
        let mut stamps_vec = stamps_iter;
        if prepared.is_some() && stamps_vec.is_empty() {
            return;
        }
        // Re-sort by year + issue date + catalog number right before layout emission so
        // updates applied in Review UI are reflected in final PDF order.
        let parse_catalog_key = |raw: &str| {
            let trimmed = raw.trim();
            let mut end = 0usize;
            for (idx, ch) in trimmed.char_indices() {
                if ch.is_ascii_digit() {
                    end = idx + ch.len_utf8();
                } else {
                    break;
                }
            }

            if end == 0 {
                (None, trimmed.to_ascii_lowercase(), trimmed.to_ascii_lowercase())
            } else {
                let head = trimmed[..end].parse::<i64>().ok();
                let tail = trimmed[end..].trim().to_ascii_lowercase();
                (head, tail, trimmed.to_ascii_lowercase())
            }
        };
        stamps_vec.sort_by(|a, b| {
            let ay = a
                .attrs
                .get("year")
                .and_then(|v| v.trim().parse::<i32>().ok())
                .unwrap_or(0);
            let by = b
                .attrs
                .get("year")
                .and_then(|v| v.trim().parse::<i32>().ok())
                .unwrap_or(0);
            let year_cmp = ay.cmp(&by);
            if year_cmp != std::cmp::Ordering::Equal {
                return year_cmp;
            }

            let am = a
                .attrs
                .get("month_num")
                .and_then(|v| v.trim().parse::<i32>().ok())
                .unwrap_or(0);
            let bm = b
                .attrs
                .get("month_num")
                .and_then(|v| v.trim().parse::<i32>().ok())
                .unwrap_or(0);
            let month_cmp = am.cmp(&bm);
            if month_cmp != std::cmp::Ordering::Equal {
                return month_cmp;
            }

            let ad = parse_sort_day(a.attrs.get("day"));
            let bd = parse_sort_day(b.attrs.get("day"));
            let day_cmp = ad.cmp(&bd);
            if day_cmp != std::cmp::Ordering::Equal {
                return day_cmp;
            }

            let a_code = a.attrs.get("sg_number").map(|s| s.as_str()).unwrap_or("");
            let b_code = b.attrs.get("sg_number").map(|s| s.as_str()).unwrap_or("");
            let (a_num, a_suffix, a_norm) = parse_catalog_key(a_code);
            let (b_num, b_suffix, b_norm) = parse_catalog_key(b_code);
            match (a_num, b_num) {
                (Some(an), Some(bn)) => {
                    let num_cmp = an.cmp(&bn);
                    if num_cmp != std::cmp::Ordering::Equal {
                        return num_cmp;
                    }
                    let a_empty = a_suffix.is_empty();
                    let b_empty = b_suffix.is_empty();
                    if a_empty != b_empty {
                        return if a_empty {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        };
                    }
                    a_suffix.cmp(&b_suffix).then_with(|| a_norm.cmp(&b_norm))
                }
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a_norm.cmp(&b_norm),
            }
        });
        // Group stamps into rows (configurable max stamps per row). For rich
        // inline metadata, use a more conservative per-row cap to keep text
        // readable and avoid visual crowding in PDF output.
        let detail_layout = config
            .stamp_detail_layout.as_deref()
            .unwrap_or("inline");
        let configured_max = config.max_stamps_per_row.unwrap_or(5) as usize;
        let field_count = if config.colnect_details_enabled() {
            config.effective_colnect_detail_field_set().len()
        } else {
            0
        };
        let max_stamps_per_row = if detail_layout == "inline" {
            if field_count >= 12 {
                configured_max.min(2)
            } else if field_count >= 8 {
                configured_max.min(3)
            } else {
                configured_max.min(4)
            }
        } else {
            configured_max
        };
        let mut stamp_rows: Vec<Vec<Stamp>> = Vec::new();
        let mut row: Vec<Stamp> = Vec::new();
        // Compute a series-average estimated size (mm) from available images in this series.
        // This helps placeholders and missing-image stamps adopt the same size as their
        // series peers instead of falling back to the global reference size.
        let mut series_widths_mm: Vec<f64> = Vec::new();
        let mut series_heights_mm: Vec<f64> = Vec::new();
        for stamp in stamps_vec.iter() {
            let image_path = stamp.image_path.trim();
            let full_image_path = crate::file_utils::resolve_album_image_path(album_name, image_path)
                .to_string_lossy()
                .to_string();
            if let (Some(w_px), Some(h_px)) = (stamp.image_width_px, stamp.image_height_px) {
                series_widths_mm.push((w_px as f64) * (mm_per_pixel_w as f64));
                series_heights_mm.push((h_px as f64) * (mm_per_pixel_h as f64));
            } else if let Some((w_mm, h_mm)) =
                estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
            {
                series_widths_mm.push(w_mm as f64);
                series_heights_mm.push(h_mm as f64);
            }
        }
        let series_avg_w_mm = if !series_widths_mm.is_empty() {
            Some(series_widths_mm.iter().sum::<f64>() / series_widths_mm.len() as f64)
        } else {
            None
        };
        let series_avg_h_mm = if !series_heights_mm.is_empty() {
            Some(series_heights_mm.iter().sum::<f64>() / series_heights_mm.len() as f64)
        } else {
            None
        };
        // -------------------------
        // Pre-pass: compute final estimated physical sizes (mm) for every stamp
        // in this series. This ensures placeholders are generated now and
        // layout uses the final sizes rather than seeing stale or missing
        // dimensions during the single-pass layout.
        // -------------------------

        for s in stamps_vec.iter_mut() {
            // First priority: Check for Colnect dimensions (already in mm)
            let colnect_w = s.attrs.get("colnect_width_mm")
                .and_then(|w| w.parse::<f64>().ok());
            let colnect_h = s.attrs.get("colnect_height_mm")
                .and_then(|h| h.parse::<f64>().ok());

            let est_source: &str;
            if let (Some(w_mm), Some(h_mm)) = (colnect_w, colnect_h) {
                // Use Colnect real-world dimensions directly (no source-specific adjustments)
                s.est_width_mm = Some(w_mm);
                s.est_height_mm = Some(h_mm);
                est_source = "colnect";
            } else {
                // Fall back to existing dimension sources
                let image_path = s.image_path.trim();
                let full_image_path = crate::file_utils::resolve_album_image_path(album_name, image_path)
                    .to_string_lossy()
                    .to_string();
                // Prefer DB-stored pixel dimensions if available; otherwise fall back
                // to on-disk image estimation or series-average, finally reference size.
                // Track where the estimated size came from for diagnostics
                if let (Some(w_px), Some(h_px)) = (s.image_width_px, s.image_height_px) {
                    let mut w_mm = (w_px as f64) * (mm_per_pixel_w as f64);
                    let mut h_mm = (h_px as f64) * (mm_per_pixel_h as f64);
                    // In fit_box mode, scale down proportionally when taller than target.
                    if fit_box && h_mm > target_stamp_height_mm {
                        let s = target_stamp_height_mm / h_mm;
                        w_mm *= s;
                        h_mm *= s;
                    }
                    s.est_width_mm = Some(w_mm);
                    s.est_height_mm = Some(h_mm);
                    est_source = "db_pixels";
                } else if let Some((w_mm, h_mm)) =
                    estimate_real_dimensions(&full_image_path, mm_per_pixel_w, mm_per_pixel_h)
                {
                    let mut w = w_mm as f64;
                    let mut h = h_mm as f64;
                    // In fit_box mode, scale down proportionally when taller than target.
                    if fit_box && h > target_stamp_height_mm {
                        let s = target_stamp_height_mm / h;
                        w *= s;
                        h *= s;
                    }
                    s.est_width_mm = Some(w);
                    s.est_height_mm = Some(h);
                    est_source = "image_estimate";
                } else if let (Some(avg_w), Some(avg_h)) = (series_avg_w_mm, series_avg_h_mm) {
                    s.est_width_mm = Some(avg_w);
                    s.est_height_mm = Some(avg_h);
                    est_source = "series_average";
                } else {
                    s.est_width_mm = Some(reference_width_mm);
                    s.est_height_mm = Some(reference_height_mm);
                    est_source = "reference";
                }
            }

            // Emit a compact per-stamp diagnostic so we can trace where the
            // estimated mm sizes came from and what placeholder pixel sizes
            // (if any) were created. This helps debug mismatches between
            // series-declared sizes and actual placeholders.
            let diag_sg = s
                .attrs
                .get("sg_number")
                .unwrap_or(&"".to_string())
                .replace('"', "'");
            let diag_img_path = s.image_path.replace('"', "'");
            let diag_w_mm = s.est_width_mm.unwrap_or(reference_width_mm);
            let diag_h_mm = s.est_height_mm.unwrap_or(reference_height_mm);
            // If pixel dims are present (placeholders set them later), print them too.
            let diag_px_w = s.image_width_px.unwrap_or(0);
            let diag_px_h = s.image_height_px.unwrap_or(0);
            let _ = crate::album_globals::generator_new::helpers::emit_stamp_diag(
                &mut output,
                &crate::album_globals::generator_new::helpers::StampDiag {
                    sg: &diag_sg,
                    source: est_source,
                    est_w_mm: diag_w_mm,
                    est_h_mm: diag_h_mm,
                    px_w: diag_px_w,
                    px_h: diag_px_h,
                    img: &diag_img_path,
                },
            );

            // If the stamp has no image and therefore no stored pixel dims, create
            // a placeholder now using the emit box size (est + 2*stamp_box_adjust)
            // so future layout sees consistent sizes.
            let image_path = s.image_path.trim();
            let full_image_path = crate::file_utils::resolve_album_image_path(album_name, image_path)
                .to_string_lossy()
                .to_string();
            if s.image_width_px.is_none() && (image_path.is_empty() || !Path::new(&full_image_path).exists()) {
                let base_w = s.est_width_mm.unwrap_or(reference_width_mm);
                let base_h = s.est_height_mm.unwrap_or(reference_height_mm);
                let target_w_mm = base_w; // image-only width (mm)
                let target_h_mm = base_h; // image-only height (mm)
                let _sg_str = s.attrs.get("sg_number").map(|v| v.as_str()).unwrap_or("");
                let catalog_num = s.attrs.get("scott_number")
                    .or_else(|| s.attrs.get("sg_number"))
                    .map(|v| v.as_str())
                    .unwrap_or("");
                let year_str = s.attrs.get("year").map(|v| v.as_str()).unwrap_or("");
                let ph_opts = crate::album_globals::generator_new::helpers::PlaceholderCallOptions {
                    album_name,
                    series_name: Some(series),
                    catalog_number: catalog_num,
                    year: year_str,
                    target_w_mm: Some(target_w_mm),
                    target_h_mm: Some(target_h_mm),
                    mm_per_pixel_w: Some(mm_per_pixel_w),
                    mm_per_pixel_h: Some(mm_per_pixel_h),
                };
                if let Some((rel, px_w, px_h)) = crate::album_globals::generator_new::helpers::
                    ensure_placeholder_for_stamp_and_update_db(conn, &ph_opts, Some(&mut output))
                {
                    s.image_path = rel;
                    s.image_width_px = Some(px_w);
                    s.image_height_px = Some(px_h);
                }
            }
        }

        // COMMEMORATIVE SEPARATION: Check if this series has commemoratives and route accordingly
        let (commemorative_stamps, regular_stamps): (Vec<Stamp>, Vec<Stamp>) = stamps_vec.into_iter().partition(|stamp| {
            crate::album_globals::generator_new::commemorative_layout::is_commemorative(stamp, passed_start_year)
        });

        // If this series has commemoratives, emit them using the nice layout
        if !commemorative_stamps.is_empty() {
            let year = passed_start_year.unwrap_or_else(|| {
                commemorative_stamps.first()
                    .and_then(|s| s.attrs.get("year").and_then(|y| y.parse::<i32>().ok()))
                    .unwrap_or(1955)
            });

            // Split commemoratives into regular and special stamps
            let (special_stamps, regular_commemoratives): (Vec<Stamp>, Vec<Stamp>) = commemorative_stamps.into_iter().partition(|stamp| {
                crate::album_globals::commemorative_generator::is_special_stamp(stamp)
            });

            let has_commemoratives = !regular_commemoratives.is_empty() || !special_stamps.is_empty();

            // Only emit commemorative pages if there are regular commemoratives
            if !regular_commemoratives.is_empty() {
                let _ = crate::album_globals::commemorative_generator::emit_commemorative_pages(&mut output, &regular_commemoratives);
                page_counter += 1;
            }

            // Emit special stamps on a separate page if any exist
            if !special_stamps.is_empty() {
                let _ = crate::album_globals::commemorative_generator::emit_special_stamps_page(&mut output, &special_stamps);
                page_counter += 1;
            }

            // Emit summary page with all commemoratives
            if has_commemoratives && allow_legacy_us_commemorative_summary {
                let all_commemoratives: Vec<Stamp> = [regular_commemoratives, special_stamps].concat();
                let _ = crate::album_globals::commemorative_generator::emit_summary_page(&mut output, year, &all_commemoratives);
                page_counter += 1;
            }
            // If all stamps were commemorative, we're done with this series
            if regular_stamps.is_empty() {
                return;
            }
        }

        // Phase A+B: conservative matcher + dedupe pass.
        // If two rows share strong identity fingerprints and one already-seen row
        // has a real image while the new row is a no_image placeholder, drop the
        // new row to prevent overlap/duplicate emission.
        let mut deduped_regular_stamps: Vec<Stamp> = Vec::new();
        for stamp in regular_stamps.into_iter() {
            let fps = stamp_dedupe_fingerprints(&stamp);
            let has_real_image = is_real_image_path(&stamp.image_path);
            let mut matched_fp: Option<String> = None;
            let mut seen_had_real = false;

            for fp in fps.iter() {
                if let Some(prev_has_real) = seen_stamp_fingerprints.get(fp) {
                    matched_fp = Some(fp.clone());
                    seen_had_real = *prev_has_real;
                    break;
                }
            }

            let drop_duplicate = if matched_fp.is_some() {
                (seen_had_real && !has_real_image) || (!seen_had_real && !has_real_image)
            } else {
                false
            };

            if drop_duplicate {
                dedupe_dropped += 1;
                let sg = stamp
                    .attrs
                    .get("sg_number")
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let den = stamp
                    .attrs
                    .get("denomination")
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let iss = stamp
                    .attrs
                    .get("stamps_issued")
                    .map(|s| s.as_str())
                    .unwrap_or("");
                let date = stamp_date_key(&stamp);
                let fp = matched_fp.unwrap_or_default();
                dedupe_audit.push(format!(
                    "{{\"event\":\"dedupe_drop\",\"series\":\"{}\",\"sg\":\"{}\",\"denomination\":\"{}\",\"stamps_issued\":\"{}\",\"date\":\"{}\",\"fingerprint\":\"{}\",\"reason\":\"duplicate_no_image\",\"image_path\":\"{}\"}}",
                    json_escape(series),
                    json_escape(sg),
                    json_escape(den),
                    json_escape(iss),
                    json_escape(&date),
                    json_escape(&fp),
                    json_escape(&stamp.image_path)
                ));
                continue;
            }

            for fp in fps {
                seen_stamp_fingerprints
                    .entry(fp)
                    .and_modify(|v| *v = *v || has_real_image)
                    .or_insert(has_real_image);
            }
            deduped_regular_stamps.push(stamp);
        }

        // Now perform layout using the finalized est_* values for REGULAR stamps
        // let mut row_width = 0.0;
        for stamp in deduped_regular_stamps.into_iter() {
            let image_path = stamp.image_path.trim();
            let _full_image_path = crate::file_utils::resolve_album_image_path(album_name, image_path)
                .to_string_lossy()
                .to_string();
            // Use the est_width_mm precomputed in the pre-pass (should always be present)
            let est_width_mm = stamp.est_width_mm.unwrap_or(reference_width_mm);
            let _est_height_mm = stamp.est_height_mm.unwrap_or(reference_height_mm );
            // Horizontal spacing between stamps (mm). Use packing horizontal safety
            // config so users can tune how tightly stamps are packed horizontally.
            let margin = config.packing_horizontal_safety_mm.unwrap_or(5.0) as f64;
            // Page width and left/right margins come from config when present
            let page_width = config.album_page_width_mm.unwrap_or(216.0) as f64; // default US Letter
            let left_margin = config.page_left_margin_mm.unwrap_or(10.0) as f64;
            let right_margin = config.page_right_margin_mm.unwrap_or(10.0) as f64;
            // Subtract album pages horizontal spacing if configured (keeps conservative usable width)
            let pages_spacing_h = config.album_pages_spacing_h.unwrap_or(0.0) as f64;
            let usable_width = page_width
                - left_margin
                - right_margin
                - pages_spacing_h
                - (border_outer + border_inner1 + border_inner2 + border_spacing);
            // Accumulate row width as stamps are added. In `separate_rows`
            // mode we pack by the detail-box footprint so metadata boxes do
            // not overlap even when stamp images themselves are narrow.
            let row_width: f64 = row
                .iter()
                .map(|s| {
                    let est_w = s.est_width_mm.unwrap_or(reference_width_mm);
                    let emit_w = est_w + extra_width_mm + stamp_box_adjust;
                    emit_w + margin
                })
                .sum();
            // If adding this stamp would exceed usable width or row has max stamps, start a new row before adding.
            let new_stamp_footprint = est_width_mm + extra_width_mm + stamp_box_adjust;
            let new_stamp_total = new_stamp_footprint + margin;
            if row.len() >= max_stamps_per_row || row_width + new_stamp_total > usable_width {
                if !row.is_empty() {
                    stamp_rows.push(row);
                }
                row = Vec::new();
                // row_width = 0.0;
            }
            row.push(stamp);
        }
        if !row.is_empty() {
            stamp_rows.push(row);
        }

        let mut header_written = false;
        // Buffer header lines so we only write them when we actually emit
        // the first row on the same page. This avoids leaving a header on a
        // page with no associated stamps if a subsequent check forces a
        // page break after the header was previously emitted.
        let mut header_pending: Option<PendingHeader> = None;
        let mut row_idx = 0;
        while row_idx < stamp_rows.len() {
            let need_header = !header_written;
            if need_header {
                // Collect all header lines to print
                let mut header_lines_vec = Vec::new();
                let name_line = clean_text(series.trim())
                    .replace("\n", " ")
                    .replace("  ", " ");
                // Use configured text line height (mm) for header lines to more
                // accurately reflect visual space consumed by AlbumEasy font sizes.
                let header_font_h = (config.page_text_centre_size.unwrap_or(10.0) as f64)
                    * 25.4 / 72.0
                    + config.page_text_centre_vspace.unwrap_or(1.0) as f64;
                if config.include_series.unwrap_or(true) && !name_line.is_empty() {
                    // Use configurable PAGE_TEXT_CENTRE template when present in config
                    let tmpl = build_series_page_text_centre_template(&config);
                    let safe = name_line.replace('"', "\\\"");
                    header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                }
                let perf_trimmed = String::new();
                if !perf_trimmed.is_empty() {
                    let tmpl = build_page_text_template(&config, "centre");
                    let text = format!("Perforation {}", perf_trimmed);
                    let safe = text.replace('"', "\\\"");
                    header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                }
                let first_stamp = stamp_rows.first().and_then(|row| row.first());
                if let Some(stamp) = first_stamp {
                    // Optionally include a date line in the header (from DB year/month/day)
                    if config.include_date.unwrap_or(true) {
                        let date_pattern_cfg = config
                            .header_formatting
                            .as_ref()
                            .and_then(|hf| hf.date_pattern.clone());
                        let date_format_cfg = config
                            .header_formatting
                            .as_ref()
                            .and_then(|hf| hf.date_format.clone())
                            .unwrap_or_else(|| "commas".to_string());
                        let year = stamp.attrs.get("year").map(|s| s.trim()).unwrap_or("");
                        let day = stamp.attrs.get("day").map(|s| s.trim()).unwrap_or("");
                        let month_num = stamp
                            .attrs
                            .get("month_num")
                            .map(|s| s.trim())
                            .unwrap_or("");
                        let month_raw = stamp.attrs.get("month").map(|s| s.trim()).unwrap_or("");
                        let month = if !month_raw.is_empty() {
                            month_raw.to_string()
                        } else {
                            // fall back to numeric month -> name
                            if let Ok(mn) = month_num.parse::<usize>() {
                                if (1..=12).contains(&mn) {
                                    MONTH_NAMES[mn].to_string()
                                } else {
                                    "".to_string()
                                }
                            } else {
                                "".to_string()
                            }
                        };
                        let pattern_is_complete = !year.is_empty()
                            && !month.is_empty()
                            && !day.is_empty();
                        let mut date_text = if let Some(pattern) = date_pattern_cfg {
                            // Replace the documented tokens independently so
                            // users can choose either named or numeric months
                            // and any ordering/separators they prefer.
                            if pattern_is_complete {
                                pattern
                                    .replace("{year}", year)
                                    .replace("{month}", &month)
                                    .replace("{month_num}", month_num)
                                    .replace("{day}", day)
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };
                        if date_text.is_empty() {
                            if !year.is_empty() {
                                if date_format_cfg == "dashes" {
                                    if !month.is_empty() && !day.is_empty() {
                                        date_text = format!("{} - {} - {}", month, day, year);
                                    } else if !month.is_empty() {
                                        date_text = format!("{} - {}", month, year);
                                    } else {
                                        date_text = year.to_string();
                                    }
                                } else {
                                    // default to commas style: "Month Day, Year"
                                    if !month.is_empty() && !day.is_empty() {
                                        date_text = format!("{} {}, {}", month, day, year);
                                    } else if !month.is_empty() {
                                        date_text = format!("{} {}", month, year);
                                    } else {
                                        date_text = year.to_string();
                                    }
                                }
                            } else {
                                // no year available, but maybe month/day exist
                                if date_format_cfg == "dashes" {
                                    if !month.is_empty() && !day.is_empty() {
                                        date_text = format!("{} - {}", month, day);
                                    } else if !month.is_empty() {
                                        date_text = month.clone();
                                    }
                                } else if !month.is_empty() && !day.is_empty() {
                                    date_text = format!("{} {}", month, day);
                                } else if !month.is_empty() {
                                    date_text = month.clone();
                                }
                            }
                        }
                        if !date_text.is_empty() {
                            let tmpl = build_date_page_text_centre_template(&config);
                            let safe = date_text.replace('"', "\\\"");
                            header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                        }
                    }

                    let watermark = clean_text(
                        stamp
                            .attrs
                            .get("watermark")
                            .unwrap_or(&"".to_string())
                            .trim(),
                    )
                    .replace("\n", " ")
                    .replace("  ", " ");
                    if config.include_watermark.unwrap_or(true) && !watermark.is_empty() {
                        let tmpl = build_watermark_page_text_centre_template(&config);
                        let text = format!("Watermark: {}", watermark);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                    // Prefer an explicit `engraving` attribute from the DB if present.
                    // Some data sources put engraving into the separate `engraving` column
                    // while others append it to the `design` column as "... Engraving: ...".
                    let design_field = stamp
                        .attrs
                        .get("design")
                        .unwrap_or(&"".to_string())
                        .replace("\n", " ")
                        .replace("  ", " ");
                    // Extract design portion before any embedded "Engraving:" or "Perforation:"
                    let design_raw = design_field
                        .split("Engraving:")
                        .next()
                        .unwrap_or("")
                        .split("Perforation:")
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();

                    // Perforation value: prefer explicit DB column, otherwise try to parse
                    // it from the `design` field (some sources embed "Perforation: ...").
                    let perforation_from_column = stamp
                        .attrs
                        .get("perforation")
                        .map(|s| s.trim())
                        .unwrap_or("");
                    let perforation_raw = if !perforation_from_column.is_empty() {
                        perforation_from_column.to_string()
                    } else {
                        design_field
                            .split("Perforation:")
                            .nth(1)
                            .map(|p| {
                                p.split("Engraving:")
                                    .next()
                                    .unwrap_or("")
                                    .trim()
                                    .to_string()
                            })
                            .unwrap_or_default()
                    };
                    let perforation = clean_text(&perforation_raw);
                    if config.include_perforation.unwrap_or(true) && !perforation.is_empty() {
                        let tmpl = build_page_text_template(&config, "centre");
                        let text = format!("Perforation: {}", perforation);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }

                    // engraving may be stored separately in the `engraving` attr; prefer that
                    let engraving_from_column =
                        stamp.attrs.get("engraving").map(|s| s.trim()).unwrap_or("");
                    let engraving_raw = if !engraving_from_column.is_empty() {
                        engraving_from_column.to_string()
                    } else {
                        // fallback: try to parse engraving from the design_field
                        design_field
                            .split("Engraving:")
                            .nth(1)
                            .map(|p| {
                                p.split("Perforation:")
                                    .next()
                                    .unwrap_or("")
                                    .trim()
                                    .to_string()
                            })
                            .unwrap_or_default()
                    };
                    let design = clean_text(&design_raw);
                    let engraving = clean_text(&engraving_raw);
                    if config.include_engraving.unwrap_or(true) && !engraving.is_empty() {
                        let tmpl = build_engraving_page_text_centre_template(&config);
                        let text = format!("Engraving: {}", engraving);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                    if config.include_design.unwrap_or(true) && !design.is_empty() {
                        let tmpl = build_page_text_template(&config, "centre");
                        let text = format!("Design: {}", design);
                        let safe = text.replace('"', "\\\"");
                        header_lines_vec.push((tmpl.replace("{}", &safe), header_font_h));
                    }
                }

                for (line, height) in &mut header_lines_vec {
                    *height = layout::page_text_height(line, *height);
                }
                let header_block_height: f64 = header_lines_vec.iter().map(|(_, h)| *h).sum();
                let vspace_before_header = config
                    .header_vspace_before_mm
                    .unwrap_or(cfg_defaults.header_vspace_before_mm.unwrap())
                    as f64;
                let vspace_after_header = config
                    .header_vspace_after_mm
                    .unwrap_or(cfg_defaults.header_vspace_after_mm.unwrap())
                    as f64;
                // Calculate height of first row
                let stamps_in_row = &stamp_rows[row_idx];
                let mut max_row_height = 0.0f64;
                for stamp in stamps_in_row {
                    let width = stamp.est_width_mm.unwrap_or(reference_width_mm) + extra_width_mm + stamp_box_adjust;
                    let height = stamp.est_height_mm.unwrap_or(reference_height_mm) + extra_height_mm + stamp_box_adjust;
                    max_row_height = max_row_height.max(stamp_content_height(stamp, &config, width, height));
                }
                let vertical_safety = config
                    .packing_vertical_safety_mm
                    .unwrap_or(cfg_defaults.packing_vertical_safety_mm.unwrap())
                    as f64;
                let total_header_and_row_height = vspace_before_header
                    + header_block_height
                    + vspace_after_header
                    + max_row_height
                    + row_font_spacing
                    + vertical_safety; // include vertical safety padding
                                       // Reserve extra room for footers/AlbumEasy page decorations to avoid overlap
                                       // Bumped to 11mm (was 10.0) to avoid a subtle footer bleed observed in rendered PDFs.
                                       // This +1mm tweak is intentionally small to preserve compact layouts while
                                       // matching AlbumEasy's rendering metrics more closely.
                let conservative_buffer = config.page_fit_conservative_buffer_mm
                    .unwrap_or(cfg_defaults.page_fit_conservative_buffer_mm.unwrap_or(2.0)) as f64;
                let conservative_needed = total_header_and_row_height + conservative_buffer;
                // debug!("[PAGE FIT CHECK] Header block height: {:.2}, vspace before: {:.2}, vspace after: {:.2}, max row height: {:.2}, total needed: {:.2}, buffer: {:.2}, current: {:.2}, usable: {:.2}, will_fit: {}",
                // header_block_height, vspace_before_header, vspace_after_header, max_row_height, total_header_and_row_height, conservative_buffer, current_height, usable_height, (current_height + conservative_needed <= usable_height));
                // Peek ahead: if this is a single-row series and next series is also single-row and both fit, combine
                let force_new_page = current_height + conservative_needed > usable_height;
                // Diagnostic: write page-fit numbers to the output so we can inspect why
                // a forced page break may be happening repeatedly and preventing stamps
                // from being emitted.
                // Sanitize series to a single line and escape quotes for diagnostics
                let safe_series = clean_text(series)
                    .replace('\n', " ")
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .replace('"', "'");
                let will_fit = current_height + conservative_needed <= usable_height;
                let _ = writeln!(output, "# PAGE_FIT series=\"{}\" need=\"{:.2}\" current=\"{:.2}\" usable=\"{:.2}\" will_fit=\"{}\"",
                        safe_series, conservative_needed, current_height, usable_height, will_fit);
                // If enabled, emit a detailed breakdown of the header and per-stamp
                // contributions that make up the conservative_needed value so we
                // can debug any under-counting.
                if config.detailed_page_fit_diagnostics.unwrap_or(false) {
                    // Detailed diagnostics are enabled, but we no longer emit the
                    // verbose per-stamp or state comments into the album output by default.
                    // Keep the flag in place so callers can enable other diagnostics if needed.
                }
                // Note: peek-ahead optimization for combining single-row series
                // was removed because it introduced complex control flow and
                // syntax issues during the merge. We keep the simpler
                // conservative page-fit check instead.
                if force_new_page {
                    // start a new page using the centralized layout helper
                    let _ = crate::album_globals::generator_new::layout::start_stamp_page(
                        &mut output,
                        &mut page_counter,
                        &config,
                    );

                    current_height = 0.0;
                    rows_on_page = 0;
                    // debug!("Forced page break before header and first row");
                    // forced page break: state tracked internally but not emitted
                }
                // Buffer the header lines (do not write yet). We'll write
                // them only when we actually emit a row on this page so
                // they don't get stranded on a previous page.
                header_pending =
                    Some((header_lines_vec, vspace_before_header, vspace_after_header));
            }
            // Calculate row height: image height + spacing + text below stamp
            let stamps_in_row = &stamp_rows[row_idx];
            let mut max_row_height = 0.0f64;
            for stamp in stamps_in_row {
                let width = stamp.est_width_mm.unwrap_or(series_avg_w_mm.unwrap_or(reference_width_mm)) + extra_width_mm + stamp_box_adjust;
                let height = stamp.est_height_mm.unwrap_or(series_avg_h_mm.unwrap_or(reference_height_mm)) + extra_height_mm + stamp_box_adjust;
                max_row_height = max_row_height.max(stamp_content_height(stamp, &config, width, height));
            }
            let vertical_safety = config
                .packing_vertical_safety_mm
                .unwrap_or(cfg_defaults.packing_vertical_safety_mm.unwrap())
                as f64;
            let series_gap = if rows_on_page > 0 && row_idx > 0 {
                config.series_row_gap_mm.unwrap_or(0.0).max(0.0) as f64
            } else {
                0.0
            };
            let total_row_height = max_row_height + row_font_spacing + vertical_safety + series_gap;
            // ROW_START_ES emits a line with configured vspace; account for it
            // in fit checks and consumed height to avoid stranded headers.
            let row_start_es_height = config.row_start_es_vspace
                .unwrap_or(cfg_defaults.row_start_es_vspace.unwrap()) as f64;
            // A freshly started page already emits one ROW_START_ES from
            // `start_stamp_page`; include that when placing the first row.
            let page_entry_row_start_es_height = if rows_on_page == 0 {
                row_start_es_height
            } else {
                0.0
            };
            let row_start_total_height = row_start_es_height + page_entry_row_start_es_height;

            // Determine configured max rows per page (preference). Use canonical default.
            let cfg_max_rows = config
                .max_rows_per_page
                .unwrap_or(cfg_defaults.max_rows_per_page.unwrap())
                as i32;
            // Check if this row fits before adding, and if max rows per page reached
            // Use the effective usable height when deciding if the row fits on the
            // current page. This reserves space for footers/decoration and avoids
            // rows that visually overlap the footer in AlbumEasy PDFs.
            if current_height + row_start_total_height + total_row_height > effective_usable || rows_on_page >= cfg_max_rows
            {
                // If the row (including header) is larger than a single page's usable
                // height, we must still emit it once rather than endlessly starting
                // new pages. In that case, if we're mid-page start a new page and
                // fall through to emit the row; otherwise (we're already at a fresh
                // page) allow emission even though it will overflow the page.
                if total_row_height > usable_height {
                    // Force a new page if we already have rows on this page,
                    // otherwise proceed to emit the (oversized) row on the
                    // current fresh page so we don't loop forever.
                    // If configured to allow single-large-stamp pages, and the
                    // oversized row exceeds the configured fraction of usable
                    // height, permit it to occupy a fresh page instead of
                    // forcing additional page breaks.
                    let allow_single = config.allow_single_large_stamp_per_page.unwrap_or(true);
                    let threshold = config.single_stamp_threshold_pct.unwrap_or(0.6) as f64;
                    let percent_of_usable = total_row_height / effective_usable;
                    if rows_on_page != 0 && !(allow_single && percent_of_usable >= threshold) {
                        page_counter += 1;
                        // use the new layout helper which expects a mutable page counter
                        let mut pc = page_counter;
                        let _ = layout::start_stamp_page(&mut output, &mut pc, &config);
                        page_counter = pc;
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                    } else if rows_on_page != 0 && (allow_single && percent_of_usable >= threshold)
                    {
                        // We will allow the oversized row to be emitted as a
                        // single-row page: start a fresh page and let it emit.
                        page_counter += 1;
                        let mut pc = page_counter;
                        let _ = layout::start_stamp_page(&mut output, &mut pc, &config);
                        page_counter = pc;

                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                    }
                    // Don't continue; fall through to emit the row even if it
                    // doesn't fully fit in the usable height. This avoids an
                    // infinite loop of empty page headers.
                } else {
                    // Row is larger than effective_usable but not larger than
                    // full usable_height. On a fresh page, emit once rather than
                    // looping forever with repeated page breaks.
                    if rows_on_page != 0 {
                        // Start new page and repeat header then re-check on next loop.
                        let _ =
                            layout::start_stamp_page(&mut output, &mut page_counter, &config);
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                        continue; // re-check header on next loop
                    }
                }
            }
            // If we have a pending header, check again whether writing the
            // header + this row (plus the conservative buffer) would overflow
            // the usable page area. If so, force a new page before emitting
            // anything so the header and row start cleanly on the next page.
            if let Some((pending_lines_ref, v_before, v_after)) = &header_pending {
                let conservative_buffer_cfg = config
                    .page_fit_conservative_buffer_mm
                    .unwrap_or(cfg_defaults.page_fit_conservative_buffer_mm.unwrap())
                    as f64;
                let header_block_height: f64 = pending_lines_ref.iter().map(|(_, h)| *h).sum();
                let needed_if_emitted = v_before
                    + header_block_height
                    + v_after
                    + row_start_total_height
                    + total_row_height
                    + conservative_buffer_cfg;
                // If the header+row would not fit on the page at all (oversized),
                // allow emission on a fresh page rather than endlessly forcing new
                // pages. If we're mid-page, start a new page so the oversized
                // content can be emitted there; otherwise allow emission.
                if needed_if_emitted > usable_height {
                    if rows_on_page != 0 {
                        // Start a fresh stamp page using the centralized layout helper.
                        // `start_stamp_page` will increment the page counter and
                        // emit the standard header and separators.
                        let _ = layout::start_stamp_page(&mut output, &mut page_counter, &config);
                        current_height = 0.0;
                        header_written = false;
                        rows_on_page = 0;
                        // Do not `continue` here: fall through and emit the oversized
                        // header+row on the fresh page to avoid infinite loop.
                    }
                } else if current_height + needed_if_emitted > usable_height {
                    // Normal case: header+row would overflow current page; force a new page now
                    // Use the centralized helper which increments the page counter
                    // and emits the standard header/separators and ROW_START_ES.
                    let _ = layout::start_stamp_page(&mut output, &mut page_counter, &config);
                    current_height = 0.0;
                    header_written = false;
                    rows_on_page = 0;
                    // Re-check emission of the same row on next loop iteration
                    continue;
                }
            }
            // If we have a pending header, write it now (we are about to
            // emit the first row for this series on the current page).
            if let Some((pending_lines, v_before, v_after)) = header_pending.take() {
                // Delegate header emission to layout helper which writes the
                // page vspace markers and header lines and returns the
                // total vertical millimetre height consumed.
                match crate::album_globals::generator_new::layout::write_header_block(
                    &mut output,
                    &pending_lines,
                    v_before,
                    v_after,
                    &config,
                ) {
                    Ok(h) => current_height += h,
                    Err(e) => {
                        log::error!("Failed to write header block: {}", e);
                        // Fallback: pretend no height was written so generation continues
                    }
                }
                header_written = true;
            }
            // Build ROW_START_ES from parameterized config if provided, otherwise fall back to
            // the legacy `row_start_es_marker` string. Parameters are: font, size, spacing, vspace
            // Before emitting the row, compute row metadata (width, stamps) so
            // we can optionally emit diagnostics or a per-row horizontal offset
            // to center the row when configured.
            {
                let stamps_in_row = &stamp_rows[row_idx];
                let mut row_w = 0.0f64;
                let margin = config.packing_horizontal_safety_mm.unwrap_or(5.0) as f64;
                let stamp_box_adjust_row = config.stamp_box_adjust.unwrap_or(0.0) as f64;
                for s in stamps_in_row.iter() {
                    let est_w = s.est_width_mm.unwrap_or(reference_width_mm);
                    let emit_w = est_w + extra_width_mm + stamp_box_adjust_row;
                    row_w += emit_w + margin;
                }
                // The computed row_w currently includes a trailing margin; subtract one margin
                // so row width represents the true occupied width.
                if !stamps_in_row.is_empty() {
                    row_w -= margin;
                }
                let usable_w = page_width
                    - config.page_left_margin_mm.unwrap_or(10.0) as f64
                    - config.page_right_margin_mm.unwrap_or(10.0) as f64
                    - config.album_pages_spacing_h.unwrap_or(0.0) as f64
                    - (border_outer + border_inner1 + border_inner2 + border_spacing);
                let extra = usable_w - row_w;
                let left_offset = if extra > 0.0 { extra / 2.0 } else { 0.0 };
                if config.show_stamps_per_row.unwrap_or(false) {
                    let _ = writeln!(output, "# ROW_META stamps=\"{}\" row_w=\"{:.2}\" usable=\"{:.2}\" extra=\"{:.2}\" left_offset=\"{:.2}\"",
                        stamps_in_row.len(), row_w, usable_w, extra, left_offset);
                }
                // Row centering requested but generator-side filler disabled: do not
                // emit filler images or mutate album images. Keep ROW_META diagnostics
                // (if enabled) but avoid generating spacer artifacts that change layout.
            }

            if series_gap > 0.0 {
                if let Err(error) = layout::write_page_vspace(&mut output, series_gap) {
                    log::error!("Failed to write series row spacing: {}", error);
                }
            }
            if let Err(error) = writeln!(output, "# ROW_HEIGHT mm={:.4}",
                total_row_height + row_start_total_height - series_gap) {
                log::error!("Failed to write row height measurement: {}", error);
            }
            let _ = crate::album_globals::generator_new::layout::emit_row_start(&mut output, RowSpacingMode::FS, &config);

            // row start written (diagnostics suppressed)
            for stamp in stamps_in_row {
                // Use correct fields from StampStub
                // Use image_path from the database directly
                let image_path = stamp.image_path.trim();
                let full_image_path = crate::file_utils::resolve_album_image_path(album_name, image_path)
                    .to_string_lossy()
                    .to_string();
                // Use precomputed estimates from the pre-pass so row packing,
                // page-fit checks, and emitted dimensions stay in sync.
                let mut est_width_mm = stamp
                    .est_width_mm
                    .unwrap_or(series_avg_w_mm.unwrap_or(reference_width_mm ));
                let mut est_height_mm = stamp
                    .est_height_mm
                    .unwrap_or(series_avg_h_mm.unwrap_or(reference_height_mm ));
                // Apply extra_width_mm/extra_height_mm adjustments to width/height (additive)
                est_width_mm += extra_width_mm;
                est_height_mm += extra_height_mm;
                // Debug: emit detailed per-stamp sizing diagnostics when debug logging enabled
                // debug!("Stamp [{}|{}] DB_px=({:?},{:?}) mm_per_px=({:.6},{:.6}) pre_scale_mm=({:.2},{:.2}) scale={:.4} img_new=(w:{:.2},h:{:.2}) box_adjust={:.2} emit_mm_before_adjust=({:.2},{:.2}) emit_mm_final=({:.2},{:.2})",
                //         album_name,
                //         stamp.attrs.get("sg_number").unwrap_or(&String::new()),
                //         stamp.image_width_px,
                //         stamp.image_height_px,
                //         mm_per_pixel_w as f64,
                //         mm_per_pixel_h as f64,
                //         (est_width_mm - img_new_w) / (if scale==0.0 {1.0} else {scale}), // approximate pre-scale width
                //         (est_height_mm - img_new_h) / (if scale==0.0 {1.0} else {scale}), // approximate pre-scale height
                //         scale,
                //         img_new_w,
                //         img_new_h,
                //         stamp_box_adjust,
                //         est_width_mm - 2.0 * stamp_box_adjust,
                //         est_height_mm - 2.0 * stamp_box_adjust,
                //         est_width_mm + 2.0 * stamp_box_adjust,
                //         est_height_mm + 2.0 * stamp_box_adjust
                //     );
                // add stamp_box_adjust to the emitted box width/height (STAMP_BOXES_SIZE_ADJUST)
                let raw_emit_width = (est_width_mm + stamp_box_adjust).max(0.1);
                let raw_emit_height = (est_height_mm + stamp_box_adjust).max(0.1);
                // Prevent zero/negative dimensions, which can flip or invalidate output.
                let emit_width = raw_emit_width;
                let emit_height = raw_emit_height;
                let StampText { left, centre, right, raw_left, raw_stamps_issued, wrapped_desc, raw_color } =
                    build_stamp_text(stamp, &config, emit_width, emit_height);

                // Right: mint_condition (or unused), used, stamps_issued
                let mint_raw = stamp
                    .attrs
                    .get("mint_condition")
                    .unwrap_or(&"".to_string())
                    .trim()
                    .to_string();
                let _mint = clean_text(&mint_raw);
                let _unused = clean_text(stamp.attrs.get("unused").unwrap_or(&"".to_string()));

                // Prefer Scott catalog number if available, fall back to SG number
                let sgnum = if let Some(catalog_number) = stamp.attrs.get("catalog_number") {
                    clean_text(catalog_number)
                } else if let Some(scott) = stamp.attrs.get("scott_number") {
                    clean_text(scott)
                } else {
                    clean_text(stamp.attrs.get("sg_number").unwrap_or(&"".to_string()))
                };

                // Resolve heading catalog family from the stamp row first.
                // This keeps STAMP_HEADING aligned with Review UI edits even
                // when in-memory attrs were defaulted earlier in the pipeline.
                let heading_catalog_code = {
                    let default_code = crate::catalog::normalize_catalog_type(
                        &config.effective_default_catalog_code(),
                    );
                    let explicit_code = stamp
                        .attrs
                        .get("catalog_code")
                        .map(|v| crate::catalog::normalize_catalog_type(v))
                        .filter(|v| !v.trim().is_empty());

                    let db_catalog_code = stamp
                        .attrs
                        .get("year")
                        .and_then(|v| v.trim().parse::<i32>().ok())
                        .and_then(|year| {
                            let sg_db = stamp
                                .attrs
                                .get("sg_number")
                                .map(|v| v.trim())
                                .unwrap_or("");
                            if sg_db.is_empty() {
                                return None;
                            }
                            conn.query_row(
                                "SELECT COALESCE(catalog_code, '')
                                 FROM stamps
                                 WHERE series = ?1
                                   AND year = ?2
                                   AND sg_number = ?3
                                 ORDER BY CASE WHEN image_path = ?4 THEN 0 ELSE 1 END, rowid DESC
                                 LIMIT 1",
                                [series, &year.to_string(), sg_db, image_path],
                                |row| row.get::<_, String>(0),
                            )
                            .ok()
                            .map(|v| crate::catalog::normalize_catalog_type(&v))
                            .filter(|v| !v.trim().is_empty())
                        });

                    db_catalog_code
                        .or(explicit_code)
                        .unwrap_or(default_code)
                };
                let heading_label = if config.include_catalog_numbers.unwrap_or(true) {
                    crate::album_globals::generator_new::emit::resolve_stamp_heading_label(
                        &config,
                        Some(heading_catalog_code.as_str()),
                    )
                } else {
                    String::new()
                };
                let heading_diag_sg = sgnum.replace('"', "'");
                let heading_diag_code = heading_catalog_code.replace('"', "'");
                let heading_diag_label = heading_label.replace('"', "'");
                let heading_diag_series = clean_text(series).replace('"', "'");
                let _ = writeln!(
                    output,
                    "# HEADING_DIAG sg=\"{}\" catalog_code=\"{}\" label=\"{}\" series=\"{}\"",
                    heading_diag_sg, heading_diag_code, heading_diag_label, heading_diag_series
                );

                let _safe_sg = left
                    .replace("\\n", " ")
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .replace("\"", "'");
                let _safe_img = image_path.replace("\"", "'");

                if !image_path.is_empty() && Path::new(&full_image_path).exists() {
                    let emit_type = "standard";
                    if is_us1847 {
                        if emit_type == "image" {
                            // Emit image
                            let line = format!(
                                "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"\" \"\" \"\")",
                                emit_width, emit_height, image_path
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        } else if emit_type == "scott" {
                            // Emit Scott number as text
                            let line = format!(
                                "STAMP_ADD ({:.1} {:.1} \"{}\" \"HN\" \"12\" \"black\")",
                                emit_width, 15.0, sgnum
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        } else if emit_type == "info" {
                            // Emit stamp info as text box
                            let info = format!(
                                "Denomination: {}\\nDescription: {}\\nColor: {}\\nIssued: {}",
                                raw_left, wrapped_desc, raw_color, raw_stamps_issued
                            );
                            let line = format!(
                                "STAMP_ADD ({:.1} {:.1} \"{}\" \"HN\" \"8\" \"black\")",
                                emit_width, 40.0, info.replace("\n", "\\n").replace("\"", "'")
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        } else if emit_type == "standard" {
                            // Emit image
                            let line = format!(
                                "STAMP_ADD_IMG ({:.1} {:.1} \"{}\" \"\" \"\" \"\")",
                                emit_width, emit_height, image_path
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                            // Emit Scott number as text
                            let line = format!(
                                "STAMP_ADD ({:.1} {:.1} \"{}\" \"HN\" \"12\" \"black\")",
                                emit_width, 15.0, sgnum
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                            // Emit stamp info as text box
                            let info = format!(
                                "Denomination: {}\\nDescription: {}\\nColor: {}\\nIssued: {}",
                                raw_left, wrapped_desc, raw_color, raw_stamps_issued
                            );
                            let line = format!(
                                "STAMP_ADD ({:.1} {:.1} \"{}\" \"HN\" \"8\" \"black\")",
                                100.0, 40.0, info.replace("\n", "\\n").replace("\"", "'")
                            );
                            let _ = writeln!(output, "{}", line.replace("\r", "").replace("\n", "\\n"));
                        }
                    } else {
                        // Standard formatting
                        // Delegate formatted emission to the centralized emitter
                        // so formatting and escaping are consistent across call sites.
                        let sh_font = config
                            .stamp_heading_font
                            .clone()
                            .unwrap_or_else(|| "HN".to_string());
                        let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
                        let stamp_name = stamp.attrs.get("colnect_name").map(|s| s.as_str());
                        let heading_perforation = stamp
                            .attrs
                            .get("colnect_perforation")
                            .or_else(|| stamp.attrs.get("perforation"))
                            .map(|s| s.as_str());
                        let ctx = crate::album_globals::generator_new::emit::EmitContext {
                            emit_width,
                            emit_height,
                            image_path,
                            left: &left,
                            centre: &centre,
                            right: &right,
                            sh_font: &sh_font,
                            sh_size,
                            sh_label: &heading_label,
                            sgnum: &sgnum,
                            stamp_name,
                            perforation: heading_perforation,
                            include_perforation: config.include_perforation.unwrap_or(true),
                        };
                        let _ = crate::album_globals::generator_new::emit::emit_stamp(&mut output, true, &ctx);
                    }
                } else {
                    let _safe_series = clean_text(series)
                        .replace("\\n", " ")
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .replace("\"", "'");
                    let _safe_sg = left
                        .replace("\\n", " ")
                        .split_whitespace()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .replace("\"", "'");
                    let _safe_img = image_path.replace("\"", "'");

                    // Try to generate a per-stamp placeholder image when the real image
                    // is missing or points at a common placeholder. If generation fails
                    // fall back to the textual "Image Not Available" box.
                    let mut emitted_placeholder: Option<String> = None;
                    // Determine if we should attempt to create a placeholder
                    let should_generate = image_path.is_empty()
                        || !Path::new(&full_image_path).exists()
                        || image_path.starts_with("images/no_image");
                    if should_generate {
                        // Year from attributes (fallback to empty)
                        let year_attr = stamp.attrs.get("year").unwrap_or(&"".to_string()).clone();
                        let year_clean = clean_text(&year_attr);
                        // Determine a better target size for the placeholder. Prefer the
                        // current emitted dimensions, but if this stamp lacks image
                        // dimensions try to use neighbouring stamps on the same row as
                        // a second-level fallback so the placeholder visually matches
                        // its neighbours.
                        // Default target for placeholder creation: the image-only
                        // estimated width/height (do NOT include the per-side
                        // `stamp_box_adjust` here). If neighbours are used as a
                        // fallback, use their raw image mm values (not the
                        // emitted box size) so placeholders match neighbour
                        // image areas rather than the printed box padding.
                        let mut target_w_mm = stamp.est_width_mm.unwrap_or(reference_width_mm);
                        let mut target_h_mm = stamp.est_height_mm.unwrap_or(reference_height_mm);
                        // If this stamp has no explicit image pixel dimensions,
                        // attempt to compute an average from neighbours in the row.
                        if stamp.image_width_px.is_none() && stamp.image_height_px.is_none() {
                            let mut neigh_w_sum = 0.0f64;
                            let mut neigh_h_sum = 0.0f64;
                            let mut neigh_count = 0u32;
                            // stamps_in_row is available in this scope and refers to the
                            // vector of stamps in the current row; iterate to collect
                            // neighbor sizes when available.
                            for neigh in stamps_in_row.iter() {
                                // skip self
                                let neigh_sg = neigh
                                    .attrs
                                    .get("sg_number")
                                    .unwrap_or(&"".to_string())
                                    .clone();
                                if neigh_sg
                                    == stamp
                                        .attrs
                                        .get("sg_number")
                                        .unwrap_or(&"".to_string())
                                        .clone()
                                {
                                    continue;
                                }
                                // prefer DB-stored pixel dims
                                if let (Some(nw_px), Some(nh_px)) =
                                    (neigh.image_width_px, neigh.image_height_px)
                                {
                                    let mut nh = (nh_px as f64) * (mm_per_pixel_h as f64);
                                    if fit_box && nh > target_stamp_height_mm {
                                        nh = target_stamp_height_mm;
                                    }
                                    let nw = (nw_px as f64) * (mm_per_pixel_w as f64);
                                    neigh_w_sum += nw;
                                    neigh_h_sum += nh;
                                    neigh_count += 1;
                                } else {
                                    // try to estimate from actual image file if present
                                    let neigh_img = neigh.image_path.trim();
                                    let neigh_full = format!(
                                        "Albums/{}/{}",
                                        album_name.replace(' ', "-"),
                                        neigh_img
                                    );
                                    if let Some((wmm, hmm)) = estimate_real_dimensions(
                                        &neigh_full,
                                        mm_per_pixel_w,
                                        mm_per_pixel_h,
                                    ) {
                                        let mut nh = hmm as f64;
                                        if fit_box && nh > target_stamp_height_mm {
                                            nh = target_stamp_height_mm;
                                        }
                                        neigh_w_sum += wmm as f64;
                                        neigh_h_sum += nh;
                                        neigh_count += 1;
                                    }
                                }
                            }
                            if neigh_count > 0 {
                                // Neighbour sums are based on the raw image physical
                                // dimensions (mm). The emitted stamp box includes the
                                // per-side `stamp_box_adjust` margin (added twice,
                                // once per side) so include that here so the
                                // placeholder is generated at the same printed
                                // dimensions as neighbouring stamps.
                                let neigh_w_mm = neigh_w_sum / (neigh_count as f64);
                                let neigh_h_mm = neigh_h_sum / (neigh_count as f64);
                                // Use neighbour image-only sizes (no stamp_box_adjust)
                                target_w_mm = neigh_w_mm;
                                target_h_mm = neigh_h_mm;
                            }
                        }

                        // sgnum already contains Scott number (or SG fallback), use for both DB and filename
                        let _sg_for_db = stamp.attrs.get("sg_number").map(|v| v.as_str()).unwrap_or("");
                        let ph_call_opts = crate::album_globals::generator_new::helpers::PlaceholderCallOptions {
                            album_name,
                            series_name: Some(series),
                            catalog_number: &sgnum,
                            year: &year_clean,
                            target_w_mm: Some(target_w_mm),
                            target_h_mm: Some(target_h_mm),
                            mm_per_pixel_w: Some(mm_per_pixel_w),
                            mm_per_pixel_h: Some(mm_per_pixel_h),
                        };
                        if let Some((rel_path, _, _)) = crate::album_globals::generator_new::helpers::ensure_placeholder_for_stamp_and_update_db(
                            conn,
                            &ph_call_opts,
                            Some(&mut output),
                        ) {
                            emitted_placeholder = Some(rel_path.clone());
                        }
                    }

                    if let Some(img_rel) = emitted_placeholder {
                        // We have a generated placeholder; emit STAMP_ADD_IMG using it
                        let sh_font = config
                            .stamp_heading_font
                            .clone()
                            .unwrap_or_else(|| "HN".to_string());
                        let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
                        let stamp_name = stamp.attrs.get("colnect_name").map(|s| s.as_str());
                        let heading_perforation = stamp
                            .attrs
                            .get("colnect_perforation")
                            .or_else(|| stamp.attrs.get("perforation"))
                            .map(|s| s.as_str());
                        let ctx = crate::album_globals::generator_new::emit::EmitContext {
                            emit_width,
                            emit_height,
                            image_path: &img_rel,
                            left: &left,
                            centre: &centre,
                            right: &right,
                            sh_font: &sh_font,
                            sh_size,
                            sh_label: &heading_label,
                            sgnum: &sgnum,
                            stamp_name,
                            perforation: heading_perforation,
                            include_perforation: config.include_perforation.unwrap_or(true),
                        };
                        let _ = crate::album_globals::generator_new::emit::emit_stamp(&mut output, true, &ctx);
                    } else {
                        // Could not create a placeholder; fall back to the textual box
                        let sh_font = config
                            .stamp_heading_font
                            .clone()
                            .unwrap_or_else(|| "HN".to_string());
                        let sh_size = config.stamp_heading_size.unwrap_or(10.0) as i32;
                        let stamp_name = stamp.attrs.get("colnect_name").map(|s| s.as_str());
                        let heading_perforation = stamp
                            .attrs
                            .get("colnect_perforation")
                            .or_else(|| stamp.attrs.get("perforation"))
                            .map(|s| s.as_str());
                        let ctx = crate::album_globals::generator_new::emit::EmitContext {
                            emit_width,
                            emit_height,
                            image_path: "Image\\nNot\\nAvailable",
                            left: &left,
                            centre: &centre,
                            right: &right,
                            sh_font: &sh_font,
                            sh_size,
                            sh_label: &heading_label,
                            sgnum: &sgnum,
                            stamp_name,
                            perforation: heading_perforation,
                            include_perforation: config.include_perforation.unwrap_or(true),
                        };
                        let _ = crate::album_globals::generator_new::emit::emit_stamp(&mut output, false, &ctx);
                        let _ = writeln!(output, "# PAGE_FIT_STATE step=\"stamp_emitted\" series=\"{}\" img=\"{}\" emit_h=\"{:.2}\" current=\"{:.2}\"",
                                clean_text(series).replace('\n', " ").replace('"', "'"), image_path, emit_height, current_height + total_row_height);
                    }
                }
            }

            current_height += total_row_height + row_start_total_height;
            rows_on_page += 1;
            row_idx += 1;
        }
    }).expect("for_each_series failed");

    // Finalize: we've written to a temp file. Flush buffer and promote it into place only if it
    // contains STAMP lines; otherwise remove the temp and leave existing file intact.
    // This prevents accidental truncation when generation yields no stamps.
    {
        // Ensure buffered output is flushed before we read the temp file
        let _ = output.flush();
        // Post-process the temp output to insert vertical centering markers
        // (PAGE_VSPACE (n)) at the top of each stamp page where there is
        // unused vertical space. This keeps centering purely vertical and
        // avoids inserting filler images or modifying page content otherwise.
        if let Err(e) = center_tmp_pages(&tmp_path, &config, effective_usable) {
            log::warn!("Page-centering post-process failed: {}", e);
        }
        use std::fs;
        match fs::read_to_string(&tmp_path) {
            Ok(contents) => {
                let stamp_count = contents.lines().filter(|l| l.starts_with("STAMP")).count();

                if stamp_count == 0 {
                    // If the user explicitly requested overwrite, create a backup
                    // of the existing file even if generation produced no STAMP lines.
                    if crate::file_utils::get_overwrite_album_from_cli() && output_path.exists() {
                        // use chrono::Local;
                        if let Some(stem_os) = output_path.file_stem() {
                            if let Some(stem) = stem_os.to_str() {
                                let ts = crate::time_utils::formatted_now("%Y%m%d-%H%M%S");
                                let ext = output_path
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("");
                                let backup_name = if ext.is_empty() {
                                    format!("{}.created-{}", stem, ts)
                                } else {
                                    format!("{}.created-{}.{}", stem, ts, ext)
                                };
                                let backup_path = output_path.with_file_name(backup_name);
                                match fs::copy(&output_path, &backup_path) {
                                    Ok(_) => info!(
                                        "Created backup of existing file at {} (requested overwrite)",
                                        backup_path.display()
                                    ),
                                    Err(e) => error!(
                                        "Failed to create backup before overwrite (requested) : {}",
                                        e
                                    ),
                                }
                            }
                        }
                    }
                    info!(
                        "No stamp lines produced for {} ({}). Leaving existing file intact.",
                        album_name,
                        output_path.display()
                    );
                    let _ = fs::remove_file(&tmp_path);
                } else {
                    // Create a timestamped backup of the existing file if present
                    if output_path.exists() {
                        // use chrono::Local;
                        if let Some(stem_os) = output_path.file_stem() {
                            if let Some(stem) = stem_os.to_str() {
                                let ts = crate::time_utils::formatted_now("%Y%m%d-%H%M%S");
                                let ext = output_path
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("");
                                let backup_name = if ext.is_empty() {
                                    format!("{}.created-{}", stem, ts)
                                } else {
                                    format!("{}.created-{}.{}", stem, ts, ext)
                                };
                                let backup_path = output_path.with_file_name(backup_name);
                                match fs::copy(&output_path, &backup_path) {
                                    Ok(_) => info!(
                                        "Created backup of existing file at {}",
                                        backup_path.display()
                                    ),
                                    Err(e) => {
                                        error!("Failed to create backup before overwrite: {}", e)
                                    }
                                }
                            }
                        }
                    }
                    // Atomically move temp into place
                    match fs::rename(&tmp_path, &output_path) {
                        Ok(_) => {
                            info!(
                                "Successfully wrote album output to {} ({} stamps)",
                                output_path.display(),
                                stamp_count
                            );
                        }
                        Err(e) => {
                            error!("Failed to move temp output into place: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                error!(
                    "Failed to read generated temp file {}: {}",
                    tmp_path.display(),
                    e
                );
                let _ = fs::remove_file(&tmp_path);
            }
        }
    }

    // Persist dedupe audit for Phase A+B matcher decisions.
    let audit_path = album_dir.join("merge_dedupe_audit.jsonl");
    let mut audit_lines: Vec<String> = Vec::new();
    audit_lines.push(format!(
        "{{\"event\":\"summary\",\"album\":\"{}\",\"dropped\":{},\"seen_fingerprints\":{}}}",
        json_escape(album_name),
        dedupe_dropped,
        seen_stamp_fingerprints.len()
    ));
    audit_lines.extend(dedupe_audit);
    if let Err(e) = std::fs::write(&audit_path, audit_lines.join("\n") + "\n") {
        warn!(
            "Failed to write dedupe audit file {}: {}",
            audit_path.display(),
            e
        );
    } else {
        info!("Wrote dedupe audit file {}", audit_path.display());
    }

    output_path // Return the output file path

    // End of generate_album_output inner scope: use shared clean_text from album_utils
}

/// Backwards-compatible wrapper: accept a DB path, open the DB and call the
/// connection-based implementation. This keeps existing callers (tests and
/// other modules) working while allowing callers that already hold a
/// Connection to use the more efficient conn-based variant.
/// Returns the path to the generated output file.
pub async fn generate_album_output<F>(
    db_path: &str,
    album_name: &str,
    page_callback: Option<F>,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> std::path::PathBuf
where
    F: FnMut(usize),
{
    let conn = init_db(db_path).expect("Failed to open database");
    generate_album_output_conn(&conn, album_name, page_callback, start_year, end_year).await
}

/// Post-process the temporary album text file and insert a numeric
/// PAGE_VSPACE (n) after each PAGE_START for stamp pages so the page's
/// content is vertically centered where possible. This is a best-effort
/// purely-generator-side adjustment that uses conservative estimates of
/// header and row heights (matching the generator heuristics).
fn center_tmp_pages(
    tmp_path: &Path,
    config: &crate::config::Config,
    usable_height: f64,
) -> std::io::Result<()> {
    crate::album_globals::generator_new::postprocess::center_tmp_pages(
        tmp_path,
        config,
        usable_height,
    )
}

#[cfg(test)]
mod layout_tests {
    use super::*;
    use crate::album_globals::structs::GenericAttrs;

    fn rich_stamp() -> Stamp {
        let mut attrs = GenericAttrs::new();
        for (key, value) in [
            ("sg_number", "9"),
            ("catalog_code", "sg"),
            ("denomination", "5R"),
            ("stamps_issued", "42323"),
            (
                "description",
                "Portrait of Isabel II, (facing right) in dark coloured oval within oval banner with wording FRANCO SEIS CUARTOS CORREOS 1851",
            ),
            ("color", "rose"),
            ("mint_condition", "2247"),
            ("used", "280"),
            ("colnect_name", "Queen Isabel II"),
            ("colnect_printing", "Typography"),
            ("colnect_format", "Stamp"),
            ("colnect_emission", "Definitive"),
            ("colnect_paper", "Normal paper"),
            ("colnect_colors", "Black"),
            ("colnect_watermark", "No Watermark"),
            ("colnect_issued_on", "1851-01-01"),
            ("colnect_perforation", "Imperforate"),
        ] {
            attrs.set(key, value);
        }
        Stamp {
            attrs,
            image_path: String::new(),
            image_width_px: None,
            image_height_px: None,
            est_width_mm: Some(21.0),
            est_height_mm: Some(26.0),
        }
    }

    fn rich_config() -> crate::config::Config {
        toml::from_str(
            r#"
stamp_detail_layout = "separate_rows"
stamp_heading_size = 6.0
text_line_height_mm = 2.6
row_start_es_size = 6.0
row_start_es_spacing = 0.1
image_spacing_mm = 5.0
[colnect]
include_details = true
[colnect.fields]
description = false
printing_type = true
format = true
emission = true
paper = true
colors = true
watermark = true
issued_on = true
"#,
        )
        .expect("test configuration")
    }

    #[test]
    fn colnect_height_uses_the_exact_emitted_text_for_every_layout() {
        let stamp = rich_stamp();
        let mut config = rich_config();
        for mode in ["inline", "separate_rows"] {
            config.stamp_detail_layout = Some(mode.to_string());
            let text = build_stamp_text(&stamp, &config, 24.0, 29.0);
            assert_eq!(text.left, "5R");
            assert_eq!(text.raw_stamps_issued, "42,323");
            assert_eq!(count_albumeasy_text_lines(&text.centre), 16);
            assert!(
                text.centre
                    .contains("Printing type: Typography;\\nFormat: Stamp;")
            );
            assert!(text.centre.contains("Paper: Normal paper;"));
            let height = stamp_content_height(&stamp, &config, 24.0, 29.0);
            assert!(
                height > 80.0 && height < 90.0,
                "measured row content: {height}"
            );
            // Spain's 253.7mm usable area accommodates both rich rows, including
            // repeated row-start space, the series header, and a footer buffer.
            assert!(height * 2.0 + 18.0 + 4.0 * 3.83 + 3.0 + 2.0 + 2.2 < 253.7);
        }
        config.stamp_detail_layout = Some("minimal".to_string());
        let minimal = build_stamp_text(&stamp, &config, 24.0, 29.0);
        assert!(!minimal.centre.contains("Typography"));
        assert!(stamp_content_height(&stamp, &config, 24.0, 29.0) < 55.0);
    }

    #[test]
    fn colnect_measurement_respects_hidden_fields_fonts_and_legacy_flags() {
        let mut stamp = rich_stamp();
        let mut config = rich_config();
        let original = stamp_content_height(&stamp, &config, 24.0, 29.0);
        config.include_description = Some(false);
        config.include_color = Some(false);
        config.include_catalog_numbers = Some(false);
        config.include_stamps_issued = Some(false);
        config.include_denomination = Some(false);
        config.include_used = Some(false);
        config.include_mint_unused = Some(false);
        let hidden = build_stamp_text(&stamp, &config, 24.0, 29.0);
        assert!(hidden.left.is_empty());
        assert!(!hidden.centre.contains("Portrait"));
        assert!(!hidden.centre.contains("rose"));
        let compact = stamp_content_height(&stamp, &config, 24.0, 29.0);
        assert!(compact < original);
        for enabled in [false, true] {
            config.stamp_heading_in_row_height = Some(enabled);
            config.pages_vspace_in_row_height = Some(enabled);
            assert_eq!(stamp_content_height(&stamp, &config, 24.0, 29.0), compact);
        }
        config.row_start_es_size = Some(16.0);
        assert!(stamp_content_height(&stamp, &config, 24.0, 29.0) > compact);
        stamp.attrs.set(
            "colnect_paper",
            &"Long metadata with several words ".repeat(30),
        );
        assert!(stamp_content_height(&stamp, &config, 18.0, 100.0) > original);
    }
}
