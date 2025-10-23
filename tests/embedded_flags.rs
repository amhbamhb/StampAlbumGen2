use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn embedded_flags_writeback_and_load() {
    // Create a temporary directory and make it the current working dir
    let td = tempdir().expect("tempdir");
    let td_path = td.path().to_path_buf();
    let orig_cwd = std::env::current_dir().expect("get cwd");
    std::env::set_current_dir(&td_path).expect("chdir to temp");

    // Ensure no flags/ exists initially
    let flags_dir = td_path.join("flags");
    if flags_dir.exists() {
        fs::remove_dir_all(&flags_dir).ok();
    }

    // Call the API that should fall back to embedded JSON and attempt to write
    let m = stamp_album_gen2::flagdb::load_flag_map_from_fs();

    // Basic sanity: mapping must include 'switzerland' -> 'ch'
    assert!(m.get("switzerland").map(|s| s == "ch").unwrap_or(false), "expected switzerland -> ch in map");

    // After calling, the flags/flag_codes.json should exist in at least one
    // of the candidate locations (cwd 'flags', build manifest 'flags', or
    // exe-parent 'flags'). Tests run in different environments so accept any.
    let mut found = false;
    let cand1 = flags_dir.join("flag_codes.json");
    if cand1.exists() {
        found = true;
    }
    let cand2 = Path::new(env!("CARGO_MANIFEST_DIR")).join("flags/flag_codes.json");
    if cand2.exists() {
        found = true;
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let cand3 = parent.join("flags/flag_codes.json");
            if cand3.exists() {
                found = true;
            }
        }
    }
    assert!(found, "expected embedded JSON to be written to one of the candidate flags/flag_codes.json locations");

    // Cleanup: restore cwd
    std::env::set_current_dir(orig_cwd).expect("restore cwd");
}
