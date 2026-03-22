use image::io::Reader as ImageReader;

#[test]
fn placeholder_per_stamp_has_expected_dimensions() {
    // Create a temporary file path
    let tmpdir = tempfile::tempdir().expect("tempdir");
    let out = tmpdir.path().join("no_image_test.png");

    // Use the existing generator (per-stamp uses 300x220 in current implementation)
    stamp_album_gen2::placeholder_image::generate_placeholder_image_with_options(
        300, 220, &out, None,
    )
    .expect("generate placeholder");

    // Read back and assert dimensions
    let img = ImageReader::open(&out)
        .expect("open")
        .with_guessed_format()
        .expect("format")
        .decode()
        .expect("decode");
    assert_eq!(img.width(), 300);
    assert_eq!(img.height(), 220);
}
