use crate::types::ImageStatus;
use log::{debug, error, info};
use rusqlite::params;
use scraper::{Html, Selector};
use std::collections::HashSet;

/// Collect data from 1847us.com pages
pub fn collect_us1847_data(
    _sequencer: &mut i32,
    _separator: &str,
    site_name: &str,
    images_directory: &str,
    _month_number: &dyn Fn(&str) -> String,
    _get_image: &dyn Fn(&str, bool) -> ImageStatus,
    base_url: &str,
    start_year: i32,
    end_year: i32,
    _overwrite_images: bool,
    page_callback: Option<&dyn Fn(usize)>,
    max_pages: i32,
    _stop_after_no_hits: i32,
) -> Vec<String> {
    info!(
        "collect_us1847_data called for {} base_url={} range={}..{}",
        site_name, base_url, start_year, end_year
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .expect("Failed to build reqwest client");

    // Initialize DB
    let country_db_name = site_name.replace("/", "-");
    let db_path = format!(
        "{}/{}.db",
        images_directory.replace("/images", ""),
        country_db_name
    );
    let conn = match crate::db_utils::init_db(&db_path) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to initialize DB {}: {}", db_path, e);
            return Vec::new();
        }
    };

    let all_stamp_data: Vec<String> = Vec::new();
    let done = false;
    let mut page_num: i32 = 1;
    let _consecutive_no_hits: i32 = 0;
    let _seen_stamps: HashSet<String> = HashSet::new();

    while !done {
        // Respect max_pages if set
        if max_pages > 0 && page_num > max_pages {
            debug!("Reached max_pages={} stopping", max_pages);
            break;
        }

        let url = format!("{}{}", base_url, page_num);
        info!("Request URL: {}", url);
        let resp = match client.get(&url).send() {
            Ok(r) => r,
            Err(e) => {
                error!("Problem retrieving page {}: {}", url, e);
                break;
            }
        };
        let status = resp.status();
        let body = match resp.text() {
            Ok(b) => b,
            Err(e) => {
                error!("Failed to read page text: {}", e);
                break;
            }
        };
        info!("HTTP status for {} -> {}", url, status);

        if let Some(cb) = page_callback {
            cb(page_num as usize);
        }

        let document = Html::parse_document(&body);

        // Extract series title
        let series_title = document
            .select(&Selector::parse("h1").unwrap())
            .next()
            .map(|h| h.text().collect::<String>().trim().to_string())
            .unwrap_or_else(|| "Unknown Series".to_string());

        // Insert or get series
        let series_id = conn.query_row(
            "INSERT OR IGNORE INTO us1847_series (series_name, series_url) VALUES (?, ?) RETURNING id",
            params![series_title, base_url],
            |row| row.get::<_, i64>(0),
        ).unwrap_or_else(|_| {
            conn.query_row(
                "SELECT id FROM us1847_series WHERE series_url = ?",
                params![base_url],
                |row| row.get::<_, i64>(0),
            ).unwrap_or(0)
        });

        // Extract stamps from the page
        // Assuming stamps are in a table or divs
        let stamp_selector = Selector::parse("table tr").unwrap(); // Adjust selector as needed
        for row in document.select(&stamp_selector) {
            // Parse row for stamp details
            // This is placeholder - need to implement based on actual HTML structure
            let cells: Vec<String> = row
                .select(&Selector::parse("td").unwrap())
                .map(|td| td.text().collect::<String>().trim().to_string())
                .collect();

            if cells.len() < 5 {
                continue;
            }

            // Example parsing - adjust based on actual structure
            let denomination = cells.get(0).cloned().unwrap_or_default();
            let color = cells.get(1).cloned().unwrap_or_default();
            let description = cells.get(2).cloned().unwrap_or_default();
            let scott_number = cells.get(3).cloned().unwrap_or_default();
            let fdc_date = cells.get(4).cloned().unwrap_or_default();

            // Insert stamp
            if let Err(e) = conn.execute(
                "INSERT INTO us1847_stamps (series_id, denomination, color, description) VALUES (?, ?, ?, ?)",
                params![series_id, denomination, color, description],
            ) {
                error!("Failed to insert stamp: {}", e);
                continue;
            }

            let stamp_id = conn.last_insert_rowid();

            // Insert variation
            if let Err(e) = conn.execute(
                "INSERT INTO us1847_variations (stamp_id, scott_number, fdc_date) VALUES (?, ?, ?)",
                params![stamp_id, scott_number, fdc_date],
            ) {
                error!("Failed to insert variation: {}", e);
            }

            // TODO: Handle images, more details
        }

        page_num += 1;
    }

    info!("Total stamp data lines collected: {}", all_stamp_data.len());
    all_stamp_data
}
