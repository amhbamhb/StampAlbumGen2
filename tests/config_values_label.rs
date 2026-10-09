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

fn read_stamp_heading_label(output_txt: &std::path::Path) -> Option<String> {
    if !output_txt.exists() {
        return None;
    }
    let s = fs::read_to_string(output_txt).ok()?;
    assert!(s.lines().any(|line| line == "ROW_ALIGN_TOP"));
    assert!(!s.lines().any(|line| line == "ROW_ALIGN_MIDDLE"));
    for line in s.lines() {
        if line.starts_with("STAMP_HEADING_VA(") {
            // Look for the quoted label inside the parentheses
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    return Some(line[start + 1..start + 1 + end].to_string());
                }
            }
        }
    }
    None
}

#[tokio::test]
async fn test_stamp_heading_label_changes_output() {
    // Create tempdir and switch cwd so load_config picks up our config.toml
    let td = tempdir().expect("tempdir");
    let orig = env::current_dir().expect("cwd");
    env::set_current_dir(td.path()).expect("chdir");

    // Prepare DB
    let db_path = td.path().join("TestAlbum.db");
    let db_str = db_path.to_string_lossy().to_string();
    make_test_db(&db_str);

    file_utils::set_overwrite_album_from_cli(true);

    // First config: label LBL
    let cfg1 = r#"stamp_heading_label = "LBL""#;
    write_config(td.path(), cfg1);

    let conn = db_utils::init_db(&db_str).expect("open db");
    generate_album_output_conn(
        &conn,
        "TestAlbum",
        Option::<fn(usize)>::None,
        Some(1970),
        Some(1970),
    )
    .await;
    let out1 = td
        .path()
        .join("Albums")
        .join("TestAlbum")
        .join("TestAlbum-1970-1970.txt");
    let lbl1 = read_stamp_heading_label(&out1).expect("found heading label in output");

    // Second config: label NEWLBL
    let cfg2 = r#"stamp_heading_label = "NEWLBL""#;
    write_config(td.path(), cfg2);

    let conn2 = db_utils::init_db(&db_str).expect("open db2");
    generate_album_output_conn(
        &conn2,
        "TestAlbum",
        Option::<fn(usize)>::None,
        Some(1970),
        Some(1970),
    )
    .await;
    let out2 = td
        .path()
        .join("Albums")
        .join("TestAlbum")
        .join("TestAlbum-1970-1970.txt");
    let lbl2 = read_stamp_heading_label(&out2).expect("found heading label in output 2");

    assert_ne!(
        lbl1, lbl2,
        "changing stamp_heading_label should change emitted heading label"
    );

    env::set_current_dir(orig).expect("restore cwd");
}
