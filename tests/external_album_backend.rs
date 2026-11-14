use rusqlite::Connection;
use stamp_album_gen2::external_album::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const SOURCE: &str = r#"
ALBUM_PAGES_SIZE (217 280)
ALBUM_PAGES_MARGINS (10 10 10 10)
ALBUM_PAGES_MARGINSE (10 20 10 10)
ALBUM_PAGES_SPACING (5 5)
ALBUM_PAGES_BORDER (0.5 0.1 0 1)
ALBUM_PAGES_TITLE (CN 32 "Brazil")
ALBUM_PAGES_NUMBER (HS 8 C 1 "Page " "")
STAMP_BOXES_SIZE_ADJUST (0)
ALBUM_STAMP_IMG_SETTING (13 13 False)
ROW_ALIGN_BOTTOM
PAGE_START
PAGE_TEXT_CENTRE (HN 12 "A title\nA second line");
ROW_START_FS (CN 6 0.1 5)
STAMP_ADD_IMG (40 35 "Images/1.jpg" "left" "" "")
STAMP_ADD_IMG (40 35 "Images/1.jpg" "" "" "")
PAGE_START
PAGE_TEXT_CENTRE (HN 12 "Other page")
ROW_START_FS (CN 6 0.1 5)
STAMP_ADD_IMG (40 35 "Images/2.jpg" "" "" "")
"#;

struct Fixture {
    _directory: tempfile::TempDir,
    source: PathBuf,
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().expect("test directory");
        let source = directory.path().join("input");
        fs::create_dir_all(source.join("Images")).expect("image directory");
        fs::create_dir_all(source.join("Empty folder")).expect("empty directory");
        fs::write(source.join("album.txt"), SOURCE).expect("entry");
        fs::write(source.join("Images/1.jpg"), b"first image").expect("image");
        fs::write(source.join("Images/2.jpg"), b"second image").expect("image");
        fs::write(source.join("Read Me.bin"), b"unreferenced asset").expect("extra asset");
        fs::write(source.join("album.pdf"), b"%PDF-stale").expect("stale PDF");
        let root = directory.path().join("albums");
        Self {
            _directory: directory,
            source,
            root,
        }
    }

    #[cfg(unix)]
    fn renderer(&self, body: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        let file = self._directory.path().join("AlbumEasy renderer.sh");
        fs::write(&file, format!("#!/bin/sh\n{body}\n")).expect("renderer");
        fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).expect("permissions");
        file
    }

    #[cfg(unix)]
    fn working_renderer(&self) -> PathBuf {
        self.renderer(
            "printf '%s\\n' '%PDF-1.4 fresh test PDF' > \"${2%.txt}.pdf\"\ncat \"$2\" >> \"${2%.txt}.pdf\"",
        )
    }

    #[cfg(unix)]
    fn import(&self, cli: &Path) -> AlbumDetails {
        import_album(
            &self.root,
            "Test Album",
            &self.source,
            Path::new("album.txt"),
            cli.to_str().expect("renderer path"),
            Duration::from_secs(2),
        )
        .expect("import")
    }

    #[cfg(unix)]
    fn db(&self) -> Connection {
        Connection::open(self.root.join("test-album/test-album.db")).expect("database")
    }
}

#[test]
fn inspection_preserves_counts_legacy_units_and_source() {
    let fixture = Fixture::new();
    let inspection = inspect(&fixture.source, Path::new("album.txt")).expect("inspection");
    assert_eq!(
        (
            inspection.pages,
            inspection.placements,
            inspection.unique_images
        ),
        (2, 3, 2)
    );
    assert_eq!(inspection.commands.len(), 14);
    assert_eq!(inspection.files, 5);
    assert_eq!(inspection.warnings.len(), 1);
    assert!(inspection.errors.is_empty(), "{:?}", inspection.errors);
    assert_eq!(
        fs::read_to_string(fixture.source.join("album.txt")).expect("source"),
        SOURCE
    );
}

#[test]
fn compatibility_and_missing_images_are_explicit() {
    let fixture = Fixture::new();
    fs::write(
        fixture.source.join("album.txt"),
        format!("{SOURCE}\nPAGE_VSPACE (5)\n"),
    )
    .expect("unsupported entry");
    let inspection =
        inspect(&fixture.source, Path::new("album.txt")).expect("unsupported inspection");
    assert!(
        inspection
            .errors
            .iter()
            .any(|e| e.contains("Unsupported command PAGE_VSPACE") && e.contains("Line"))
    );
    assert!(
        import_album(
            &fixture.root,
            "Unsupported",
            &fixture.source,
            Path::new("album.txt"),
            "",
            Duration::from_secs(1)
        )
        .is_err()
    );
    assert!(!fixture.root.exists());
    fs::write(fixture.source.join("album.txt"), SOURCE).expect("restore entry");
    fs::remove_file(fixture.source.join("Images/1.jpg")).expect("remove missing image fixture");
    let inspection = inspect(&fixture.source, Path::new("album.txt")).expect("missing inspection");
    assert_eq!(inspection.errors.len(), 2);
    assert!(inspection.errors[0].contains("missing image Images/1.jpg"));
}

#[test]
fn rejects_malformed_commands_and_unsafe_cross_platform_paths() {
    let fixture = Fixture::new();
    for text in [
        SOURCE.replace("40 35", "NaN 35"),
        SOURCE.replace("\"Images/1.jpg\"", "\"../outside.jpg\""),
        SOURCE.replace("\"Images/1.jpg\"", "\"C:\\outside.jpg\""),
        SOURCE.replace("\"Images/1.jpg\"", "\"\\\\server\\image.jpg\""),
        SOURCE.replace("False)", "Maybe)"),
        SOURCE.replace("CN 6 0.1 5", "CN 6 0.1"),
    ] {
        fs::write(fixture.source.join("album.txt"), text).expect("invalid input");
        let result = inspect(&fixture.source, Path::new("album.txt"));
        assert!(result.is_err() || !result.expect("inspection diagnostics").errors.is_empty());
    }
    assert!(inspect(&fixture.source, Path::new("../outside.txt")).is_err());
}

#[test]
fn rejects_case_collisions_reserved_names_and_nested_destination_without_writes() {
    let fixture = Fixture::new();
    fs::write(fixture.source.join("Images/1.JPG"), b"case collision").expect("case fixture");
    assert!(
        inspect(&fixture.source, Path::new("album.txt"))
            .expect_err("case collision")
            .contains("case collision")
    );
    fs::remove_file(fixture.source.join("Images/1.JPG")).expect("remove case fixture");
    fs::write(fixture.source.join("CON.txt"), b"reserved").expect("reserved fixture");
    assert!(inspect(&fixture.source, Path::new("album.txt")).is_err());
    fs::remove_file(fixture.source.join("CON.txt")).expect("remove reserved fixture");
    let nested = fixture.source.join("new-albums");
    assert!(
        import_album(
            &nested,
            "Nested",
            &fixture.source,
            Path::new("album.txt"),
            "",
            Duration::from_secs(1)
        )
        .expect_err("nested destination")
        .contains("contain one another")
    );
    assert!(!nested.exists());
    assert!(
        import_album(
            &fixture.root,
            "CON",
            &fixture.source,
            Path::new("album.txt"),
            "",
            Duration::from_secs(1)
        )
        .is_err()
    );
}

