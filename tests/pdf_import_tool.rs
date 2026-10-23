use stamp_album_gen2::review_ui::pdf_parser::types::PdfStampRecord;
use stamp_album_gen2::review_ui::db_query::{process_pdf_records, is_duplicate};
use rusqlite::Connection;

#[test]
fn test_manual_import_single_record() {
    let db_path = ":memory:";
    let conn = Connection::open(db_path).expect("Failed to open DB");
    
    // Create minimal schema
    conn.execute("CREATE TABLE IF NOT EXISTS stamps (
        id INTEGER PRIMARY KEY,
        country TEXT,
        series TEXT,
        year INTEGER,
        month TEXT,
        month_num TEXT,
        day TEXT,
        sg_number TEXT,
        type TEXT,
        denomination TEXT,
        color TEXT,
        description TEXT,
        stamps_issued TEXT,
        mint_condition TEXT,
        unused TEXT,
        used TEXT,
        letter_fdc TEXT,
        currency TEXT,
        image_path TEXT,
        remote_image_url TEXT,
        original_image_url TEXT,
        watermark TEXT,
        design TEXT,
        engraving TEXT,
        perforation TEXT,
        context TEXT,
        uniq_key TEXT UNIQUE
    )", []).expect("Failed to create table");

    let record = PdfStampRecord {
        catalog_number: "571".to_string(),
        stamp_type: "MC".to_string(),
        description: "Sir James Clark Ross".to_string(),
        color: "multicoloured".to_string(),
        denomination: "3P".to_string(),
        year: "1972".to_string(),
        series: "1972 British Polar Explorers".to_string(),
        designer: "Marjorie Saynor".to_string(),
        engraver: "Harrison & Sons Ltd".to_string(),
        perforation: "14.25x14.75".to_string(),
        watermark: "None".to_string(),
        currency: "USD".to_string(),
        stamps_issued: "53,630,000".to_string(),
        mint_price: "0.28".to_string(),
        unused_price: "0.00".to_string(),
        used_price: "0.28".to_string(),
        fdc_price: "0.00".to_string(),
        local_image_path: None,
    };

    let summary = process_pdf_records(&conn, vec![record], "Great Britain");
    
    assert_eq!(summary.imported, 1);
    assert_eq!(summary.skipped, 0);
    println!("Successfully imported: {:?}", summary);
}
