use std::fs;
use std::env;
use tempfile::tempdir;
use rusqlite::{Connection, params};

/// Create an in-memory DB with the columns our generator expects, insert a
/// single stamp, and call generate_album_output_conn. Verify an output file
/// is created in a temporary Albums directory.
#[test]
fn generate_with_in_memory_conn() {
    // Create a temp dir and switch current dir so relative Albums/ paths resolve there
    let tmp = tempdir().expect("tempdir");
    let orig_cwd = env::current_dir().expect("cwd");
    env::set_current_dir(tmp.path()).expect("chdir to tmp");

    // Ensure Albums/TestAlbum exists
    fs::create_dir_all(tmp.path().join("Albums/TestAlbum")).expect("create album dir");

    // Prepare an in-memory DB and create the stamps table with required columns
    let conn = Connection::open_in_memory().expect("open in-memory DB");
    conn.execute_batch(
        "CREATE TABLE stamps (
            id INTEGER PRIMARY KEY,
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
        );",
    ).expect("create table");

    conn.execute(
        "INSERT INTO stamps (year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)",
        params![
            1950i32,
            "January",
            1i32,
            "1",
            "1",
            "Definitive",
            "1d",
            "Red",
            "Test stamp",
            "",
            "no_image_placeholder.png",
            "",
            "",
            "",
            "",
            "",
            "",
            "TestSeries",
            "USD",
            "",
            "",
            "",
            "",
            "",
        ],
    ).expect("insert stamp");

    // Ensure the file_utils path setter sees a DB under tmp (some code uses this)
    stamp_album_gen2::file_utils::set_db_path_from_cli("Albums/TestAlbum/TestAlbum.db");

    // Run the generator using the in-memory connection
    stamp_album_gen2::album_globals::generate_album_output_conn(&conn, "TestAlbum", None::<fn(usize)>, Some(1900), Some(2000));

    // Check output file exists
    let output_file = tmp.path().join("Albums/TestAlbum/TestAlbum-1900-2000.txt");
    assert!(output_file.exists(), "Expected output file to exist: {}", output_file.display());

    // Restore cwd
    env::set_current_dir(orig_cwd).expect("restore cwd");
}
use std::fs;
use tempfile::tempdir;
use rusqlite::{Connection, params};

// This integration test creates a minimal in-memory DB, inserts a single
// stamps table row, and calls generate_album_output_conn to ensure the
// generator can operate with a live Connection reference.
#[test]
fn generate_with_in_memory_conn() {
    // Initialize in-memory DB and create minimal stamps table
    let conn = Connection::open_in_memory().expect("open in-memory DB");
    conn.execute_batch(
        "CREATE TABLE stamps (
            id INTEGER PRIMARY KEY,
            use std::fs;
            use tempfile::tempdir;
            use rusqlite::{Connection, params};

            // This integration test creates a minimal in-memory DB with the columns
            // expected by `get_stamps_by_series`, inserts a single row with non-null
            // string values for text columns, then calls
            // `generate_album_output_conn` to ensure the generator runs without panics
            // and writes an output file.
            #[test]
            fn generate_with_in_memory_conn() {
                let conn = Connection::open_in_memory().expect("open in-memory DB");

                // Create a table with the columns referenced by get_stamps_by_series
                conn.execute_batch(
                    "CREATE TABLE stamps (
                        id INTEGER PRIMARY KEY,
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
                    );",
                ).expect("create table");

                // Insert values for the SELECT projection used by get_stamps_by_series
                conn.execute(
                    "INSERT INTO stamps (year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)",
                    params![
                        1950i32,
                        "January",
                        1i32,
                        "1",
                        "1",
                        "Definitive",
                        "1d",
                        "Red",
                        "Test stamp",
                        "",
                        "no_image_placeholder.png",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "TestSeries",
                        "USD",
                        "",
                        "",
                        "",
                        "",
                        "",
                    ],
                ).expect("insert stamp");

                // Use a temporary directory for Albums output so we don't write into the repo
                let tmp = tempdir().expect("tempdir");
                fs::create_dir_all(tmp.path().join("Albums/TestAlbum")).expect("create album dir");

                // Ensure file_utils sees a DB path under the temp dir (collector/generator may use this)
                let db_path = tmp.path().join("Albums/TestAlbum/TestAlbum.db");
                stamp_album_gen2::file_utils::set_db_path_from_cli(db_path.to_str().unwrap());

                // Run the generator using the in-memory connection
                stamp_album_gen2::album_globals::generate_album_output_conn(&conn, "TestAlbum", None::<fn(usize)>, Some(1900), Some(2000));

                // The generator should have produced an output file under the temp dir
                let output_file = tmp.path().join("Albums/TestAlbum/TestAlbum-1900-2000.txt");
                assert!(output_file.exists(), "Expected output file to exist: {}", output_file.display());
            }
