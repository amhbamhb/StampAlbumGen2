use rusqlite::{params, Connection};
use crate::review_ui::pdf_parser::types::{PdfStampRecord, ImportSummary};
use crate::review_ui::http_utils::{is_real_image_path, local_image_file_exists};
use crate::db_utils::{StampInsert, insert_stamp_from_struct};

pub async fn best_effort_prepare_images_for_pdf(conn: &Connection, album_name: &str) -> (usize, usize) {
    (0, 0)
}

pub fn is_duplicate(conn: &Connection, record: &PdfStampRecord) -> bool {
    let sql = "SELECT 1 FROM stamps WHERE sg_number = ?1 AND year = ?2 AND description = ?3 AND color = ?4 LIMIT 1";
    let mut stmt = conn.prepare(sql).expect("Failed to prepare statement");
    stmt.exists(params![
        record.catalog_number,
        record.year,
        record.description,
        record.color,
    ]).unwrap_or(false)
}

pub fn process_pdf_records(conn: &Connection, records: Vec<PdfStampRecord>, country: &str) -> ImportSummary {
    let mut imported = 0;
    let mut skipped = 0;
    
    for record in records {
        if is_duplicate(conn, &record) {
            skipped += 1;
        } else {
            let si = StampInsert {
                country: country.to_string(),
                series: record.series,
                year: record.year.parse().unwrap_or(0),
                month: "".to_string(),
                month_num: "".to_string(),
                day: "".to_string(),
                sg_number: record.catalog_number,
                type_: record.stamp_type,
                denomination: record.denomination,
                color: record.color,
                description: record.description,
                stamps_issued: record.stamps_issued,
                mint_condition: record.mint_price,
                unused: record.unused_price,
                used: record.used_price,
                letter_fdc: record.fdc_price,
                currency: record.currency,
                image_path: record.local_image_path.unwrap_or_default(),
                remote_image_url: "".to_string(),
                original_image_url: "".to_string(),
                watermark: record.watermark,
                design: "".to_string(),
                engraving: record.designer,
                perforation: record.perforation,
                context: "".to_string(),
            };
            
            match insert_stamp_from_struct(conn, &si) {
                Ok(_) => imported += 1,
                Err(e) => {
                    println!("Database insert failed: {}", e);
                    skipped += 1;
                }
            }
        }
    }
    
    ImportSummary {
        total_found: imported + skipped,
        imported,
        skipped,
    }
}
