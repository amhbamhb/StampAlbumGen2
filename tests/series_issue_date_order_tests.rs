use serial_test::serial;
use stamp_album_gen2::album_globals::generate_album_output_conn;
use stamp_album_gen2::db_utils;
use stamp_album_gen2::file_utils;
use std::env;
use std::fs;
use tempfile::tempdir;

struct CwdRestore {
    original: std::path::PathBuf,
}

impl CwdRestore {
    fn new() -> Self {
        Self {
            original: env::current_dir().expect("cwd"),
        }
    }
}

impl Drop for CwdRestore {
    fn drop(&mut self) {
        let _ = env::set_current_dir(&self.original);
    }
}

fn write_config(dir: &std::path::Path, contents: &str) {
    let path = dir.join("config.toml");
    fs::write(&path, contents).expect("write config");
}

fn insert_stamp(
    conn: &rusqlite::Connection,
    series: &str,
    sg_number: &str,
    month: &str,
    month_num: &str,
    day: &str,
) {
    db_utils::insert_stamp(
        conn,
        "Denmark",
        series,
        1937,
        month,
        month_num,
        day,
        sg_number,
        "stamp",
        "1",
        "red",
        "description",
        "100",
        "mint",
        "",
        "",
        "",
        "",
        &format!("images/{}.png", sg_number),
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    )
    .expect("insert stamp");
}

fn read_stamp_diag_order(output_txt: &std::path::Path) -> Vec<String> {
    let mut order = Vec::new();
    let content = fs::read_to_string(output_txt).expect("read output");
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("# STAMP_DIAG ") {
            if let Some(sg_start) = rest.find("sg=\"") {
                let tail = &rest[sg_start + 4..];
                if let Some(end) = tail.find('"') {
                    order.push(tail[..end].to_string());
                }
            }
        }
    }
    order
}

fn find_generated_album_txt(root: &std::path::Path, album_name: &str) -> std::path::PathBuf {
    let album_dir = root.join("Albums").join(album_name);
    let entries = fs::read_dir(&album_dir).expect("read album output directory");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("txt") {
            return path;
        }
    }
    panic!("no generated .txt found in {}", album_dir.display());
}

#[test]
#[serial]
fn test_series_sort_uses_calendar_month_then_day() {
    let _cwd_restore = CwdRestore::new();
    let td = tempdir().expect("tempdir");
    env::set_current_dir(td.path()).expect("chdir");

    write_config(td.path(), "stamp_heading_label = \"Scott\"\n");

    let db_path = td.path().join("Denmark.db");
    let conn = db_utils::init_db(db_path.to_str().expect("db path")).expect("init db");
    insert_stamp(&conn, "Test Series", "100", "January", "1", "12");
    insert_stamp(&conn, "Test Series", "300", "May", "5", "3");
    insert_stamp(&conn, "Test Series", "200", "September", "9", "1");

    file_utils::set_overwrite_album_from_cli(true);
    generate_album_output_conn(
        &conn,
        "Denmark",
        Option::<fn(usize)>::None,
        Some(1937),
        Some(1937),
    );

    let output_txt = find_generated_album_txt(td.path(), "Denmark");
    let order = read_stamp_diag_order(&output_txt);

    assert_eq!(
        order,
        vec!["100", "300", "200"],
        "stamps should sort by month/day within the year before catalog number"
    );
}

#[test]
#[serial]
fn test_series_order_uses_earliest_issue_date() {
    let _cwd_restore = CwdRestore::new();
    let td = tempdir().expect("tempdir");
    env::set_current_dir(td.path()).expect("chdir");

    write_config(
        td.path(),
        "stamp_heading_label = \"Scott\"\nseries_order = \"first_date\"\n",
    );

    let db_path = td.path().join("Denmark.db");
    let conn = db_utils::init_db(db_path.to_str().expect("db path")).expect("init db");

    // Intentionally use lexicographically confusing names so date ordering is obvious.
    insert_stamp(&conn, "1937 Zeta Late", "100", "September", "9", "1");
    insert_stamp(&conn, "1937 Alpha Early", "200", "January", "1", "20");
    insert_stamp(&conn, "1937 Middle", "300", "May", "5", "15");

    let series_from_db = db_utils::get_all_series(&conn, Some("first_date")).expect("series list");
    assert_eq!(
        series_from_db,
        vec![
            "1937 Alpha Early".to_string(),
            "1937 Middle".to_string(),
            "1937 Zeta Late".to_string(),
        ],
        "series should sort by earliest issue date (Jan < May < Sep)"
    );

    let mut iterated = Vec::new();
    stamp_album_gen2::album_globals::generator_new::for_each_series(
        &conn,
        Some("first_date"),
        |_, s| iterated.push(s.to_string()),
    )
    .expect("series iteration");
    assert_eq!(
        iterated,
        vec![
            "1937 Alpha Early".to_string(),
            "1937 Middle".to_string(),
            "1937 Zeta Late".to_string(),
        ],
        "generator series iterator should honor requested first_date ordering"
    );
}
