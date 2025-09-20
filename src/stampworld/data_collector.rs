// StampWorld data collection module
use log::{debug, error, info};
use crate::types::ImageStatus;
use crate::album_globals::clean_text;
use crate::album_utils::sanitize_filename;
use crate::file_utils::get_db_path_from_cli;
use crate::placeholder_image;
use rusqlite::params;

/// Extract series information from alt attribute format: [Queen Victoria - Size: ..., type A] or [Series Name, type TYPE]
fn extract_series_from_alt(alt_text: &str) -> String {
    if alt_text.is_empty() {
        return String::new();
    }

    // Remove brackets if present
    let clean_alt = alt_text.trim().trim_matches(|c| c == '[' || c == ']');
    debug!("[extract_series_from_alt] Input alt_text: '{}', clean_alt: '{}'", alt_text, clean_alt);

    // Look for pattern like "Queen Victoria - Size: ..., type A"
    if let Some(size_part) = clean_alt.split(" - Size:").next() {
        // Extract just the series name before " - Size:"
        let series_name = size_part.trim();
        if !series_name.is_empty() {
            debug!("[extract_series_from_alt] Found Size pattern, returning: '{}'", series_name);
            return series_name.to_string();
        }
    }

    // Look for pattern like "National Wild Life Week, type HB"
    if let Some(series_part) = clean_alt.split(", type").next() {
        // Extract just the series name before ", type"
        let series_name = series_part.trim();
        if !series_name.is_empty() {
            debug!("[extract_series_from_alt] Found type pattern, returning: '{}'", series_name);
            return series_name.to_string();
        }
    }

    // Fallback: if pattern doesn't match, return the whole alt text
    debug!("[extract_series_from_alt] No pattern matched, returning whole text: '{}'", clean_alt);
    clean_alt.to_string()
}

// /// Determine the proper series name based on stamp type (fallback method)
// fn determine_series_from_type(stamp_type: &str, block_desc: &str) -> String {
//     if stamp_type.is_empty() {
//         return block_desc.to_string();
//     }
    
//     // Extract series letter from stamp type
//     let series_letter = stamp_type.chars().next().unwrap_or(' ').to_ascii_uppercase();
    
//     match series_letter {
//         'L' => format!("{}-series Registration Stamps", series_letter),
//         'M' => format!("{}-series Definitive Stamps", series_letter),
//         'N' => format!("{}-series Definitive Stamps", series_letter),
//         'O' => format!("{}-series Registration Stamps", series_letter),
//         'P' => format!("{}-series Registration Stamps", series_letter),
//         'Q' => format!("{}-series Registration Stamps", series_letter),
//         'R' => format!("{}-series Special Delivery Stamps", series_letter),
//         'S' => format!("{}-series Special Delivery Stamps", series_letter),
//         'T' => format!("{}-series Tax Stamps", series_letter),
//         'U' => format!("{}-series Utility Stamps", series_letter),
//         _ => {
//             // For unknown series, try to extract from block description
//             if block_desc.contains("Registration") {
//                 format!("{}-series Registration Stamps", series_letter)
//             } else if block_desc.contains("Special Delivery") {
//                 format!("{}-series Special Delivery Stamps", series_letter)
//             } else if block_desc.contains("Definitive") {
//                 format!("{}-series Definitive Stamps", series_letter)
//             } else {
//                 block_desc.to_string()
//             }
//         }
//     }
// }

