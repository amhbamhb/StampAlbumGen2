use image::GenericImageView;
use std::fs;
use tempfile::tempdir;

// Integration test: place a small SVG in a temporary STAMP_FLAGS_DIR and call the
// public `ensure_coat_of_arms` function which should find the SVG and attempt
// to rasterize it to PNG, returning the PNG path.
#[test]
fn ensure_coa_from_local_svg_creates_png() {
    // Create a tempdir and populate flags/coats_of_arms/ca.svg
    let dir = tempdir().expect("tempdir");
    let flags_dir = dir.path().join("flags");
    let coats_dir = flags_dir.join("coats_of_arms");
    fs::create_dir_all(&coats_dir).expect("create coats dir");

    let svg_path = coats_dir.join("ca.svg");
    let svg_content = r#"<?xml version='1.0' encoding='UTF-8'?>
<svg xmlns='http://www.w3.org/2000/svg' width='200' height='160'>
  <rect width='200' height='160' fill='#ffffff'/>
  <circle cx='100' cy='80' r='60' fill='#0000ff' />
  <text x='100' y='85' font-size='24' text-anchor='middle' fill='#ffffff'>CA</text>
</svg>"#;
    fs::write(&svg_path, svg_content).expect("write svg");

    // Set env so the library will look in our temp flags dir first
    std::env::set_var("STAMP_FLAGS_DIR", dir.path());

    // Call the public API which should locate the svg and attempt to rasterize
    let res = stamp_album_gen2::flagdb::ensure_coat_of_arms("ca");
    assert!(
        res.is_ok(),
        "ensure_coat_of_arms returned an error: {:?}",
        res.err()
    );
    let out = res.unwrap();
    // Expect the returned path to point to a PNG that exists
    assert!(
        out.ends_with(".png") || out.ends_with(".svg"),
        "unexpected extension: {}",
        out
    );
    assert!(
        std::path::Path::new(&out).exists(),
        "output path does not exist: {}",
        out
    );

    // If it's a PNG, check that image crate can open it
    if out.ends_with(".png") {
        let img = image::open(&out).expect("open png");
        let (w, h) = img.dimensions();
        assert!(w > 0 && h > 0, "png has zero dimensions");
    }
}
