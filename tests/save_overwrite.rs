use regex::Regex;
use std::fs;

#[test]
fn default_creates_timestamped_copy() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let dir = tmp.path();
    let target = dir.join("album.txt");

    let cfg = stamp_album_gen2::commands_file_generator::CommandsFileGenerator {
        data_source: "test".to_string(),
        ..Default::default()
    };

    // First write creates the primary file
    cfg.save_stamp_data(&target, &["first".to_string()], "test", false);
    assert!(
        target.exists(),
        "primary file should exist after first write"
    );

    // Second write with overwrite=false should create a timestamped sibling
    cfg.save_stamp_data(&target, &["second".to_string()], "test", false);

    let entries: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|r| r.unwrap().file_name().to_string_lossy().to_string())
        .collect();

    // Expect at least two files and one matching `album.created-<ts>.txt`
    assert!(
        entries.len() >= 2,
        "expected at least 2 files, got {:?}",
        entries
    );
    let re = Regex::new(r"^album\.created-\d{8}-\d{6}\.txt$").unwrap();
    let found = entries.iter().any(|n| re.is_match(n));
    assert!(
        found,
        "expected a timestamped created file; entries={:?}",
        entries
    );
}

#[test]
fn overwrite_replaces_in_place() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let dir = tmp.path();
    let target = dir.join("album2.txt");

    let cfg = stamp_album_gen2::commands_file_generator::CommandsFileGenerator {
        data_source: "test".to_string(),
        ..Default::default()
    };

    // Create initial file (default behavior)
    cfg.save_stamp_data(&target, &["orig".to_string()], "test", false);
    assert!(target.exists());
    let orig = fs::read_to_string(&target).unwrap();
    assert!(orig.contains("orig"));

    // Now overwrite in-place
    cfg.save_stamp_data(&target, &["updated".to_string()], "test", true);

    // Ensure only one file (no .created-*) and that content was updated
    let entries: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|r| r.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    let re_created = Regex::new(r"\.created-\d{8}-\d{6}").unwrap();
    let any_created = entries.iter().any(|n| re_created.is_match(n));
    assert!(
        !any_created,
        "did not expect created siblings when overwriting; entries={:?}",
        entries
    );

    let updated = fs::read_to_string(&target).unwrap();
    assert!(
        updated.contains("updated"),
        "file should contain updated content"
    );
}
