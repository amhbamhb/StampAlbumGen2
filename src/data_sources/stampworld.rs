// Stampworld data source handler
use crate::album_globals::{ImageStatus};

pub fn collect_stampworld_data(
    sequencer: &mut i32,
    separator: &str,
    _site_name: &str,
    _images_directory: &str,
    month_number: &dyn Fn(&str) -> String,
    get_stampworld_image: &dyn Fn(&str, bool) -> ImageStatus,
    base_url: &str,
    start_year: i32,
    end_year: i32,
    overwrite: bool,
) -> Vec<String> {
    println!("[DEBUG] collect_stampworld_data called with base_url: {}", base_url);
    let all_stamp_data = Vec::new();
    let mut done = false;
    let mut page_num = 1;
    let client = reqwest::blocking::Client::new();
    let reached_end_year = false;

    // Initialize SQLite database in the album directory for the country
    let country_db_name = _site_name.replace("/", "-");
    let db_path = format!("{}/{}.db", _images_directory.replace("/images", ""), country_db_name);
    let conn = crate::db_utils::init_db(&db_path).expect("Failed to initialize database");

    while !done {
        let url = format!("{}{}", base_url, page_num);
        println!("[DEBUG] Fetching Stampworld page: {}", url);
        let resp = match client.get(&url).timeout(std::time::Duration::from_secs(60)).send() {
            Ok(r) => r,
            Err(e) => {
                if page_num == 1 {
                    println!("Please check that you have entered the correct site country name");
                } else {
                    println!("Problem retrieving web page: {}", e);
                }
                std::process::exit(1);
            }
        };
        let body = match resp.text() {
            Ok(b) => b,
            Err(e) => {
                println!("Problem retrieving web page text: {}", e);
                std::process::exit(1);
            }
        };
        let document = scraper::Html::parse_document(&body);
        let content_selector = scraper::Selector::parse("div.container-fluid.content_table").unwrap();
        let all_block_years_past_end = true;
        'block_loop: for content_div in document.select(&content_selector) {
            let mut stop_scraping = false;
            let mut all_stamps_past_end_year = true;
            // Improved context extraction: look for preceding header tags or text nodes
            let mut context_above = String::new();
            let mut context_details = String::new();
            if let Some(parent) = content_div.parent() {
                let mut prev = parent.prev_sibling();
                while let Some(node) = prev {
                    if let Some(text_node) = node.value().as_text() {
                        if !text_node.trim().is_empty() {
                            context_above = text_node.trim().to_string();
                            break;
                        }
                    }
                    prev = node.prev_sibling();
                }
            }
            // Try to get details from the first few <p> tags in the block
            let p_selector = scraper::Selector::parse("p").unwrap();
            let mut details_vec = Vec::new();
            for p in content_div.select(&p_selector) {
                let p_text = p.text().collect::<String>();
                if !p_text.is_empty() {
                    details_vec.push(p_text);
                }
            }
            if !details_vec.is_empty() {
                context_details = details_vec.join(" | ");
            }
            // Extract year from block header using .table_header a
            let table_header_selector = scraper::Selector::parse(".table_header a").unwrap();
            let mut block_year = String::new();
            let mut desc = String::new();
            if let Some(header_a) = content_div.select(&table_header_selector).next() {
                let text = header_a.text().collect::<String>();
                println!("[DEBUG][stampworld] Raw block header HTML: {}", header_a.html());
                desc = text.clone();
                let year_regex = regex::Regex::new(r"(\d{4})(?:\s*-\s*(\d{4}))?").unwrap();
                if let Some(caps) = year_regex.captures(&desc) {
                    block_year = caps.get(1).map_or("", |m| m.as_str()).to_string();
                    if let Some(end_year_match) = caps.get(2) {
                        block_year = format!("{}-{}", block_year, end_year_match.as_str());
                    }
                }
            }
            // For filtering, use the start year from block_year (handle ranges)
            let block_year_int = if block_year.contains('-') {
                block_year.split('-').next().unwrap_or("").trim().parse::<i32>().unwrap_or(0)
            } else {
                block_year.trim().parse::<i32>().unwrap_or(0)
            };
               println!("[DEBUG][stampworld] Extracted block_year='{}', block_year_int={}", block_year, block_year_int);
            if block_year_int > end_year {
                println!("[DEBUG][stampworld] Block year {} > endYear {}. Stopping scrape.", block_year_int, end_year);
                done = true;
                break 'block_loop;
            }
            // Extract images from .images_container
            let images_selector = scraper::Selector::parse(".images_container img").unwrap();
            let images: Vec<String> = content_div.select(&images_selector)
                .map(|img| img.value().attr("src").map(|src| if src.starts_with("http") { src.to_string() } else { format!("https://www.stampworld.com{}", src) }).unwrap_or_default())
                .collect();
            // Table of stamps
            let table_selector = scraper::Selector::parse("tr.stamp_tr").unwrap();
            let mut stamp_row_count = 0;
            let mut image_idx = 0;
            for tr in content_div.select(&table_selector) {
                stamp_row_count += 1;
                let th_selector = scraper::Selector::parse("th").unwrap();
                let ths: Vec<_> = tr.select(&th_selector).collect();
                let td_selector = scraper::Selector::parse("td").unwrap();
                let tds: Vec<_> = tr.select(&td_selector).collect();
                if ths.is_empty() || tds.len() < 12 { continue; }
                println!("[DEBUG] Raw stamp row HTML: {}", tr.html());
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
                let type_ = tds[0].text().collect::<String>().trim().to_string();
                let denomination = tds[1].text().collect::<String>().trim().to_string();
                let color = tds[3].text().collect::<String>().trim().to_string();
                let description = tds[5].text().collect::<String>().trim().to_string();
                let stamps_issued = if tds.len() > 7 { tds[7].text().collect::<String>().trim().to_string() } else { String::new() };
                let mint_condition = tds[7].text().collect::<String>().trim().to_string();
                let unused = tds[9].text().collect::<String>().trim().to_string(); // used column is actually unused
                let used = tds[10].text().collect::<String>().trim().to_string(); // letter_fdc column is actually used
                let letter_fdc = String::new(); // not present
                let mut currency = String::new();
                for td in &tds {
                    let td_text = td.text().collect::<String>().replace('\u{a0}', " ").trim().to_uppercase();
                    if td_text.contains("USD") {
                        currency = "USD".to_string();
                        println!("[DEBUG] Found currency cell: '{}', bytes: {:?}", td_text, td_text.as_bytes());
                        break;
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
                println!("[DEBUG][stampworld] STAMP_ROW: sg_number='{}' year='{}' image_url='{}'", sg_number, block_year, image_url);
                println!("[DEBUG] FIELDS: sg_number='{}' type='{}' denomination='{}' color='{}' description='{}' mint_condition='{}' unused='{}' used='{}' currency='{}'", sg_number, type_, denomination, color, description, mint_condition, unused, used, currency);
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
                        if stamp_year < start_year {
                            println!("[DEBUG][stampworld] Skipping stamp: sg_number='{}' stamp_year={} (before startYear)", sg_number, stamp_year);
                            continue;
                        }
                        if stamp_year > end_year {
                            println!("[DEBUG][stampworld] Found stamp_year > endYear ({} > {}). Stopping scrape.", stamp_year, end_year);
                            stop_scraping = true;
                            done = true;
                            break;
                        }
                        println!("[DEBUG][stampworld] STAMP: stamp_year={} context_above='{}' context_details='{}' image_url='{}'", stamp_year, context_above, context_details, image_url);
                        // Only download image if stamp_year is within range
                        if image_url.is_empty() {
                            println!("[DEBUG][stampworld] Skipping image download: image_url is empty");
                        } else if stamp_year >= start_year && stamp_year <= end_year {
                            println!("[DEBUG][stampworld] Attempting image download: image_url='{}'", image_url);
                            let image_status = get_stampworld_image(&image_url, overwrite);
                            println!("[DEBUG][stampworld] Image download result: {:?}", image_status);
                        } else {
                            println!("[DEBUG][stampworld] Skipping image download: stamp_year {} not in range {}-{}", stamp_year, start_year, end_year);
                        }


                // Month/Day extraction: look for patterns like "3. June" in details_vec
                let mut month = String::new();
                let mut day = String::new();
                for detail in &details_vec {
                    let re = regex::Regex::new(r"(\d{1,2})\.\s*([A-Za-z]+)").unwrap();
                    if let Some(cap) = re.captures(detail) {
                        day = cap.get(1).map_or("", |m| m.as_str()).to_string();
                        month = cap.get(2).map_or("", |m| m.as_str()).to_string();
                        break;
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
                // Extract watermark, design, engraving from details_vec
                let mut watermark = String::new();
                let mut design = String::new();
                let mut engraving = String::new();
                for detail in &details_vec {
                    if detail.contains("Watermark:") {
                        watermark = detail.split("Watermark:").nth(1).unwrap_or("").trim().to_string();
                    }
                    if detail.contains("WM") {
                        // Try to extract watermark from 'WM' pattern
                        let wm_re = regex::Regex::new(r"WM[:\s]+([A-Za-z0-9 ]+)").unwrap();
                        if let Some(cap) = wm_re.captures(detail) {
                            watermark = cap.get(1).map_or("", |m| m.as_str()).trim().to_string();
                        }
                    }
                    if detail.contains("Design:") {
                        design = detail.split("Design:").nth(1).unwrap_or("").trim().to_string();
                    }
                    if detail.contains("Engraving:") {
                        engraving = detail.split("Engraving:").nth(1).unwrap_or("").trim().to_string();
                    }
                }
                println!("[DEBUG] EXTRACTED: watermark='{}' design='{}' engraving='{}'", watermark, design, engraving);

                // Perforation extraction
                let mut perforation = String::new();
                for p in content_div.select(&p_selector) {
                    let p_text = p.text().collect::<String>();
                    if p_text.contains("Perforation:") {
                        perforation = p_text.split("Perforation:").nth(1).unwrap_or("").trim().to_string();
                        break;
                    }
                }

                let local_image_path = if !image_url.is_empty() {
                    let image_name = image_url.split('/').last().unwrap_or("");
                    format!("images/{}", image_name)
                } else {
                    String::new()
                };
                let remote_image_url = image_url.clone();
                let cleaned_context_details = context_details
                    .lines()
                    .filter(|line| !line.trim().is_empty() && line.trim() != "\u{200b}")
                    .collect::<Vec<_>>()
                    .join(" | ");

                println!("[DEBUG][stampworld] PRE-INSERT: sg_number='{}' year='{}' image_path='{}' remote_image_url='{}' watermark='{}' stamps_issued='{}'", sg_number, stamp_year, local_image_path, remote_image_url, watermark, stamps_issued);
                let result = crate::db_utils::insert_stamp(
                    &conn,
                    _site_name, // country
                    &desc,      // series
                    stamp_year, // year as i32
                    &month,     // month
                    &day,       // day
                    &sg_number,
                    &type_,
                    &denomination,
                    &color,
                    &description,
                    &stamps_issued,
                    &mint_condition,
                    &unused,
                    &used,
                    &letter_fdc,
                    &currency,
                    &local_image_path,
                    &remote_image_url,
                    &watermark,
                    &design,
                    &engraving,
                    &perforation,
                    &cleaned_context_details,
                );
                match result {
                    Ok(_) => println!("[DEBUG] Inserted stamp into DB: {} {} {}", _site_name, desc, stamp_year),
                    Err(e) => println!("[ERROR] Failed to insert stamp into DB: {} {} {} | Error: {}", _site_name, desc, stamp_year, e),
                }
            }
            println!("[DEBUG][stampworld] Processed {} stamp rows in this block.", stamp_row_count);
               println!("[DEBUG][stampworld] About to check stop_scraping: stop_scraping={}", stop_scraping);
               if stop_scraping {
                   println!("[DEBUG][stampworld] Stopping further page fetches due to stamp_year > endYear.");
                   break 'block_loop;
               }
               println!("[DEBUG][stampworld] Finished stop_scraping check for this block.");
            // End of for content_div
        }
        let top_bar_selector = scraper::Selector::parse("#topBlueBarGroupsCatalogLabel").unwrap();
        let mut last_page = false;
        if let Some(top_bar) = document.select(&top_bar_selector).next() {
            let label_text = top_bar.text().collect::<String>();
            if !label_text.is_empty() && label_text.chars().any(|c| c.is_digit(10)) {
                if let Some(idx) = label_text.find("page ") {
                    let page_info = &label_text[idx + 5..];
                    if let Some(end_idx) = page_info.find(")") {
                        let page_info = &page_info[..end_idx];
                        let parts: Vec<&str> = page_info.split('/').collect();
                        if parts.len() == 2 && parts[0] == parts[1] {
                            last_page = true;
                        }
                    }
                }
            } else {
                last_page = true;
            }
        }
        if last_page {
            done = true;
        } else {
            page_num += 1;
        }
    } // End of while !done
    all_stamp_data
}