pub fn collect_stampworld_data(
    _sequencer: &mut i32,
    _separator: &str,
    _site_name: &str,
    _images_directory: &str,
    _month_number: &dyn Fn(&str) -> String,
    get_stampworld_image: &dyn Fn(&str, bool) -> ImageStatus,
    base_url: &str,
    start_year: i32,
    end_year: i32,
    overwrite_images: bool,
    page_callback: Option<&dyn Fn(usize)>,
) -> Vec<String> {
    debug!("collect_stampworld_data called with base_url: {}", base_url);
    let mut all_stamp_data = Vec::new();
    let mut done = false;
    let mut page_num = 1;
    const MAX_PAGES: i32 = 10; // Prevent infinite loop
    let mut previous_body = String::new();
    let client = reqwest::blocking::Client::new();

    // Initialize SQLite database in the album directory for the country
    // Use the CLI-provided db_path for consistency
    let db_path = get_db_path_from_cli();
    let conn = crate::db_utils::init_db(&db_path).expect("Failed to initialize database");
    
    // If overwrite is true, clear existing data
    if overwrite_images {
        debug!("Overwrite flag is true, clearing existing stamp data");
        conn.execute("DELETE FROM stamps", []).expect("Failed to clear existing data");
    }

    while !done {
        if page_num > MAX_PAGES {
            debug!("Reached maximum pages limit: {}", MAX_PAGES);
            break;
        }
        let url = format!("{}{}", base_url, page_num);
        println!("[INFO] Scraping StampWorld page {}: {}", page_num, url);
        
        if let Some(cb) = page_callback {
            cb(page_num as usize);
        }
    debug!("Fetching Stampworld page: {}", url);
        let resp = match client.get(&url).timeout(std::time::Duration::from_secs(60)).send() {
            Ok(r) => r,
            Err(e) => {
                if page_num == 1 {
                    error!("Please check that you have entered the correct site country name");
                } else {
                    error!("Problem retrieving web page: {}", e);
                }
                std::process::exit(1);
            }
        };
        let body = match resp.text() {
            Ok(b) => b,
            Err(e) => {
                error!("Problem retrieving web page text: {}", e);
                std::process::exit(1);
            }
        };
        println!("[INFO] Page {} downloaded successfully ({} bytes)", page_num, body.len());
        
        if !previous_body.is_empty() && body == previous_body {
            debug!("Page content is the same as previous page, stopping to prevent infinite loop");
            break;
        }
        previous_body = body.clone();
        let document = scraper::Html::parse_document(&body);
        debug!("HTML document parsed successfully, length: {}", body.len());
        // Extract default year from base_url
        let mut default_year = String::new();
        if let Some(year_part) = base_url.split("Postage%20stamps/").nth(1) {
            if let Some(year_str) = year_part.split('?').next() {
                default_year = year_str.to_string();
            }
        }
        let content_selector = scraper::Selector::parse("table.data_table").unwrap();
        debug!("Content selector created successfully");
        let content_count = document.select(&content_selector).count();
        debug!("Found {} content blocks with selector 'table.data_table'", content_count);
        
        if content_count == 0 {
            debug!("No tables with class 'data_table' found. Available tables:");
            let table_selector = scraper::Selector::parse("table").unwrap();
            let all_tables = document.select(&table_selector);
            for (i, table) in all_tables.enumerate() {
                let class_attr = table.value().attr("class").unwrap_or("no-class");
                debug!("Table {}: class='{}'", i, class_attr);
            }
        }
        
        // Extract metadata fields from the entire document (these persist across all tables)
        let mut watermark = String::new();
        let mut design = String::new();
        let mut engraving = String::new();
        let mut perforation = String::new();
        
        // Try to get details from p tags containing metadata in the broader document
        let p_selector = scraper::Selector::parse("p").unwrap();
        let table_header_p_selector = scraper::Selector::parse(".table_header p").unwrap();
        let mut details_vec = Vec::new();
        
        // First, collect all p tags within table headers (these contain the date and metadata)
        for p in document.select(&table_header_p_selector) {
            let p_text = p.text().collect::<String>();
            if !p_text.is_empty() {
                // Split the text by newlines and add each line
                for line in p_text.lines() {
                    let trimmed = line.trim();
                    if !trimmed.is_empty() {
                        details_vec.push(trimmed.to_string());
                    }
                }
            }
        }
        
        // Also check other p tags for metadata (fallback)
        if details_vec.is_empty() {
            for p in document.select(&p_selector) {
                let p_text = p.text().collect::<String>();
                if !p_text.is_empty() && (p_text.contains("Design:") || p_text.contains("Engraving:") || p_text.contains("Watermark:") || p_text.contains("WM:") || p_text.contains("Perforation:")) {
                    // Split the text by newlines and add each metadata line
                    for line in p_text.lines() {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() && (trimmed.contains("Design:") || trimmed.contains("Engraving:") || trimmed.contains("Watermark:") || trimmed.contains("WM:") || trimmed.contains("Perforation:")) {
                            details_vec.push(trimmed.to_string());
                        }
                    }
                }
            }
        }
        
        // Extract fields from details_vec
        for detail in &details_vec {
            let detail_lower = detail.to_lowercase();

                    // Extract watermark - handle both "WM:" and "Watermark:" patterns
                    if detail_lower.contains("watermark:") {
                        let parts: Vec<&str> = detail.splitn(2, ':').collect();
                        if parts.len() > 1 {
                            watermark = parts[1].trim().to_string();
                        }
                    } else if detail_lower.starts_with("wm") && detail.contains(":") {
                        let parts: Vec<&str> = detail.splitn(2, ':').collect();
                        if parts.len() > 1 {
                            watermark = parts[1].trim().to_string();
                        }
                    }
            if detail_lower.contains("design:") {
                let parts: Vec<&str> = detail.splitn(2, ':').collect();
                if parts.len() > 1 {
                    // Take only the first line to avoid picking up extra content
                    let design_content = parts[1].trim();
                    let first_line = design_content.lines().next().unwrap_or(design_content);
                    design = first_line.trim().to_string();
                }
            }

            // Extract engraving - take only the first line after "Engraving:"
            if detail_lower.contains("engraving:") {
                let parts: Vec<&str> = detail.splitn(2, ':').collect();
                if parts.len() > 1 {
                    // Take only the first line to avoid picking up extra content
                    let engraving_content = parts[1].trim();
                    let first_line = engraving_content.lines().next().unwrap_or(engraving_content);
                    engraving = first_line.trim().to_string();
                }
            }

            // Extract perforation - look for the last perforation value
            if detail_lower.contains("perforation:") {
                let parts: Vec<&str> = detail.splitn(2, ':').collect();
                if parts.len() > 1 {
                    let perf_content = parts[1].trim();
                    // Extract just the perforation value (usually a number)
                    let perf_re = regex::Regex::new(r"(\d+(?:\.\d+)?(?:\s*x\s*\d+(?:\.\d+)?)?)").unwrap();
                    if let Some(cap) = perf_re.find(perf_content) {
                        perforation = cap.as_str().to_string();
                    } else {
                        perforation = perf_content.to_string();
                    }
                }
            }
        }
        // Extract series information from standalone series links that appear before tables
        let mut series_links = Vec::new();
        // Try multiple selectors for series links - be more specific to exclude navigation
        let series_selectors = vec![
            "a[href*='/stamps/'][href*='/g']",  // Links with /g in href (series links)
            ".series-title",                    // Series title class
        ];

        for selector_str in &series_selectors {
            if let Ok(selector) = scraper::Selector::parse(selector_str) {
                for link in document.select(&selector) {
                    if let Some(href) = link.value().attr("href") {
                        let series_text = link.text().collect::<String>().trim().to_string();
                        // Exclude common navigation terms regardless of href pattern
                        if series_text == "Next" || series_text == "Previous" || 
                           series_text == "First" || series_text == "Last" ||
                           series_text.is_empty() {
                            continue;
                        }
                        // For series links, require the /g pattern and exclude page parameters
                        if href.contains("/stamps/") &&
                           href.contains("/g") &&
                           !href.contains("?page=") &&
                           !href.contains("&page=") &&
                           !href.contains("page=") &&
                           !series_text.chars().all(|c| c.is_numeric() || c == '-') &&
                           !series_links.contains(&series_text) {
                            series_links.push(series_text.clone());
                            debug!("[stampworld] Found series link with selector '{}': '{}'", selector_str, series_text);
                        }
                    }
                }
            }
        }
        
        // Also try to find series in paragraph text that might contain series names
        let p_selector = scraper::Selector::parse("p").unwrap();
        for p in document.select(&p_selector) {
            let p_text = p.text().collect::<String>();
            // Look for patterns that indicate series names, exclude navigation terms
            if p_text.contains(" - ") &&
               (p_text.contains("Stamps") || p_text.contains("Victoria") || p_text.contains("Queen") ||
                p_text.contains("King") || p_text.contains("Anniversary") || p_text.contains("Registration")) &&
               p_text != "Next" && p_text != "Previous" && p_text != "First" && p_text != "Last" {
                let trimmed = p_text.trim();
                if !trimmed.is_empty() && !series_links.contains(&trimmed.to_string()) {
                    series_links.push(trimmed.to_string());
                    debug!("[stampworld] Found series in paragraph: '{}'", trimmed);
                }
            }
        }
        
        // Try to extract series from image alt text (only as fallback, and only if we don't already have a good series name)
        let img_selector = scraper::Selector::parse("img").unwrap();
        for img in document.select(&img_selector) {
            if let Some(alt) = img.value().attr("alt") {
                let alt_text = alt.trim();
                if alt_text.contains(" - ") &&
                   (alt_text.contains("Stamps") || alt_text.contains("Victoria") || alt_text.contains("Queen") ||
                    alt_text.contains("King") || alt_text.contains("Anniversary") || alt_text.contains("Registration")) &&
                   alt_text != "Next" && alt_text != "Previous" && alt_text != "First" && alt_text != "Last" {
                    // Only use image alt text if we don't already have series links from better sources
                    // and only if the alt text looks like it contains a complete series name
                    if series_links.is_empty() || series_links.len() < 2 {
                        let cleaned_series = extract_series_from_alt(alt_text);
                        if !cleaned_series.is_empty() && !series_links.contains(&cleaned_series) {
                            series_links.push(cleaned_series.clone());
                            debug!("[stampworld] Found series in image alt (fallback): '{}' -> cleaned: '{}'", alt_text, cleaned_series);
                        }
                    }
                }
            }
        }
        
        debug!("[stampworld] Total series links found: {}", series_links.len());
        for (i, link) in series_links.iter().enumerate() {
            debug!("[stampworld] Series link {}: '{}'", i, link);
        }
        

        
        'block_loop: for (table_index, content_div) in document.select(&content_selector).enumerate() {
            debug!("Processing table {} with HTML: {}", table_index, content_div.html().chars().take(200).collect::<String>());
            let stop_scraping = false;
            // Improved context extraction: look for preceding header tags or text nodes
            let context_above = String::new();
            let context_details = details_vec.join(" | ");
            
            // Extract year from block header using .table_header a within this table
            let table_header_selector = scraper::Selector::parse(".table_header a").unwrap();
            let mut block_year = String::new();
            let mut desc = String::new();
            
            // Try to get series from our extracted series links based on table index
            let proper_series = if table_index < series_links.len() {
                debug!("[stampworld] Table index {}: Using series from link '{}' (total series links: {})", table_index, series_links[table_index], series_links.len());
                series_links[table_index].clone()
            } else {
                debug!("[stampworld] Table index {} is out of bounds for series_links (length: {}), using fallback", table_index, series_links.len());
                // Fallback to extracting from table header
                if let Some(header_a) = content_div.select(&table_header_selector).next() {
                    let text = header_a.text().collect::<String>();
                    debug!("[stampworld] Raw block header HTML: {}", header_a.html());
                    desc = text.clone();
                    // More flexible regex to handle newlines and various whitespace
                    let year_regex = regex::Regex::new(r"(\d{4})\s*(?:-\s*(\d{4}))?").unwrap();
                    if let Some(caps) = year_regex.captures(&desc) {
                        block_year = caps.get(1).map_or("", |m| m.as_str()).to_string();
                        if let Some(end_year_match) = caps.get(2) {
                            block_year = format!("{}-{}", block_year, end_year_match.as_str());
                        }
                        debug!("[stampworld] Extracted block_year '{}' from header: '{}'", block_year, desc);
                    } else {
                        debug!("[stampworld] No year found in header: '{}'", desc);
                    }
                }
                // Fallback: if block_year is empty, try to extract from details_vec or context_above
                if block_year.is_empty() {
                    let year_regex = regex::Regex::new(r"(\d{4})\s*(?:-\s*(\d{4}))?").unwrap();
                    for detail in &details_vec {
                        if let Some(caps) = year_regex.captures(detail) {
                            block_year = caps.get(1).map_or("", |m| m.as_str()).to_string();
                            if let Some(end_year_match) = caps.get(2) {
                                block_year = format!("{}-{}", block_year, end_year_match.as_str());
                            }
                            desc = detail.clone();
                            debug!("[stampworld] Extracted block_year '{}' from detail: '{}'", block_year, detail);
                            break;
                        }
                    }
                    if block_year.is_empty() {
                        if let Some(caps) = year_regex.captures(&context_above) {
                            block_year = caps.get(1).map_or("", |m| m.as_str()).to_string();
                            if let Some(end_year_match) = caps.get(2) {
                                block_year = format!("{}-{}", block_year, end_year_match.as_str());
                            }
                            desc = context_above.clone();
                            debug!("[stampworld] Extracted block_year '{}' from context_above: '{}'", block_year, context_above);
                        }
                    }
                    if block_year.is_empty() {
                        block_year = default_year.clone();
                        desc = default_year.clone();
                    }
                }
                desc.clone()
            };
            // For filtering, use the start year from block_year (handle ranges)
            let block_year_int = if block_year.contains('-') {
                block_year.split('-').next().unwrap_or("").trim().parse::<i32>().unwrap_or(0)
            } else {
                block_year.trim().parse::<i32>().unwrap_or(0)
            };
               debug!("[stampworld] Extracted block_year='{}', block_year_int={}", block_year, block_year_int);
            // Only stop if the entire block is way outside the range (more than 10 years past end_year)
            // This allows scraping of pages that contain some stamps outside the range
            if block_year_int > end_year + 10 {
                debug!("[stampworld] Block year {} is significantly > endYear {}. Stopping scrape.", block_year_int, end_year);
                done = true;
                break 'block_loop;
            }
            // Extract images from various possible selectors
            let mut images: Vec<String> = Vec::new();
            let mut image_alt_texts: Vec<String> = Vec::new();

            // Try multiple selectors for images, avoiding generic icons
            let image_selectors = vec![
                ".images_container img",  // Most specific - images in stamp containers
                ".stamp_img img",         // Direct stamp image selector  
                ".stamp_img_js img",      // JavaScript-enabled stamp images
            ];

            for selector_str in &image_selectors {
                if let Ok(selector) = scraper::Selector::parse(selector_str) {
                    // Look for images in the entire document
                    let all_imgs: Vec<_> = document.select(&selector).collect();
                    // debug!("Selector '{}' found {} total img elements", selector_str, all_imgs.len());

                    let found_images: Vec<(String, String)> = all_imgs
                        .into_iter()
                        .filter_map(|img| {
                            let src = img.value().attr("src")?;
                            let alt = img.value().attr("alt").unwrap_or("");
                            let full_url = if src.starts_with("http") {
                                src.to_string()
                            } else if src.starts_with("//") {
                                format!("https:{}", src)
                            } else if src.starts_with("/") {
                                format!("https://www.stampworld.com{}", src)
                            } else {
                                format!("https://www.stampworld.com/{}", src)
                            };

                            // Only log if it's a valid stamp image
                            if !full_url.is_empty()
                                && (full_url.contains("stamps") || full_url.contains("Stamps") || full_url.contains(".jpg") || full_url.contains(".png"))
                                && !full_url.contains("stamp.gif")
                                && !full_url.contains("layout")
                                && !full_url.contains("static")
                                && !full_url.contains(".gif")
                                && (full_url.contains(".jpg") || full_url.contains(".jpeg") || full_url.contains(".png")) {
                                // debug!("Found valid image URL: {} with alt: {}", full_url, alt);
                            }

                            Some((full_url, alt.to_string()))
                        })
                        .filter(|(url, _)| !url.is_empty()
                            && (url.contains("stamps") || url.contains("Stamps"))
                            && !url.contains("stamp.gif")
                            && !url.contains("layout")
                            && !url.contains("static")
                            && !url.contains(".gif")
                        )
                        .collect();

                    // debug!("Selector '{}' found {} valid stamp images", selector_str, found_images.len());

                    if !found_images.is_empty() {
                        // debug!("Using selector '{}' with {} images", selector_str, found_images.len());
                        for (url, alt) in found_images {
                            images.push(url);
                            image_alt_texts.push(alt);
                        }
                        break;
                    }
                } else {
                    // debug!("Failed to parse selector: {}", selector_str);
                }
            }

            debug!("Total images found for this block: {}", images.len());

            // Create a mapping of images to stamp rows by looking for images within each row
            let mut stamp_images: std::collections::HashMap<usize, (String, String)> = std::collections::HashMap::new();

            // Try to associate images with specific stamp rows
            let stamp_rows: Vec<_> = content_div.select(&scraper::Selector::parse("tr.stamp_tr").unwrap()).collect();
            for (row_idx, tr) in stamp_rows.iter().enumerate() {
                // Look for images within this specific stamp row
                let mut row_images = Vec::new();
                for selector_str in &["img", ".stamp_img img", ".stamp_img_js img"] {
                    if let Ok(selector) = scraper::Selector::parse(selector_str) {
                        let imgs_in_row: Vec<_> = tr.select(&selector).collect();
                        for img in imgs_in_row {
                            if let Some(src) = img.value().attr("src") {
                                let alt = img.value().attr("alt").unwrap_or("");
                                let full_url = if src.starts_with("http") {
                                    src.to_string()
                                } else if src.starts_with("//") {
                                    format!("https:{}", src)
                                } else if src.starts_with("/") {
                                    format!("https://www.stampworld.com{}", src)
                                } else {
                                    format!("https://www.stampworld.com/{}", src)
                                };

                                // Filter for valid stamp images
                                if !full_url.is_empty()
                                    && (full_url.contains("stamps") || full_url.contains("Stamps"))
                                    && !full_url.contains("stamp.gif")
                                    && !full_url.contains("layout")
                                    && !full_url.contains("static")
                                    && !full_url.contains(".gif")
                                    && (full_url.contains(".jpg") || full_url.contains(".jpeg") || full_url.contains(".png")) {
                                    row_images.push((full_url, alt.to_string()));
                                }
                            }
                        }
                    }
                }

                // If we found images in this row, associate the first one with this stamp
                if !row_images.is_empty() {
                    stamp_images.insert(row_idx, row_images[0].clone());
                    debug!("[stampworld] Associated image {} with stamp row {}: {}", row_images[0].0, row_idx, row_images[0].1);
                }
            }

            debug!("Associated {} images with specific stamp rows", stamp_images.len());
            // Table of stamps
            let table_selector = scraper::Selector::parse("tr.stamp_tr").unwrap();
            let mut stamp_row_count = 0;
            let mut image_idx = 0;
            let stamp_rows: Vec<_> = content_div.select(&table_selector).collect();
            println!("[INFO] Found {} stamp rows on page {}", stamp_rows.len(), page_num);
            
            debug!("Found {} stamp rows in table with selector 'tr.stamp_tr'", stamp_rows.len());
            // Removed detailed row debug output to reduce log size
            for tr in stamp_rows {
                stamp_row_count += 1;
                let th_selector = scraper::Selector::parse("th").unwrap();
                let ths: Vec<_> = tr.select(&th_selector).collect();
                let td_selector = scraper::Selector::parse("td").unwrap();
                let tds: Vec<_> = tr.select(&td_selector).collect();
                if ths.is_empty() || tds.len() < 13 { 
                    // debug!("Skipping row: ths.len()={}, tds.len()={}", ths.len(), tds.len());
                    continue; 
                }
                // debug!("Raw stamp row HTML: {}", tr.html());
                // Mapping based on provided example:
                // th[0] = No. (sg_number)
                // td[0] = Type
                // td[1] = Denomination
                // td[2] = (blank)
                // td[3] = Color
                // td[4] = Description
                // td[5] = Stamps Issued
                // td[6] = (blank)
                // td[7] = Mint Condition
                // td[8] = Unused
                // td[9] = Used
                // td[10] = Letter/FDC
                // td[11] = Currency
                let sg_number = ths[0].text().collect::<String>().trim().to_string();
                // Extract stamp type from data-stamp-type attribute
                let stamp_type_attr = tr.value().attr("data-stamp-type").unwrap_or("");
                let type_ = if !stamp_type_attr.is_empty() {
                    stamp_type_attr.to_string()
                } else {
                    // Fallback to td[0] text if attribute is missing
                    tds[0].text().collect::<String>().trim().to_string()
                };
                let denomination = tds[1].text().collect::<String>().trim().to_string();
                let color = tds[3].text().collect::<String>().trim().to_string();
                let description = tds[5].text().collect::<String>().trim().to_string();
                let stamps_issued = tds[6].text().collect::<String>().trim().to_string();
                // Clean up stamps_issued by removing parentheses
                let stamps_issued = stamps_issued.trim_start_matches('(').trim_end_matches(')').to_string();
                debug!("[stampworld] td[5] (desc): '{}' | td[6] (stamps_issued): '{}'", description, stamps_issued);
                let mint_condition = tds[8].text().collect::<String>().trim().to_string();
                let unused = tds[9].text().collect::<String>().trim().to_string();
                let used = tds[10].text().collect::<String>().trim().to_string();
                let letter_fdc = tds[11].text().collect::<String>().trim().to_string();
                let currency_raw = if tds.len() > 12 { tds[12].text().collect::<String>().trim().to_string() } else { String::new() };
                // Extract currency from the dedicated currency column first
                let mut currency = currency_raw.clone();
                
                // If currency column is empty, try to extract from denomination field (fallback)
                if currency.is_empty() {
                    let denomination_upper = denomination.to_uppercase();
                    if denomination_upper.contains("$") || denomination_upper.contains("USD") || denomination_upper.contains("DOLLAR") {
                        currency = "USD".to_string();
                    } else if denomination_upper.contains("£") || denomination_upper.contains("GBP") || denomination_upper.contains("POUND") {
                        currency = "GBP".to_string();
                    } else if denomination_upper.contains("€") || denomination_upper.contains("EUR") || denomination_upper.contains("EURO") {
                        currency = "EUR".to_string();
                    } else if denomination_upper.contains("C") && !denomination_upper.contains("C.") && !denomination_upper.contains("CT") {
                        // Canadian cents (C) - common in Canadian stamps
                        currency = "CAD".to_string();
                    } else if denomination_upper.contains("¢") || denomination_upper.contains("CENT") {
                        // Generic cents - could be USD or CAD
                        currency = "USD".to_string();
                    }
                }

                // Final fallback: look for currency in any table cell
                if currency.is_empty() {
                    for td in &tds {
                        let td_text = td.text().collect::<String>().replace('\u{a0}', " ").trim().to_uppercase();
                        if td_text.contains("USD") || td_text.contains("$") {
                            currency = "USD".to_string();
                            break;
                        } else if td_text.contains("GBP") || td_text.contains("£") {
                            currency = "GBP".to_string();
                            break;
                        } else if td_text.contains("EUR") || td_text.contains("€") {
                            currency = "EUR".to_string();
                            break;
                        }
                    }
                }
                // Assign image from stamp_images mapping or fallback to global images list
                let (image_url, alt_text) = if let Some((url, alt)) = stamp_images.get(&(stamp_row_count - 1)) {
                    debug!("[stampworld] Assigned associated image to stamp {}: {} (alt: {})", sg_number, url, alt);
                    (url.clone(), alt.clone())
                } else if image_idx < images.len() {
                    let url = images[image_idx].clone();
                    let alt = image_alt_texts.get(image_idx).unwrap_or(&String::new()).clone();
                    image_idx += 1;
                    debug!("[stampworld] Assigned fallback image {} to stamp {}: {} (alt: {})", image_idx-1, sg_number, url, alt);
                    (url, alt)
                } else {
                    debug!("[stampworld] No image available for stamp {} (row={}, image_idx={}, images.len()={})", sg_number, stamp_row_count, image_idx, images.len());
                    (String::new(), String::new())
                };
                debug!("[stampworld] STAMP_ROW: table_index={} sg_number='{}' year='{}' image_url='{}' alt_text='{}'", table_index, sg_number, block_year, image_url, alt_text);
                debug!("FIELDS: sg_number='{}' type='{}' denomination='{}' color='{}' description='{}' mint_condition='{}' unused='{}' used='{}' currency='{}'", sg_number, type_, denomination, color, description, mint_condition, unused, used, currency);
                // Extract per-stamp year from the series (prefer series year over individual stamp data)
                let mut stamp_year = block_year_int;
                let year_re = regex::Regex::new(r"(18|19|20)\d{2}").unwrap();
                
                // First try to extract year from the series name
                if let Some(cap) = year_re.find(&proper_series) {
                    stamp_year = cap.as_str().parse::<i32>().unwrap_or(block_year_int);
                    debug!("[stampworld] Extracted year {} from series: '{}'", stamp_year, proper_series);
                } else {
                    // Fallback: try to extract from individual stamp row
                    let mut found_stamp_year = false;
                    for idx in 0..tds.len() {
                        let text = tds[idx].text().collect::<String>();
                        if let Some(cap) = year_re.find(&text) {
                            stamp_year = cap.as_str().parse::<i32>().unwrap_or(block_year_int);
                            found_stamp_year = true;
                            debug!("[stampworld] Extracted year {} from stamp row: '{}'", stamp_year, text);
                            break;
                        }
                    }
                    
                    // Final fallback: try description
                    if !found_stamp_year {
                        if let Some(cap) = year_re.find(&description) {
                            stamp_year = cap.as_str().parse::<i32>().unwrap_or(block_year_int);
                            debug!("[stampworld] Extracted year {} from description: '{}'", stamp_year, description);
                        }
                    }
                }
                        if stamp_year < start_year && stamp_year > 0 {
                            debug!("[stampworld] Skipping stamp: sg_number='{}' stamp_year={} (before startYear)", sg_number, stamp_year);
                            continue;
                        }
                        if stamp_year > end_year && stamp_year > 0 {
                            debug!("[stampworld] Stamp {} has year {} > endYear {}, but continuing to check other stamps on this page", sg_number, stamp_year, end_year);
                            // Don't stop scraping here - continue to check other stamps on the page
                            // Only skip this individual stamp
                            continue;
                        }
                        debug!("[stampworld] STAMP: stamp_year={} context_above='{}' context_details='{}' image_url='{}'", stamp_year, context_above, context_details, image_url);
                        // Only download image if stamp_year is within range
                        let mut final_image_name = String::new();
                        let original_image_url; // Declare here, assign later
                        if image_url.is_empty() {
                            debug!("[stampworld] Skipping image download: image_url is empty for stamp {}", sg_number);
                        } else if stamp_year >= start_year && stamp_year <= end_year {
                            if !image_url.is_empty() {
                                // Check if image already exists and we're not overwriting
                                let raw_image_name = image_url.split('/').last().unwrap_or("stampworld.jpg");
                                let sanitized_image_name = sanitize_filename(raw_image_name);
                                let image_path = format!("{}/{}", _images_directory, sanitized_image_name);
                                let original_image_exists = std::path::Path::new(&image_path).exists();
                                
                                // Check if any versioned files exist for this base filename
                                let base_filename = sanitized_image_name.to_string();
                                let path = std::path::Path::new(&base_filename);
                                let stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
                                let extension = path.extension().unwrap_or_default().to_string_lossy().to_string();
                                
                                let mut existing_versioned_file = None;
                                let mut highest_version = 0;
                                
                                // Look for existing versioned files
                                for entry in std::fs::read_dir(_images_directory).unwrap_or_else(|_| std::fs::read_dir(".").unwrap()) {
                                    if let Ok(entry) = entry {
                                        let filename = entry.file_name().to_string_lossy().to_string();
                                        if filename.starts_with(&stem) && filename.ends_with(&extension) {
                                            if filename == base_filename {
                                                existing_versioned_file = Some(filename);
                                                break; // Original file exists, use it
                                            } else if let Some(version_part) = filename.strip_prefix(&format!("{}_", stem)) {
                                                if let Some(version_str) = version_part.strip_suffix(&format!(".{}", extension)) {
                                                    if let Ok(version) = version_str.parse::<i32>() {
                                                        if version > highest_version {
                                                            highest_version = version;
                                                            existing_versioned_file = Some(filename);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                let image_already_exists = !overwrite_images && (original_image_exists || existing_versioned_file.is_some());

                                // Check if any stamp in database already uses this base filename
                                let existing_count_sql = "SELECT COUNT(*) FROM stamps WHERE original_image_url LIKE ?1";
                                let mut stmt = conn.prepare(existing_count_sql).unwrap();
                                let existing_count: i64 = stmt.query_row(params![format!("{}%", base_filename)], |row| row.get(0)).unwrap_or(0);
                                
                                // Generate unique filename if this base filename is already used by another stamp
                                let unique_filename = if existing_count > 0 {
                                    let mut counter = 1;
                                    loop {
                                        let candidate_name = if extension.is_empty() {
                                            format!("{}_{}", stem, counter)
                                        } else {
                                            format!("{}_{}.{}", stem, counter, extension)
                                        };
                                        
                                        // Check if this versioned name is already used
                                        let versioned_count_sql = "SELECT COUNT(*) FROM stamps WHERE original_image_url = ?1";
                                        let mut version_stmt = conn.prepare(versioned_count_sql).unwrap();
                                        let versioned_count: i64 = version_stmt.query_row(params![candidate_name.clone()], |row| row.get(0)).unwrap_or(0);
                                        
                                        if versioned_count == 0 {
                                            break candidate_name;
                                        }
                                        counter += 1;
                                        
                                        // Prevent infinite loop
                                        if counter > 1000 {
                                            break format!("{}_{}", base_filename, counter);
                                        }
                                    }
                                } else {
                                    base_filename.clone()
                                };

                                if image_already_exists && !overwrite_images {
                                    debug!("[stampworld] Skipping image download for stamp {} - image already exists: {}", sg_number, image_path);
                                    // Use existing versioned file if available, otherwise use the generated unique filename
                                    final_image_name = existing_versioned_file.unwrap_or(unique_filename);
                                } else {
                                    // Download with the original filename first
                                    let image_status = get_stampworld_image(&image_url, overwrite_images);
                                    
                                    // If download was successful and we have a unique filename, rename the file
                                    if image_status.exists && unique_filename != image_status.filename {
                                        use std::fs;
                                        let original_path = format!("{}/{}", _images_directory, image_status.filename);
                                        let unique_path = format!("{}/{}", _images_directory, unique_filename);
                                        
                                        if let Err(e) = fs::rename(&original_path, &unique_path) {
                                            error!("Failed to rename image from {} to {}: {}", original_path, unique_path, e);
                                            final_image_name = image_status.filename; // Fall back to original
                                        } else {
                                            info!("Renamed image from {} to {}", image_status.filename, unique_filename);
                                            final_image_name = unique_filename;
                                        }
                                    } else {
                                        final_image_name = image_status.filename;
                                    }
                                    
                                    if image_status.exists {
                                        // println!("✓ Downloaded: {}", sg_number);
                                    } else {
                                        // println!("✗ Failed: {}", sg_number);
                                    }
                                }
                            }
                        } else {
                            debug!("[stampworld] Skipping image download: stamp_year {} not in range {}-{} for stamp {}", stamp_year, start_year, end_year, sg_number);
                        }

                        // Update original_image_url to use only the filename
                        original_image_url = if !final_image_name.is_empty() {
                            final_image_name.clone()
                        } else if !image_url.is_empty() {
                            sanitize_filename(image_url.split('/').last().unwrap_or(""))
                        } else {
                            String::new()
                        };


                // Month/Day extraction: look for various date patterns
                let mut month = String::new();
                let mut day = String::new();

                // Try different date patterns
                let date_patterns = vec![
                    r"(\d{1,2})\.\s*([A-Za-z]+)",  // "3. June"
                    r"([A-Za-z]+)\s+(\d{1,2})",    // "June 3"
                    r"(\d{1,2})\s+([A-Za-z]+)",   // "3 June"
                    r"([A-Za-z]+)\s+(\d{1,2})",   // "June 3"
                ];

                for detail in &details_vec {
                    for pattern in &date_patterns {
                        let re = regex::Regex::new(pattern).unwrap();
                        if let Some(cap) = re.captures(detail) {
                            let part1 = cap.get(1).map_or("", |m| m.as_str());
                            let part2 = cap.get(2).map_or("", |m| m.as_str());

                            // Determine which is day and which is month
                            if part1.chars().all(char::is_numeric) {
                                day = part1.to_string();
                                month = part2.to_string();
                            } else {
                                month = part1.to_string();
                                day = part2.to_string();
                            }
                            break;
                        }
                    }
                    if !month.is_empty() { break; }
                }

                // Also check description and other fields for dates
                if month.is_empty() {
                    let all_text = format!("{} {} {}", description, stamps_issued, denomination);
                    for pattern in &date_patterns {
                        let re = regex::Regex::new(pattern).unwrap();
                        if let Some(cap) = re.captures(&all_text) {
                            let part1 = cap.get(1).map_or("", |m| m.as_str());
                            let part2 = cap.get(2).map_or("", |m| m.as_str());

                            if part1.chars().all(char::is_numeric) {
                                day = part1.to_string();
                                month = part2.to_string();
                            } else {
                                month = part1.to_string();
                                day = part2.to_string();
                            }
                            break;
                        }
                    }
                }

                if month.is_empty() {
                    // fallback: try to find month name in details_vec
                    for detail in &details_vec {
                        for m in &["January","February","March","April","May","June","July","August","September","October","November","December"] {
                            if detail.contains(m) {
                                month = m.to_string();
                                break;
                            }
                        }
                        if !month.is_empty() { break; }
                    }
                }
                debug!("EXTRACTED: watermark='{}' design='{}' engraving='{}' perforation='{}'",
                       watermark, design, engraving, perforation);

                let local_image_path = if !image_url.is_empty() && !final_image_name.is_empty() {
                    let image_path = format!("images/{}", final_image_name);

                    // Check if this is a valid image format
                    let path_lower = image_path.to_lowercase();
                    if path_lower.ends_with(".jpg") || path_lower.ends_with(".jpeg") || path_lower.ends_with(".png") {
                        image_path
                    } else {
                        // Invalid format, generate per-stamp placeholder including SG/year
                        match placeholder_image::ensure_placeholder_image_for_stamp(_site_name, &sg_number, &stamp_year.to_string()) {
                            Ok(p) => p,
                            Err(_) => placeholder_image::get_placeholder_image_path(),
                        }
                    }
                } else {
                    // No image URL available, generate per-stamp placeholder including SG/year
                    match placeholder_image::ensure_placeholder_image_for_stamp(_site_name, &sg_number, &stamp_year.to_string()) {
                        Ok(p) => p,
                        Err(_) => placeholder_image::get_placeholder_image_path(),
                    }
                };

                // Generate remote image URL based on StampWorld's pattern
                let remote_image_url = if !image_url.is_empty() {
                    image_url
                } else {
                    // Try to construct a plausible remote URL based on StampWorld's pattern
                    let type_fallback = if type_.is_empty() { "unknown" } else { &type_ };
                    format!("https://www.stampworld.com/media/catalogue/{}/Postage-stamps/{}-s.jpg", _site_name, type_fallback)
                };
                let cleaned_context_details = context_details
                    .lines()
                    .filter(|line| !line.trim().is_empty() && line.trim() != "\u{200b}")
                    .collect::<Vec<_>>()
                    .join(" | ");

                debug!("[stampworld] PRE-INSERT: sg_number='{}' year='{}' watermark='{}' design='{}' engraving='{}' perforation='{}'", sg_number, stamp_year, watermark, design, engraving, perforation);

                debug!("INSERTING: sg_number='{}' type='{}' series='{}' (from desc: '{}') local_image_path='{}' remote_image_url='{}'", sg_number, type_, proper_series, desc, local_image_path, remote_image_url);

                let result = crate::db_utils::insert_stamp(
                    &conn,
                    _site_name, // country
                    &proper_series,      // series (fixed)
                    stamp_year, // year as i32
                    &month,     // month
                    &day,       // day
                    &sg_number,
                    &type_,
                    &denomination,
                    &color,
                    &clean_text(&description),
                    &stamps_issued,
                    &mint_condition,
                    &unused,
                    &used,
                    &letter_fdc,
                    &currency,
                    &local_image_path,
                    &remote_image_url,
                    &original_image_url, // original_image_url
                    &clean_text(&watermark),
                    &clean_text(&design),
                    &clean_text(&engraving),
                    &clean_text(&perforation),
                    &cleaned_context_details,
                );
                match result {
                    Ok(_) => {
                        println!("[INFO] Saved stamp to database: {} ({}) - {}", sg_number, stamp_year, clean_text(&description));
                        debug!("Inserted stamp into DB: {} {} {}", _site_name, desc, stamp_year);
                    }
                    Err(e) => error!("Failed to insert stamp into DB: {} {} {} | Error: {}", _site_name, desc, stamp_year, e),
                }
                // Build the stamp data line for the extract file
                let stamp_line = format!("STAMP;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                    stamp_year, month, day, desc, sg_number, type_, denomination, color, clean_text(&description), stamps_issued, mint_condition, unused, used, letter_fdc, currency, local_image_path, remote_image_url, clean_text(&watermark), clean_text(&design), clean_text(&engraving), clean_text(&perforation), cleaned_context_details);
                all_stamp_data.push(stamp_line);
            }
            debug!("[stampworld] Processed {} stamp rows in this block.", stamp_row_count);
               debug!("[stampworld] About to check stop_scraping: stop_scraping={}", stop_scraping);
               if stop_scraping {
                   debug!("[stampworld] Stopping further page fetches due to stamp_year > endYear.");
                   break 'block_loop;
               }
               debug!("[stampworld] Finished stop_scraping check for this block.");
            // End of for content_div
        }
        let next_page_selector = scraper::Selector::parse("a[href*='page=']").unwrap();
        let next_page_count = document.select(&next_page_selector).count();
        debug!("Found {} next_page links", next_page_count);
        let mut last_page = false;
        if next_page_count == 0 {
            last_page = true;
        }
        if last_page {
            done = true;
        } else {
            page_num += 1;
        }
    } // End of while !done
    debug!("Total stamp data lines collected: {}", all_stamp_data.len());
    all_stamp_data
}
