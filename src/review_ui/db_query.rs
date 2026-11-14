use crate::db_utils::{StampInsert, insert_stamp_from_struct};
use crate::review_ui::http_utils::{is_real_image_path, local_image_file_exists};
use crate::review_ui::pdf_parser::types::{ImportSummary, PdfStampRecord};
use rusqlite::{Connection, params};

pub async fn best_effort_prepare_images_for_pdf(
    conn: &Connection,
    album_name: &str,
) -> (usize, usize) {
    (0, 0)
}

pub fn is_duplicate(conn: &Connection, record: &PdfStampRecord) -> rusqlite::Result<bool> {
    let sql = "SELECT 1 FROM stamps WHERE sg_number = ?1 AND year = ?2 AND description = ?3 AND color = ?4 LIMIT 1";
    let mut stmt = conn.prepare(sql)?;
    stmt.exists(params![
        record.catalog_number,
        record.year,
        record.description,
        record.color,
    ])
}

pub fn process_pdf_records(
    conn: &Connection,
    records: Vec<PdfStampRecord>,
    country: &str,
) -> Result<ImportSummary, String> {
    let transaction = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    let mut imported = 0;
    let mut skipped = 0;

    for record in records {
        let duplicate = transaction.query_row(
            "SELECT EXISTS(SELECT 1 FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3)",
            params![country, record.series, record.catalog_number],
            |row| row.get::<_, bool>(0),
        ).map_err(|e| format!("Duplicate check failed: {}", e))?;
        if duplicate {
            skipped += 1;
        } else {
            let si = StampInsert {
                country: country.to_string(),
                series: record.series,
                year: record
                    .year
                    .parse()
                    .map_err(|_| format!("Invalid year for stamp {}", record.catalog_number))?,
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
                design: record.designer,
                engraving: record.engraver,
                perforation: record.perforation,
                context: "".to_string(),
            };

            match insert_stamp_from_struct(&transaction, &si)
                .map_err(|e| format!("Database insert failed for stamp {}: {}", si.sg_number, e))?
            {
                true => imported += 1,
                false => skipped += 1,
            }
        }
    }

    transaction
        .commit()
        .map_err(|e| format!("Import commit failed: {}", e))?;
    Ok(ImportSummary {
        total_found: imported + skipped,
        imported,
        skipped,
    })
}
