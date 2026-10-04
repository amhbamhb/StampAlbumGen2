use rusqlite::Connection;
use stamp_album_gen2::album_globals::generate_album_output_conn;
use stamp_album_gen2::db_utils::init_db;

#[derive(Clone, Copy)]
struct FixtureStamp<'a> {
    year: i32,
    month: &'a str,
    month_num: i32,
    day: &'a str,
    sg_number: &'a str,
    denomination: &'a str,
    color: &'a str,
    description: &'a str,
    image_path: &'a str,
}

fn insert_fixture_stamp(conn: &Connection, stamp: &FixtureStamp) {
    let FixtureStamp {
        year,
        month,
        month_num,
        day,
        sg_number,
        denomination,
        color,
        description,
        image_path,
    } = *stamp;
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
        &FixtureStamp {
            year: 1975,
            month: "January",
            month_num: 1,
            day: "1",
            sg_number: "1001",
            denomination: "1p",
            color: "Red",
            description: "Desc 1975",
            image_path: "images/1975.jpg",
        },
    );
    insert_fixture_stamp(
        &conn,
        &FixtureStamp {
            year: 1976,
            month: "February",
            month_num: 2,
            day: "2",
            sg_number: "1002",
            denomination: "2p",
            color: "Blue",
            description: "Desc 1976",
            image_path: "images/1976.jpg",
        },
    );
    insert_fixture_stamp(
        &conn,
        &FixtureStamp {
            year: 1977,
            month: "March",
            month_num: 3,
            day: "3",
            sg_number: "1003",
            denomination: "3p",
            color: "Green",
            description: "Desc 1977",
            image_path: "images/1977.jpg",
        },
    );
    conn
}

#[tokio::test]
async fn test_generate_album_filters_by_year_exact() {
    let conn = populate_test_db();
    // Generate album output restricted to 1976 only
    let output_path = generate_album_output_conn(
        &conn,
        "Example Series",
        Some(|_p| {}),
        Some(1976),
        Some(1976),
    )
    .await;
    let contents = std::fs::read_to_string(&output_path).expect("read generated output file");
    // Should contain only the 1976 stamp (1002) and exclude 1975/1977 (1001/1003).
    // Catalog heading prefix depends on configured default catalog (e.g. SG/Scott),
    // so assert on the shared heading suffix instead of a hardcoded label.
    assert!(contents.contains(" : 1002 \\n Perforation"));
    assert!(!contents.contains(" : 1001 \\n Perforation"));
    assert!(!contents.contains(" : 1003 \\n Perforation"));
}

#[tokio::test]
async fn test_generate_album_filters_by_year_open_ended() {
    let conn = populate_test_db();
    // start=1976, end=None -> include 1976 and 1977
    let output_path =
        generate_album_output_conn(&conn, "Example Series", Some(|_p| {}), Some(1976), None).await;
    let contents = std::fs::read_to_string(&output_path).expect("read generated output file");
    assert!(contents.contains(" : 1002 \\n Perforation"));
    assert!(contents.contains(" : 1003 \\n Perforation"));
    assert!(!contents.contains(" : 1001 \\n Perforation"));
}
