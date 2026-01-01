use rusqlite::Connection;
use std::path::Path;
use tempfile::tempdir;

use image::GenericImageView;

// Ensure tests that modify cwd or filesystem run serially
use serial_test::serial;

struct CwdGuard {
    old: std::path::PathBuf,
}
impl CwdGuard {
    fn chdir_to(path: &std::path::Path) -> Self {
        let old = std::env::current_dir().expect("get cwd");
        std::env::set_current_dir(path).expect("chdir to tempdir");
        CwdGuard { old }
    }
}
impl Drop for CwdGuard {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.old);
    }
}

#[test]
#[serial]
fn test_ensure_placeholder_for_stamp_and_update_db() {
    // Create a temporary directory and switch into it so Albums/ is created there
    let td = tempdir().expect("create tempdir");
    let _guard = CwdGuard::chdir_to(td.path());

    // Prepare an in-memory SQLite DB and create a minimal `stamps` table
    let conn = Connection::open_in_memory().expect("open in-memory db");
    conn.execute(
        "CREATE TABLE stamps (country TEXT, series TEXT, sg_number TEXT, image_width_px INTEGER, image_height_px INTEGER)",
        [],
    )
    .expect("create stamps table");

    // Insert a row that should be updated by the helper
    let album_name = "TestAlbum";
    let series_name = "SeriesX";
    let sg_number = "123";
    conn.execute(
        "INSERT INTO stamps (country, series, sg_number, image_width_px, image_height_px) VALUES (?1, ?2, ?3, NULL, NULL)",
        rusqlite::params![album_name, series_name, sg_number],
    )
    .expect("insert stub");

    // Call the helper with explicit mm->px mapping so desired pixel dims are deterministic
    let target_w_mm = 40.0_f64;
    let target_h_mm = 25.0_f64;
    // Choose mm_per_px so px dims are above minimum thresholds in placeholder logic
    let mm_per_px_w = 0.5_f32; // -> 80 px
    let mm_per_px_h = 0.5_f32; // -> 50 px

    let res = stamp_album_gen2::album_globals::generator_new::helpers::ensure_placeholder_for_stamp_and_update_db(
        &conn,
        album_name,
        series_name,
        sg_number,
        sg_number, // catalog_number (same as sg for this test)
        "1900",
        target_w_mm,
        target_h_mm,
        mm_per_px_w,
        mm_per_px_h,
        None, // no writer
    );

    assert!(res.is_some(), "helper should return Some on success");
    let (rel_path, px_w, px_h) = res.unwrap();

    // File should exist under Albums/<Album>/images/<rel_path>
    let expected_path = Path::new("Albums").join(album_name).join(&rel_path);
    assert!(
        expected_path.exists(),
        "placeholder file should exist: {}",
        expected_path.display()
    );

    // Verify image pixel dimensions on disk match reported px_w/px_h
    let img = image::open(&expected_path).expect("open placeholder image");
    let (iw, ih) = img.dimensions();
    assert_eq!(iw as i32, px_w, "image width on disk should match px_w");
    assert_eq!(ih as i32, px_h, "image height on disk should match px_h");

    // DB should have been updated
    let mut stmt = conn
        .prepare("SELECT image_width_px, image_height_px FROM stamps WHERE country=?1 AND series=?2 AND sg_number=?3")
        .expect("prepare select");
    let mut rows = stmt
        .query(rusqlite::params![album_name, series_name, sg_number])
        .expect("query");
    let row = rows.next().expect("row present").expect("row ok");
    let db_w: Option<i32> = row.get(0).expect("get w");
    let db_h: Option<i32> = row.get(1).expect("get h");
    assert_eq!(
        db_w.unwrap_or(0),
        px_w,
        "DB width should match computed px_w"
    );
    assert_eq!(
        db_h.unwrap_or(0),
        px_h,
        "DB height should match computed px_h"
    );

    // Cleanup: tempdir will be removed automatically; CwdGuard restores cwd
}