#[cfg(unix)]
#[test]
fn rejects_symlinks_and_special_files() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    symlink(
        fixture.source.join("Images/1.jpg"),
        fixture.source.join("alias.jpg"),
    )
    .expect("symlink fixture");
    assert!(inspect(&fixture.source, Path::new("album.txt")).is_err());
    fs::remove_file(fixture.source.join("alias.jpg")).expect("remove alias");
    let alias = fixture._directory.path().join("source-alias");
    symlink(&fixture.source, &alias).expect("source alias");
    assert!(inspect(&alias, Path::new("album.txt")).is_ok());
}

#[cfg(unix)]
#[test]
fn import_roundtrip_full_copy_and_fidelity_approval_gate() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    assert_eq!(
        imported.pages.iter().map(|p| p.index).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(imported.status, "validated");
    assert_ne!(imported.reference_pdf, imported.generated_pdf);
    assert!(
        fixture
            .root
            .join("test-album")
            .join(&imported.reference_pdf)
            .is_file()
    );
    assert!(
        fixture
            .root
            .join("test-album")
            .join(&imported.generated_pdf)
            .is_file()
    );
    assert!(fixture.root.join("test-album/source/Empty folder").is_dir());
    assert_eq!(
        fs::read(fixture.root.join("test-album/source/Read Me.bin")).expect("full copy"),
        b"unreferenced asset"
    );
    assert_eq!(
        fs::read(fixture.source.join("album.pdf")).expect("original PDF"),
        b"%PDF-stale"
    );
    assert_eq!(
        fs::read_to_string(fixture.source.join("album.txt")).expect("original TXT"),
        SOURCE
    );
    let conn = fixture.db();
    assert!(is_imported(&conn).expect("import marker"));
    let placements: i64 = conn
        .query_row("SELECT count(*) FROM external_layout_placements", [], |r| {
            r.get(0)
        })
        .expect("placements");
    let stamps: i64 = conn
        .query_row("SELECT count(*) FROM stamps", [], |r| r.get(0))
        .expect("stamp records");
    assert_eq!((placements, stamps), (3, 3));
    assert!(
        export_layout(&conn)
            .expect_err("approval gate")
            .contains("not ready")
    );
    assert!(
        approve_album(&fixture.root, "test-album", imported.revision + 1)
            .expect_err("conflict")
            .contains("Revision conflict")
    );
    let approved = approve_album(&fixture.root, "test-album", imported.revision).expect("approve");
    assert_eq!(approved.status, "ready");
    let output = export_layout(&conn).expect("export");
    let output_inspection = inspect(
        output.parent().expect("export parent"),
        Path::new("album.txt"),
    )
    .expect("inspect export");
    assert_eq!(
        (
            output_inspection.pages,
            output_inspection.placements,
            output_inspection.unique_images
        ),
        (2, 3, 2)
    );
    let source = fs::read_to_string(output).expect("regenerated text");
    assert!(source.contains("ALBUM_STAMP_IMG_SETTING (13 13 False)"));
    assert!(source.contains("\"A title\\nA second line\""));
    assert!(!source.contains("#"));
    assert_eq!(list_albums(&fixture.root).expect("list").len(), 1);
    assert!(
        import_album(
            &fixture.root,
            "TEST ALBUM",
            &fixture.source,
            Path::new("album.txt"),
            renderer.to_str().expect("renderer"),
            Duration::from_secs(2)
        )
        .expect_err("destination collision")
        .contains("already exists")
    );
}

