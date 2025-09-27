use log::{debug, error};
use crate::types::ImageStatus;
use scraper::{Html, Selector};
use regex::Regex;
use std::collections::HashSet;

/// Consolidated StampWorld collector implementing the key heuristics:
/// - Prefer images scoped to a stamp row; fall back to block/global images only when needed
/// - Stop after N consecutive pages that add zero new stamps (`stop_after_no_hits`)
/// - In-run dedupe using an in-memory HashSet keyed by country||series||sg_number
/// - Enforce requested year range strictly
pub fn collect_stampworld_data(
    sequencer: &mut i32,
    _separator: &str,
    site_name: &str,
    images_directory: &str,
    month_number: &dyn Fn(&str) -> String,
    get_stampworld_image: &dyn Fn(&str, bool) -> ImageStatus,
    base_url: &str,
    start_year: i32,
    end_year: i32,
    overwrite_images: bool,
    page_callback: Option<&dyn Fn(usize)>,
    max_pages: i32,
    stop_after_no_hits: i32,
) -> Vec<String> {
    debug!("collect_stampworld_data called for {} base_url={} range={}..{}", site_name, base_url, start_year, end_year);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .expect("Failed to build reqwest client");

    // Initialize DB (per previous behavior: DB under images dir parent)
    let country_db_name = site_name.replace("/", "-");
    let db_path = format!("{}/{}.db", images_directory.replace("/images", ""), country_db_name);
    let conn = match crate::db_utils::init_db(&db_path) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to initialize DB {}: {}", db_path, e);
            return Vec::new();
        }
    };

    let mut all_stamp_data: Vec<String> = Vec::new();
    let mut done = false;
    let mut page_num: i32 = 1;
    let mut consecutive_no_hits: i32 = 0;
    let mut seen_stamps: HashSet<String> = HashSet::new();

    // Helper selectors used across pages/blocks
    let content_selector = Selector::parse("div.container-fluid.content_table").unwrap_or_else(|_| Selector::parse("div.content_table").unwrap());
    let table_selector = Selector::parse("tr.stamp_tr").unwrap_or_else(|_| Selector::parse("tr").unwrap());

    // Precompile regexes
    let year_re = Regex::new(r"(18|19|20)\d{2}").unwrap();
    let range_re = Regex::new(r"^\s*\d+\s*[-\u{2011}\u{2013}\u{2012}\u{2010}]\s*\d+\s*$").unwrap();

    while !done {
        // Respect max_pages if set
        if max_pages > 0 && page_num > max_pages {
            debug!("Reached max_pages={} stopping", max_pages);
            break;
        }

        // Fetch page
        let url = format!("{}{}", base_url, page_num);
        debug!("Fetching StampWorld page: {}", url);
        let resp = match client.get(&url).send() {
            Ok(r) => r,
            Err(e) => {
                error!("Problem retrieving page {}: {}", url, e);
                break;
            }
        };
        let body = match resp.text() {
            Ok(b) => b,
            Err(e) => { error!("Failed to read page text: {}", e); break; }
        };
        if let Some(cb) = page_callback { cb(page_num as usize); }

        let document = Html::parse_document(&body);

        let mut new_stamps_this_page: i32 = 0;

        for content_div in document.select(&content_selector) {
            // Extract block-level details
            let mut context_details = String::new();
            let p_selector = Selector::parse("p").unwrap();
            let mut details_vec: Vec<String> = Vec::new();
            for p in content_div.select(&p_selector) {
                let t = p.text().collect::<String>();
                if !t.trim().is_empty() { details_vec.push(t); }
            }
            if !details_vec.is_empty() { context_details = details_vec.join(" | "); }

            // Extract block year from .table_header a if present
            let table_header_selector = Selector::parse(".table_header a").unwrap();
            let mut block_year_int: i32 = 0;
            let mut series_desc = String::new();
            if let Some(header_a) = content_div.select(&table_header_selector).next() {
                let text = header_a.text().collect::<String>();
                series_desc = text.clone();
                if let Some(caps) = Regex::new(r"(\d{4})(?:\s*-\s*(\d{4}))?").unwrap().captures(&text) {
                    block_year_int = caps.get(1).map_or("0", |m| m.as_str()).parse::<i32>().unwrap_or(0);
                }
            }

            // If block start year is well beyond requested end_year, stop to avoid unnecessary scraping
            if block_year_int > 0 && block_year_int > end_year + 10 {
                debug!("Block year {} significantly > endYear {} — stopping", block_year_int, end_year);
                done = true;
                break;
            }

            // Collect block/global images (fallback)
            let images_selector = Selector::parse(".images_container img").unwrap_or_else(|_| Selector::parse("img").unwrap());
            let mut global_images: Vec<(String, String)> = Vec::new(); // (url, alt)
            for img in content_div.select(&images_selector) {
                if let Some(src) = img.value().attr("src") {
                    let url = if src.starts_with("http") { src.to_string() } else if src.starts_with("//") { format!("https:{}", src) } else { format!("https://www.stampworld.com{}", src) };
                    let alt = img.value().attr("alt").unwrap_or("").to_string();
                    // Filter out obvious icons and gifs
                    if url.to_lowercase().contains(".gif") || url.to_lowercase().contains("layout") || url.to_lowercase().contains("static") { continue; }
                    global_images.push((url, alt));
                }
            }

            // Associate images with specific rows first
            let mut image_idx = 0usize;
            for tr in content_div.select(&table_selector) {
                // Basic row parsing
                let th_selector = Selector::parse("th").unwrap();
                let td_selector = Selector::parse("td").unwrap();
                let ths: Vec<_> = tr.select(&th_selector).collect();
                let tds: Vec<_> = tr.select(&td_selector).collect();
                if ths.is_empty() || tds.len() < 6 { continue; }

                let sg_number = ths[0].text().collect::<String>().trim().to_string();
                // skip ranges/groups like "23-25"
                if range_re.is_match(&sg_number) { continue; }
                let type_ = tds[0].text().collect::<String>().trim().to_string();
                let denomination = tds[1].text().collect::<String>().trim().to_string();
                let color = if tds.len() > 3 { tds[3].text().collect::<String>().trim().to_string() } else { String::new() };
                let description = if tds.len() > 4 { tds[4].text().collect::<String>().trim().to_string() } else { String::new() };
                let stamps_issued = if tds.len() > 5 { tds[5].text().collect::<String>().trim().to_string() } else { String::new() };

                // Prefer images scoped inside the row
                let mut selected_image_url = String::new();
                let mut _selected_alt = String::new();
                // row-local images
                for img in tr.select(&Selector::parse("img").unwrap()) {
                    if let Some(src) = img.value().attr("src") {
                        let url = if src.starts_with("http") { src.to_string() } else if src.starts_with("//") { format!("https:{}", src) } else { format!("https://www.stampworld.com{}", src) };
                        let alt = img.value().attr("alt").unwrap_or("").to_string();
                        if url.to_lowercase().contains(".gif") || url.to_lowercase().contains("layout") { continue; }
                        selected_image_url = url;
                        _selected_alt = alt;
                        break;
                    }
                }

                // Fallback to global images by order
                if selected_image_url.is_empty() {
                    if image_idx < global_images.len() {
                        selected_image_url = global_images[image_idx].0.clone();
                        _selected_alt = global_images[image_idx].1.clone();
                        image_idx += 1;
                    }
                }

                // Determine stamp year: prefer explicit per-row values, else fallback to block_year_int
                let mut stamp_year = block_year_int;
                let mut found_stamp_year = false;
                for td in &tds {
                    let text = td.text().collect::<String>();
                    if let Some(m) = year_re.find(&text) {
                        stamp_year = m.as_str().parse::<i32>().unwrap_or(block_year_int);
                        found_stamp_year = true;
                        break;
                    }
                }
                if !found_stamp_year {
                    if let Some(m) = year_re.find(&description) {
                        stamp_year = m.as_str().parse::<i32>().unwrap_or(block_year_int);
                    }
                }

                // Enforce year range
                if stamp_year < start_year { debug!("Skipping {}: year {} < start", sg_number, stamp_year); continue; }
                if stamp_year > end_year { debug!("Stamp {} year {} > endYear {}; will stop further scraping", sg_number, stamp_year, end_year); done = true; break; }

                // Build unique key and dedupe in-run
                let series_name = if !series_desc.is_empty() { series_desc.clone() } else { description.clone() };
                let key = format!("{}||{}||{}", site_name, series_name, sg_number);
                if seen_stamps.contains(&key) {
                    debug!("Duplicate in-run stamp skipped: {}", key);
                    continue;
                }

                // Determine local image path after download (if any)
                let mut local_image_path = String::new();
                let mut remote_image_url = String::new();
                let mut original_image_url = String::new();
                if !selected_image_url.is_empty() {
                    // Use downloader closure; it may accept an id or full URL
                    let image_res = get_stampworld_image(&selected_image_url, overwrite_images);
                    debug!("Downloader returned: exists={} filename='{}'", image_res.exists, image_res.filename);
                    if image_res.exists {
                        // create per-row filename to reduce accidental reuse
                        let sanitized_sg = crate::album_utils::sanitize_filename(&sg_number.replace('\u{2011}', "-").replace('\u{2013}', "-"));
                        let target_name = format!("{}-{}", sanitized_sg, image_res.filename);
                        let downloaded_path = format!("{}/{}", images_directory, image_res.filename);
                        let target_path = format!("{}/{}", images_directory, target_name);
                        // Copy into per-row file if necessary
                        if !std::path::Path::new(&target_path).exists() {
                            if std::path::Path::new(&downloaded_path).exists() {
                                match std::fs::copy(&downloaded_path, &target_path) {
                                    Ok(_) => { local_image_path = format!("images/{}", target_name); },
                                    Err(e) => { debug!("Failed to copy {} to {}: {}", downloaded_path, target_path, e); local_image_path = format!("images/{}", image_res.filename); }
                                }
                            } else {
                                // If downloaded path doesn't exist (downloader saved directly to target), use image_res.filename
                                local_image_path = format!("images/{}", image_res.filename);
                            }
                        } else {
                            local_image_path = format!("images/{}", target_name);
                        }
                        remote_image_url = selected_image_url.clone();
                        original_image_url = image_res.filename.clone();
                    }
                }

                // Month/day extraction - keep simple improved heuristic
                let mut month = String::new();
                let mut day = String::new();
                for d in &details_vec {
                    let re = Regex::new(r"(\d{1,2})\.\s*([A-Za-z]+)").unwrap();
                    if let Some(cap) = re.captures(d) {
                        day = cap.get(1).map_or("", |m| m.as_str()).to_string();
                        month = cap.get(2).map_or("", |m| m.as_str()).to_string();
                        break;
                    }
                }
                let month_num = if month.is_empty() { "0".to_string() } else { month_number(&month) };

                let cleaned_context_details = context_details.lines().filter(|l| !l.trim().is_empty() && l.trim() != "\u{200b}").collect::<Vec<_>>().join(" | ");

                // Call DB insert following the existing callers' parameter order
                let res = crate::db_utils::insert_stamp(
                    &conn,
                    site_name,
                    &series_name,
                    stamp_year,
                    &month,
                    &month_num,
                    &day,
                    &sg_number,
                    &type_,
                    &denomination,
                    &color,
                    &description,
                    &stamps_issued,
                    "", // mint_condition (not reliably parsed here)
                    "", // unused
                    "", // used
                    "", // letter_fdc
                    "", // currency
                    &local_image_path,
                    &remote_image_url,
                    &original_image_url,
                    "", // watermark
                    "", // design
                    "", // engraving
                    "", // perforation
                    &cleaned_context_details,
                );
                match res {
                    Ok(inserted) => {
                        if inserted {
                            new_stamps_this_page += 1;
                            seen_stamps.insert(key.clone());
                            let stamp_line = format!("STAMP;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                                stamp_year, month, day, series_name, sg_number, type_, denomination, color, description, stamps_issued, "", "", "", "", local_image_path, remote_image_url, original_image_url, "", "", "", cleaned_context_details);
                            all_stamp_data.push(stamp_line);
                        }
                    }
                    Err(e) => { error!("DB insert error for {} {} {}: {}", site_name, series_name, sg_number, e); }
                }

                if *sequencer < 0 { *sequencer = 0; } // dummy use to silence unused warnings when needed
            } // end row loop

            if done { break; }
        } // end content_div loop

        if new_stamps_this_page == 0 {
            consecutive_no_hits += 1;
        } else {
            consecutive_no_hits = 0;
        }
        if stop_after_no_hits > 0 && consecutive_no_hits >= stop_after_no_hits {
            debug!("Stopping due to {} consecutive pages with no new stamps", consecutive_no_hits);
            break;
        }

        // Move to the next page unless done
        if done { break; }
        page_num += 1;
    }

    debug!("Total stamp data lines collected: {}", all_stamp_data.len());
    all_stamp_data
}
