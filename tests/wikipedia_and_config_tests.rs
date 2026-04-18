use std::fs;

use tempfile::tempdir;

// These tests exercise two behaviors added recently:
// 1) When no `config.toml` exists, `load_config` will create one by copying
//    `config_template.toml` from the current working directory (preserving
//    comments). We validate the file is created and contains the template
//    marker.
// 2) `ingest_wikipedia_to_db` will read cached content from the wikipedia
//    cache DB and insert up to `max_entries` placeholder rows into the
//    album DB when the album DB contains no rows for the given topic.

#[test]
fn config_template_is_copied_to_config_toml_when_missing() {
    // Prepare a temporary directory and switch cwd into it for the duration
    // of the test so `load_config` searches there and will copy the template.
    let td = tempdir().unwrap();
    let orig_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(td.path()).unwrap();

    // Write a tiny template file with a recognisable marker comment
    let template_path = td.path().join("config_template.toml");
    let template_content =
        "# TEST TEMPLATE MARKER\n# Human-friendly template\n[example]\nkey = \"value\"\n";
    fs::write(&template_path, template_content).unwrap();

    // Use a unique filename so load_config won't accidentally find an existing
    // `config.toml` elsewhere (the loader searches parent dirs and exe path).
    let test_cfg_name = "my_test_config.toml";
    let cfg_path = td.path().join(test_cfg_name);
    if cfg_path.exists() {
        fs::remove_file(&cfg_path).unwrap();
    }

    // Invoke the loader which should create the test config by copying the template
    let cfg = stamp_album_gen2::config::load_config(test_cfg_name);

    // Restore original cwd immediately to avoid affecting other tests
    std::env::set_current_dir(orig_cwd).unwrap();

    // Verify the config file was created and contains the template marker
    assert!(
        cfg_path.exists(),
        "config file should have been created from template"
    );
    let created = fs::read_to_string(&cfg_path).unwrap();
    assert!(
        created.contains("TEST TEMPLATE MARKER"),
        "created file should contain the template marker"
    );

    // Basic sanity check: loader returned a Config (since the loader returns
    // `Config::default()` after creating the file we expect optional fields
    // to be unset/null). Ensure the returned struct is usable.
    assert!(cfg.reference_width_mm.is_none());
}

#[test]
fn wikipedia_ingest_inserts_up_to_max_entries_from_cache() {
    use rusqlite::{params, Connection};

    let td = tempdir().unwrap();
    let cache_db = td.path().join("wiki_cache.db");
    let album_db = td.path().join("album.db");

    let topic = "Testland-Ingest";

    // Create cache DB and insert a content row with two paragraphs
    let conn = Connection::open(&cache_db).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wikipedia_content (id INTEGER PRIMARY KEY, topic TEXT UNIQUE, content TEXT, source TEXT)",
        [],
    )
    .unwrap();

    let content = "First paragraph.\n\nSecond paragraph which is longer.";
    conn.execute(
        "INSERT OR REPLACE INTO wikipedia_content (topic, content, source) VALUES (?1, ?2, ?3)",
        params![topic, content, "wikipedia"],
    )
    .unwrap();

    // Ensure album DB does not contain rows initially
    if album_db.exists() {
        fs::remove_file(&album_db).unwrap();
    }

    // Call ingest with max_entries = 1 (should insert only one paragraph)
    let inserted = stamp_album_gen2::wikipedia::ingest_wikipedia_to_db(
        topic,
        cache_db.to_str().unwrap(),
        album_db.to_str().unwrap(),
        1,
        "Wikipedia import",
    )
    .expect("ingest should run without error");

    assert_eq!(
        inserted, 1,
        "ingest should insert exactly one row when max_entries=1"
    );

    // Verify the album DB now contains one row for the given country/topic
    let aconn = Connection::open(&album_db).unwrap();
    let count: i64 = aconn
        .query_row(
            "SELECT COUNT(*) FROM stamps WHERE country = ?1",
            params![topic],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(count, 1, "album DB should contain one inserted stamp row");
}
