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
    assert_eq!(
        country_to_code("United States").await,
        Some("us".to_string())
    );
    assert_eq!(country_to_code("U.S.A.").await, Some("us".to_string()));
    assert_eq!(country_to_code("UK").await, Some("gb".to_string()));
}

#[tokio::test]
async fn test_missing_country_returns_none() {
    assert_eq!(country_to_code("Atlantis").await, None);
}

#[tokio::test]
async fn iso_names_codes_and_existing_aliases_resolve_without_historical_guessing() {
    for (name, code) in [
        ("South Korea", "kr"),
        ("North_Korea", "kp"),
        ("  SOUTH--KOREA ", "kr"),
        ("KR", "kr"),
        ("ca", "ca"),
        ("US", "us"),
        ("Guinea-Bissau", "gw"),
        ("Åland Islands", "ax"),
        ("New Brunswick", "ca"),
        ("U.S.A.", "us"),
        ("UK", "gb"),
    ] {
        assert_eq!(country_to_code(name).await.as_deref(), Some(code), "{name}");
    }
    for name in ["ZZ", "British Canada", "North Korean Occ. South Korea"] {
        assert_eq!(country_to_code(name).await, None, "{name}");
    }
}

#[tokio::test]
async fn unmapped_country_can_reuse_a_valid_album_local_flag_without_network() {
    let temp = tempfile::tempdir().expect("album images");
    let path = temp.path().join("flag_placeholder.png");
    image::RgbaImage::from_pixel(2, 1, image::Rgba([255, 0, 0, 255]))
        .save(&path)
        .expect("local PNG");
    let result = stamp_album_gen2::flagdb::copy_flag_for_album(
        "Atlantis",
        temp.path().to_str().expect("path"),
        "w320",
    )
    .await
    .expect("album-local fallback");
    assert_eq!(std::path::Path::new(&result), path);
    std::fs::write(&path, "<html>bad cache</html>").expect("invalid local image");
    assert!(stamp_album_gen2::flagdb::copy_flag_for_album(
        "Atlantis",
        temp.path().to_str().expect("path"),
        "w320",
    )
    .await
    .is_err());
}
