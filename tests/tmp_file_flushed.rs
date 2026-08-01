use stamp_album_gen2::album_globals::generate_album_output_conn;
use stamp_album_gen2::db_utils;
use stamp_album_gen2::file_utils;
use std::env;
use std::fs;
use tempfile::tempdir;

fn write_config(dir: &std::path::Path, contents: &str) {
    let p = dir.join("config.toml");
    std::fs::write(&p, contents).expect("write config");
}

fn make_test_db(db_path: &str) {
    let conn = db_utils::init_db(db_path).expect("init db");
    db_utils::insert_stamp_placeholder(&conn, "TestAlbum", "Test Series", 1970, "1")
        .expect("insert placeholder");
}

#[test]
fn test_temp_file_flush_and_output_non_empty() {
    // Minimal test that ensures generator produces STAMP lines in the output file
    // (this implicitly verifies the temp file was flushed before reading).
    let td = tempdir().expect("tempdir");
    let orig = env::current_dir().expect("cwd");
    env::set_current_dir(td.path()).expect("chdir");

    // Prepare DB
    let db_path = td.path().join("TestAlbum.db");
    let db_str = db_path.to_string_lossy().to_string();
    make_test_db(&db_str);

    file_utils::set_overwrite_album_from_cli(true);

    // Ensure minimal config exists (defaults are fine)
    let cfg = r#""#;
    write_config(td.path(), cfg);

    let conn = db_utils::init_db(&db_str).expect("open db");
    generate_album_output_conn(
        &conn,
        "TestAlbum",
        Option::<fn(usize)>::None,
        Some(1970),
        Some(1970),
    );

    let out = td
        .path()
        .join("Albums")
        .join("TestAlbum")
        .join("TestAlbum-1970-1970.txt");
    assert!(
        out.exists(),
        "Expected output file to exist: {}",
        out.display()
    );
    let s = fs::read_to_string(&out).expect("read output");
    // At least one STAMP_* line should be present
    assert!(
        s.lines().any(|l| l.starts_with("STAMP")),
        "Expected STAMP lines in output but file was:\n{}",
        s
    );

    env::set_current_dir(orig).expect("restore cwd");
}
