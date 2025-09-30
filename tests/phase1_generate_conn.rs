use std::env;
use std::fs;
use tempfile::tempdir;
use rusqlite::{Connection, params};

/// Integration test for Phase 1: verify `generate_album_output_conn` works with
/// a live `rusqlite::Connection` by creating an in-memory DB and checking that
/// an AlbumEasy output file is produced in a temporary `Albums/...` directory.
#[test]
fn phase1_generate_with_in_memory_conn() {
    // Prepare temporary workspace and switch current directory so relative
    // paths like `Albums/...` resolve into the temp directory.
    let tmp = tempdir().expect("tempdir");
    let orig_dir = env::current_dir().expect("get cwd");
    env::set_current_dir(tmp.path()).expect("chdir tmp");

    // Ensure Albums dir exists for output
    fs::create_dir_all(tmp.path().join("Albums/TestAlbum")).expect("create album dir");

    // Create in-memory SQLite DB and build the minimal stamps table schema
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
    ).expect("create stamps table");

    // Insert a single non-null row for every column that get_stamps_by_series
    // expects as a String; use conservative placeholder values.
    conn.execute(
        "INSERT INTO stamps (year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)",
        params![
            1950i32,
            "January",
            1i32,
            "1",
            "1",
            "Type",
            "Denom",
            "Color",
            "A test stamp description",
            "",
            "no_image_placeholder.png",
            "http://example.com/remote.jpg",
            "http://example.com/orig.jpg",
            "",
            "Design",
            "Engraving",
            "Perf",
            "TestAlbumSeries",
            "USD",
            "",
            "",
            "",
            "",
            "",
        ],
    ).expect("insert stamp");

    // Some internal helpers may read a CLI-provided DB path; point that to a
    // path under the temp dir just in case.
    stamp_album_gen2::file_utils::set_db_path_from_cli("Albums/TestAlbum/TestAlbum.db");

    // Call the conn-taking API; this should write Albums/TestAlbum/TestAlbum-1900-2000.txt
    stamp_album_gen2::album_globals::generate_album_output_conn(&conn, "TestAlbum", None::<fn(usize)>, Some(1900), Some(2000));

    let output_path = tmp.path().join("Albums/TestAlbum/TestAlbum-1900-2000.txt");
    assert!(output_path.exists(), "expected output file at {}", output_path.display());

    // Restore working directory
    env::set_current_dir(orig_dir).expect("restore cwd");
}