#[cfg(unix)]
#[test]
fn failed_draft_can_retry_but_copied_pdf_cannot_validate() {
    let fixture = Fixture::new();
    let renderer = fixture.renderer("exit 0");
    let imported = fixture.import(&renderer);
    assert_eq!(imported.status, "draft");
    assert!(imported.message.contains("missing") || imported.message.contains("Inspect path"));
    assert!(imported.reference_pdf.is_empty());
    assert!(approve_album(&fixture.root, "test-album", imported.revision).is_err());
    let working = fixture.working_renderer();
    let retried = retry_album(
        &fixture.root,
        "test-album",
        working.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("retry");
    assert_eq!(retried.status, "validated");
    assert!(retried.revision > imported.revision);
    assert!(approve_album(&fixture.root, "test-album", imported.revision).is_err());
}

#[cfg(unix)]
#[test]
fn renderer_diagnostics_nonzero_bad_pdf_and_timeout_block_validation() {
    for (body, expected) in [
        (
            "echo 'Error in line 14: invalid command' >&2; exit 0",
            "Error in line 14",
        ),
        ("echo renderer_failed >&2; exit 7", "renderer_failed"),
        ("printf 'not a PDF' > \"${2%.txt}.pdf\"", "not a PDF"),
        ("exec sleep 5", "timed out"),
    ] {
        let fixture = Fixture::new();
        let renderer = fixture.renderer(body);
        let started = Instant::now();
        let imported = import_album(
            &fixture.root,
            "Test Album",
            &fixture.source,
            Path::new("album.txt"),
            renderer.to_str().expect("renderer"),
            Duration::from_millis(100),
        )
        .expect("visible failed draft");
        assert_eq!(imported.status, "draft");
        assert!(imported.message.contains(expected), "{}", imported.message);
        assert!(started.elapsed() < Duration::from_secs(3));
    }
}

#[cfg(unix)]
#[test]
fn approval_rejects_modified_pdfs_and_preserved_assets() {
    let fixture = Fixture::new();
    let imported = fixture.import(&fixture.working_renderer());
    fs::write(
        fixture
            .root
            .join("test-album")
            .join(&imported.generated_pdf),
        b"%PDF-altered",
    )
    .expect("alter PDF");
    assert!(
        approve_album(&fixture.root, "test-album", imported.revision)
            .expect_err("changed PDF")
            .contains("changed")
    );
    let renderer = fixture.working_renderer();
    let retried = retry_album(
        &fixture.root,
        "test-album",
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("retry");
    fs::write(
        fixture.root.join("test-album/source/Images/1.jpg"),
        b"altered",
    )
    .expect("alter asset");
    assert!(
        approve_album(&fixture.root, "test-album", retried.revision)
            .expect_err("asset changed")
            .contains("changed")
    );
}

#[cfg(unix)]
#[test]
fn page_edits_add_delete_reflow_only_affected_page_and_preserve_ready_state() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let initial = fixture.import(&renderer);
    let approved = approve_album(&fixture.root, "test-album", initial.revision).expect("approve");
    let mut commands = approved.pages[0].commands.clone();
    commands[1].args[2].value = "Edited title".into();
    let added = commands.last().expect("last placement").clone();
    commands.extend([added.clone(), added.clone(), added]);
    let edited = edit_page(
        &fixture.root,
        "test-album",
        1,
        commands,
        approved.revision,
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("edit and reflow");
    assert_eq!(edited.status, "ready");
    assert_eq!(edited.pages.len(), 2);
    assert_eq!(edited.pages[1], approved.pages[1]);
    assert_eq!(
        edited.pages[0]
            .commands
            .iter()
            .filter(|c| c.name == "ROW_START_FS")
            .count(),
        2
    );
    let count: i64 = fixture
        .db()
        .query_row("SELECT count(*) FROM external_layout_placements", [], |r| {
            r.get(0)
        })
        .expect("placement count");
    assert_eq!(count, 6);
    let mut deleted = edited.pages[0].commands.clone();
    deleted.retain(|c| c.name != "STAMP_ADD_IMG");
    let after = edit_page(
        &fixture.root,
        "test-album",
        1,
        deleted,
        edited.revision,
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("delete placements");
    assert_eq!(after.pages[1], approved.pages[1]);
    let conn = fixture.db();
    let stamps: i64 = conn
        .query_row("SELECT count(*) FROM stamps", [], |r| r.get(0))
        .expect("remaining stamps");
    assert_eq!(stamps, 1);
}

#[cfg(unix)]
#[test]
fn invalid_edits_roll_back_and_render_failure_saves_retryable_draft() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    let album = fixture.root.join("test-album");
    let before = fs::read_dir(&album).expect("album directory").count();
    let mut commands = imported.pages[0].commands.clone();
    commands[3].args[0].value = "500".into();
    assert!(
        edit_page(
            &fixture.root,
            "test-album",
            1,
            commands.clone(),
            imported.revision,
            renderer.to_str().expect("renderer"),
            Duration::from_secs(2)
        )
        .expect_err("overflow")
        .contains("Overflow")
    );
    commands[3].args[0].value = "40".into();
    commands[3].args[1].value = "1000".into();
    assert!(
        edit_page(
            &fixture.root,
            "test-album",
            1,
            commands,
            imported.revision,
            renderer.to_str().expect("renderer"),
            Duration::from_secs(2)
        )
        .expect_err("vertical overflow")
        .contains("Overflow")
    );
    assert!(
        edit_page(
            &fixture.root,
            "test-album",
            1,
            imported.pages[0].commands.clone(),
            imported.revision + 1,
            renderer.to_str().expect("renderer"),
            Duration::from_secs(2)
        )
        .expect_err("conflict")
        .contains("Revision conflict")
    );
    let broken = fixture.renderer("echo 'Error: failed candidate' >&2; exit 0");
    let draft = edit_page(
        &fixture.root,
        "test-album",
        1,
        imported.pages[0].commands.clone(),
        imported.revision,
        broken.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("visible retryable draft");
    assert_eq!(draft.status, "draft");
    assert!(draft.message.contains("failed candidate"));
    let after = load_album(&fixture.root, "test-album").expect("saved draft");
    assert_eq!(after.revision, imported.revision + 1);
    assert_eq!(after.pages, imported.pages);
    assert!(after.generated_pdf.is_empty());
    assert!(after.reference_pdf.is_empty());
    assert_eq!(
        fs::read_dir(&album).expect("album directory").count(),
        before
    );
    let working = fixture.working_renderer();
    let retried = retry_album(
        &fixture.root,
        "test-album",
        working.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("retry edit");
    assert_eq!(retried.status, "validated");
    assert_eq!(
        fs::read_to_string(fixture.source.join("album.txt")).expect("original unchanged"),
        SOURCE
    );
}

#[cfg(unix)]
#[tokio::test]
async fn generator_dispatch_uses_saved_layout_and_draft_never_falls_back() {
    let fixture = Fixture::new();
    let imported = fixture.import(&fixture.working_renderer());
    let conn = fixture.db();
    let blocked = stamp_album_gen2::album_globals::generate_album_output_conn_with_pdf_override(
        &conn,
        "test-album",
        None::<fn(usize)>,
        None,
        None,
        Some(false),
    )
    .await;
    assert!(blocked.output_txt_path.as_os_str().is_empty());
    assert!(
        blocked
            .pdf_failure_reason
            .expect("draft failure")
            .contains("not ready")
    );
    approve_album(&fixture.root, "test-album", imported.revision).expect("approve");
    let output = stamp_album_gen2::album_globals::generate_album_output_conn_with_pdf_override(
        &conn,
        "test-album",
        None::<fn(usize)>,
        None,
        None,
        Some(false),
    )
    .await;
    assert!(output.pdf_failure_reason.is_none());
    let text = fs::read_to_string(output.output_txt_path).expect("saved-layout output");
    assert_eq!(text.matches("PAGE_START").count(), 2);
    assert_eq!(text.matches("STAMP_ADD_IMG").count(), 3);
    assert!(text.contains("ALBUM_STAMP_IMG_SETTING (13 13 False)"));
    let ordinary = Connection::open_in_memory().expect("ordinary database");
    assert!(!is_imported(&ordinary).expect("ordinary marker"));
}

#[test]
#[ignore = "Requires the user-provided Brazil folder; run explicitly for the local acceptance fixture"]
fn brazil_exact_acceptance_counts() {
    let source = std::env::var_os("STAMP_ALBUM_EXTERNAL_BRAZIL")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Brazil"))
        .canonicalize()
        .expect("Brazil source folder");
    let inspection = inspect(&source, Path::new("Brazil.txt")).expect("Brazil inspection");
    assert!(inspection.errors.is_empty(), "{:?}", inspection.errors);
    assert_eq!(inspection.commands.len(), 14);
    assert_eq!(
        (
            inspection.pages,
            inspection.placements,
            inspection.unique_images
        ),
        (85, 655, 649)
    );
}

#[cfg(unix)]
#[test]
fn database_write_failure_rolls_back_layout_and_render_artifacts() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    fixture
        .db()
        .execute_batch(
            "CREATE TRIGGER reject_candidate BEFORE INSERT ON external_layout_commands
         WHEN NEW.page_index=1 AND NEW.name='STAMP_ADD_IMG'
         BEGIN SELECT RAISE(ABORT,'injected layout write failure'); END;",
        )
        .expect("failure fixture");
    let album = fixture.root.join("test-album");
    let before = fs::read_dir(&album).expect("album").count();
    let failure = edit_page(
        &fixture.root,
        "test-album",
        1,
        imported.pages[0].commands.clone(),
        imported.revision,
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect_err("transaction rollback");
    assert!(
        failure.contains("injected layout write failure"),
        "{failure}"
    );
    let after = load_album(&fixture.root, "test-album").expect("unchanged database");
    assert_eq!(after.pages, imported.pages);
    assert_eq!(after.revision, imported.revision);
    assert_eq!(after.generated_pdf, imported.generated_pdf);
    assert_eq!(fs::read_dir(album).expect("album").count(), before);
    let count: i64 = fixture
        .db()
        .query_row("SELECT count(*) FROM external_layout_placements", [], |r| {
            r.get(0)
        })
        .expect("remaining placements");
    assert_eq!(count, 3);
}

#[cfg(unix)]
#[test]
fn direct_database_layout_changes_invalidate_render_approval() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    let conn = fixture.db();
    let mut args = imported.pages[0].commands[1].args.clone();
    args[2].value = "Changed directly in the editable DB".into();
    conn.execute(
        "UPDATE external_layout_commands SET args=? WHERE page_index=1 AND name='PAGE_TEXT_CENTRE'",
        [serde_json::to_string(&args).expect("args")],
    )
    .expect("direct database edit");
    assert!(
        approve_album(&fixture.root, "test-album", imported.revision)
            .expect_err("stale fidelity evidence")
            .contains("changed since validation")
    );
    let retried = retry_album(
        &fixture.root,
        "test-album",
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("revalidate edited DB");
    approve_album(&fixture.root, "test-album", retried.revision).expect("approve revalidated DB");
    assert!(export_layout(&conn).is_ok());
}

#[cfg(unix)]
#[test]
#[ignore = "Requires the user-provided Brazil folder; mock renderer is not a visual fidelity test"]
fn brazil_database_roundtrip_and_repeated_placements() {
    let fixture = Fixture::new();
    let source = std::env::var_os("STAMP_ALBUM_EXTERNAL_BRAZIL")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Brazil"))
        .canonicalize()
        .expect("Brazil source");
    let renderer = fixture.working_renderer();
    let imported = import_album(
        &fixture.root,
        "Brazil",
        &source,
        Path::new("Brazil.txt"),
        renderer.to_str().expect("renderer"),
        Duration::from_secs(3),
    )
    .expect("Brazil import");
    assert_eq!(imported.status, "validated");
    assert_eq!(imported.pages.len(), 85);
    let conn = Connection::open(fixture.root.join("brazil/brazil.db")).expect("Brazil DB");
    let counts: (i64, i64) = conn
        .query_row(
            "SELECT count(*),count(DISTINCT image_path) FROM external_layout_placements",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .expect("Brazil counts");
    assert_eq!(counts, (655, 649));
    report_brazil_db_counts(&conn, &fixture.root.join("brazil/brazil.db"));
    let no_guessed_dimensions: i64 = conn.query_row(
        "SELECT count(*) FROM stamps WHERE colnect_width IS NOT NULL OR colnect_height IS NOT NULL OR sg_number IS NOT NULL",
        [], |r| r.get(0)).expect("physical dimensions");
    assert_eq!(no_guessed_dimensions, 0);
    approve_album(&fixture.root, "brazil", imported.revision)
        .expect("mock approval for lifecycle test only");
    let exported = export_layout(&conn).expect("Brazil export");
    let inspection = inspect(
        exported.parent().expect("entry directory"),
        Path::new("Brazil.txt"),
    )
    .expect("Brazil reconstructed inspection");
    assert!(inspection.errors.is_empty(), "{:?}", inspection.errors);
    assert_eq!(
        (
            inspection.pages,
            inspection.placements,
            inspection.unique_images
        ),
        (85, 655, 649)
    );
    let loaded = load_album(&fixture.root, "brazil").expect("Brazil DB load");
    assert_eq!(loaded.pages, imported.pages);
    assert_eq!(loaded.globals, imported.globals);
}

#[cfg(unix)]
fn placement_identities(conn: &Connection) -> Vec<(i64, i64, usize)> {
    let mut statement = conn
        .prepare(
            "SELECT p.command_id,p.stamp_id,c.source_line FROM external_layout_placements p
         JOIN external_layout_commands c ON c.id=p.command_id ORDER BY c.page_index,c.ordinal",
        )
        .expect("placement identities");
    statement
        .query_map([], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                usize::try_from(r.get::<_, i64>(2)?).expect("source line"),
            ))
        })
        .expect("identity query")
        .collect::<rusqlite::Result<Vec<_>>>()
        .expect("identity rows")
}

#[cfg(unix)]
#[test]
fn globals_are_editable_at_zero_and_page_placement_identities_survive_saves() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    let before = placement_identities(&fixture.db());
    let mut globals = imported.globals.clone();
    globals
        .iter_mut()
        .find(|c| c.name == "ALBUM_PAGES_TITLE")
        .expect("title")
        .args[2]
        .value = "Edited country title".into();
    let edited = edit_page(
        &fixture.root,
        "test-album",
        0,
        globals.clone(),
        imported.revision,
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("global settings save");
    assert_eq!(edited.globals, globals);
    assert_eq!(edited.pages, imported.pages);
    assert_eq!(placement_identities(&fixture.db()), before);
    let mut commands = edited.pages[0].commands.clone();
    commands[3].args[4].value = "Edited stamp caption".into();
    commands.swap(3, 4);
    let mut added = commands[3].clone();
    added.source_line = 0;
    commands.push(added);
    let page = edit_page(
        &fixture.root,
        "test-album",
        1,
        commands,
        edited.revision,
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("caption, reorder and add");
    assert_eq!(page.pages[1], imported.pages[1]);
    let after = placement_identities(&fixture.db());
    for identity in &before {
        assert!(
            after.contains(identity),
            "Existing placement identity lost: {identity:?}"
        );
    }
    assert_eq!(after.len(), before.len() + 1);
    assert_eq!(after[0], before[1]);
    assert_eq!(after[1], before[0]);
    let again = edit_page(
        &fixture.root,
        "test-album",
        1,
        page.pages[0].commands.clone(),
        page.revision,
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("save same page");
    assert_eq!(placement_identities(&fixture.db()), after);
    let mut commands = again.pages[0].commands.clone();
    commands.retain(|c| c.source_line != before[0].2);
    edit_page(
        &fixture.root,
        "test-album",
        1,
        commands,
        again.revision,
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("delete one original placement");
    let remaining = placement_identities(&fixture.db());
    assert!(!remaining.contains(&before[0]));
    for identity in after.into_iter().filter(|identity| identity != &before[0]) {
        assert!(remaining.contains(&identity));
    }
}

#[cfg(unix)]
#[test]
fn failed_global_render_persists_draft_and_bad_global_geometry_rolls_back() {
    let fixture = Fixture::new();
    let imported = fixture.import(&fixture.working_renderer());
    let before = placement_identities(&fixture.db());
    let mut globals = imported.globals.clone();
    globals
        .iter_mut()
        .find(|c| c.name == "ALBUM_PAGES_SIZE")
        .expect("size")
        .args[0]
        .value = "5".into();
    assert!(
        edit_page(
            &fixture.root,
            "test-album",
            0,
            globals,
            imported.revision,
            "",
            Duration::from_secs(1)
        )
        .expect_err("invalid geometry")
        .contains("Overflow")
    );
    assert_eq!(
        load_album(&fixture.root, "test-album")
            .expect("unchanged")
            .revision,
        imported.revision
    );
    let mut globals = imported.globals.clone();
    globals
        .iter_mut()
        .find(|c| c.name == "ALBUM_PAGES_TITLE")
        .expect("title")
        .args[2]
        .value = "Draft global change".into();
    let draft = edit_page(
        &fixture.root,
        "test-album",
        0,
        globals.clone(),
        imported.revision,
        "",
        Duration::from_secs(1),
    )
    .expect("persist unavailable-renderer draft");
    assert_eq!(draft.status, "draft");
    assert_eq!(draft.globals, globals);
    assert!(draft.message.contains("executable is required"));
    assert!(draft.reference_pdf.is_empty() && draft.generated_pdf.is_empty());
    assert_eq!(placement_identities(&fixture.db()), before);
    assert!(approve_album(&fixture.root, "test-album", draft.revision).is_err());
}

#[test]
#[ignore = "Opt-in real AlbumEasy acceptance: requires ALBUMEASY_TEST_CLI and the local Brazil source"]
fn brazil_real_albumeasy_import_approval_and_page_edit() {
    let cli = std::env::var("ALBUMEASY_TEST_CLI")
        .expect("Set ALBUMEASY_TEST_CLI to the real AlbumEasy executable or launcher");
    let source = std::env::var_os("STAMP_ALBUM_EXTERNAL_BRAZIL")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Brazil"));
    let original = inspect(&source, Path::new("Brazil.txt")).expect("Original read-only inventory");
    let original_text = fs::read(source.join("Brazil.txt")).expect("Original TXT");
    let original_hashes = source_checksums(&source);
    let directory = tempfile::tempdir().expect("Real-render acceptance workspace");
    let root = directory.path().join("albums");
    let imported = import_album(
        &root,
        "Brazil",
        &source,
        Path::new("Brazil.txt"),
        &cli,
        Duration::from_secs(120),
    )
    .expect("Real Brazil copied import");
    assert_eq!(imported.status, "validated", "{}", imported.message);
    assert_eq!(imported.pages.len(), 85);
    let album = root.join("brazil");
    let reference = album.join(&imported.reference_pdf);
    let reconstructed = album.join(&imported.generated_pdf);
    assert_ne!(reference, reconstructed);
    assert!(reference.starts_with(&album) && reconstructed.starts_with(&album));
    let original_generated_pdf = fs::read(&reconstructed).expect("Original reconstructed PDF");
    eprintln!(
        "Real PDFs: reference={} reconstructed={}",
        reference.display(),
        reconstructed.display()
    );
    // Optional local Poppler verification; no downloaded tooling or source fixtures.
    if let Ok(tool) = which::which("pdftoppm") {
        let raster = directory.path().join("raster");
        fs::create_dir(&raster).expect("Raster comparison directory");
        for (pdf, prefix) in [(&reference, "reference"), (&reconstructed, "reconstructed")] {
            rasterize_pdf(&tool, pdf, &raster.join(prefix));
        }
        let mut compared = 0;
        for entry in fs::read_dir(&raster).expect("Raster pages") {
            let entry = entry.expect("Raster page");
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Some(suffix) = name.strip_prefix("reference-") {
                let other = raster.join(format!("reconstructed-{suffix}"));
                assert_eq!(
                    fs::read(entry.path()).expect("Reference raster"),
                    fs::read(other).expect("Reconstructed raster"),
                    "Visual mismatch on page {suffix}"
                );
                compared += 1;
            }
        }
        assert_eq!(compared, 85, "Both real PDFs must contain exactly 85 pages");
        eprintln!(
            "Real reference/reconstructed raster comparison: {compared} pages identical at 72 DPI full color"
        );
    } else {
        eprintln!("Poppler pdftoppm unavailable: raster comparison not performed");
    }
    // Exercise the approval transition only; this is not the user's visual-fidelity approval.
    let approved =
        approve_album(&root, "brazil", imported.revision).expect("Test-only approval API gate");
    assert_eq!(approved.status, "ready");
    let mut commands = approved.pages[0].commands.clone();
    let text = commands
        .iter_mut()
        .find(|c| c.name == "PAGE_TEXT_CENTRE")
        .expect("First page text");
    text.args[2].value.push_str(" (edited)");
    fs::create_dir(album.join("imported-assets")).expect("Uploaded image directory");
    fs::copy(
        source.join("Images/1.jpg"),
        album.join("imported-assets/real-upload.jpg"),
    )
    .expect("Copy image into test upload directory");
    commands
        .iter_mut()
        .find(|c| c.name == "STAMP_ADD_IMG")
        .expect("First stamp")
        .args[2]
        .value = "imported-assets/real-upload.jpg".into();
    let edited = edit_page(
        &root,
        "brazil",
        1,
        commands,
        approved.revision,
        &cli,
        Duration::from_secs(120),
    )
    .expect("Real page edit");
    assert_eq!(edited.status, "ready", "{}", edited.message);
    assert_eq!(edited.reference_pdf, approved.reference_pdf);
    assert_eq!(&edited.pages[1..], &approved.pages[1..]);
    assert_eq!(edited.pages.len(), 85);
    assert_ne!(
        fs::read(album.join(&edited.generated_pdf)).expect("Edited reconstructed PDF"),
        original_generated_pdf,
        "Editing page text must regenerate the PDF contents even when its path is reused"
    );
    if let Ok(tool) = which::which("pdftoppm") {
        let raster = directory.path().join("raster");
        rasterize_pdf(
            &tool,
            &album.join(&edited.generated_pdf),
            &raster.join("edited"),
        );
        let mut unchanged = 0;
        let mut changed = 0;
        for entry in fs::read_dir(&raster).expect("Edited raster pages") {
            let entry = entry.expect("Edited raster page");
            let name = entry.file_name().to_string_lossy().into_owned();
            if let Some(suffix) = name.strip_prefix("reconstructed-") {
                let page = suffix
                    .split('.')
                    .next()
                    .expect("Raster page index")
                    .parse::<usize>()
                    .expect("Numeric raster page index");
                let original = fs::read(entry.path()).expect("Original reconstructed raster");
                let candidate =
                    fs::read(raster.join(format!("edited-{suffix}"))).expect("Edited raster");
                if page == 1 {
                    assert_ne!(
                        original, candidate,
                        "Editing page text must change page one"
                    );
                    changed += 1;
                } else {
                    assert_eq!(
                        original, candidate,
                        "Editing page one changed unrelated page {page}"
                    );
                    unchanged += 1;
                }
            }
        }
        assert_eq!((changed, unchanged), (1, 84));
        eprintln!(
            "Edited PDF comparison: page 1 changed; all 84 other pages identical at 72 DPI full color"
        );
    }
    let conn = Connection::open(album.join("brazil.db")).expect("Real acceptance DB");
    let counts: (i64, i64) = conn
        .query_row(
            "SELECT count(*),count(DISTINCT image_path) FROM external_layout_placements",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .expect("Real acceptance placement counts");
    assert_eq!(counts.0, 655);
    assert!((649..=650).contains(&counts.1));
    report_brazil_db_counts(&conn, &album.join("brazil.db"));
    assert_eq!(
        fs::read(source.join("Brazil.txt")).expect("Unchanged original TXT"),
        original_text
    );
    let after =
        inspect(&source, Path::new("Brazil.txt")).expect("Original inventory after rendering");
    assert_eq!((after.files, after.bytes), (original.files, original.bytes));
    assert_eq!(
        source_checksums(&source),
        original_hashes,
        "No original source files may change"
    );
    eprintln!(
        "Real import/approval-transition/edit complete; originals untouched; user fidelity approval remains required"
    );
}

fn rasterize_pdf(tool: &Path, pdf: &Path, prefix: &Path) {
    let mut child = std::process::Command::new(tool)
        .args(["-r", "72"])
        .arg(pdf)
        .arg(prefix)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Rasterize real PDF");
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("Poll PDF rasterizer") {
            assert!(status.success(), "PDF rasterizer failed: {status}");
            break;
        }

        if started.elapsed() >= Duration::from_secs(120) {
            child.kill().expect("Terminate timed-out PDF rasterizer");
            child.wait().expect("Reap PDF rasterizer");
            panic!("PDF rasterizer exceeded the 120-second deadline");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[cfg(unix)]
#[test]
fn approved_page_and_global_edits_remain_ready_without_reapproving_initial_fidelity() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    let approved =
        approve_album(&fixture.root, "test-album", imported.revision).expect("Initial approval");
    let reference = approved.reference_pdf.clone();
    let reference_bytes =
        fs::read(fixture.root.join("test-album").join(&reference)).expect("Approved reference");
    let approved_generated_bytes = fs::read(
        fixture
            .root
            .join("test-album")
            .join(&approved.generated_pdf),
    )
    .expect("Approved generated PDF");
    let identities = placement_identities(&fixture.db());
    let mut commands = approved.pages[0].commands.clone();
    commands[3].args[4].value = "Intentional caption edit".into();
    let edited = edit_page(
        &fixture.root,
        "test-album",
        1,
        commands,
        approved.revision,
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Ready page edit");
    assert_eq!(edited.status, "ready");
    assert!(edited.message.is_empty());
    assert_eq!(edited.reference_pdf, reference);
    assert_eq!(edited.generated_pdf, approved.generated_pdf);
    let page_edit_pdf = fs::read(fixture.root.join("test-album").join(&edited.generated_pdf))
        .expect("Generated PDF after page edit");
    assert_ne!(page_edit_pdf, approved_generated_bytes);
    assert_eq!(edited.pages[1], approved.pages[1]);
    assert_eq!(placement_identities(&fixture.db()), identities);
    assert!(
        export_layout(&fixture.db()).is_ok(),
        "Editing must not disable ready-album generation"
    );
    let mut globals = edited.globals.clone();
    globals
        .iter_mut()
        .find(|c| c.name == "ALBUM_PAGES_TITLE")
        .expect("Title")
        .args[2]
        .value = "Intentional title edit".into();
    let global = edit_page(
        &fixture.root,
        "test-album",
        0,
        globals,
        edited.revision,
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Ready global edit");
    assert_eq!(global.status, "ready");
    assert!(global.message.is_empty());
    assert_eq!(global.reference_pdf, reference);
    assert_eq!(global.generated_pdf, edited.generated_pdf);
    assert_ne!(
        fs::read(fixture.root.join("test-album").join(&global.generated_pdf))
            .expect("Generated PDF after global edit"),
        page_edit_pdf
    );
    assert_eq!(
        fs::read(fixture.root.join("test-album").join(&reference))
            .expect("Unchanged approved reference"),
        reference_bytes
    );
    assert_eq!(placement_identities(&fixture.db()), identities);
    assert!(export_layout(&fixture.db()).is_ok());
    let broken = fixture.renderer("echo 'Error: intentional edit render failed' >&2; exit 0");
    let draft = edit_page(
        &fixture.root,
        "test-album",
        1,
        global.pages[0].commands.clone(),
        global.revision,
        broken.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Failed edit saved as draft");
    assert_eq!(draft.status, "draft");
    assert!(draft.message.contains("intentional edit render failed"));
    assert!(export_layout(&fixture.db()).is_err());
}

#[cfg(unix)]
#[test]
fn legacy_zero_based_database_exposes_one_based_pages_and_migrates_without_identity_changes() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    let imported = approve_album(&fixture.root, "test-album", imported.revision)
        .expect("Approve fixture before legacy migration");
    let exported = fs::read(export_layout(&fixture.db()).expect("Approved export"))
        .expect("Approved source bytes");
    let identities = placement_identities(&fixture.db());
    fixture
        .db()
        .execute_batch(
            "UPDATE external_layout_commands SET page_index=-page_index;
         UPDATE external_layout_commands SET page_index=-page_index-1;",
        )
        .expect("Simulate old zero-based layout DB");
    assert_eq!(
        list_albums(&fixture.root).expect("Read-only legacy listing")[0].revision,
        imported.revision
    );
    let public = load_album(&fixture.root, "test-album").expect("Migrate legacy numbering");
    assert_eq!(
        public.pages.iter().map(|p| p.index).collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(public.pages, imported.pages);
    assert_eq!(public.globals, imported.globals);
    assert_eq!(public.status, imported.status);
    assert_eq!(public.revision, imported.revision);
    assert_eq!(public.reference_pdf, imported.reference_pdf);
    assert_eq!(public.generated_pdf, imported.generated_pdf);
    assert_eq!(
        fs::read(export_layout(&fixture.db()).expect("Migrated export"))
            .expect("Migrated source bytes"),
        exported
    );
    assert_eq!(placement_identities(&fixture.db()), identities);
    let numbering: (i64, i64) = fixture
        .db()
        .query_row(
            "SELECT min(page_index),max(page_index) FROM external_layout_commands",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .expect("Persisted numbering");
    assert_eq!(numbering, (1, 2));
    let mut globals = public.globals.clone();
    globals
        .iter_mut()
        .find(|c| c.name == "ALBUM_PAGES_TITLE")
        .expect("Title")
        .args[2]
        .value = "Global edit".into();
    let global = edit_page(
        &fixture.root,
        "test-album",
        0,
        globals.clone(),
        public.revision,
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Globals at page zero");
    assert_eq!(global.globals, globals);
    assert_eq!(global.pages, imported.pages);
    assert_eq!(global.status, "ready");
    assert_eq!(global.reference_pdf, imported.reference_pdf);
    let mut first = global.pages[0].commands.clone();
    first[1].args[2].value = "Actual first-page edit".into();
    let edited = edit_page(
        &fixture.root,
        "test-album",
        1,
        first.clone(),
        global.revision,
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("First page at one");
    assert_eq!(edited.pages[0].commands, first);
    assert_eq!(edited.pages[1], imported.pages[1]);
    assert_eq!(edited.globals, globals);
    assert_eq!(edited.status, "ready");
    assert_eq!(edited.reference_pdf, imported.reference_pdf);
    assert_eq!(placement_identities(&fixture.db()), identities);
}

fn report_brazil_db_counts(conn: &Connection, path: &Path) {
    let counts: (i64, i64, i64, i64, i64, i64) = conn
        .query_row(
            "SELECT
          (SELECT count(*) FROM stamps),
          (SELECT count(*) FROM external_layout_placements),
          (SELECT count(*) FROM external_layout_placements p
           JOIN stamps s ON s.id=p.stamp_id
           JOIN external_layout_commands c ON c.id=p.command_id
           WHERE c.name='STAMP_ADD_IMG'),
          (SELECT count(DISTINCT stamp_id) FROM external_layout_placements),
          (SELECT count(*) FROM external_layout_placements p
           LEFT JOIN stamps s ON s.id=p.stamp_id
           LEFT JOIN external_layout_commands c ON c.id=p.command_id
           WHERE s.id IS NULL OR c.id IS NULL),
          (SELECT count(*) FROM stamps s
           WHERE NOT EXISTS(SELECT 1 FROM external_layout_placements p WHERE p.stamp_id=s.id))",
            [],
            |r| {
                Ok((
                    r.get(0)?,
                    r.get(1)?,
                    r.get(2)?,
                    r.get(3)?,
                    r.get(4)?,
                    r.get(5)?,
                ))
            },
        )
        .expect("Per-database placement integrity counts");
    assert_eq!(counts, (655, 655, 655, 655, 0, 0));
    eprintln!(
        "DB {}: stamps={}, placements={}, valid command/stamp links={}, distinct linked stamp IDs={}, orphan links={}, unlinked stamps={}; duplicate placements retained, skipped=0",
        path.display(),
        counts.0,
        counts.1,
        counts.2,
        counts.3,
        counts.4,
        counts.5
    );
}

fn source_checksums(source: &Path) -> std::collections::BTreeMap<PathBuf, String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let mut pending = vec![source.to_owned()];
    let mut files = std::collections::BTreeMap::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory).expect("Read source checksum inventory") {
            let entry = entry.expect("Checksum inventory entry");
            if entry.file_type().expect("Checksum entry type").is_dir() {
                pending.push(entry.path());
            } else {
                let mut file =
                    fs::File::open(entry.path()).expect("Read original file for checksum");
                let mut hash = Sha256::new();
                let mut buffer = [0u8; 64 * 1024];
                loop {
                    let count = file.read(&mut buffer).expect("Read checksum block");
                    if count == 0 {
                        break;
                    }
                    hash.update(&buffer[..count]);
                }
                files.insert(
                    entry
                        .path()
                        .strip_prefix(source)
                        .expect("Relative checksum path")
                        .to_owned(),
                    hash.finalize().iter().map(|b| format!("{b:02x}")).collect(),
                );
            }
        }
    }
    files
}

#[cfg(unix)]
#[test]
fn uploaded_images_are_rendered_exported_and_checksum_gated() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    let album = fixture.root.join("test-album");
    let assets = asset_directory(&fixture.root, "test-album").expect("Checked API upload folder");
    assert_eq!(assets, album.join("imported-assets"));
    assert_eq!(
        asset_directory(&fixture.root, "test-album").expect("Existing upload folder"),
        assets
    );
    assert!(asset_directory(&fixture.root, "../test-album").is_err());
    fs::write(assets.join("upload.jpg"), b"uploaded image").expect("Upload");
    let identities = placement_identities(&fixture.db());
    let mut commands = imported.pages[0].commands.clone();
    commands[3].args[2].value = "imported-assets/upload.jpg".into();
    let edited = edit_page(
        &fixture.root,
        "test-album",
        1,
        commands,
        imported.revision,
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("Save uploaded image");
    assert_eq!(edited.status, "validated", "{}", edited.message);
    assert_eq!(placement_identities(&fixture.db()), identities);
    let generated_base = album
        .join(&edited.generated_pdf)
        .parent()
        .expect("Generated directory")
        .to_owned();
    assert_eq!(
        fs::read(generated_base.join("imported-assets/upload.jpg")).expect("Copied render upload"),
        b"uploaded image"
    );
    let reference_base = album
        .join(&edited.reference_pdf)
        .parent()
        .expect("Reference directory")
        .to_owned();
    assert!(
        !reference_base.join("imported-assets").exists(),
        "Original source render must not be rewritten with uploads"
    );
    assert!(!fixture.source.join("imported-assets").exists());
    fs::write(album.join("imported-assets/upload.jpg"), b"changed upload").expect("Mutate upload");
    assert!(
        approve_album(&fixture.root, "test-album", edited.revision)
            .expect_err("Stale upload approval")
            .contains("changed since validation")
    );
    let retried = retry_album(
        &fixture.root,
        "test-album",
        renderer.to_str().expect("renderer"),
        Duration::from_secs(2),
    )
    .expect("Retry upload");
    approve_album(&fixture.root, "test-album", retried.revision).expect("Approve new upload");
    let output = export_layout(&fixture.db()).expect("Export uploaded layout");
    assert_eq!(
        fs::read(
            output
                .parent()
                .expect("Export directory")
                .join("imported-assets/upload.jpg")
        )
        .expect("Exported upload"),
        b"changed upload"
    );
    let mut missing = retried.pages[0].commands.clone();
    missing[3].args[2].value = "imported-assets/missing.jpg".into();
    let ready = load_album(&fixture.root, "test-album").expect("Ready album");
    assert!(
        edit_page(
            &fixture.root,
            "test-album",
            1,
            missing,
            ready.revision,
            renderer.to_str().expect("renderer"),
            Duration::from_secs(2)
        )
        .expect_err("Missing upload")
        .contains("missing image")
    );
    assert_eq!(
        load_album(&fixture.root, "test-album")
            .expect("Unchanged")
            .revision,
        ready.revision
    );
    use std::os::unix::fs::symlink;
    fs::remove_file(album.join("imported-assets/upload.jpg")).expect("Replace upload with symlink");
    symlink(
        fixture.source.join("Images/1.jpg"),
        album.join("imported-assets/upload.jpg"),
    )
    .expect("Upload symlink fixture");
    assert!(
        export_layout(&fixture.db())
            .expect_err("Reject uploaded symlink")
            .contains("Symlinks")
    );
    fs::remove_file(assets.join("upload.jpg")).expect("Remove fixture link");
    fs::remove_dir(&assets).expect("Remove empty upload folder");
    symlink(fixture.source.join("Images"), &assets).expect("Upload directory symlink");
    assert!(
        asset_directory(&fixture.root, "test-album")
            .expect_err("Reject upload directory link")
            .contains("Symlinks")
    );
}

#[cfg(unix)]
#[test]
fn uploads_work_for_nested_entries_and_old_metadata_migrates_additively() {
    let fixture = Fixture::new();
    fs::create_dir(fixture.source.join("nested")).expect("Nested source entry directory");
    fs::rename(
        fixture.source.join("album.txt"),
        fixture.source.join("nested/album.txt"),
    )
    .expect("Move fixture entry");
    fs::rename(
        fixture.source.join("Images"),
        fixture.source.join("nested/Images"),
    )
    .expect("Move fixture images");
    let renderer = fixture.working_renderer();
    let imported = import_album(
        &fixture.root,
        "Test Album",
        &fixture.source,
        Path::new("nested/album.txt"),
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Nested import");
    let conn = fixture.db();
    conn.execute_batch("ALTER TABLE external_album DROP COLUMN uploaded_hash;")
        .expect("Simulate earlier metadata schema");
    let old_revision = imported.revision;
    let loaded = load_album(&fixture.root, "test-album").expect("Additive metadata migration");
    assert_eq!(loaded.revision, old_revision);
    assert_eq!(loaded.pages, imported.pages);
    let column: bool = conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM pragma_table_info('external_album') WHERE name='uploaded_hash')",
        [], |r| r.get(0)).expect("Check added metadata column");
    assert!(column);
    let album = fixture.root.join("test-album");
    let assets =
        asset_directory(&fixture.root, "test-album").expect("Nested entry upload directory");
    assert_eq!(assets, album.join("imported-assets"));
    fs::write(assets.join("nested.jpg"), b"nested upload").expect("Uploaded asset");
    let mut commands = loaded.pages[0].commands.clone();
    commands[3].args[2].value = "imported-assets/nested.jpg".into();
    let edited = edit_page(
        &fixture.root,
        "test-album",
        1,
        commands,
        loaded.revision,
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Nested upload edit");
    assert_eq!(edited.status, "validated", "{}", edited.message);
    approve_album(&fixture.root, "test-album", edited.revision).expect("Approve nested upload");
    let output = export_layout(&conn).expect("Export nested entry");
    assert_eq!(
        fs::read(
            output
                .parent()
                .expect("Nested export parent")
                .join("imported-assets/nested.jpg")
        )
        .expect("Uploaded asset beside nested entry"),
        b"nested upload"
    );
}

#[cfg(unix)]
#[test]
fn repeated_images_remain_independent_placements_when_one_is_edited_or_deleted() {
    let fixture = Fixture::new();
    let renderer = fixture.working_renderer();
    let imported = fixture.import(&renderer);
    let original_ids = placement_identities(&fixture.db());
    assert_ne!(original_ids[0].0, original_ids[1].0);
    assert_ne!(original_ids[0].1, original_ids[1].1);
    let mut commands = imported.pages[0].commands.clone();
    commands[3].args[4].value = "Only the first repeated image changes".into();
    let edited = edit_page(
        &fixture.root,
        "test-album",
        1,
        commands,
        imported.revision,
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Edit first repeated placement");
    assert_eq!(placement_identities(&fixture.db()), original_ids);
    assert_eq!(edited.pages[0].commands[4], imported.pages[0].commands[4]);
    let mut commands = edited.pages[0].commands.clone();
    commands.remove(3);
    let deleted = edit_page(
        &fixture.root,
        "test-album",
        1,
        commands,
        edited.revision,
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Delete first repeated placement");
    assert_eq!(placement_identities(&fixture.db()), original_ids[1..]);
    assert_eq!(deleted.pages[0].commands[3], imported.pages[0].commands[4]);
    let inspection =
        inspect(&fixture.source, Path::new("album.txt")).expect("Original input untouched");
    assert_eq!((inspection.placements, inspection.unique_images), (3, 2));
}

#[cfg(unix)]
#[test]
fn inspection_does_not_read_or_decode_image_contents() {
    use std::os::unix::fs::PermissionsExt;
    let fixture = Fixture::new();
    let image = fixture.source.join("Images/1.jpg");
    fs::set_permissions(&image, fs::Permissions::from_mode(0o000)).expect("Make image unreadable");
    let inspection =
        inspect(&fixture.source, Path::new("album.txt")).expect("Metadata-only image inspection");
    fs::set_permissions(&image, fs::Permissions::from_mode(0o600))
        .expect("Restore fixture permissions");
    assert!(inspection.errors.is_empty());
    assert_eq!((inspection.placements, inspection.unique_images), (3, 2));
}

#[cfg(unix)]
#[test]
fn selected_source_and_root_allow_symlinked_ancestors_but_internal_links_are_rejected() {
    use std::os::unix::fs::symlink;
    let fixture = Fixture::new();
    let alias = fixture._directory.path().join("selected-parent");
    symlink(fixture._directory.path(), &alias).expect("Alias ancestor");
    let source = alias.join("input");
    let root = alias.join("new/albums");
    let inspection = inspect(&source, &source.join("album.txt"))
        .expect("Inspect aliased source and absolute entry");
    assert!(inspection.errors.is_empty());
    let renderer = fixture.working_renderer();
    let imported = import_album(
        &root,
        "Alias Album",
        &source,
        &source.join("album.txt"),
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Import via ancestor aliases");
    assert_eq!(imported.status, "validated", "{}", imported.message);
    assert_eq!(list_albums(&root).expect("List aliased root").len(), 1);
    let approved =
        approve_album(&root, "alias-album", imported.revision).expect("Approve alias import");
    let db =
        Connection::open(root.join("alias-album/alias-album.db")).expect("Open DB through alias");
    let exported = export_layout(&db).expect("Export via aliased DB path");
    assert!(exported.is_file());
    let edited = edit_page(
        &root,
        "alias-album",
        1,
        approved.pages[0].commands.clone(),
        approved.revision,
        renderer.to_str().expect("Renderer"),
        Duration::from_secs(2),
    )
    .expect("Edit via aliased root");
    assert_eq!(edited.status, "ready");
    let copied = root.join("alias-album/source/Images/1.jpg");
    fs::remove_file(&copied).expect("Replace copied image with internal symlink");
    symlink(fixture.source.join("Images/1.jpg"), &copied).expect("Internal dependency symlink");
    assert!(
        export_layout(&db)
            .expect_err("Internal link rejection")
            .contains("Symlinks")
    );
    let album_alias = root.join("linked-album");
    symlink(root.join("alias-album"), &album_alias).expect("Album entry symlink");
    assert!(
        load_album(&root, "linked-album")
            .expect_err("Album entry rejection")
            .contains("Symlinks")
    );
    symlink(
        fixture.source.join("Images"),
        fixture.source.join("LinkedImages"),
    )
    .expect("Source internal directory symlink");
    assert!(
        inspect(&source, Path::new("album.txt"))
            .expect_err("Source entry rejection")
            .contains("Symlinks")
    );
}
