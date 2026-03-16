use rusqlite::Connection;
use stamp_album_gen2::album_globals::generator_new::helpers;

#[test]
fn test_fetch_and_filter_stamps_basic() {
    let conn = Connection::open_in_memory().expect("open");
    conn.execute_batch(
        "CREATE TABLE stamps (
            id INTEGER PRIMARY KEY,
            country TEXT,
            series TEXT,
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
            mint_condition TEXT,
            unused TEXT,
            used TEXT,
            letter_fdc TEXT,
            currency TEXT,
            image_path TEXT,
            remote_image_url TEXT,
            original_image_url TEXT,
            image_width_px INTEGER,
            image_height_px INTEGER,
               issue_location TEXT,
               quantity_printed TEXT,
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT,
            uniq_key TEXT
        )",
    )
    .unwrap();

    conn.execute(
        "INSERT INTO stamps (series, year, month_num, sg_number) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["S1", 1900, 0, "1"],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO stamps (series, year, month_num, sg_number) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["S1", 1910, 0, "2"],
    )
    .unwrap();

    // No filtering
    let stamps = helpers::fetch_and_filter_stamps(&conn, "S1", None, None).expect("fetch");
    assert_eq!(stamps.len(), 2);

    // Filter years >= 1905
    let stamps2 = helpers::fetch_and_filter_stamps(&conn, "S1", Some(1905), None).expect("fetch2");
    assert_eq!(stamps2.len(), 1);
    assert_eq!(stamps2[0].attrs.get("year").unwrap(), "1910");
}
