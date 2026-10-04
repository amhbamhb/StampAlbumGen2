use image::ImageReader;

#[test]
fn placeholder_series_size_matches_hint() {
    let tmpdir = tempfile::tempdir().expect("tempdir");
    let out = tmpdir.path().join("size_test.png");

    // Use series hint 'L' which is large
    let series = "L-s.jpg";
    stamp_album_gen2::placeholder_image::generate_placeholder_for_stamp_with_bg(
        "TestAlbum",
        "99",
        "2000",
        None,
        Some(series),
        &out,
    )
    .expect("generate");

    let img = ImageReader::open(&out)
        .expect("open")
        .with_guessed_format()
        .expect("format")
        .decode()
        .expect("decode");
    let (w, h) = (img.width(), img.height());

    let (exp_w, exp_h) =
        stamp_album_gen2::placeholder_image::compute_placeholder_size_for_series(series);
    // Allow a ±1 pixel tolerance because mm->px conversions may round differently across platforms.
    let dw = w.abs_diff(exp_w);
    let dh = h.abs_diff(exp_h);
    assert!(
        dw <= 1,
        "width differs by {} pixels (expected {}, got {})",
        dw,
        exp_w,
        w
    );
    assert!(
        dh <= 1,
        "height differs by {} pixels (expected {}, got {})",
        dh,
        exp_h,
        h
    );
}
