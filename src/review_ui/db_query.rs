use rusqlite::Connection;
use crate::review_ui::http_utils::{is_real_image_path, local_image_file_exists};

/// Prepares stamp images for PDF generation by checking local files against remote URLs.
pub async fn best_effort_prepare_images_for_pdf(conn: &Connection, album_name: &str) -> (usize, usize) {
    let mut stmt = match conn.prepare(
        "SELECT
            COALESCE(sg_number, ''),
            COALESCE(image_path, ''),
            CASE
                WHEN TRIM(COALESCE(remote_image_url, '')) != '' THEN TRIM(COALESCE(remote_image_url, ''))
                WHEN TRIM(COALESCE(colnect_image_url, '')) != '' THEN TRIM(COALESCE(colnect_image_url, ''))
                WHEN TRIM(COALESCE(original_image_url, '')) != '' THEN TRIM(COALESCE(original_image_url, ''))
                ELSE ''
            END AS best_remote_url
         FROM stamps",
    ) {
        Ok(s) => s,
        Err(_) => return (0, 0),
    };

    let rows = match stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    }) {
        Ok(r) => r,
        Err(_) => return (0, 0),
    };

    let mut stubs: Vec<crate::db_utils::StampStub> = Vec::new();
    let mut real_image_rows = 0usize;
    let mut existing_real_files_before = 0usize;

    for row in rows.flatten() {
        let (sg_number, image_path, remote_url) = row;
        if !is_real_image_path(&image_path) {
            continue;
        }

        real_image_rows += 1;
        if local_image_file_exists(album_name, &image_path) {
            existing_real_files_before += 1;
        }

        if remote_url.trim().is_empty() {
            continue;
        }

        stubs.push(crate::db_utils::StampStub {
            year: 0,
            month: String::new(),
            month_num: 0,
            day: String::new(),
            sg_number: sg_number.clone(),
            r#type: String::new(),
            stamp_type: String::new(),
            denomination: String::new(),
            color: String::new(),
            description: String::new(),
            desc: String::new(),
            catalog_num: sg_number,
            stamps_issued: String::new(),
            mint_condition: String::new(),
            unused: String::new(),
            used: String::new(),
            letter_fdc: String::new(),
            currency: String::new(),
            image_path: String::new(),
            remote_image_url: remote_url,
            original_image_url: String::new(),
            image_width_px: None,
            image_height_px: None,
            watermark: String::new(),
            design: String::new(),
            engraving: String::new(),
            perforation: String::new(),
            series: String::new(),
            context: String::new(),
            issue_location: String::new(),
            quantity_printed: String::new(),
            multi_perf: false,
        });
    }

    let _ = crate::image_downloader::download_missing_stamp_images(&stubs, album_name).await;

    let mut existing_real_files_after = 0usize;
    if let Ok(mut verify_stmt) = conn.prepare("SELECT COALESCE(image_path, '') FROM stamps") {
        if let Ok(paths) = verify_stmt.query_map([], |row| row.get::<_, String>(0)) {
            for path in paths.flatten() {
                if is_real_image_path(&path) && local_image_file_exists(album_name, &path) {
                    existing_real_files_after += 1;
                }
            }
        }
    }

    (real_image_rows, existing_real_files_after)
}
