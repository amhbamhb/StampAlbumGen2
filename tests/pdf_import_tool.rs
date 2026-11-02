use stamp_album_gen2::review_ui::db_query::process_pdf_records;
use stamp_album_gen2::review_ui::pdf_parser::{import_stampworld_file, types::PdfStampRecord};
use std::path::{Path, PathBuf};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("download-pdf")
        .join(name)
}

fn sample_record(number: &str) -> PdfStampRecord {
    PdfStampRecord {
        catalog_number: number.to_string(),
        stamp_type: "MC".to_string(),
        description: "Sir James Clark Ross".to_string(),
        color: "multicoloured".to_string(),
        denomination: "3P".to_string(),
        year: "1972".to_string(),
        series: "1972 British Polar Explorers".to_string(),
        designer: "Marjorie Saynor".to_string(),
        engraver: "Harrison & Sons Ltd".to_string(),
        perforation: "14¼ x 14¾".to_string(),
        watermark: "None".to_string(),
        currency: "USD".to_string(),
        stamps_issued: "53,630,000".to_string(),
        mint_price: "0.28".to_string(),
        used_price: "0.28".to_string(),
        ..Default::default()
    }
}

#[test]
fn test_manual_import_single_record() {
    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("initialize schema");
    let summary = process_pdf_records(&conn, vec![sample_record("571")], "Great Britain")
        .expect("import record");
    assert_eq!((summary.imported, summary.skipped), (1, 0));
    let metadata: (String, String, String) = conn
        .query_row(
            "SELECT design, engraving, perforation FROM stamps WHERE sg_number='571'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
        )
        .expect("persisted metadata");
    assert_eq!(
        metadata,
        (
            "Marjorie Saynor".into(),
            "Harrison & Sons Ltd".into(),
            "14¼ x 14¾".into()
        )
    );
}

#[test]
fn imports_real_pdfs_into_disk_database_and_skips_reimports() {
    let temp = tempfile::tempdir().expect("isolated output");
    let db = temp.path().join("album.db");
    let conn =
        stamp_album_gen2::db_utils::init_db(&db.to_string_lossy()).expect("initialize schema");
    for (country, name, count) in [
        (
            "Great Britain",
            "Great Britain - Postage stamps (1970 - 1979).pdf",
            22,
        ),
        (
            "United States",
            "United States - Postage stamps (1970 - 1979).pdf",
            27,
        ),
    ] {
        let path = fixture(name);
        let first =
            import_stampworld_file(&conn, &path, country, "1840", "9999").expect("first import");
        assert_eq!(
            (first.total_found, first.imported, first.skipped),
            (count, count, 0)
        );
        let second =
            import_stampworld_file(&conn, &path, country, "1840", "9999").expect("repeat import");
        assert_eq!(
            (second.total_found, second.imported, second.skipped),
            (count, 0, count)
        );
        println!("{country}: DB={}, schema=compatible, imported={count}, repeat_duplicates={count}, failed=0", db.display());
    }
    drop(conn);
    let reopened = rusqlite::Connection::open(db).expect("reopen DB");
    assert_eq!(
        reopened
            .query_row("SELECT COUNT(*) FROM stamps", [], |r| r.get::<_, i64>(0))
            .expect("persisted count"),
        49
    );
}

