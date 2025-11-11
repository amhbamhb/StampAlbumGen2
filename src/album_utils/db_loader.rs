/// Loads stamp data from SQLite database
pub fn load_stamps_from_db(db_path: &str) -> Vec<crate::db_utils::StampStub> {
    let conn = crate::db_utils::init_db(db_path).expect("Failed to open database");

    // Load all stamps directly instead of filtering by series
    let mut stmt = conn
        .prepare(
            "SELECT year, month, day, w_number, type, denomination, color, month_num, description, stamps_issued, image_path, remote_image_url, watermark, design, engraving, perforation, series, currency, context FROM stamps ORDER BY sg_number"
        )
        .expect("Failed to prepare statement");

    let stamp_iter = stmt
        .query_map([], |row| {
            // Debug: print raw row data
            // let sg_result: Result<String, _> = row.get(3);
            // let desc_result: Result<String, _> = row.get(8);
            // println!("[DEBUG] Raw row data: sg_number={:?}, description={:?}", sg_result, desc_result);

            Ok(crate::db_utils::StampStub {
                year: row.get::<_, i32>(0).unwrap_or(0),
                month: row.get(1).unwrap_or(String::new()),
                month_num: row.get::<_, i32>(7).unwrap_or(0),
                day: row.get(2).unwrap_or(String::new()),
                sg_number: row.get(3).unwrap_or(String::new()),
                r#type: row.get(4).unwrap_or(String::new()),
                stamp_type: row.get(4).unwrap_or(String::new()),
                denomination: row.get(5).unwrap_or(String::new()),
                color: row.get(6).unwrap_or(String::new()),
                description: row.get(8).unwrap_or(String::new()),
                desc: row.get(8).unwrap_or(String::new()), // description is at index 8
                catalog_num: row.get(3).unwrap_or(String::new()), // Use sg_number for catalog_num
                stamps_issued: crate::db_utils::format_stamps_issued(
                    &row.get::<_, String>(9).unwrap_or(String::new()),
                ), // stamps_issued at index 9
                mint_condition: String::new(),
                unused: String::new(),
                used: String::new(),
                letter_fdc: String::new(),
                currency: row.get(17).unwrap_or(String::new()), // currency at index 17
                image_path: row.get(10).unwrap_or(String::new()), // image_path at index 10
                remote_image_url: row.get(11).unwrap_or(String::new()), // remote_image_url at index 11
                original_image_url: String::new(),
                image_width_px: row
                    .get::<_, Option<i64>>("image_width_px")
                    .ok()
                    .flatten()
                    .map(|v| v as i32),
                image_height_px: row
                    .get::<_, Option<i64>>("image_height_px")
                    .ok()
                    .flatten()
                    .map(|v| v as i32),
                watermark: row.get(12).unwrap_or(String::new()), // watermark at index 12
                design: row.get(13).unwrap_or(String::new()),    // design at index 13
                engraving: row.get(14).unwrap_or(String::new()), // engraving at index 14
                perforation: row.get(15).unwrap_or(String::new()), // perforation at index 15
                series: row.get(16).unwrap_or(String::new()),    // series at index 16
                context: row.get(18).unwrap_or(String::new()),   // context at index 18
                multi_perf: false,
            })
        })
        .expect("Failed to query stamps");

    let mut all_stamps = Vec::new();
    for stamp_result in stamp_iter {
        match stamp_result {
            Ok(stamp) => {
                // println!("[DEBUG] Created stamp: sg={}, desc='{}'", stamp.sg_number, stamp.desc);
                all_stamps.push(stamp);
            }
            Err(_e) => {
                // println!("[DEBUG] Error creating stamp: {}", e);
            }
        }
    }
    all_stamps
}
