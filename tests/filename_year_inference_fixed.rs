use lazy_static::lazy_static;
use std::fs;
use std::sync::Mutex;
use tempfile;

lazy_static! {
    static ref TEST_CWD_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn filename_infers_db_min_max_when_none_fixed() {
    let _lock = TEST_CWD_MUTEX.lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();

    // Create DB and insert stamps with min/max years
    let db_path = cwd.join("st.db");
    let conn = stamp_album_gen2::db_utils::init_db(&db_path.to_string_lossy()).unwrap();
    // insert two stamps (use insert_stamp exact signature: 25 args after conn)
    let _ = stamp_album_gen2::db_utils::insert_stamp(
        &conn, "C", "S", 1901, "", "0", "", "1", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "",
    );
    let _ = stamp_album_gen2::db_utils::insert_stamp(
        &conn, "C", "S", 1955, "", "0", "", "2", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "",
    );

    // Run generator with no start/end
    let album = "TestAlbum";
    std::env::set_current_dir(cwd).unwrap();
    // Ensure no leftover Albums dir
    let _ = std::fs::remove_dir_all(cwd.join("Albums"));
    stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        album,
        None::<fn(usize)>,
        None,
        None,
    );

    // Expect file Albums/TestAlbum/TestAlbum-1901-1955.txt (look for any matching file)
    let album_dir = cwd.join(format!("Albums/{}", album));
    let found = if let Ok(entries) = std::fs::read_dir(&album_dir) {
        entries.filter_map(|e| e.ok()).any(|ent| {
            let n = ent.file_name().to_string_lossy().to_string();
            n.contains(&format!("{}-{}-{}", album.replace(' ', "-"), 1901, 1955))
        })
    } else {
        false
    };
    assert!(found, "expected generated file in {:?}", album_dir);
}

#[test]
fn filename_uses_passed_start_and_db_max_when_end_missing_fixed() {
    let _lock = TEST_CWD_MUTEX.lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();
    let db_path = cwd.join("st2.db");
    let conn = stamp_album_gen2::db_utils::init_db(&db_path.to_string_lossy()).unwrap();
    let _ = stamp_album_gen2::db_utils::insert_stamp(
        &conn, "C", "S", 1910, "", "0", "", "1", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "",
    );
    let _ = stamp_album_gen2::db_utils::insert_stamp(
        &conn, "C", "S", 1940, "", "0", "", "2", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "",
    );

    let album = "TestAlbum2";
    std::env::set_current_dir(cwd).unwrap();
    let _ = std::fs::remove_dir_all(cwd.join("Albums"));
    stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        album,
        None::<fn(usize)>,
        Some(1920),
        None,
    );

    let album_dir = cwd.join(format!("Albums/{}", album));
    let found = if let Ok(entries) = std::fs::read_dir(&album_dir) {
        entries.filter_map(|e| e.ok()).any(|ent| {
            let n = ent.file_name().to_string_lossy().to_string();
            n.contains(&format!("{}-{}-{}", album.replace(' ', "-"), 1920, 1940))
        })
    } else {
        false
    };
    assert!(found, "expected generated file in {:?}", album_dir);
}

#[test]
fn filename_uses_db_min_and_passed_end_when_start_missing_fixed() {
    let _lock = TEST_CWD_MUTEX.lock().unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();
    let db_path = cwd.join("st3.db");
    let conn = stamp_album_gen2::db_utils::init_db(&db_path.to_string_lossy()).unwrap();
    let _ = stamp_album_gen2::db_utils::insert_stamp(
        &conn, "C", "S", 1888, "", "0", "", "1", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "",
    );
    let _ = stamp_album_gen2::db_utils::insert_stamp(
        &conn, "C", "S", 1905, "", "0", "", "2", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "",
    );

    let album = "TestAlbum3";
    std::env::set_current_dir(cwd).unwrap();
    let _ = std::fs::remove_dir_all(cwd.join("Albums"));
    stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        album,
        None::<fn(usize)>,
        None,
        Some(1900),
    );

    let album_dir = cwd.join(format!("Albums/{}", album));
    let found = if let Ok(entries) = std::fs::read_dir(&album_dir) {
        entries.filter_map(|e| e.ok()).any(|ent| {
            let n = ent.file_name().to_string_lossy().to_string();
            n.contains(&format!("{}-{}-{}", album.replace(' ', "-"), 1888, 1900))
        })
    } else {
        false
    };
    assert!(found, "expected generated file in {:?}", album_dir);
}
