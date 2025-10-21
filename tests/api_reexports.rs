use rusqlite::{params, Connection};

// Integration test that ensures the library-level re-exports from `src/lib.rs`
// are available and behave as expected. We call `insert_stamp_placeholder`
// via the crate root re-export and assert correct insertion semantics.

#[test]
fn insert_stamp_placeholder_via_crate_reexport() {
    // Create an in-memory DB with the canonical stamps table shape (minimal)
    let conn = Connection::open_in_memory().expect("open memory db");
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
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT,
            uniq_key TEXT
        );",
    ).expect("create table");

    // Call the re-exported helper from the crate root. Use the public re-export
    // path (crate_name::insert_stamp_placeholder) to validate the API surface.
    let inserted = stamp_album_gen2::insert_stamp_placeholder(&conn, "Switzerland", "Example Series", 1975, "1040").expect("insert placeholder");
    assert!(inserted, "First insertion should insert a placeholder");

    // Second insertion should not insert (already exists)
    let inserted2 = stamp_album_gen2::insert_stamp_placeholder(&conn, "Switzerland", "Example Series", 1975, "1040").expect("second insert");
    assert!(!inserted2, "Second insertion should be a no-op because row exists");

    // Verify inserted row and that the type column is the expected non-NULL default
    let (id, t): (i64, String) = conn.query_row(
        "SELECT id, type FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3",
        params!["Switzerland", "Example Series", "1040"],
        |r| Ok((r.get(0)?, r.get(1)?)),
    ).expect("query row");
    assert!(id > 0, "row id should be present");
    assert_eq!(t, "PLACEHOLDER");
}
