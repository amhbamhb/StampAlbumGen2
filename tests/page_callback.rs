use rusqlite::Connection;
use stamp_album_gen2::album_globals::generate_album_output_conn;

#[test]
fn test_page_callback_invoked_per_series() {
    let conn = Connection::open_in_memory().expect("open");
    // Create a stamps table with the columns used by the generator and stamp_album_db helpers.
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

    // Insert two stamps in different series so generator will iterate two series
    conn.execute(
        "INSERT INTO stamps (series, year, month_num, sg_number) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["S1", 1900, 0, "1"],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO stamps (series, year, month_num, sg_number) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params!["S2", 1901, 0, "1"],
    )
    .unwrap();

    // Capture callback invocations via a shared, thread-safe Vec
    use std::sync::{Arc, Mutex};
    let seen = Arc::new(Mutex::new(Vec::new()));
    let seen_cb = seen.clone();

    // Call generator; it will write files under Albums/, which is acceptable for test environment.
    let album_name = "TestCallbackAlbum";
    let _ = generate_album_output_conn(
        &conn,
        album_name,
        Some(move |p: usize| {
            let mut v = seen_cb.lock().unwrap();
            v.push(p);
        }),
        None,
        None,
    );

    // Ensure function returned without panic and examine captured calls
    let recorded = seen.lock().unwrap();
    // We inserted two distinct series; the callback should be invoked at least once and
    // no more than the number of series. Check basic properties.
    assert!(!recorded.is_empty(), "page_callback was not invoked");
    assert!(recorded.len() <= 2, "too many callback invocations");
    // Ensure non-decreasing page numbers
    for w in recorded.windows(2) {
        assert!(w[0] <= w[1], "page numbers should be non-decreasing");
    }
}
