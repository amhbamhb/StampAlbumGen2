use stamp_album_gen2::album_globals::generate_album_output_conn;
use stamp_album_gen2::file_utils;
use std::env;
use std::fs;
use tempfile::tempdir;

fn write_config(dir: &std::path::Path, contents: &str) {
    let p = dir.join("config.toml");
    std::fs::write(&p, contents).expect("write config");
}

/// Helper: create a DB with one placeholder stamp for year 1970
fn make_test_db(db_path: &str) {
    let conn = stamp_album_gen2::db_utils::init_db(db_path).expect("init db");
    // insert a minimal placeholder stamp row
    stamp_album_gen2::db_utils::insert_stamp_placeholder(
        &conn,
        "TestAlbum",
        "Test Series",
        1970,
        "1",
    )
    .expect("insert placeholder");
}

fn read_stamp_heading_size(output_txt: &std::path::Path) -> Option<i32> {
    if !output_txt.exists() {
        return None;
    }
    let s = fs::read_to_string(output_txt).ok()?;
    for line in s.lines() {
        if line.starts_with("STAMP_HEADING(") {
            // Format: STAMP_HEADING(<font> <size> "<label : sg>")
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                if let Ok(sz) = parts[1].parse::<i32>() {
                    return Some(sz);
                }
            }
        }
    }
    None
}

#[test]
fn test_stamp_heading_size_changes_output() {
    // Create tempdir and switch cwd so load_config picks up our config.toml
    let td = tempdir().expect("tempdir");
    let orig = env::current_dir().expect("cwd");
    env::set_current_dir(td.path()).expect("chdir");

    // Prepare DB
    let db_path = td.path().join("TestAlbum.db");
    let db_str = db_path.to_string_lossy().to_string();
    make_test_db(&db_str);

    // Ensure overwrite behaviour so repeated runs replace the file
    file_utils::set_overwrite_album_from_cli(true);

    // First config: heading size 10
    let cfg1 = r#"stamp_heading_size = 10
"#;
    write_config(td.path(), cfg1);

    let conn = stamp_album_gen2::db_utils::init_db(&db_str).expect("open db");
    generate_album_output_conn(
        &conn,
        "TestAlbum",
        Option::<fn(usize)>::None,
        Some(1970),
        Some(1970),
    );
    let out1 = td
        .path()
        .join("Albums")
        .join("TestAlbum")
        .join("TestAlbum-1970-1970.txt");
    let sz1 = read_stamp_heading_size(&out1).expect("found heading size in output");

    // Second config: heading size 20
    let cfg2 = r#"stamp_heading_size = 20
"#;
    write_config(td.path(), cfg2);

    // Re-run generator (same DB)
    let conn2 = stamp_album_gen2::db_utils::init_db(&db_str).expect("open db2");
    generate_album_output_conn(
        &conn2,
        "TestAlbum",
        Option::<fn(usize)>::None,
        Some(1970),
        Some(1970),
    );
    let out2 = td
        .path()
        .join("Albums")
        .join("TestAlbum")
        .join("TestAlbum-1970-1970.txt");
    let sz2 = read_stamp_heading_size(&out2).expect("found heading size in output 2");

    assert_ne!(
        sz1, sz2,
        "changing stamp_heading_size should change emitted heading size"
    );

    // Restore cwd
    env::set_current_dir(orig).expect("restore cwd");
}

// Additional tests for other config values can follow the same pattern: write a config,
// run the generator, read the produced TXT, and assert expected tokens or numeric changes.
