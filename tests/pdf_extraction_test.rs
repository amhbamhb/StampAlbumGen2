#[test]
fn stampworld_pdf_extraction_layout() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("download-pdf/Great Britain - Postage stamps (1970 - 1979).pdf");
    let records = stamp_album_gen2::review_ui::pdf_parser::extraction::parse_stampworld_pdf(
        path, "GB", "1970", "1979",
    )
    .expect("parse local StampWorld PDF");
    assert_eq!(records.len(), 22);
    let first = &records[0];
    assert_eq!(first.catalog_number, "571");
    assert_eq!(first.year, "1972");
    assert_eq!(first.series, "1972 British Polar Explorers");
    assert_eq!(first.description, "Sir James Clark Ross");
    assert_eq!(first.color, "multicoloured");
    assert_eq!(first.designer, "Marjorie Saynor");
    assert_eq!(first.engraver, "Harrison & Sons Ltd");
    assert_eq!(first.stamps_issued, "53.630.000");
    assert_eq!(records[2].denomination, "7½P");
    assert!(records[4].description.ends_with("Tutankhamen"));
    assert_eq!(records[16].description, "");
}

#[test]
fn canonical_html_preserves_individual_rows_and_metadata() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Example-Stampworldwebpage.txt");
    let records = stamp_album_gen2::review_ui::pdf_parser::extraction::parse_stampworld_html(&path)
        .expect("parse canonical HTML sample");
    assert_eq!(records.len(), 4);
    assert_eq!(records[0].catalog_number, "1040");
    assert_eq!(records[0].stamp_type, "ANZ");
    assert_eq!(records[0].year, "1975");
    assert_eq!(records[0].denomination, "15(C)");
    assert_eq!(records[0].designer, "Roland Hirter");
    assert_eq!(records[0].engraver, "Courvoisier S.A");
    assert_eq!(records[3].stamp_type, "AOC");
}

#[test]
fn all_supplied_stampworld_pdfs_have_individual_rows() {
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("download-pdf");
    for (name, count) in [
        ("Great Britain - Postage stamps (1970 - 1979).pdf", 22),
        ("United States - Postage stamps (1970 - 1979).pdf", 27),
    ] {
        let path = directory.join(name);
        let records = stamp_album_gen2::review_ui::pdf_parser::extraction::parse_stampworld_pdf(
            &path, "GB", "1970", "1979",
        )
        .unwrap_or_else(|e| {
            panic!(
                "{}: {}\n{}",
                path.display(),
                e,
                stamp_album_gen2::review_ui::pdf_parser::extraction::extract_stampworld_pdf_text(
                    &path
                )
                .expect("layout text")
                .chars()
                .take(2000)
                .collect::<String>()
            )
        });
        assert_eq!(records.len(), count, "{}", path.display());
        assert!(
            records
                .iter()
                .all(|r| !r.year.is_empty() && !r.series.is_empty())
        );
        assert!(records.iter().all(|r| r.currency == "USD"));
    }
}

#[test]
#[ignore = "requires the six local, untracked Denmark PDFs in download-pdf"]
fn supplied_denmark_pages_have_266_individual_stamps() {
    let mut total = 0;
    for (index, count) in [50, 52, 48, 50, 50, 16].into_iter().enumerate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("download-pdf")
            .join(format!(
                "Denmark - Postage stamps (1980 - 1989) - Page {}.pdf",
                index + 1
            ));
        let records = stamp_album_gen2::review_ui::pdf_parser::extraction::parse_stampworld_pdf(
            &path, "DK", "1980", "1989",
        )
        .unwrap_or_else(|e| panic!("{}: {}", path.display(), e));
        assert!(records.iter().all(|r| r.watermark == "None"));
        let first = 700 + total;
        for (index, record) in records.iter().enumerate() {
            assert_eq!(
                record.catalog_number,
                (first + index).to_string(),
                "{record:?}\n{}",
                stamp_album_gen2::review_ui::pdf_parser::extraction::extract_stampworld_pdf_text(
                    &path
                )
                .expect("layout text")
                .chars()
                .take(2100)
                .collect::<String>()
            );
        }
        assert_eq!(records.len(), count, "{}", path.display());
        total += records.len();
    }
    assert_eq!(total, 266);
}

#[test]
fn malformed_rows_and_missing_headings_are_errors_not_empty_successes() {
    use stamp_album_gen2::review_ui::pdf_parser::extraction::parse_stampworld_text;
    for text in [
        "",
        "1972 Explorers\n571 MC 3P   red   Missing prices",
        "571 MC 3P   red   Ross (100) 0.28 - 0.28 - USD",
    ] {
        assert!(parse_stampworld_text(text).is_err(), "{text}");
    }
    let records = parse_stampworld_text(
        "2001 Series\n1972 ABC 7½P   dark blue   A stamp (2.5 mill) 1.00 - 0.50 - EUR",
    )
    .expect("four-digit catalogue number is not a year heading");
    assert_eq!(records[0].catalog_number, "1972");
    assert_eq!(records[0].color, "dark blue");
    assert_eq!(records[0].stamps_issued, "2500000");
    assert_eq!(records[0].currency, "EUR");
}
