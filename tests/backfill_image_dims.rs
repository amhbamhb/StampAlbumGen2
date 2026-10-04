use rusqlite::params;
use std::fs;
use tempfile::tempdir;

#[test]
fn backfill_image_dimensions_persists_to_db() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary album/images layout and a tiny test image
    let td = tempdir()?;
    let album_dir = td.path().join("Albums/Switzerland");
    let images_dir = album_dir.join("images");
    fs::create_dir_all(&images_dir)?;

    // Create a small PNG image (200x145) using the image crate
    let img = image::RgbImage::from_fn(200, 145, |x, y| {
        image::Rgb([((x + y) % 255) as u8, (x % 255) as u8, (y % 255) as u8])
    });
    let img_path = images_dir.join("test_img.png");
    img.save(&img_path)?;

    // Create DB and insert a stamp row with image_path but NULL dimensions
    let db_path = album_dir.join("Switzerland.db");
    let db_path_str = db_path.to_str().unwrap();
    let conn = stamp_album_gen2::db_utils::init_db(db_path_str)?;

    conn.execute(
        "INSERT INTO stamps (country, series, year, sg_number, image_path, uniq_key) VALUES (?1,?2,?3,?4,?5,?6)",
        params![
            "Switzerland",
            "TestSeries",
            1976,
            "1063",
            "images/test_img.png",
            "switzerland|TestSeries|1063"
        ],
    )?;

    // Verify dims are NULL initially
    let (w1, h1): (Option<i32>, Option<i32>) = conn.query_row(
        "SELECT image_width_px, image_height_px FROM stamps WHERE country=?1 AND series=?2 AND sg_number=?3",
        params!["Switzerland", "TestSeries", "1063"],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    assert!(w1.is_none() && h1.is_none());

    // Resolve dimensions like the collector does: try as-given, else join with images dir
    let p = "images/test_img.png";
    let mut dims = stamp_album_gen2::image_utils::get_image_dimensions(p);
    if dims.is_none() {
        let rest = p.strip_prefix("images/").unwrap_or(p);
        let alt = format!(
            "{}/{}",
            images_dir.to_str().unwrap().trim_end_matches('/'),
            rest
        );
        dims = stamp_album_gen2::image_utils::get_image_dimensions(alt);
    }
    assert!(dims.is_some(), "test image dims could not be read");
    let (w_px, h_px) = dims.unwrap();

    // Persist via the same DB helper used by the collector
    stamp_album_gen2::db_utils::update_image_dimensions(
        &conn,
        "Switzerland",
        "TestSeries",
        "1063",
        Some(w_px as i32),
        Some(h_px as i32),
    )?;

    // Confirm values were written
    let (w2, h2): (Option<i32>, Option<i32>) = conn.query_row(
        "SELECT image_width_px, image_height_px FROM stamps WHERE country=?1 AND series=?2 AND sg_number=?3",
        params!["Switzerland", "TestSeries", "1063"],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )?;
    assert_eq!(w2, Some(w_px as i32));
    assert_eq!(h2, Some(h_px as i32));

    Ok(())
}
