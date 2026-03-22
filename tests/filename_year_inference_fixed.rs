use lazy_static::lazy_static;
use std::sync::Mutex;
use tempfile;

lazy_static! {
    static ref TEST_CWD_MUTEX: Mutex<()> = Mutex::new(());
}

fn insert_minimal_stamp(conn: &rusqlite::Connection, year: i32, sg_number: &str) {
    let uniq_key = format!("c|S|{}", sg_number);
    conn.execute(
        "INSERT INTO stamps (
            country, series, year, month, month_num, day, sg_number, catalog_code,
            type, denomination, color, description, stamps_issued, mint_condition,
            unused, used, letter_fdc, currency, image_path, remote_image_url,
            original_image_url, watermark, design, engraving, perforation, context,
            uniq_key
        ) VALUES (
            ?1, ?2, ?3, '', 0, '', ?4, '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', ?5
        )",
        rusqlite::params!["C", "S", year, sg_number, uniq_key],
    )
    .expect("insert minimal stamp");
}

#[test]
fn filename_infers_db_min_max_when_none_fixed() {
    let _lock = TEST_CWD_MUTEX.lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();
    let orig_dir = std::env::current_dir().unwrap();

    // Create DB and insert stamps with min/max years
    let db_path = cwd.join("st.db");
    let conn = stamp_album_gen2::db_utils::init_db(&db_path.to_string_lossy()).unwrap();
    insert_minimal_stamp(&conn, 1901, "1");
    insert_minimal_stamp(&conn, 1955, "2");

    // Run generator with no start/end
    let album = "TestAlbum";
    std::env::set_current_dir(cwd).unwrap();
    // Ensure no leftover Albums dir
    let _ = std::fs::remove_dir_all(cwd.join("Albums"));
    let output_path = stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        album,
        None::<fn(usize)>,
        None,
        None,
    );
    assert_eq!(
        output_path.file_name().and_then(|s| s.to_str()),
        Some("TestAlbum-1901-1955.txt")
    );
    assert!(
        output_path.exists(),
        "expected generated file at {:?}",
        output_path
    );
    std::env::set_current_dir(orig_dir).unwrap();
}

#[test]
fn filename_uses_passed_start_and_db_max_when_end_missing_fixed() {
    let _lock = TEST_CWD_MUTEX.lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();
    let orig_dir = std::env::current_dir().unwrap();
    let db_path = cwd.join("st2.db");
    let conn = stamp_album_gen2::db_utils::init_db(&db_path.to_string_lossy()).unwrap();
    insert_minimal_stamp(&conn, 1910, "1");
    insert_minimal_stamp(&conn, 1940, "2");

    let album = "TestAlbum2";
    std::env::set_current_dir(cwd).unwrap();
    let _ = std::fs::remove_dir_all(cwd.join("Albums"));
    let output_path = stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        album,
        None::<fn(usize)>,
        Some(1920),
        None,
    );
    assert_eq!(
        output_path.file_name().and_then(|s| s.to_str()),
        Some("TestAlbum2-1920-1940.txt")
    );
    assert!(
        output_path.exists(),
        "expected generated file at {:?}",
        output_path
    );
    std::env::set_current_dir(orig_dir).unwrap();
}

#[test]
fn filename_uses_db_min_and_passed_end_when_start_missing_fixed() {
    let _lock = TEST_CWD_MUTEX.lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();
    let orig_dir = std::env::current_dir().unwrap();
    let db_path = cwd.join("st3.db");
    let conn = stamp_album_gen2::db_utils::init_db(&db_path.to_string_lossy()).unwrap();
    insert_minimal_stamp(&conn, 1888, "1");
    insert_minimal_stamp(&conn, 1905, "2");

    let album = "TestAlbum3";
    std::env::set_current_dir(cwd).unwrap();
    let _ = std::fs::remove_dir_all(cwd.join("Albums"));
    let output_path = stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        album,
        None::<fn(usize)>,
        None,
        Some(1900),
    );
    assert_eq!(
        output_path.file_name().and_then(|s| s.to_str()),
        Some("TestAlbum3-1888-1900.txt")
    );
    assert!(
        output_path.exists(),
        "expected generated file at {:?}",
        output_path
    );
    std::env::set_current_dir(orig_dir).unwrap();
}
