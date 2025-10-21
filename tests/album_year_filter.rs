use stamp_album_gen2::db_utils::{init_db, insert_stamp};
use stamp_album_gen2::album_globals::generate_album_output_conn;
use rusqlite::Connection;

// Helper: create an in-memory DB and populate with a few stamps across years
fn populate_test_db() -> Connection {
    // Use a temporary on-disk DB so init_db migrations run the same path
    let conn = init_db(":memory:").expect("init in-memory db");
    // Insert sample series and stamps
    // Use insert_stamp to populate full rows
    let _ = insert_stamp(&conn, "TestCountry", "Example Series", 1975, "Jan", "1", "1", "A1", "Postage", "1p", "Red", "Desc 1975", "", "Mint", "", "", "", "USD", "images/1975.jpg", "", "", "WM", "Design", "Engraving", "11", "ctx");
    let _ = insert_stamp(&conn, "TestCountry", "Example Series", 1976, "Feb", "2", "2", "A2", "Postage", "2p", "Blue", "Desc 1976", "", "Mint", "", "", "", "USD", "images/1976.jpg", "", "", "WM", "Design", "Engraving", "11", "ctx");
    let _ = insert_stamp(&conn, "TestCountry", "Example Series", 1977, "Mar", "3", "3", "A3", "Postage", "3p", "Green", "Desc 1977", "", "Mint", "", "", "", "USD", "images/1977.jpg", "", "", "WM", "Design", "Engraving", "11", "ctx");
    conn
}

#[test]
fn test_generate_album_filters_by_year_exact() {
    let conn = populate_test_db();
    // Generate album output restricted to 1976 only
    generate_album_output_conn(&conn, "Example Series", Some(|_p| {}), Some(1976), Some(1976));
    // Re-run but capture output by writing to file created by generator; easier to assert
    // The generator writes to Albums/{Album}/{Album}-1976-1976.txt — read that file
    let path = format!("Albums/{}/{}-1976-1976.txt", "Example Series".replace(' ', "-"), "Example Series".replace(' ', "-"));
    let contents = std::fs::read_to_string(&path).expect("read output file");
    // Should contain the 1976 description but not 1975/1977
    assert!(contents.contains("Desc 1976"));
    assert!(!contents.contains("Desc 1975"));
    assert!(!contents.contains("Desc 1977"));
}

#[test]
fn test_generate_album_filters_by_year_open_ended() {
    let conn = populate_test_db();
    // start=1976, end=None -> include 1976 and 1977
    generate_album_output_conn(&conn, "Example Series", Some(|_p| {}), Some(1976), None);
    // Generator only appends a range to the filename when both start and end
    // were provided and are sensible. Fallback to the default filename if the
    // ranged filename does not exist.
    let ranged = format!("Albums/{}/{}-1976-9999.txt", "Example Series".replace(' ', "-"), "Example Series".replace(' ', "-"));
    let default = format!("Albums/{}/{}.txt", "Example Series".replace(' ', "-"), "Example Series".replace(' ', "-"));
    let path = if std::path::Path::new(&ranged).exists() { ranged } else { default };
    let contents = std::fs::read_to_string(&path).expect("read output file");
    assert!(contents.contains("Desc 1976"));
    assert!(contents.contains("Desc 1977"));
    assert!(!contents.contains("Desc 1975"));
}
