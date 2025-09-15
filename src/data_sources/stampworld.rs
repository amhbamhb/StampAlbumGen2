// Stampworld data source handler
use log::{debug, error};
use crate::globals::ImageStatus;
use crate::text_utils::{clean_text, sanitize_filename};
use crate::file_utils::get_db_path_from_cli;

/// Determine the proper series name based on stamp type
fn determine_series_from_type(stamp_type: &str, block_desc: &str) -> String {
    if stamp_type.is_empty() {
        return block_desc.to_string();
    }
    
    // Extract series letter from stamp type
    let series_letter = stamp_type.chars().next().unwrap_or(' ').to_ascii_uppercase();
    
    match series_letter {
        'L' => format!("{}-series Registration Stamps", series_letter),
        'M' => format!("{}-series Definitive Stamps", series_letter),
        'N' => format!("{}-series Definitive Stamps", series_letter),
        'O' => format!("{}-series Registration Stamps", series_letter),
        'P' => format!("{}-series Registration Stamps", series_letter),
        'Q' => format!("{}-series Registration Stamps", series_letter),
        'R' => format!("{}-series Special Delivery Stamps", series_letter),
        'S' => format!("{}-series Special Delivery Stamps", series_letter),
        'T' => format!("{}-series Tax Stamps", series_letter),
        'U' => format!("{}-series Utility Stamps", series_letter),
        _ => {
            // For unknown series, try to extract from block description
            if block_desc.contains("Registration") {
                format!("{}-series Registration Stamps", series_letter)
            } else if block_desc.contains("Special Delivery") {
                format!("{}-series Special Delivery Stamps", series_letter)
            } else if block_desc.contains("Definitive") {
                format!("{}-series Definitive Stamps", series_letter)
            } else {
                block_desc.to_string()
            }
        }
    }
}

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
        let mut details_vec = Vec::new();
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
        

        
        'block_loop: for content_div in document.select(&content_selector) {
            debug!("Processing table with HTML: {}", content_div.html().chars().take(200).collect::<String>());
            let stop_scraping = false;
            // Improved context extraction: look for preceding header tags or text nodes
            let context_above = String::new();
            let context_details = details_vec.join(" | ");
            // Extract year from block header using .table_header a within this table
            let table_header_selector = scraper::Selector::parse(".table_header a").unwrap();
            let mut block_year = String::new();
            let mut desc = String::new();
            if let Some(header_a) = content_div.select(&table_header_selector).next() {
                let text = header_a.text().collect::<String>();
                debug!("[stampworld] Raw block header HTML: {}", header_a.html());
                desc = text.clone();
                let year_regex = regex::Regex::new(r"(\d{4})(?:\s*-\s*(\d{4}))?").unwrap();
                if let Some(caps) = year_regex.captures(&desc) {
                    block_year = caps.get(1).map_or("", |m| m.as_str()).to_string();
                    if let Some(end_year_match) = caps.get(2) {
                        block_year = format!("{}-{}", block_year, end_year_match.as_str());
                    }
                }
            }
            // Fallback: if block_year is empty, try to extract from details_vec or context_above
            if block_year.is_empty() {
                let year_regex = regex::Regex::new(r"(\d{4})(?:\s*-\s*(\d{4}))?").unwrap();
                for detail in &details_vec {
                    if let Some(caps) = year_regex.captures(detail) {
                        block_year = caps.get(1).map_or("", |m| m.as_str()).to_string();
                        if let Some(end_year_match) = caps.get(2) {
                            block_year = format!("{}-{}", block_year, end_year_match.as_str());
                        }
                        desc = detail.clone();
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
                    }
                }
                if block_year.is_empty() {
                    block_year = default_year.clone();
                    desc = default_year.clone();
                }
            }
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

            // Try multiple selectors for images, avoiding generic icons
            let image_selectors = vec![
                ".images_container img",
                "img[src*='stamps']",
                "img[src*='Stamps']",
                ".stamp-image img",
                "td img",
                "img",  // Try all images as a fallback
            ];

            for selector_str in &image_selectors {
                if let Ok(selector) = scraper::Selector::parse(selector_str) {
                    let all_imgs: Vec<_> = content_div.select(&selector).collect();
                    debug!("Selector '{}' found {} total img elements", selector_str, all_imgs.len());

                    let found_images: Vec<String> = all_imgs
                        .into_iter()
                        .filter_map(|img| {
                            let src = img.value().attr("src")?;
                            let full_url = if src.starts_with("http") {
                                src.to_string()
                            } else if src.starts_with("//") {
                                format!("https:{}", src)
                            } else if src.starts_with("/") {
                                format!("https://www.stampworld.com{}", src)
                            } else {
                                format!("https://www.stampworld.com/{}", src)
                            };

                            // Log each image URL found
                            debug!("Found image URL: {}", full_url);

                            Some(full_url)
                        })
                        .filter(|url| !url.is_empty()
                            && (url.contains("stamps") || url.contains("Stamps") || url.contains(".jpg") || url.contains(".png"))
                            && !url.contains("stamp.gif")  // Exclude generic stamp icon
                            && !url.contains("layout")     // Exclude layout images
                            && !url.contains("static")     // Exclude static assets
                            && !url.contains(".gif")       // Exclude GIF files
                            && (url.contains(".jpg") || url.contains(".jpeg") || url.contains(".png")) // Only valid formats
                        )
                        .collect();

                    debug!("Selector '{}' found {} valid stamp images: {:?}", selector_str, found_images.len(), found_images);

                    if !found_images.is_empty() {
                        debug!("Using selector '{}' with {} images", selector_str, found_images.len());
                        images.extend(found_images);
                        break; // Use the first selector that finds images
                    }
                } else {
                    debug!("Failed to parse selector: {}", selector_str);
                }
            }

            debug!("Total images found: {}", images.len());

            // Debug: Show a sample of the HTML content to understand structure
            if images.is_empty() {
                let html_sample = content_div.html();
                let sample_len = html_sample.len().min(2000);
                debug!("HTML content sample (first {} chars): {}", sample_len, &html_sample[..sample_len]);

                // Also try to find any img tags at all
                if let Ok(all_img_selector) = scraper::Selector::parse("img") {
                    let all_imgs: Vec<_> = content_div.select(&all_img_selector)
                        .filter_map(|img| img.value().attr("src"))
                        .collect();
                    debug!("Found {} total img tags with src attributes: {:?}", all_imgs.len(), all_imgs);
                }
                
                // Look for any elements that might contain stamp images
                let stamp_img_selectors = vec![
                    ".stamp_img",
                    ".stamp-image", 
                    "[class*='stamp']",
                    "[alt*='stamp']",
                    "[alt*='Stamp']",
                    "span[class*='img']",
                    ".img-fluid"
                ];
                
                for selector_str in &stamp_img_selectors {
                    if let Ok(selector) = scraper::Selector::parse(selector_str) {
                        let elements: Vec<_> = content_div.select(&selector).collect();
                        if !elements.is_empty() {
                            debug!("Found {} elements with selector '{}'", elements.len(), selector_str);
                            for (i, elem) in elements.iter().enumerate().take(3) {
                                debug!("Element {}: {}", i, elem.html());
                            }
                        }
                    }
                }
            }
            // Table of stamps
            let table_selector = scraper::Selector::parse("tr.stamp_tr").unwrap();
            let mut stamp_row_count = 0;
            let mut image_idx = 0;
            let stamp_rows: Vec<_> = content_div.select(&table_selector).collect();
            println!("[INFO] Found {} stamp rows on page {}", stamp_rows.len(), page_num);
            
            debug!("Found {} stamp rows in table with selector 'tr.stamp_tr'", stamp_rows.len());
            for (i, tr) in stamp_rows.iter().enumerate() {
                debug!("Stamp row {}: class='{}', data-stamp-type='{}'", i, tr.value().attr("class").unwrap_or("no-class"), tr.value().attr("data-stamp-type").unwrap_or("no-type"));
            }
            for tr in stamp_rows {
                stamp_row_count += 1;
                let th_selector = scraper::Selector::parse("th").unwrap();
                let ths: Vec<_> = tr.select(&th_selector).collect();
                let td_selector = scraper::Selector::parse("td").unwrap();
                let tds: Vec<_> = tr.select(&td_selector).collect();
                if ths.is_empty() || tds.len() < 10 { 
                    debug!("Skipping row: ths.len()={}, tds.len()={}", ths.len(), tds.len());
                    continue; 
                }
                debug!("Raw stamp row HTML: {}", tr.html());
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
                debug!("[stampworld] td[5] (desc): '{}' | td[6] (stamps_issued): '{}'", description, stamps_issued);
                let mint_condition = tds[8].text().collect::<String>().trim().to_string();
                let unused = tds[9].text().collect::<String>().trim().to_string();
                let used = tds[10].text().collect::<String>().trim().to_string();
                let letter_fdc = tds[11].text().collect::<String>().trim().to_string();
                // Extract currency from denomination field (often contains currency symbols)
                let mut currency = String::new();
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

                // Fallback: look for currency in any table cell
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
                // Assign image from images list by order
                let image_url = if image_idx < images.len() {
                    let url = images[image_idx].clone();
                    image_idx += 1;
                    url
                } else {
                    String::new()
                };
                debug!("[stampworld] STAMP_ROW: sg_number='{}' year='{}' image_url='{}'", sg_number, block_year, image_url);
                debug!("FIELDS: sg_number='{}' type='{}' denomination='{}' color='{}' description='{}' mint_condition='{}' unused='{}' used='{}' currency='{}'", sg_number, type_, denomination, color, description, mint_condition, unused, used, currency);
                // Extract per-stamp year from the stamp row (prefer td[4] or td[5] if it contains a year)
        // Extract per-stamp year from the stamp row (prefer td[4] or td[5] if it contains a year)
        let mut stamp_year = block_year_int;
        let year_re = regex::Regex::new(r"(19|20)\\d{2}").unwrap();
        let mut found_stamp_year = false;
        for idx in 0..tds.len() {
            let text = tds[idx].text().collect::<String>();
            if let Some(cap) = year_re.find(&text) {
                stamp_year = cap.as_str().parse::<i32>().unwrap_or(block_year_int);
                found_stamp_year = true;
                break;
            }
        }
                       if !found_stamp_year {
            if let Some(cap) = year_re.find(&description) {
                stamp_year = cap.as_str().parse::<i32>().unwrap_or(block_year_int);
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
                        if image_url.is_empty() {
                            debug!("[stampworld] Skipping image download: image_url is empty");
                        } else if stamp_year >= start_year && stamp_year <= end_year {
                            println!("[INFO] Processing image for stamp {} ({})", sg_number, stamp_year);
                            debug!("[stampworld] Attempting image download: image_url='{}'", image_url);
                            let image_status = get_stampworld_image(&image_url, overwrite_images);
                            if image_status.exists {
                                println!("[INFO] Image processed successfully for stamp {}", sg_number);
                            } else {
                                println!("[WARN] Failed to process image for stamp {}", sg_number);
                            }
                            debug!("[stampworld] Image download result: {:?}", image_status);
                        } else {
                            debug!("[stampworld] Skipping image download: stamp_year {} not in range {}-{}", stamp_year, start_year, end_year);
                        }


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

                let local_image_path = if !image_url.is_empty() {
                    let raw_image_name = image_url.split('/').last().unwrap_or("");
                    let sanitized_image_name = sanitize_filename(raw_image_name);
                    let image_path = format!("images/{}", sanitized_image_name);

                    // Check if this is a valid image format
                    let path_lower = image_path.to_lowercase();
                    if path_lower.ends_with(".jpg") || path_lower.ends_with(".jpeg") || path_lower.ends_with(".png") {
                        image_path
                    } else {
                        // Invalid format, use fallback based on stamp type
                        let type_fallback = if type_.is_empty() { "unknown" } else { &type_ };
                        format!("images/{}-s.jpg", sanitize_filename(type_fallback))
                    }
                } else {
                    // Generate predictable filename from stamp type (A, A1, A2, etc.)
                    let type_fallback = if type_.is_empty() { "unknown" } else { &type_ };
                    format!("images/{}-s.jpg", sanitize_filename(type_fallback))
                };

                // Generate remote image URL based on StampWorld's pattern
                let remote_image_url = if !image_url.is_empty() {
                    image_url
                } else {
                    // Try to construct a plausible remote URL based on StampWorld's pattern
                    let type_fallback = if type_.is_empty() { "unknown" } else { &type_ };
                    format!("https://www.stampworld.com/media/catalogue/{}/Postage-stamps/{}-s.jpg", _site_name, sanitize_filename(type_fallback))
                };
                let cleaned_context_details = context_details
                    .lines()
                    .filter(|line| !line.trim().is_empty() && line.trim() != "\u{200b}")
                    .collect::<Vec<_>>()
                    .join(" | ");

                debug!("[stampworld] PRE-INSERT: sg_number='{}' year='{}' watermark='{}' design='{}' engraving='{}' perforation='{}'", sg_number, stamp_year, watermark, design, engraving, perforation);
                
                // Determine proper series based on stamp type
                let proper_series = determine_series_from_type(&type_, &desc);
                
                debug!("INSERTING: sg_number='{}' type='{}' local_image_path='{}' remote_image_url='{}'", sg_number, type_, local_image_path, remote_image_url);
                
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
