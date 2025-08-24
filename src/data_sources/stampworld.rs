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
        for content_div in document.select(&content_selector) {
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
            // Extract year from block header
            let year_month_day_elem = content_div.select(&scraper::Selector::parse("a[href]").unwrap()).next();
            let mut block_year = String::new();
            let mut desc = String::new();
            if let Some(a_elem) = year_month_day_elem {
                let text = a_elem.text().collect::<String>();
                desc = text.clone();
                if text.len() > 4 && text.chars().nth(7).map(|c| !c.is_digit(10)).unwrap_or(false) {
                    desc = text[5..].to_string();
                }
                // Extract year or year range from desc using regex
                let desc_trimmed = desc.trim();
                let year_regex = regex::Regex::new(r"(\d{4})(?:\s*-\s*(\d{4}))?").unwrap();
                if let Some(caps) = year_regex.captures(desc_trimmed) {
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
            if block_year_int > end_year {
                done = true;
                break;
            }
            // Table of stamps
            let table_selector = scraper::Selector::parse("tr.stamp_tr ").unwrap();
            // Extract all block-level stamp images
            let block_img_selector = scraper::Selector::parse("img").unwrap();
            let block_images: Vec<String> = content_div
                .select(&block_img_selector)
                .filter_map(|img_elem| {
                    if let Some(src) = img_elem.value().attr("src") {
                        if src.contains("/media/catalogue/") {
                            Some(if src.starts_with("http") {
                                src.to_string()
                            } else {
                                format!("https://www.stampworld.com{}", src)
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            let mut img_index = 0;
            for tr in content_div.select(&table_selector) {
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
                let description = tds[5].text().collect::<String>().trim().to_string(); // stamps_issued column is actually description
                let stamps_issued = String::new(); // not present
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
                println!("[DEBUG] FIELDS: sg_number='{}' type='{}' denomination='{}' color='{}' description='{}' mint_condition='{}' unused='{}' used='{}' currency='{}'", sg_number, type_, denomination, color, description, mint_condition, unused, used, currency);
                let year = block_year.clone();
                let filter_year = if year.contains('-') {
                    year.split('-').next().unwrap_or("").trim().parse::<i32>().unwrap_or(0)
                } else {
                    year.trim().parse::<i32>().unwrap_or(0)
                };
                if filter_year < start_year || filter_year > end_year { continue; }
                let image_url = if img_index < block_images.len() {
                    let url = block_images[img_index].clone();
                    img_index += 1;
                    url
                } else {
                    String::new()
                };
                println!("[DEBUG] STAMP: year={} context_above='{}' context_details='{}' image_url='{}'", year, context_above, context_details, image_url);
                let image_status = get_stampworld_image(&image_url, overwrite);

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
                let cleaned_context_details = context_details
                    .lines()
                    .filter(|line| !line.trim().is_empty() && line.trim() != "\u{200b}")
                    .collect::<Vec<_>>()
                    .join(" | ");

                let result = crate::db_utils::insert_stamp(
                    &conn,
                    _site_name, // country
                    &desc,      // series
                    filter_year, // year as i32
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
                    &image_url,
                    &watermark,
                    &design,
                    &engraving,
                    &perforation,
                    &cleaned_context_details,
                );
                match result {
                    Ok(_) => println!("[DEBUG] Inserted stamp into DB: {} {} {}", _site_name, desc, filter_year),
                    Err(e) => println!("[ERROR] Failed to insert stamp into DB: {} {} {} | Error: {}", _site_name, desc, filter_year, e),
                }
            }
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
        } else {
            last_page = true;
        }
        if last_page {
            done = true;
        } else {
            page_num += 1;
        }
    }
    all_stamp_data
}
