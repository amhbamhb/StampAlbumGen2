use regex::Regex;
use std::collections::HashSet;

// Normalize stamps_issued values and provide small parser helpers used by
// both runtime code and unit tests.
pub fn normalize_stamps_issued(s: &str) -> String {
    let mut v = s.trim().to_string();
    if v.starts_with('(') && v.ends_with(')') {
        v = v[1..v.len() - 1].to_string();
    }
    // Remove any characters except digits, dot and comma
    let re_non = Regex::new(r"[^0-9\.,]").unwrap();
    v = re_non.replace_all(&v, "").to_string();
    // If dots are used as thousand separators and no commas are present, replace them with commas
    if v.contains('.') && !v.contains(',') {
        v = v.replace('.', ",");
    }
    v
}

// Remove duplicate detail lines while preserving order.
pub fn dedupe_details(details: &[String]) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for d in details {
        let t = d.trim().to_string();
        if t.is_empty() {
            continue;
        }
        if !seen.contains(&t) {
            seen.insert(t.clone());
            out.push(t);
        }
    }
    out
}

// Extract common block metadata fields from a list of detail lines.
// Returns (watermark, design, engraving, perforation)
pub fn extract_block_metadata(details: &[String]) -> (String, String, String, String) {
    let mut watermark = String::new();
    let mut design = String::new();
    let mut engraving = String::new();
    let mut perforation = String::new();
    // Precompile simple regexes for per-line capture (case-insensitive)
    let re_wm = Regex::new(r"(?i)^\s*(?:wm|watermark)\s*[:\-]?\s*(.+)$").unwrap();
    let re_design = Regex::new(r"(?i)^\s*design\s*[:\-]?\s*(.+)$").unwrap();
    let re_engr = Regex::new(r"(?i)^\s*engraving\s*[:\-]?\s*(.+)$").unwrap();
    let re_perf = Regex::new(r"(?i)^\s*(?:perforation|perf)\s*[:\-]?\s*(.+)$").unwrap();
    for d in details {
        // Some <p> blocks include multiple logical lines concatenated with newlines.
        // Split into actual lines and inspect each one independently so we don't
        // accidentally capture the remainder of the block when multiple labels
        // appear in the same <p> element.
        for line in d.lines() {
            let l = line.trim();
            if l.is_empty() {
                continue;
            }
            if watermark.is_empty() {
                if let Some(cap) = re_wm.captures(l) {
                    watermark = cap.get(1).map_or("", |m| m.as_str()).trim().to_string();
                    continue;
                }
            }
            if design.is_empty() {
                if let Some(cap) = re_design.captures(l) {
                    design = cap.get(1).map_or("", |m| m.as_str()).trim().to_string();
                    continue;
                }
            }
            if engraving.is_empty() {
                if let Some(cap) = re_engr.captures(l) {
                    engraving = cap.get(1).map_or("", |m| m.as_str()).trim().to_string();
                    continue;
                }
            }
            if perforation.is_empty() {
                if let Some(cap) = re_perf.captures(l) {
                    // capture only numeric portion where possible
                    let mut val = cap.get(1).map_or("", |m| m.as_str()).trim().to_string();
                    // If the captured value contains non-digit extras, try to extract the first number token
                    if !val.chars().all(|c| c.is_ascii_digit()) {
                        for tok in val.split_whitespace() {
                            if tok
                                .chars()
                                .next()
                                .map(|c| c.is_ascii_digit())
                                .unwrap_or(false)
                            {
                                val = tok
                                    .trim()
                                    .trim_matches(|c: char| !c.is_ascii_digit())
                                    .to_string();
                                break;
                            }
                        }
                    }
                    perforation = val;
                    continue;
                } else {
                    // fallback: if the line contains the word 'perf' or 'perforation' anywhere, try to extract digits
                    let low = l.to_lowercase();
                    if (low.contains("perf") || low.contains("perforation"))
                        && perforation.is_empty()
                    {
                        for tok in l.split_whitespace() {
                            if tok
                                .chars()
                                .next()
                                .map(|c| c.is_ascii_digit())
                                .unwrap_or(false)
                            {
                                perforation = tok
                                    .trim()
                                    .trim_matches(|c: char| !c.is_ascii_digit())
                                    .to_string();
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    // Final trimming and normalization
    watermark = watermark
        .trim()
        .trim_matches(|c: char| c == ':' || c == '.')
        .to_string();
    design = design
        .trim()
        .trim_matches(|c: char| c == ':' || c == '.')
        .to_string();
    engraving = engraving
        .trim()
        .trim_matches(|c: char| c == ':' || c == '.')
        .to_string();
    perforation = perforation
        .trim()
        .trim_matches(|c: char| c == ':' || c == '.')
        .to_string();
    (watermark, design, engraving, perforation)
}

// Compress a context string by removing consecutive duplicate tokens separated by '|'
// and truncate if it exceeds a reasonable length.
pub fn compress_context(s: &str, max_len: usize) -> String {
    let parts: Vec<&str> = s
        .split('|')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();
    let mut out_parts: Vec<&str> = Vec::new();
    let mut last: Option<&str> = None;
    for p in parts.iter() {
        if last.map(|l| l == *p).unwrap_or(false) {
            continue;
        }
        out_parts.push(*p);
        last = Some(*p);
        // Quick break if we already exceeded max parts
        if out_parts.len() >= 20 {
            break;
        }
    }
    let mut out = out_parts.join(" | ");
    if out.len() > max_len {
        out.truncate(max_len);
        // avoid trailing partial token
        if let Some(pos) = out.rfind('|') {
            out.truncate(pos);
        }
    }
    out
}
