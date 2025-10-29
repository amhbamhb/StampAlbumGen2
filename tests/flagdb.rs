use stamp_album_gen2::flagdb::{country_to_code, default_flag_map};

#[test]
fn test_load_builtin_map() {
    let m = default_flag_map();
    assert_eq!(m.get("israel").map(|s| s.as_str()), Some("il"));
    assert_eq!(m.get("united states").map(|s| s.as_str()), Some("us"));
}

#[test]
fn test_country_to_code_normalization() {
    assert_eq!(country_to_code("Israel"), Some("il".to_string()));
    assert_eq!(country_to_code("United States"), Some("us".to_string()));
    assert_eq!(country_to_code("U.S.A."), Some("us".to_string()));
    assert_eq!(country_to_code("UK"), Some("gb".to_string()));
}

#[test]
fn test_missing_country_returns_none() {
    assert_eq!(country_to_code("Atlantis"), None);
}
