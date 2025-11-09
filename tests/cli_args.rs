use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help_shows_options() {
    let mut cmd = Command::cargo_bin("stamp_album_gen2").unwrap();
    cmd.arg("--help");
    cmd.assert().success().stdout(
        predicate::str::contains("--site-name")
            .and(predicate::str::contains("--db-path"))
            .and(predicate::str::contains("--overwrite-album")),
    );
}

#[test]
fn no_args_prints_guidance_and_help() {
    let mut cmd = Command::cargo_bin("stamp_album_gen2").unwrap();
    // With no arguments, the program now prints guidance and help
    cmd.assert().failure().stderr(predicate::str::contains(
        "It looks like you didn't provide any command options. Here's a short guide to get started:",
    ));
}

#[test]
fn scrape_mode_missing_site_name_reports_error() {
    let mut cmd = Command::cargo_bin("stamp_album_gen2").unwrap();
    // Pass a harmless option so main does not early-exit with the generic guidance message
    cmd.arg("--max-pages").arg("1");
    cmd.assert().failure().stderr(predicate::str::contains(
        "You started the program in scraping mode but did not provide the site name.",
    ));
}

#[test]
fn db_path_and_album_name_runs_ok() {
    let tmp = tempfile::tempdir().unwrap();
    let cwd = tmp.path();
    let db_path = cwd.join("test.db");

    let mut cmd = Command::cargo_bin("stamp_album_gen2").unwrap();
    cmd.current_dir(cwd)
        .arg("--db-path")
        .arg(db_path.to_string_lossy().to_string())
        .arg("--album-name")
        .arg("TestAlbum");
    // Expect success (program should create DB and generate album output without panicking)
    cmd.assert().success();
}
