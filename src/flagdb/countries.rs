use std::collections::HashMap;

pub(super) fn normalize(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace(['-', '_'], " ")
        .replace(['\'', ',', '’'], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn valid_code(code: &str) -> bool {
    code.len() == 2 && code.bytes().all(|byte| byte.is_ascii_alphabetic())
}

pub(super) fn parse_iso_codes(input: &str) -> Result<HashMap<String, String>, String> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(input.as_bytes());
    let mut map = HashMap::new();
    for (index, record) in reader.records().enumerate() {
        let record = record.map_err(|error| format!("ISO country row {}: {error}", index + 1))?;
        let code = record.get(0).unwrap_or("").trim().to_lowercase();
        let name = normalize(record.get(1).unwrap_or(""));
        if record.len() != 2 || !valid_code(&code) || name.is_empty() {
            return Err(format!("Invalid ISO country row {}", index + 1));
        }
        if map.insert(code.clone(), code.clone()).is_some() || map.insert(name, code).is_some() {
            return Err(format!("Duplicate ISO country row {}", index + 1));
        }
    }
    if map.is_empty() {
        return Err("ISO country data is empty".into());
    }
    Ok(map)
}

pub(super) fn merge_overrides(
    map: &mut HashMap<String, String>,
    overrides: HashMap<String, String>,
) -> Result<(), String> {
    for (name, code) in &overrides {
        if normalize(name).is_empty() || !valid_code(code.trim()) {
            return Err(format!("Invalid flag code mapping for {name:?}"));
        }
        let key = normalize(name);
        if valid_code(&key)
            && map
                .get(&key)
                .is_some_and(|existing| existing != &code.trim().to_lowercase())
        {
            return Err(format!("Cannot redefine ISO code {name:?}"));
        }
    }
    for (name, code) in overrides {
        let code = code.trim().to_lowercase();
        map.insert(normalize(&name), code.clone());
        // Explicit aliases may introduce historical codes, but cannot redefine ISO codes.
        map.entry(code.clone()).or_insert(code);
    }
    Ok(())
}

pub(super) fn bundled_map() -> Result<HashMap<String, String>, String> {
    let mut map = parse_iso_codes(include_str!("../../ISOCountryCodes.csv"))?;
    let aliases = serde_json::from_str(include_str!("../../flags/flag_codes.json"))
        .map_err(|error| format!("Invalid bundled flag aliases: {error}"))?;
    merge_overrides(&mut map, aliases)?;
    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supplied_iso_file_has_246_unique_codes_and_names() {
        let map = parse_iso_codes(include_str!("../../ISOCountryCodes.csv")).expect("ISO data");
        assert_eq!(map.len(), 246 * 2);
        for (name, code) in [
            ("South Korea", "kr"),
            ("North Korea", "kp"),
            ("Åland Islands", "ax"),
            ("Netherlands Antilles", "an"),
            ("U.S. Virgin Islands", "vi"),
        ] {
            assert_eq!(map.get(&normalize(name)).map(String::as_str), Some(code));
            assert_eq!(map.get(code).map(String::as_str), Some(code));
        }
    }

    #[test]
    fn malformed_or_duplicate_iso_rows_are_rejected() {
        for text in [
            "",
            "CA\t",
            "CAN\tCanada",
            "CA\tCanada\nCA\tCanada",
            "CA\tCanada\nUS\tCanada",
        ] {
            assert!(parse_iso_codes(text).is_err(), "{text:?}");
        }
    }

    #[test]
    fn partial_overrides_keep_iso_coverage_and_invalid_overrides_do_not_apply() {
        let mut map = bundled_map().expect("bundled map");
        merge_overrides(
            &mut map,
            HashMap::from([("Local name".into(), "KR".into())]),
        )
        .expect("override");
        assert_eq!(map["local name"], "kr");
        assert_eq!(map["canada"], "ca");
        assert_eq!(map["uk"], "gb");
        let before = map.clone();
        assert!(merge_overrides(&mut map, HashMap::from([("bad".into(), "../".into())])).is_err());
        assert_eq!(map, before);
        assert!(merge_overrides(&mut map, HashMap::from([("CA".into(), "US".into())])).is_err());
        assert_eq!(map, before);
    }
}
