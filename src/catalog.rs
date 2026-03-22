pub fn normalize_catalog_type(code: &str) -> String {
    code.trim().to_ascii_lowercase()
}

pub fn catalog_display_name(code: &str) -> &'static str {
    match normalize_catalog_type(code).as_str() {
        "sn" => "Scott",
        "sg" => "Stanley Gibbons",
        "mi" => "Michel",
        "yt" => "Yvert",
        "afa" => "AFA",
        "nvp" => "NVP",
        "un" => "Unificato",
        "col" => "Colnect",
        _ => "Catalog",
    }
}

pub fn normalize_catalog_number(raw: &str) -> String {
    let mut value = raw.trim();
    if let Some((_, rest)) = value.split_once(':') {
        value = rest.trim();
    }

    let mut parts = value.split_whitespace();
    if let Some(first) = parts.next() {
        if first.chars().all(|c| c.is_ascii_alphabetic()) {
            let rest = value[first.len()..].trim_start();
            if !rest.is_empty() {
                return rest.to_string();
            }
        }
    }

    value.to_string()
}

pub fn parse_catalog_codes(raw: &str) -> Vec<(String, String)> {
    raw.split([',', '|'])
        .filter_map(|part| {
            let trimmed = part.trim();
            let (catalog_type, code) = trimmed.split_once(':')?;
            let catalog_type = normalize_catalog_type(catalog_type);
            let code = code.trim();
            if catalog_type.is_empty() || code.is_empty() {
                return None;
            }
            Some((catalog_type, code.to_string()))
        })
        .collect()
}

pub fn preferred_catalog_entry(raw: &str, default_catalog_code: &str) -> Option<(String, String)> {
    let pairs = parse_catalog_codes(raw);
    if pairs.is_empty() {
        return None;
    }

    let preferred = normalize_catalog_type(default_catalog_code);
    if let Some((catalog_type, code)) = pairs
        .iter()
        .find(|(catalog_type, _)| *catalog_type == preferred)
    {
        return Some((catalog_type.clone(), code.clone()));
    }

    pairs.into_iter().next()
}

pub fn preferred_catalog_number(raw: &str, default_catalog_code: &str) -> Option<String> {
    preferred_catalog_entry(raw, default_catalog_code)
        .map(|(_, code)| normalize_catalog_number(&code))
}

pub fn infer_catalog_type_for_number(
    raw_catalog_codes: &str,
    displayed_number: &str,
    fallback_catalog_code: &str,
) -> String {
    let normalized_display = normalize_catalog_number(displayed_number);
    for (catalog_type, code) in parse_catalog_codes(raw_catalog_codes) {
        if normalize_catalog_number(&code) == normalized_display {
            return catalog_type;
        }
    }
    normalize_catalog_type(fallback_catalog_code)
}

#[cfg(test)]
mod tests {
    use super::{
        infer_catalog_type_for_number, normalize_catalog_number, preferred_catalog_entry,
        preferred_catalog_number,
    };

    #[test]
    fn strips_country_prefix_from_catalog_number() {
        assert_eq!(normalize_catalog_number("MT A28PL102"), "A28PL102");
        assert_eq!(normalize_catalog_number("Sn: CA 454ai"), "454ai");
    }

    #[test]
    fn prefers_requested_catalog_family() {
        let raw = "Mi: CH 964, Sn: CH 540, Sg: CH 824";
        assert_eq!(preferred_catalog_number(raw, "sn").as_deref(), Some("540"));
        assert_eq!(preferred_catalog_number(raw, "sg").as_deref(), Some("824"));
        assert_eq!(preferred_catalog_entry(raw, "mi").unwrap().0, "mi");
    }

    #[test]
    fn infers_catalog_type_from_normalized_number() {
        let raw = "Sn: CA 454ai | Mi: CA 123 | Sg: CA 98";
        assert_eq!(infer_catalog_type_for_number(raw, "454ai", "sg"), "sn");
        assert_eq!(infer_catalog_type_for_number(raw, "98", "sn"), "sg");
    }
}
