pub mod extraction;
pub mod types;

use rusqlite::Connection;
use std::path::Path;
use types::ImportSummary;

pub fn import_stampworld_file(
    conn: &Connection,
    path: &Path,
    country: &str,
    start_year: &str,
    end_year: &str,
) -> Result<ImportSummary, String> {
    let start = start_year.to_string();
    let end = end_year.to_string();
    if country.trim().is_empty() {
        return Err("A destination country is required".to_string());
    }
    if start_year > end_year {
        return Err("Start year must not exceed end year".to_string());
    }
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let records = match extension.to_ascii_lowercase().as_str() {
        "pdf" => extraction::parse_stampworld_pdf(path, country,&start, &end)?,
        "html" | "htm" => extraction::parse_stampworld_html(path)?,
        _ => return Err("Select a StampWorld PDF or HTML file".to_string()),
    };
    let total_found = records.len();
    let mut filtered = Vec::new();
    for record in records {
        let year = record
            .year
            .parse::<i32>()
            .map_err(|_| format!("Invalid year for stamp {}", record.catalog_number))?;
        if (start_year.parse::<i32>().unwrap()..=end_year.parse::<i32>().unwrap()).contains(&year) {
            filtered.push(record);
        }
    }
    let excluded = total_found - filtered.len();
    let mut summary = crate::review_ui::db_query::process_pdf_records(conn, filtered, country)?;
    summary.total_found = total_found;
    summary.skipped += excluded;
    Ok(summary)
}
