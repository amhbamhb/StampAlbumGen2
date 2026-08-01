use image::Rgba;

#[test]
fn test_luminance_and_is_color_dark() {
    // Very dark color
    let dark = Rgba([10u8, 10u8, 10u8, 255u8]);
    assert!(stamp_album_gen2::placeholder_image::is_color_dark(dark));

    // Very light color
    let light = Rgba([240u8, 240u8, 240u8, 255u8]);
    assert!(!stamp_album_gen2::placeholder_image::is_color_dark(light));

    // Mid-gray borderline
    let mid = Rgba([128u8, 128u8, 128u8, 255u8]);
    // We don't require strict value, but ensure the function returns a boolean deterministically
    let _ = stamp_album_gen2::placeholder_image::is_color_dark(mid);
}

#[test]
fn test_contrasting_color_returns_white_for_dark() {
    let dark = Rgba([5u8, 5u8, 5u8, 255u8]);
    let contrast = stamp_album_gen2::placeholder_image::contrasting_color(dark);
    assert_eq!(contrast, Rgba([255u8, 255u8, 255u8, 255u8]));

    let light = Rgba([250u8, 250u8, 250u8, 255u8]);
    let contrast2 = stamp_album_gen2::placeholder_image::contrasting_color(light);
    assert_eq!(contrast2, Rgba([0u8, 0u8, 0u8, 255u8]));
}
