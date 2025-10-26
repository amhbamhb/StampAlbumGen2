use stamp_album_gen2::db_utils::get_all_series;
use rusqlite::{Connection, params};

// Minimal test: build a tiny in-memory stamps table and verify first_sg asc/desc ordering
#[test]
fn test_get_all_series_asc_desc_simple() {
    let conn = Connection::open_in_memory().unwrap();
    // Create table with the columns selected by get_stamps_by_series
    conn.execute_batch(
        "CREATE TABLE stamps (
            year INTEGER,
            month TEXT,
            month_num INTEGER,
            day TEXT,
            sg_number TEXT,
            type TEXT,
            denomination TEXT,
            color TEXT,
            description TEXT,
            stamps_issued TEXT,
            image_path TEXT,
            remote_image_url TEXT,
            original_image_url TEXT,
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            series TEXT,
            currency TEXT,
            context TEXT,
            mint_condition TEXT,
            unused TEXT,
            used TEXT,
            letter_fdc TEXT
            ,image_width_px INTEGER,
            image_height_px INTEGER
        );",
    ).unwrap();

    let insert_sql = "INSERT INTO stamps (year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)";

    conn.execute(insert_sql, params![1975i32, "Jan", 1i32, "1", "100", "T", "", "", "desc", "", "", "", "", "", "", "", "", "S100", "CHF", "", "", "", "", ""]).unwrap();
    conn.execute(insert_sql, params![1975i32, "Jan", 1i32, "1", "50", "T", "", "", "desc", "", "", "", "", "", "", "", "", "S50", "CHF", "", "", "", "", ""]).unwrap();
    conn.execute(insert_sql, params![1975i32, "Jan", 1i32, "1", "200", "T", "", "", "desc", "", "", "", "", "", "", "", "", "S200", "CHF", "", "", "", "", ""]).unwrap();

    // Sanity check: get_stamps_by_series should return the inserted stamps
    let s50 = stamp_album_gen2::db_utils::get_stamps_by_series(&conn, "S50", "0");
    println!("get_stamps_by_series S50 -> {:?}", s50);
    assert!(s50.is_ok());
    let v50 = s50.unwrap();
    assert!(!v50.is_empty(), "expected stamps for S50");

    let asc = get_all_series(&conn, Some("first_sg")).unwrap();
    assert_eq!(asc, vec!["S50".to_string(), "S100".to_string(), "S200".to_string()]);

    let desc = get_all_series(&conn, Some("first_sg_desc")).unwrap();
    assert_eq!(desc, vec!["S200".to_string(), "S100".to_string(), "S50".to_string()]);
}

