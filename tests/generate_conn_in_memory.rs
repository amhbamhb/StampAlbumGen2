use rusqlite::params;
use std::env;
use std::fs;
use tempfile::tempdir;

/// Clean integration test: create an in-memory DB, insert one stamp row, call
/// the conn-taking generator API and assert an output file is written under a
/// temporary Albums directory.
#[tokio::test]
async fn test_generate_conn_in_memory_clean() {
    // Use a temp dir and switch cwd so relative Albums paths resolve there
    let tmp = tempdir().expect("tempdir");
    let orig_cwd = env::current_dir().expect("cwd");
    env::set_current_dir(tmp.path()).expect("chdir to tmp");

    fs::create_dir_all(tmp.path().join("Albums/TestAlbum")).expect("create album dir");

    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("open in-memory DB");

    conn.execute(
        "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, catalog_code, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context, uniq_key)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, '', ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)",
        params![
            "TestCountry",
            "TestSeries",
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
            "",
            "",
            "",
            "",
            "no_image_placeholder.png",
            "",
            "",
            "",
            "",
            "",
            "",
            "USD",
            "",
            "testcountry|TestSeries|1",
        ],
    ).expect("insert stamp");

    // Ensure file_utils sees a DB path under the temp dir
    let db_path = tmp.path().join("Albums/TestAlbum/TestAlbum.db");
    stamp_album_gen2::file_utils::set_db_path_from_cli(db_path.to_str().unwrap());

    let output_file = stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        "TestAlbum",
        None::<fn(usize)>,
        Some(1900),
        Some(2000),
    )
    .await;
    assert!(
        output_file.exists(),
        "Expected output file to exist: {}",
        output_file.display()
    );

    env::set_current_dir(orig_cwd).expect("restore cwd");
}
