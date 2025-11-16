use std::fs;
use std::path::Path;

use image::GenericImageView;
use stamp_album_gen2::placeholder_image::ensure_placeholder_image_for_stamp_with_hint;

/// Integration test: ensure placeholder regeneration creates PNG of expected pixel dims
#[test]
fn placeholder_regeneration_forced() {
    // Scenario A: known registration stamp size
    let album_a = "Test-Placeholder-regen";
    let sg_a = "XTEST1";
    let year_a = "2025";
    let dpi = 96.0_f32;
    let mm_per_px = 25.4_f32 / dpi;
    let target_w_mm = 54.6_f64;
    let target_h_mm = 24.2_f64;

    let rel_a = ensure_placeholder_image_for_stamp_with_hint(
        album_a,
        sg_a,
        year_a,
        None,
        None,
        Some(target_w_mm),
        Some(target_h_mm),
        Some(mm_per_px),
        Some(mm_per_px),
    )
    .expect("placeholder creation failed");

    let album_dir_a = Path::new("Albums").join(album_a.replace(' ', "-"));
    println!("placeholder rel='{}'", rel_a);
    // Show directory state for debugging
    if album_dir_a.exists() {
        println!("album dir exists: {}", album_dir_a.display());
        if let Ok(entries) = std::fs::read_dir(&album_dir_a) {
            for e in entries.flatten() {
                println!(" - {}", e.path().display());
            }
        }
    } else {
        println!("album dir does not exist: {}", album_dir_a.display());
    }

    let img_path_a = album_dir_a.join(&rel_a);
    assert!(
        img_path_a.exists(),
        "placeholder image not created: {}",
        img_path_a.display()
    );
    let img_a = image::open(&img_path_a).expect("open placeholder image A");
    let (w_a, h_a) = img_a.dimensions();
    let exp_w_a = (target_w_mm as f32 / mm_per_px).round() as u32;
    let exp_h_a = (target_h_mm as f32 / mm_per_px).round() as u32;
    assert_eq!(w_a, exp_w_a, "width mismatch A");
    assert_eq!(h_a, exp_h_a, "height mismatch A");

    // Scenario B: different target mm
    let album_b = "Test-Placeholder";
    let sg_b = "TEST123";
    let year_b = "2000";
    let target_b_w = 50.0_f64;
    let target_b_h = 30.0_f64;
    let mm_per_px_b = mm_per_px; // same mapping

    // Clean up if present
    let album_dir_b = Path::new("Albums").join(album_b.replace(' ', "-"));
    if album_dir_b.exists() {
        let _ = fs::remove_dir_all(&album_dir_b);
    }

    let rel_b = ensure_placeholder_image_for_stamp_with_hint(
        album_b,
        sg_b,
        year_b,
        None,
        None,
        Some(target_b_w),
        Some(target_b_h),
        Some(mm_per_px_b),
        Some(mm_per_px_b),
    )
    .expect("placeholder creation failed B");

    let img_path_b = album_dir_b.join(&rel_b);
    assert!(
        img_path_b.exists(),
        "placeholder image not created B: {}",
        img_path_b.display()
    );
    let img_b = image::open(&img_path_b).expect("open placeholder image B");
    let (w_b, h_b) = img_b.dimensions();
    let exp_w_b = ((target_b_w as f32 / mm_per_px_b).round() as u32).max(50);
    let exp_h_b = ((target_b_h as f32 / mm_per_px_b).round() as u32).max(40);
    assert_eq!(w_b, exp_w_b, "width mismatch B");
    assert_eq!(h_b, exp_h_b, "height mismatch B");

    // cleanup both
    let _ = fs::remove_dir_all(&album_dir_a);
    let _ = fs::remove_dir_all(&album_dir_b);
}
