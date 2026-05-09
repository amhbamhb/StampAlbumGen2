use rusqlite::Connection;
use stamp_album_gen2::colnect::db::get_colnect_info;
use stamp_album_gen2::colnect::ingest::ingest_colnect_data;
use stamp_album_gen2::colnect::parser::parse_csv;
use stamp_album_gen2::db_utils::init_db;
use std::path::Path;

/// Build an in-memory SQLite DB with the minimal schema for `get_colnect_info`.
fn setup_colnect_db(rows: &[(&str, &str)]) -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute_batch(
        "CREATE TABLE colnect_stamps (
            colnect_id TEXT PRIMARY KEY,
            sg_number  TEXT,
            name       TEXT,
            watermark  TEXT, format TEXT, emission TEXT, perforation TEXT,
            printing   TEXT, paper  TEXT, gum      TEXT, themes      TEXT,
            description TEXT, issued_on TEXT, expiry_date TEXT, print_run TEXT,
            face_value  TEXT, currency  TEXT, colors    TEXT, designers TEXT,
            printers    TEXT, score     TEXT, accuracy  TEXT,
            width TEXT, height TEXT
        );
        CREATE TABLE colnect_catalog_codes (
            colnect_id TEXT,
            catalog_type TEXT,
            code TEXT
        );",
    )
    .unwrap();
    for (sg, id) in rows {
        conn.execute(
            "INSERT INTO colnect_stamps (colnect_id, sg_number, name) VALUES (?1, ?2, ?3)",
            rusqlite::params![id, sg, format!("Stamp {}", sg)],
        )
        .unwrap();
    }
    conn
}

#[test]
fn test_colnect_integration() {
    let db_path = "test_colnect.db";
    let _ = std::fs::remove_file(db_path);

    let conn = init_db(db_path).expect("init_db");

    let csv_path = "en_stamps_csv_list_country_206-Switzerland_year_1972.csv";
    if !Path::new(csv_path).exists() {
        // Skip test if file not found (e.g. in CI)
        println!("Skipping test_colnect_integration: CSV file not found");
        return;
    }

    let stamps = parse_csv(csv_path).expect("parse_csv");
    assert!(!stamps.is_empty());
    println!("Parsed {} stamps", stamps.len());

    ingest_colnect_data(&conn, &stamps).expect("ingest");

    // Verify data
    // One stamp has Sg:CH 824 -> normalized 824
    // Let's check the CSV content for "Sg:CH 824"
    // "Breakdown Service",Switzerland,"Automobile Associations","Mi:CH 964, Sn:CH 540, Yt:CH 895, Sg:CH 824, ...

    let info = get_colnect_info(&conn, "824").expect("get_colnect_info");

    if info.is_none() {
        // Maybe normalization failed or stored differently?
        // Try "CH 824"
        let info2 = get_colnect_info(&conn, "CH 824").expect("get_colnect_info 2");
        if info2.is_some() {
            println!("Found as CH 824");
        } else {
            println!("Not found as 824 or CH 824");
            // Debug: list all sg_numbers
            let mut stmt = conn
                .prepare("SELECT sg_number FROM colnect_stamps")
                .unwrap();
            let rows = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();
            for r in rows {
                println!("Stored SG: {}", r.unwrap());
            }
        }
    }

    assert!(info.is_some(), "Should find stamp with SG 824");
    let info = info.unwrap();
    println!("Fetched info: {:?}", info);

    // Check description
    // "75th anniv. of the touring and automobile clubs of Switzerland: Wrench, Road Sign, Club Emblems  "
    assert!(info.description.unwrap().contains("75th anniv"));

    let _ = std::fs::remove_file(db_path);
}

// ── Variant-fallback validation tests ────────────────────────────────────────

/// Exact match always wins regardless of other rows.
#[test]
fn test_exact_match_returns_correct_stamp() {
    let conn = setup_colnect_db(&[("944", "id-944"), ("944a", "id-944a")]);
    let info = get_colnect_info(&conn, "944").unwrap();
    assert!(info.is_some(), "exact match '944' should be found");
    assert_eq!(
        info.unwrap().name.unwrap(),
        "Stamp 944",
        "should return exact stamp, not variant"
    );
}

/// A non-digit suffix variant (e.g. "944a") is a genuine variant — should match.
#[test]
fn test_nondgit_suffix_variant_is_accepted() {
    // Only "944a" is in DB; query for "944" should return it via the fallback.
    let conn = setup_colnect_db(&[("944a", "id-944a")]);
    let info = get_colnect_info(&conn, "944").unwrap();
    assert!(
        info.is_some(),
        "genuine variant '944a' should match query '944'"
    );
}

/// A digit-suffix candidate (e.g. "468d" against query "4") must be REJECTED.
/// This was the Australia regression: sg=4 was matching Colnect record "468d".
#[test]
fn test_digit_suffix_variant_is_rejected() {
    let conn = setup_colnect_db(&[("468d", "id-468d")]);
    let info = get_colnect_info(&conn, "4").unwrap();
    assert!(
        info.is_none(),
        "digit-suffix candidate '468d' must NOT match query '4'"
    );
}

/// Multiple LIKE candidates: only the genuine variant should be selected.
#[test]
fn test_only_genuine_variant_chosen_when_mixed() {
    // "40d" suffix starts with digit → reject
    // "4a"  suffix starts with 'a'   → accept
    let conn = setup_colnect_db(&[("40d", "id-40d"), ("4a", "id-4a")]);
    let info = get_colnect_info(&conn, "4").unwrap();
    assert!(info.is_some(), "should find the genuine variant '4a'");
    assert_eq!(
        info.unwrap().name.unwrap(),
        "Stamp 4a",
        "should pick '4a', not '40d'"
    );
}

/// Star suffix ("944*") is non-digit — should be accepted as a genuine variant.
#[test]
fn test_star_suffix_variant_is_accepted() {
    let conn = setup_colnect_db(&[("944*", "id-944s")]);
    let info = get_colnect_info(&conn, "944").unwrap();
    assert!(
        info.is_some(),
        "star-suffix variant '944*' should match query '944'"
    );
}

/// No candidates at all → None returned (no panic).
#[test]
fn test_no_match_returns_none() {
    let conn = setup_colnect_db(&[("999", "id-999")]);
    let info = get_colnect_info(&conn, "4").unwrap();
    assert!(info.is_none(), "unrelated stamp should not match");
}