#[test]
#[ignore = "requires the six local, untracked Denmark PDFs in download-pdf"]
fn imports_denmark_pages_and_skips_all_266_on_second_pass() {
    let temp = tempfile::tempdir().expect("isolated output");
    let db = temp.path().join("Denmark.db");
    let conn =
        stamp_album_gen2::db_utils::init_db(&db.to_string_lossy()).expect("initialize schema");
    for pass in 0..2 {
        let mut imported = 0;
        let mut skipped = 0;
        for page in 1..=6 {
            let path = fixture(&format!(
                "Denmark - Postage stamps (1980 - 1989) - Page {page}.pdf"
            ));
            if pass == 1 {
                let records =
                    stamp_album_gen2::review_ui::pdf_parser::extraction::parse_stampworld_pdf(
                        &path, "DK", "1980", "1989",
                    )
                    .expect("repeat parse");
                for record in records {
                    let stored: String = conn
                        .query_row(
                            "SELECT series FROM stamps WHERE country='Denmark' AND sg_number=?1",
                            [&record.catalog_number],
                            |r| r.get(0),
                        )
                        .expect("stored stamp");
                    assert_eq!(
                        stored, record.series,
                        "repeat parse stamp {}",
                        record.catalog_number
                    );
                }
            }
            let summary = import_stampworld_file(&conn, &path, "Denmark", "1980", "1989")
                .expect("Denmark page import");
            imported += summary.imported;
            skipped += summary.skipped;
        }
        assert_eq!(
            (imported, skipped),
            if pass == 0 { (266, 0) } else { (0, 266) }
        );
        println!("Denmark: pass={pass}, DB={}, schema=compatible, imported={imported}, skipped_duplicates={skipped}, failed=0", db.display());
    }
}

#[test]
fn duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata() {
    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("initialize schema");
    process_pdf_records(&conn, vec![sample_record("571")], "Great Britain").expect("first country");
    process_pdf_records(&conn, vec![sample_record("571")], "Other Country")
        .expect("second country");
    let mut changed = sample_record("571");
    changed.description = "Do not overwrite".to_string();
    let summary = process_pdf_records(&conn, vec![changed], "Great Britain").expect("duplicate");
    assert_eq!((summary.imported, summary.skipped), (0, 1));
    assert_eq!(
        conn.query_row(
            "SELECT COUNT(*) FROM stamps WHERE description='Sir James Clark Ross'",
            [],
            |r| r.get::<_, i64>(0)
        )
        .expect("original rows"),
        2
    );
}

#[test]
fn insert_errors_roll_back_the_entire_file_and_are_not_counted_as_duplicates() {
    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("initialize schema");
    conn.execute_batch(
        "CREATE TRIGGER reject_second BEFORE INSERT ON stamps
        WHEN NEW.sg_number = '572' BEGIN SELECT RAISE(ABORT, 'fixture insert rejected'); END;",
    )
    .expect("failure trigger");
    let error = process_pdf_records(
        &conn,
        vec![sample_record("571"), sample_record("572")],
        "Great Britain",
    )
    .expect_err("must fail, not report skipped");
    assert!(error.contains("fixture insert rejected"), "{error}");
    assert_eq!(
        conn.query_row("SELECT COUNT(*) FROM stamps", [], |r| r.get::<_, i64>(0))
            .expect("rollback count"),
        0
    );
}

#[test]
fn filters_years_imports_html_and_reports_invalid_inputs() {
    let conn = stamp_album_gen2::db_utils::init_db(":memory:").expect("initialize schema");
    let path = fixture("Great Britain - Postage stamps (1970 - 1979).pdf");
    let summary =
        import_stampworld_file(&conn, &path, "Great Britain", "1973", "1974").expect("filter");
    assert_eq!(
        (summary.total_found, summary.imported, summary.skipped),
        (22, 0, 22)
    );
    let temp = tempfile::tempdir().expect("isolated HTML");
    let html = temp.path().join("stampworld.html");
    std::fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("Example-Stampworldwebpage.txt"),
        &html,
    )
    .expect("HTML fixture");
    let summary =
        import_stampworld_file(&conn, &html, "Switzerland", "1840", "9999").expect("HTML import");
    assert_eq!((summary.imported, summary.skipped), (4, 0));
    assert!(import_stampworld_file(&conn, &path, "", "1840", "9999").is_err());
    assert!(import_stampworld_file(&conn, &path, "GB", "1974", "1973").is_err());
    assert!(
        import_stampworld_file(&conn, Path::new("not-a-pdf.csv"), "GB", "1840", "9999").is_err()
    );
    let invalid = temp.path().join("broken.pdf");
    std::fs::write(&invalid, b"not a PDF").expect("invalid fixture");
    assert!(import_stampworld_file(&conn, &invalid, "GB", "1840", "9999").is_err());
}
