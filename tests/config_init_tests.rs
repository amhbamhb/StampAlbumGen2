use std::fs;

use serial_test::serial;
use tempfile::tempdir;

/// Test that `create_config_from_template` creates a config.toml from the
/// bundled template when no target file exists.
#[test]
#[serial]
fn test_create_config_from_template_creates_when_missing() {
    let td = tempdir().unwrap();
    let cwd = td.path();

    // Create a simple template file
    let template_path = cwd.join("config_template.toml");
    fs::write(&template_path, "# template\nkey = \"value\"\n").unwrap();

    // run the initializer
    let orig_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&cwd).unwrap();
    let res = stamp_album_gen2::config::create_config_from_template("config.toml", false);
    std::env::set_current_dir(orig_cwd).unwrap();
    assert!(res.is_ok());

    let target = cwd.join("config.toml");
    assert!(target.exists());
    let content = fs::read_to_string(&target).unwrap();
    assert!(content.contains("template"));
}

/// Test that --force causes the existing config to be backed up and replaced
#[test]
#[serial]
fn test_create_config_from_template_force_backups_and_overwrites() {
    let td = tempdir().unwrap();
    let cwd = td.path();

    // Create an existing config and a template
    let target = cwd.join("config.toml");
    fs::write(&target, "orig = 1\n").unwrap();

    let template_path = cwd.join("config_template.toml");
    fs::write(&template_path, "# new template\nnew = 2\n").unwrap();

    let orig_cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&cwd).unwrap();
    let res = stamp_album_gen2::config::create_config_from_template("config.toml", true);
    std::env::set_current_dir(orig_cwd).unwrap();
    assert!(res.is_ok());

    // The original file should have been replaced by the template contents
    let content = fs::read_to_string(&target).unwrap();
    assert!(content.contains("new = 2"));

    // There should be a backup file with a .bak. timestamp suffix
    let mut found_backup = false;
    for entry in fs::read_dir(&cwd).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if name.starts_with("config.toml.bak.") {
            // Attempt to read the backup file; be defensive if a transient IO
            // error occurs and continue scanning other entries.
            if let Ok(bcontent) = fs::read_to_string(entry.path()) {
                if bcontent.contains("orig = 1") {
                    found_backup = true;
                    break;
                }
            }
        }
    }

    assert!(found_backup, "Expected to find a timestamped backup file");
}

#[test]
fn test_template_defaults_use_fit_box_sizing() {
    let template = include_str!("../config_template.toml");
    assert!(
        template.contains("sizing_mode = \"fit_box\""),
        "Freshly initialized configs should default to fit_box sizing"
    );
}

#[cfg(not(feature = "rfd"))]
#[test]
fn test_first_run_without_terminal_creates_defaults() {
    const CHILD: &str = "STAMP_ALBUM_TEST_FIRST_RUN_CHILD";
    if std::env::var_os(CHILD).is_some() {
        stamp_album_gen2::config::ensure_config_and_template_files(
            "config.toml",
            "config_template.toml",
            "manual.pdf",
        )
        .unwrap();
        return;
    }

    let td = tempdir().unwrap();
    let executable = std::env::current_exe().unwrap();
    let child_executable = td.path().join(executable.file_name().unwrap());
    fs::copy(&executable, &child_executable).unwrap();
    let mut command = std::process::Command::new(&child_executable);
    command
        .current_dir(td.path())
        .env(CHILD, "1")
        .env_remove("DISPLAY")
        .env_remove("WAYLAND_DISPLAY")
        .env_remove("STAMP_ALBUM_FORCE_FIRST_RUN")
        .stdin(std::process::Stdio::null())
        .args([
            "--exact",
            "test_first_run_without_terminal_creates_defaults",
            "--nocapture",
        ]);
    assert_cmd::Command::from_std(command)
        .timeout(std::time::Duration::from_secs(30))
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "No terminal available; skipping console prompt",
        ));

    let template = fs::read_to_string(td.path().join("config_template.toml")).unwrap();
    assert_eq!(
        template,
        stamp_album_gen2::config::embedded_config_template()
    );
    assert_eq!(
        fs::read_to_string(td.path().join("config.toml")).unwrap(),
        template
    );
    assert_eq!(
        fs::read(td.path().join("manual.pdf")).unwrap(),
        stamp_album_gen2::config::embedded_pdf_help()
    );
}
