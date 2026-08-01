use stamp_album_gen2::album_globals::compute_usable_height;
use stamp_album_gen2::config::Config;

#[test]
fn default_usable_height() {
    let cfg = Config::default();
    let h = compute_usable_height(&cfg);
    // Default: 279.4 - 10 -10 - (0.5+0.1+0.0+3.0) - 6 - 0 = 279.4 -20 -3.6 -6 = 249.8
    let expected = 249.8_f64;
    let diff = (h - expected).abs();
    assert!(diff < 0.01, "usable height {} != expected {}", h, expected);
}

#[test]
fn custom_padding_and_margins() {
    let mut cfg = Config::default();
    cfg.album_page_height_mm = Some(300.0);
    cfg.page_top_margin_mm = Some(15.0);
    cfg.page_bottom_margin_mm = Some(20.0);
    cfg.page_border_outer = Some(1.0);
    cfg.page_border_inner1 = Some(0.5);
    cfg.page_border_spacing = Some(4.0);
    cfg.usable_height_padding_mm = Some(10.0);
    cfg.album_pages_spacing_v = Some(5.0);
    let h = compute_usable_height(&cfg);
    // expected = 300 -15 -20 - (1+0.5+0.0+4) -10 -5 = 300 -35 -5.5 -10 -5 = 244.5
    let expected = 244.5_f64;
    assert!(
        (h - expected).abs() < 0.01,
        "usable height {} != expected {}",
        h,
        expected
    );
}
