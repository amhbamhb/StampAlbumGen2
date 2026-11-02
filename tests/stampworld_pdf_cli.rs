use clap::Parser;
use stamp_album_gen2::{cli::Cli, modes::run_stampworld_file_import};
use std::path::Path;

#[test]
fn cli_import_targets_explicit_db_instead_of_output_only_mode() {
    let temp = tempfile::tempdir().expect("isolated CLI target");
    let db = temp.path().join("Great Britain.db");
    let pdf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("download-pdf/Great Britain - Postage stamps (1970 - 1979).pdf");
    let cli = Cli::try_parse_from([
        "stamp_album_gen2",
        "--source",
        "pdf",
        "--source-file",
        pdf.to_str().expect("fixture path"),
        "--site-name",
        "Great Britain",
        "--db-path",
        db.to_str().expect("DB path"),
    ])
    .expect("parse PDF arguments");
    run_stampworld_file_import(&cli).expect("CLI import");
    run_stampworld_file_import(&cli).expect("CLI repeat import");
    let conn = rusqlite::Connection::open(&db).expect("CLI output DB");
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM stamps", [], |r| r.get(0))
        .expect("CLI persisted rows");
    assert_eq!(count, 22);
}

#[test]
fn cli_import_requires_a_file_and_destination_country_before_writing() {
    for args in [
        vec![
            "stamp_album_gen2",
            "--source",
            "pdf",
            "--album-name",
            "Test",
        ],
        vec![
            "stamp_album_gen2",
            "--source",
            "pdf",
            "--source-file",
            "test.pdf",
        ],
    ] {
        let cli = Cli::try_parse_from(args).expect("parse incomplete import");
        assert!(run_stampworld_file_import(&cli).is_err());
    }
}

#[test]
fn executable_routes_pdf_with_db_path_to_import_not_album_generation() {
    let temp = tempfile::tempdir().expect("isolated CLI runtime");
    std::fs::create_dir(temp.path().join("Albums")).expect("runtime marker");
    let config =
        toml::to_string(&stamp_album_gen2::config::Config::with_defaults()).expect("test config");
    std::fs::write(temp.path().join("config.toml"), config).expect("isolated config");
    let db = temp.path().join("Great Britain.db");
    let pdf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("download-pdf/Great Britain - Postage stamps (1970 - 1979).pdf");
    for expected in ["imported=22, skipped=0", "imported=0, skipped=22"] {
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_stamp_album_gen2"))
            .current_dir(temp.path())
            .env("HOME", temp.path())
            .env("STAMP_ALBUM_ALLOW_ANCESTOR_RUNTIME_ROOT", "0")
            .env("STAMP_ALBUM_FORCE_FIRST_RUN", "0")
            .env_remove("DISPLAY")
            .env_remove("WAYLAND_DISPLAY")
            .args(["--source", "pdf", "--source-file"])
            .arg(&pdf)
            .args(["--site-name", "Great Britain", "--db-path"])
            .arg(&db)
            .output()
            .expect("run actual CLI binary");
        assert!(
            output.status.success(),
            "stdout={}\nstderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            String::from_utf8_lossy(&output.stdout).contains(expected),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
    let conn = rusqlite::Connection::open(db).expect("CLI persisted DB");
    assert_eq!(
        conn.query_row("SELECT COUNT(*) FROM stamps", [], |r| r.get::<_, i64>(0))
            .expect("count"),
        22
    );
}
