use stamp_album_gen2::colnect::{
    ingest::ingest_colnect_data_with_mode,
    parser::parse_csv,
    template::{TEMPLATE_CSV, TEMPLATE_GUIDE, ensure_manual_import_files},
};

#[test]
fn creates_missing_files_and_preserves_existing_copies() {
    let runtime = tempfile::tempdir().expect("runtime folder");
    let created = ensure_manual_import_files(runtime.path()).expect("create template files");
    assert_eq!(created.len(), 2);
    let directory = runtime.path().join("UserImport");
    let csv = directory.join("manual_import_template.csv");
    let guide = directory.join("README.md");
    assert_eq!(std::fs::read_to_string(&csv).expect("CSV"), TEMPLATE_CSV);
    assert_eq!(
        std::fs::read_to_string(&guide).expect("guide"),
        TEMPLATE_GUIDE
    );
    assert!(parse_csv(&csv).expect("blank template parses").is_empty());
    assert!(
        ensure_manual_import_files(runtime.path())
            .expect("repeat startup")
            .is_empty()
    );

    std::fs::write(&csv, "user's working data").expect("edit template");
    std::fs::remove_file(&guide).expect("remove guide");
    assert_eq!(
        ensure_manual_import_files(runtime.path()).expect("restore guide"),
        vec![guide.clone()]
    );
    assert_eq!(
        std::fs::read_to_string(&csv).expect("preserved CSV"),
        "user's working data"
    );
    std::fs::write(&guide, "custom instructions").expect("edit guide");
    std::fs::remove_file(&csv).expect("remove CSV");
    assert_eq!(
        ensure_manual_import_files(runtime.path()).expect("restore CSV"),
        vec![csv.clone()]
    );
    assert_eq!(std::fs::read_to_string(csv).expect("CSV"), TEMPLATE_CSV);
    assert_eq!(
        std::fs::read_to_string(guide).expect("preserved guide"),
        "custom instructions"
    );
}

#[test]
fn reports_conflicting_template_path_without_overwriting_it() {
    let runtime = tempfile::tempdir().expect("runtime folder");
    let csv = runtime
        .path()
        .join("UserImport")
        .join("manual_import_template.csv");
    std::fs::create_dir_all(&csv).expect("conflicting directory");
    let error = ensure_manual_import_files(runtime.path()).expect_err("must report conflict");
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert!(
        error
            .to_string()
            .contains("Manual import file path is not a file")
    );
    assert!(csv.is_dir());
}

#[test]
fn manual_rows_parse_and_replace_only_the_same_staged_identity() {
    let runtime = tempfile::tempdir().expect("runtime folder");
    ensure_manual_import_files(runtime.path()).expect("create files");
    let mut reader = csv::Reader::from_reader(TEMPLATE_CSV.as_bytes());
    let headers = reader.headers().expect("template headers").clone();
    assert_eq!(headers.len(), 38);
    let working = runtime.path().join("completed.csv");
    let write_rows = |name: &str, include_second: bool| {
        let mut writer = csv::Writer::from_path(&working).expect("working CSV");
        writer.write_record(&headers).expect("headers");
        for (id, title, number) in [
            ("manual-ch-0001", name, "1041"),
            ("manual-ch-0002", "Humanitarian Law", "1042"),
        ]
        .into_iter()
        .take(if include_second { 2 } else { 1 })
        {
            let row: Vec<String> = headers
                .iter()
                .map(|header| match header {
                    "Name" => title.to_string(),
                    "Country" => "Switzerland".to_string(),
                    "Series" => "1975 Commemoratives".to_string(),
                    "Catalog Codes" => format!("Sg:CH {number}, Col:{id}"),
                    "Issued on" => "1975-02-13".to_string(),
                    "Width" => "24.5".to_string(),
                    "Height" => "30".to_string(),
                    "Currency" => "CHF".to_string(),
                    "FaceValue" => "0.40".to_string(),
                    "Description" => "Design titled \"Peace\", blue".to_string(),
                    _ => String::new(),
                })
                .collect();
            writer.write_record(row).expect("row");
        }
        writer.flush().expect("flush");
    };

    write_rows("International Women's Year", true);
    let stamps = parse_csv(&working).expect("manual rows");
    assert_eq!(stamps.len(), 2);
    assert_eq!(stamps[0].link, "");
    assert_eq!(stamps[0].width.as_deref(), Some("24.5"));
    assert_eq!(
        stamps[0].description.as_deref(),
        Some("Design titled \"Peace\", blue")
    );
    let db_path = runtime.path().join("Switzerland.db");
    let conn = stamp_album_gen2::db_utils::init_db(
        db_path.to_str().expect("UTF-8 temporary database path"),
    )
    .expect("database");
    conn.execute(
        "INSERT INTO stamps (country, sg_number, description)
         VALUES ('Switzerland', '1041', 'Existing album description')",
        [],
    )
    .expect("existing album stamp");
    ingest_colnect_data_with_mode(&conn, &stamps, "sg", false).expect("initial staging");
    write_rows("Corrected stamp name", false);
    let updated = parse_csv(&working).expect("updated rows");
    ingest_colnect_data_with_mode(&conn, &updated, "sg", false).expect("replace staging");
    let (count, name): (i64, String) = conn
        .query_row(
            "SELECT (SELECT COUNT(*) FROM colnect_stamps), name
             FROM colnect_stamps WHERE colnect_id = 'manual-ch-0001'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .expect("staged data");
    assert_eq!(count, 2, "unrelated candidate preserved; no duplicate ID");
    assert_eq!(name, "Corrected stamp name");
    let (main_count, main_description): (i64, String) = conn
        .query_row(
            "SELECT (SELECT COUNT(*) FROM stamps), description FROM stamps WHERE sg_number = '1041'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .expect("main records");
    assert_eq!(main_count, 1, "no unapproved stamp additions");
    assert_eq!(main_description, "Existing album description");
    println!(
        "Switzerland database {}: schema initialized by init_db; 2 staged inserts, \
         1 staged identity replaced, 1 unrelated candidate preserved; \
         duplicate ID count = 0; main table unchanged (1 existing stamp). \
         Input CSV edited in place, not moved. Temporary folder {} removed on drop.",
        db_path.display(),
        runtime.path().display()
    );
}

#[test]
fn documented_example_is_accepted_by_the_existing_parser() {
    let runtime = tempfile::tempdir().expect("runtime folder");
    let example = TEMPLATE_GUIDE
        .split_once("```csv\n")
        .expect("CSV example start")
        .1
        .split_once("```")
        .expect("CSV example end")
        .0;
    let path = runtime.path().join("example.csv");
    std::fs::write(&path, example).expect("example file");
    let stamps = parse_csv(path).expect("documented CSV example");
    assert_eq!(stamps.len(), 2);
    assert_eq!(stamps[0].country, "Switzerland");
    assert_eq!(stamps[0].issued_on, "1975-02-13");
    assert_eq!(stamps[0].catalog_codes, "Sg:CH 1041, Col:manual-ch-0001");
    assert!(stamps.iter().all(|stamp| stamp.link.is_empty()));
}
