use stamp_album_gen2::stampworld::parser::{
    dedupe_details, extract_block_metadata, normalize_stamps_issued,
};
use stamp_album_gen2::stampworld::data_collector::DAY_MONTH_RE;

#[test]
fn test_normalize_stamps_issued_basic() {
    assert_eq!(normalize_stamps_issued("1.234.567"), "1,234,567");
    assert_eq!(normalize_stamps_issued("(1234)"), "1234");
    assert_eq!(normalize_stamps_issued("1,234"), "1,234");
    assert_eq!(normalize_stamps_issued("N/A"), "");
}

#[test]
fn test_dedupe_details_preserves_order() {
    let src = vec![
        "WM: None".to_string(),
        "Perforation: 12".to_string(),
        "WM: None".to_string(),
        "Note".to_string(),
    ];
    let out = dedupe_details(&src);
    assert_eq!(
        out,
        vec![
            "WM: None".to_string(),
            "Perforation: 12".to_string(),
            "Note".to_string()
        ]
    );
}

#[test]
fn test_extract_block_metadata() {
    let src = vec![
        "WM: None".to_string(),
        "Design: King George".to_string(),
        "Engraving: Recess".to_string(),
        "Perf: 12".to_string(),
    ];
    let (wm, design, engr, perf) = extract_block_metadata(&src);
    assert_eq!(wm, "None");
    assert_eq!(design, "King George");
    assert_eq!(engr, "Recess");
    assert_eq!(perf, "12");
}

#[test]
fn test_extract_block_metadata_multiline_block() {
    let block = "Design: Herman Herbert Schwartz\nEngraving: Herman Herbert Schwartz\nPerforation: 12\nWM: None".to_string();
    let src = vec![block];
    let (wm, design, engr, perf) = extract_block_metadata(&src);
    assert_eq!(wm, "None");
    assert_eq!(design, "Herman Herbert Schwartz");
    assert_eq!(engr, "Herman Herbert Schwartz");
    assert_eq!(perf, "12");
}

#[test]
fn test_day_month_regex_unicode() {
    let samples = vec!["3. März", "12. Feb", " 1. januari", "21. Décembre"];
    for s in samples {
        let caps = DAY_MONTH_RE
            .captures(s)
            .expect(&format!("DAY_MONTH_RE failed to match '{}'", s));
        let day = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let month = caps.get(2).map_or("", |m| m.as_str()).to_string();
        assert!(!day.is_empty(), "day empty for '{}'", s);
        assert!(!month.is_empty(), "month empty for '{}'", s);
    }
}
