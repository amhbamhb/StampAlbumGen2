pub fn normalize_catalog_type(code: &str) -> String {
    let normalized = code.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "stampworld" | "sw" => "sw".to_string(),
        "stanleygibbons" | "stanley_gibbons" | "sg" => "sg".to_string(),
        // Colnect stores "Pol" for the Polish Fischer catalog; UI uses "fischer"
        "pol" | "fischer" => "pol".to_string(),
        // Colnect stores "Nor" for Norgeskatalogen; UI uses "norgeskatalogen"
        "nor" | "norgeskatalogen" => "nor".to_string(),
        // Colnect stores "Zum" for Zumstein; UI uses "zumstein"
        "zum" | "zumstein" => "zum".to_string(),
        _ => normalized,
    }
}

pub fn catalog_display_name(code: &str) -> &'static str {
    match normalize_catalog_type(code).as_str() {
        // Major international catalogs
        "sn" => "Scott",
        "sg" => "Stanley Gibbons",
        "sw" => "StampWorld",
        "mi" => "Michel",
        "yt" => "Yvert et Tellier",
        "un" => "Unificato",
        "col" => "Colnect",
        "wad" => "WADP",
        // Regional / national catalogs
        "afa" => "AFA",
        "afm" => "Afinsa-Mundifil",
        "ank" => "ANK",
        "barefoot" => "Barefoot",
        "belaruspost" => "Belarus Post",
        "belgium" => "Belgium",
        "bertossa" => "Bertossa",
        "bolaffi" => "Bolaffi",
        "bw" => "Brusden-White",
        "bustamante" => "Bustamante",
        "campbellpaterson" => "Campbell Paterson",
        "cps" => "Chilean Philatelic Society",
        "css" => "China Stamp Society",
        "ciardi" => "Ciardi",
        "croatianpost" => "Croatian Post",
        "dainippon" => "Dai Nippon",
        "durland" => "Durland",
        "edifil" => "Edifil",
        "facit" => "Facit",
        "farahbakhsh" => "Farahbakhsh",
        "forbin" => "Forbin",
        "gotigjalil" => "Götig and Jalil",
        "gz" => "Gz (Cefiloza)",
        "hectormena" => "Hector Mena",
        "hibernian" => "Hibernian",
        "iscmalaysia" => "ISC Malaysia",
        "isfila" => "İsfila",
        "ipf" => "Israel Philatelic Federation",
        "karamitsos" => "Karamitsos",
        "lapefinland" => "LaPe Finland",
        "leotemprano" => "Leo Temprano",
        "markapmr" => "Markapmr.ru",
        "maury" => "Maury",
        "ngo" => "Ngo",
        "nilepost" => "Nile Post",
        "nor" => "Norgeskatalogen",
        "nvp" => "NVPH",
        "philaindia" => "Phila-India",
        "phh" => "Philatelia Hungarica",
        "pofis" => "POFIS",
        "pol" => "Fischer",
        "rhm" => "RHM",
        "romfilatelia" => "Romfilatelia",
        "sakura" => "Sakura",
        "sassone" => "Sassone",
        "sevenseas" => "Seven Seas",
        "sfk" => "SFK Serbia",
        "siddiqui" => "Siddiqui",
        "slovenika" => "Slovenika",
        "soloviev" => "Soloviev",
        "safrican" => "South African",
        "thaistamp" => "Thai Stamp",
        "ukrposhta" => "Ukrposhta",
        "vandam" => "Van Dam",
        "wsp" => "World Stamps Project",
        "yang" => "Yang",
        "zagorskiy" => "Zagorskiy",
        "zimbrul" => "Zimbrul Carpatin",
        "zonnebloem" => "Zonnebloem",
        "zum" => "Zumstein",
        _ => "Catalog",
    }
}

pub fn normalize_catalog_number(raw: &str) -> String {
    let mut value = raw.trim();
    if let Some((_, rest)) = value.split_once(':') {
        value = rest.trim();
    }

    // Colnect numbers often start with a country code prefix like "NO 1" or
    // "CA. 454". Remove a leading 2-3 letter prefix with optional trailing dot.
    let mut parts = value.split_whitespace();
    if let Some(first) = parts.next() {
        let code = first.trim_end_matches('.');
        if (2..=3).contains(&code.len()) && code.chars().all(|c| c.is_ascii_alphabetic()) {
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
        catalog_display_name, infer_catalog_type_for_number, normalize_catalog_number,
        normalize_catalog_type, preferred_catalog_entry, preferred_catalog_number,
    };

    #[test]
    fn strips_country_prefix_from_catalog_number() {
        assert_eq!(normalize_catalog_number("MT A28PL102"), "A28PL102");
        assert_eq!(normalize_catalog_number("Sn: CA 454ai"), "454ai");
        assert_eq!(normalize_catalog_number("NO 1"), "1");
        assert_eq!(normalize_catalog_number("Sn: NO. 1"), "1");
        assert_eq!(normalize_catalog_number("A 12"), "A 12");
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

    #[test]
    fn pol_and_aliases_normalize_and_display_correctly() {
        // Colnect stores "Pol"; UI alias is "fischer"
        assert_eq!(normalize_catalog_type("Pol"), "pol");
        assert_eq!(normalize_catalog_type("fischer"), "pol");
        assert_eq!(catalog_display_name("pol"), "Fischer");
        assert_eq!(catalog_display_name("Pol"), "Fischer");
        assert_eq!(catalog_display_name("fischer"), "Fischer");
    }

    #[test]
    fn nor_and_zum_aliases_display_correctly() {
        assert_eq!(normalize_catalog_type("Nor"), "nor");
        assert_eq!(normalize_catalog_type("norgeskatalogen"), "nor");
        assert_eq!(catalog_display_name("nor"), "Norgeskatalogen");
        assert_eq!(normalize_catalog_type("Zum"), "zum");
        assert_eq!(normalize_catalog_type("zumstein"), "zum");
        assert_eq!(catalog_display_name("zum"), "Zumstein");
    }

    #[test]
    fn all_known_catalog_codes_have_non_catalog_display_names() {
        let known = [
            "sn",
            "sg",
            "sw",
            "mi",
            "yt",
            "un",
            "col",
            "wad",
            "afa",
            "afm",
            "ank",
            "barefoot",
            "belaruspost",
            "belgium",
            "bertossa",
            "bolaffi",
            "bw",
            "bustamante",
            "campbellpaterson",
            "cps",
            "css",
            "ciardi",
            "croatianpost",
            "dainippon",
            "durland",
            "edifil",
            "facit",
            "farahbakhsh",
            "forbin",
            "gotigjalil",
            "gz",
            "hectormena",
            "hibernian",
            "iscmalaysia",
            "isfila",
            "ipf",
            "karamitsos",
            "lapefinland",
            "leotemprano",
            "markapmr",
            "maury",
            "ngo",
            "nilepost",
            "nor",
            "nvp",
            "philaindia",
            "phh",
            "pofis",
            "pol",
            "rhm",
            "romfilatelia",
            "sakura",
            "sassone",
            "sevenseas",
            "sfk",
            "siddiqui",
            "slovenika",
            "soloviev",
            "safrican",
            "thaistamp",
            "ukrposhta",
            "vandam",
            "wsp",
            "yang",
            "zagorskiy",
            "zimbrul",
            "zonnebloem",
            "zum",
        ];
        for code in known {
            let name = catalog_display_name(code);
            assert_ne!(
                name, "Catalog",
                "catalog_display_name(\"{}\") returned the fallback \"Catalog\"",
                code
            );
        }
    }
}
