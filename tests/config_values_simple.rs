// Simplified test version that disables placeholder generation to avoid hangs
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

fn make_test_db_simple(db_path: &str) {
    let conn = db_utils::init_db(db_path).expect("init db");
    // Use insert_stamp with fake image paths to skip placeholder generation
    // This should prevent the placeholder image hang during tests
    let _ = db_utils::insert_stamp(
        &conn,
        "TestAlbum",
        "Test Series",
        1974,
        "1",
        "1",
        "1",
        "1",
        "stamp",
        "1p",
        "red",
        "description",
        "100",
        "mint",
        "1",
        "1",
        "fdc",
        "£",
        "fake_image_1.png", // Fake image path to skip placeholder generation
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    );
    let _ = db_utils::insert_stamp(
        &conn,
        "TestAlbum",
        "Test Series",
        1975,
        "1",
        "1",
        "1",
        "2",
        "stamp",
        "2p",
        "blue",
        "description",
        "100",
        "mint",
        "1",
        "1",
        "fdc",
        "£",
        "fake_image_2.png", // Fake image path to skip placeholder generation
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    );
}

#[tokio::test]
async fn config_values_simple_no_placeholder() {
    // Initialize logging for this test
    let _ = env_logger::builder().is_test(true).try_init();

    let td = tempdir().expect("tempdir");
    let orig = env::current_dir().expect("cwd");
    env::set_current_dir(td.path()).expect("chdir");

    let db_path = td.path().join("Simple.db");
    let db_str = db_path.to_string_lossy().to_string();
    make_test_db_simple(&db_str);
    file_utils::set_overwrite_album_from_cli(true);

    let album = "TestAlbum";

    // Test case: change stamp_heading_size
    println!("[TEST] Starting config value test (no placeholder)");

    // Config 1
    write_config(td.path(), "stamp_heading_size = 10\n");
    let conn1 = db_utils::init_db(&db_str).expect("open db1");
    println!("[TEST] Generating with stamp_heading_size = 10");
    generate_album_output_conn(
        &conn1,
        album,
        Option::<fn(usize)>::None,
        Some(1974),
        Some(1975),
    )
    .await;

    let out1 = td
        .path()
        .join("Albums")
        .join(album)
        .join("TestAlbum-1974-1975.txt");
    assert!(out1.exists(), "First output should exist");
    let s1 = fs::read_to_string(&out1).expect("read first output");

    // Config 2
    write_config(td.path(), "stamp_heading_size = 20\n");
    let conn2 = db_utils::init_db(&db_str).expect("open db2");
    println!("[TEST] Generating with stamp_heading_size = 20");
    generate_album_output_conn(
        &conn2,
        album,
        Option::<fn(usize)>::None,
        Some(1974),
        Some(1975),
    )
    .await;

    let out2 = td
        .path()
        .join("Albums")
        .join(album)
        .join("TestAlbum-1974-1975.txt");
    assert!(out2.exists(), "Second output should exist");
    let s2 = fs::read_to_string(&out2).expect("read second output");

    // Verify outputs differ
    assert_ne!(
        s1, s2,
        "Outputs should differ when stamp_heading_size changes"
    );

    // Verify both contain expected stamp content
    assert!(
        s1.contains("STAMP_"),
        "First output should contain STAMP lines"
    );
    assert!(
        s2.contains("STAMP_"),
        "Second output should contain STAMP lines"
    );

    println!("[TEST] Config test passed - outputs differ as expected");
    println!(
        "[TEST] Output 1: {} chars, Output 2: {} chars",
        s1.len(),
        s2.len()
    );

    env::set_current_dir(orig).expect("restore cwd");
}
