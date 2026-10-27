//! Formatting utilities for database data

use regex::Regex;

/// Format stamps_issued into thousands-separated form and preserve decimals.
/// Output uses comma for thousands and dot for decimal separator, e.g. "1,234.56".
pub fn format_stamps_issued(s: &str) -> String {
    let t = s.trim();
    if t.is_empty() {
        return String::new();
    }

    // Remove surrounding parentheses
    let mut v = t.to_string();
    if v.starts_with('(') && v.ends_with(')') {
        v = v[1..v.len() - 1].to_string();
    }

    // Keep only digits, dot, comma
    let re_keep = Regex::new(r"[^0-9\.,]").unwrap();
    v = re_keep.replace_all(&v, "").to_string();
    if v.is_empty() {
        return String::new();
    }

    // Determine decimal separator using heuristics:
    let dot_count = v.matches('.').count();
    let comma_count = v.matches(',').count();
    let (int_part, frac_part_opt) = if dot_count > 0 && comma_count > 0 {
        let last_dot = v.rfind('.').unwrap_or(0);
        let last_comma = v.rfind(',').unwrap_or(0);
        if last_dot > last_comma {
            (
                v[..last_dot].to_string(),
                Some(v[last_dot + 1..].to_string()),
            )
        } else {
            (
                v[..last_comma].to_string(),
                Some(v[last_comma + 1..].to_string()),
            )
        }
    } else if dot_count > 0 {
        if dot_count > 1 {
            (v.replace('.', ""), None)
        } else {
            let idx = v.rfind('.').unwrap_or(v.len());
            let right_len = v.len() - idx - 1;
            if right_len == 3 {
                (v.replace('.', ""), None)
            } else {
                (v[..idx].to_string(), Some(v[idx + 1..].to_string()))
            }
        }
    } else if comma_count > 0 {
        if comma_count > 1 {
            (v.replace(',', ""), None)
        } else {
            let idx = v.rfind(',').unwrap_or(v.len());
            let right_len = v.len() - idx - 1;
            if right_len == 3 {
                (v.replace(',', ""), None)
            } else {
                (v[..idx].to_string(), Some(v[idx + 1..].to_string()))
            }
        }
    } else {
        (v.clone(), None)
    };

    let re_digits = Regex::new(r"\D").unwrap();
    let int_digits = re_digits.replace_all(&int_part, "").to_string();
    if int_digits.is_empty() {
        return String::new();
    }

    let rev: Vec<char> = int_digits.chars().rev().collect();
    let mut grouped = String::new();
    for (i, c) in rev.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(*c);
    }
    let int_fmt: String = grouped.chars().rev().collect();

    if let Some(frac_raw) = frac_part_opt {
        let frac_digits = re_digits.replace_all(&frac_raw, "").to_string();
        if frac_digits.is_empty() {
            return int_fmt;
        }
        return format!("{}.{}", int_fmt, frac_digits);
    }

    int_fmt
}
