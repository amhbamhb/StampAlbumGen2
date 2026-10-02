use rusqlite::params;
use std::env;
use std::fs;
use tempfile::tempdir;

/// Test that the generated series header includes a formatted date line when
/// the DB row contains year/month/day and the config enables `include_date`.
#[tokio::test]
async fn test_series_header_includes_date() {
    // Use a temp dir and switch cwd so relative Albums paths resolve there
    let tmp = tempdir().expect("tempdir");
    let orig_cwd = env::current_dir().expect("cwd");
    env::set_current_dir(tmp.path()).expect("chdir to tmp");

    fs::create_dir_all(tmp.path().join("Albums/TestAlbum")).expect("create album dir");

    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("open in-memory DB");

    // Insert a single stamp with a full date
    conn.execute(
        "INSERT INTO stamps (country, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc, uniq_key)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)",
        params![
            "TestCountry",
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
            "testcountry|TestSeries|1",
        ],
    ).expect("insert stamp");

    // Ensure file_utils sees a DB path under the temp dir so generated Albums path is inside tmp
    let db_path = tmp.path().join("Albums/TestAlbum/TestAlbum.db");
    stamp_album_gen2::file_utils::set_db_path_from_cli(db_path.to_str().unwrap());

    // Generate album output for a wide year range so the stamp is included
    stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        "TestAlbum",
        None::<fn(usize)>,
        Some(1900),
        Some(2000),
    )
    .await;

    let output_file = tmp.path().join("Albums/TestAlbum/TestAlbum-1900-2000.txt");
    assert!(
        output_file.exists(),
        "Expected output file to exist: {}",
        output_file.display()
    );

    let contents = fs::read_to_string(&output_file).expect("read output");

    // The default date_format is 'commas' and include_date defaults to true,
    // so we expect to find the date string "January 1, 1950" in the header
    assert!(
        contents.contains("January 1, 1950"),
        "Expected header to contain formatted date; got:\n{}",
        contents
    );

    env::set_current_dir(orig_cwd).expect("restore cwd");
}
