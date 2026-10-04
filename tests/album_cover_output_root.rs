use std::path::PathBuf;

struct RestoreRuntime {
    cwd: PathBuf,
    albums_root: Option<std::ffi::OsString>,
}

impl Drop for RestoreRuntime {
    fn drop(&mut self) {
        std::env::set_current_dir(&self.cwd).expect("restore cwd");
        match &self.albums_root {
            Some(value) => std::env::set_var("STAMP_ALBUMS_ROOT", value),
            None => std::env::remove_var("STAMP_ALBUMS_ROOT"),
        }
    }
}

#[tokio::test]
#[serial_test::serial]
async fn generated_text_and_local_coat_use_the_same_configured_root() {
    let dir = tempfile::tempdir().expect("isolated album runtime");
    let _restore = RestoreRuntime {
        cwd: std::env::current_dir().expect("original cwd"),
        albums_root: std::env::var_os("STAMP_ALBUMS_ROOT"),
    };
    std::env::set_current_dir(dir.path()).expect("test cwd");
    let root = dir.path().join("Configured Albums");
    std::env::set_var("STAMP_ALBUMS_ROOT", &root);
    std::fs::write(
        dir.path().join("config.toml"),
        "print_flag_cover = true\nflag_cover_elements = ['coat', 'name']\nalbumeasy_auto_generate_pdf = false\n",
    )
    .expect("test config");
    let images = root.join("Atlantis/images");
    std::fs::create_dir_all(&images).expect("images directory");
    image::RgbaImage::from_pixel(2, 2, image::Rgba([255, 0, 0, 255]))
        .save(images.join("coat_of_arms.png"))
        .expect("local coat fixture");
    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("test database");
    conn.execute(
        "INSERT INTO stamps (country, series, year, sg_number, type, denomination,
         color, description, image_path, image_width_px, image_height_px, uniq_key)
         VALUES ('Atlantis', 'Test', 2024, '1', 'Postage', '1p', 'Red', 'Test stamp',
         'images/coat_of_arms.png', 2, 2, 'cover-root-test')",
        [],
    )
    .expect("fixture stamp");
    let output = stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        "Atlantis",
        None::<fn(usize)>,
        Some(2024),
        Some(2024),
    )
    .await;
    assert_eq!(output.parent(), Some(root.join("Atlantis").as_path()));
    let text = std::fs::read_to_string(&output).expect("generated text");
    assert!(text.contains("# Coat of arms for album"));
    assert!(text.contains("images/coat_of_arms.png"));
    assert!(output
        .parent()
        .expect("album directory")
        .join("images/coat_of_arms.png")
        .is_file());
    assert!(!dir.path().join("Albums/Atlantis").exists());
    std::fs::remove_file(images.join("coat_of_arms.png")).expect("remove optional coat fixture");
    conn.execute("UPDATE stamps SET image_path = ''", [])
        .expect("clear stamp image reference for missing-cover test");
    let output = stamp_album_gen2::album_globals::generate_album_output_conn(
        &conn,
        "Atlantis",
        None::<fn(usize)>,
        Some(2024),
        Some(2024),
    )
    .await;
    let text = std::fs::read_to_string(&output).expect("regenerated text without coat");
    assert!(!text.contains("# Coat of arms for album"));
    assert!(!text.contains("images/coat_of_arms.png"));
}

#[cfg(unix)]
#[test]
fn database_cli_reports_renderer_error_and_returns_failure() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().expect("CLI runtime");
    let launcher = dir.path().join("AlbumEasy fixture.sh");
    std::fs::write(
        &launcher,
        include_str!("fixtures/albumeasy_render_error.sh"),
    )
    .expect("test renderer");
    std::fs::set_permissions(&launcher, std::fs::Permissions::from_mode(0o755))
        .expect("renderer permissions");
    let mut cfg = stamp_album_gen2::config::Config::with_defaults();
    cfg.print_flag_cover = Some(false);
    cfg.albumeasy_auto_generate_pdf = Some(true);
    cfg.albumeasy_cli_path = Some(launcher.to_string_lossy().into_owned());
    std::fs::write(
        dir.path().join("config.toml"),
        toml::to_string(&cfg).expect("serialize configuration"),
    )
    .expect("CLI configuration");
    let db = dir.path().join("Atlantis.db");
    let conn =
        stamp_album_gen2::db_utils::init_db(db.to_str().expect("DB path")).expect("CLI database");
    conn.execute(
        "INSERT INTO stamps (country, series, year, sg_number, type, denomination,
         color, description, image_path, uniq_key)
         VALUES ('Atlantis', 'Test', 2024, '1', 'Postage', '1p', 'Red', 'Test stamp',
         '', 'cli-render-error')",
        [],
    )
    .expect("CLI stamp");
    drop(conn);
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_stamp_album_gen2"))
        .current_dir(dir.path())
        .env("STAMP_ALBUMS_ROOT", dir.path().join("Albums"))
        .args([
            "--db-path",
            db.to_str().expect("DB path"),
            "--album-name",
            "Atlantis",
            "--start-year",
            "2024",
            "--end-year",
            "2024",
        ])
        .output()
        .expect("run CLI");
    assert!(!output.status.success());
    let diagnostics = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
    assert!(diagnostics.contains("Error in line 74"), "{diagnostics}");
    assert!(
        diagnostics.contains("images/coat_of_arms.png"),
        "{diagnostics}"
    );
    assert!(!diagnostics.contains("Album 'Atlantis' generated successfully"));
    assert!(dir
        .path()
        .join("Albums/Atlantis/Atlantis-2024-2024.txt")
        .is_file());
}
