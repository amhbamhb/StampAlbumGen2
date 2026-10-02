use stamp_album_gen2::flagdb::{country_to_code, default_flag_map};

#[tokio::test]
async fn test_load_builtin_map() {
    let m = default_flag_map().await;
    assert_eq!(m.get("israel").map(|s| s.as_str()), Some("il"));
    assert_eq!(m.get("united states").map(|s| s.as_str()), Some("us"));
}

#[tokio::test]
async fn test_country_to_code_normalization() {
    assert_eq!(country_to_code("Israel").await, Some("il".to_string()));
    assert_eq!(country_to_code("United States").await, Some("us".to_string()));
    assert_eq!(country_to_code("U.S.A.").await, Some("us".to_string()));
    assert_eq!(country_to_code("UK").await, Some("gb".to_string()));
}

#[tokio::test]
async fn test_missing_country_returns_none() {
    assert_eq!(country_to_code("Atlantis").await, None);
}
