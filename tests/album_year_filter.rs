use rusqlite::Connection;
use stamp_album_gen2::album_globals::generate_album_output_conn;
use stamp_album_gen2::db_utils::init_db;

fn insert_fixture_stamp(
    conn: &Connection,
    year: i32,
    month: &str,
    month_num: i32,
    day: &str,
    sg_number: &str,
    denomination: &str,
    color: &str,
    description: &str,
    image_path: &str,
) {
    let uniq_key = format!("testcountry|Example Series|{}", sg_number);
    conn.execute(
        "INSERT INTO stamps (
            country, series, year, month, month_num, day, sg_number, catalog_code,
            type, denomination, color, description, stamps_issued, mint_condition,
            unused, used, letter_fdc, currency, image_path, remote_image_url,
            original_image_url, watermark, design, engraving, perforation, context,
            uniq_key
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, '', ?8, ?9, ?10, ?11, '',
            'Mint', '', '', '', 'USD', ?12, '', '', 'WM', 'Design',
            'Engraving', '11', 'ctx', ?13
        )",
        rusqlite::params![
            "TestCountry",
            "Example Series",
            year,
            month,
            month_num,
            day,
            sg_number,
            "Postage",
            denomination,
            color,
            description,
            image_path,
            uniq_key,
        ],
    )
    .expect("insert fixture stamp");
}

// Helper: create an in-memory DB and populate with a few stamps across years
fn populate_test_db() -> Connection {
    // Use a temporary on-disk DB so init_db migrations run the same path
    let conn = init_db(":memory:").expect("init in-memory db");
    insert_fixture_stamp(
        &conn,
        1975,
        "January",
        1,
        "1",
        "1001",
        "1p",
        "Red",
        "Desc 1975",
        "images/1975.jpg",
    );
    insert_fixture_stamp(
        &conn,
        1976,
        "February",
        2,
        "2",
        "1002",
        "2p",
        "Blue",
        "Desc 1976",
        "images/1976.jpg",
    );
    insert_fixture_stamp(
        &conn,
        1977,
        "March",
        3,
        "3",
        "1003",
        "3p",
        "Green",
        "Desc 1977",
        "images/1977.jpg",
    );
    conn
}

#[test]
fn test_generate_album_filters_by_year_exact() {
    let conn = populate_test_db();
    // Generate album output restricted to 1976 only
    let output_path = generate_album_output_conn(
        &conn,
        "Example Series",
        Some(|_p| {}),
        Some(1976),
        Some(1976),
    );
    let contents = std::fs::read_to_string(&output_path).expect("read generated output file");
    // Should contain only the 1976 stamp (1002) and exclude 1975/1977 (1001/1003).
    assert!(contents.contains("Scott : 1002"));
    assert!(!contents.contains("Scott : 1001"));
    assert!(!contents.contains("Scott : 1003"));
}

#[test]
fn test_generate_album_filters_by_year_open_ended() {
    let conn = populate_test_db();
    // start=1976, end=None -> include 1976 and 1977
    let output_path =
        generate_album_output_conn(&conn, "Example Series", Some(|_p| {}), Some(1976), None);
    let contents = std::fs::read_to_string(&output_path).expect("read generated output file");
    assert!(contents.contains("Scott : 1002"));
    assert!(contents.contains("Scott : 1003"));
    assert!(!contents.contains("Scott : 1001"));
}
