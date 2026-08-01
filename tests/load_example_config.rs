use std::env;
use std::fs;
use std::path::Path;

#[test]
fn test_load_example_config_roundtrip() {
    // Locate the repo root relative to the workspace (we assume tests run from repo root)
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example = repo_root.join("config_full_example.toml");
    assert!(
        example.exists(),
        "example config not found: {}",
        example.display()
    );

    // Create a tempdir and copy the file as config.toml so load_config finds it
    let td = tempfile::tempdir().expect("tempdir");
    let td_path = td.path();
    fs::copy(&example, td_path.join("config.toml")).expect("copy config");

    // Change current dir so load_config searches upward and finds our temp config
    let orig = env::current_dir().expect("cwd");
    env::set_current_dir(&td_path).expect("chdir to tempdir");

    // Call loader from the library crate
    let cfg = stamp_album_gen2::config::load_config("config.toml");

    // Basic assertions for a few keys present in the example
    assert_eq!(cfg.target_stamp_height_mm.unwrap(), 24.0_f32);
    assert_eq!(cfg.page_fit_conservative_buffer_mm.unwrap(), 11.0_f32);
    assert_eq!(cfg.expand_grouped_ranges.as_deref(), Some("placeholders"));

    // restore cwd
    env::set_current_dir(orig).expect("restore cwd");
}
