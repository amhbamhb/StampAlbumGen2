use image::GenericImageView;
use image::Pixel;
use image::Rgba;

// This test documents the expected inversion behavior: when a stamp background color is dark,
// the placeholder generator should choose a light background with dark text for legibility.
//
// NOTE: This test is marked ignored until the generator implements inversion logic.

#[test]
fn spec_placeholder_inverts_for_dark_stamp_background() {
    // Example dark stamp background
    let stamp_bg = Rgba([10u8, 10u8, 12u8, 255u8]);

    // TODO: the API for the generator should accept stamp background color (or we detect from metadata).
    // For now this test outlines intended behavior for a future implementation. The test will:
    // - call a function like `generate_placeholder_for_stamp_with_bg(album, sg, year, stamp_bg)`
    // - load generated image and assert that the top-left pixel (background) is a light color and
    //   that the main text color is dark (contrasting_color(stamp_bg)).

    // Generate into a temp file and assert inversion properties.
    let tmp = tempfile::tempdir().expect("tempdir");
    let out = tmp.path().join("inv_test.png");
    // dark stamp background should produce a light placeholder background
    stamp_album_gen2::placeholder_image::generate_placeholder_for_stamp_with_bg(
        "TestAlbum",
        "1",
        "1970",
        Some(stamp_bg),
        None,
        &out,
    )
    .expect("generate");

    let img = image::open(&out).expect("open");
    // Sample a pixel just inside the 2px border so we observe the true background color
    let pixel_inner = img.get_pixel(4, 4).to_rgba();
    // inner pixel should be light (close to white)
    assert!(stamp_album_gen2::placeholder_image::luminance(pixel_inner) > 0.9);

    // Ensure there's at least one dark pixel in the central region (text rendered)
    let (w, h) = (img.width(), img.height());
    let mut found_dark = false;
    for x in (w / 3)..((w * 2) / 3) {
        for y in (h / 3)..((h * 2) / 3) {
            let p = img.get_pixel(x, y).to_rgba();
            if stamp_album_gen2::placeholder_image::luminance(p) < 0.2 {
                found_dark = true;
                break;
            }
        }
        if found_dark {
            break;
        }
    }
    assert!(found_dark, "expected dark text pixels in center");

    // Keep test ignored until implementation
}
