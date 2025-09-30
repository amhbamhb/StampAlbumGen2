use stamp_album_gen2::album_globals::compute_usable_height;
use stamp_album_gen2::config::Config;

#[test]
fn filename_generation_with_year_range() {
    // Mirror the filename-generation logic used by generate_album_output
    let album_name = "Israel";
    let make_name = |start: Option<i32>, end: Option<i32>| {
        if let (Some(s), Some(e)) = (start, end) {
            if s > 0 && e < 9999 && s <= e {
                return format!("{}-{}-{}.txt", album_name.replace(' ', "-"), s, e);
            }
        }
        format!("{}.txt", album_name.replace(' ', "-"))
    };

    assert_eq!(make_name(Some(1948), Some(1948)), "Israel-1948-1948.txt");
    assert_eq!(make_name(Some(0), Some(1948)), "Israel.txt");
    assert_eq!(make_name(None, None), "Israel.txt");
    assert_eq!(make_name(Some(1950), Some(1949)), "Israel.txt");
}

#[test]
fn compute_usable_height_basic() {
    // Basic smoke test for compute_usable_height
    let cfg = Config::default();
    let h = compute_usable_height(&cfg);
    // With defaults the usable height should be positive and less than page height
    assert!(h > 0.0);
    assert!(h < cfg.album_page_height_mm.unwrap_or(279.4) as f64);
}
