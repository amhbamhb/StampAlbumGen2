use stamp_album_gen2::album_globals::generate_album_output_conn;

#[test]
fn test_page_callback_invoked_per_series() {
    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("open");

    // Insert two stamps in different series so generator will iterate two series
    conn.execute(
        "INSERT INTO stamps (country, series, year, month_num, sg_number, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params!["TestCountry", "S1", 1900, 0, "1", "testcountry|S1|1"],
    )
    .unwrap();
    conn.execute(
        "INSERT INTO stamps (country, series, year, month_num, sg_number, uniq_key) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params!["TestCountry", "S2", 1901, 0, "1", "testcountry|S2|1"],
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
