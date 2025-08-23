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
    let mut all_stamp_data = Vec::new();
    let mut done = false;
    let mut page_num = 1;
    let client = reqwest::blocking::Client::new();
    let mut reached_end_year = false;
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
        let mut all_block_years_past_end = true;
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
            for tr in content_div.select(&table_selector) {
                let td_selector = scraper::Selector::parse("td").unwrap();
                let tds: Vec<_> = tr.select(&td_selector).collect();
                if tds.len() < 6 { continue; }
                let stamp_id = tds[0].text().collect::<String>().trim().to_string();
                // Try to extract year from the row, fallback to block year
                let year = block_year.clone();
                // For filtering, use the start year from block_year (handle ranges)
                let filter_year = if year.contains('-') {
                    year.split('-').next().unwrap_or("").trim().parse::<i32>().unwrap_or(0)
                } else {
                    year.trim().parse::<i32>().unwrap_or(0)
                };
                if filter_year < start_year || filter_year > end_year { continue; }
                // Extract image URL from img tag in the row
                let img_selector = scraper::Selector::parse("img").unwrap();
                let mut image_url = String::new();
                for td in &tds {
                    if let Some(img_elem) = td.select(&img_selector).next() {
                        if let Some(src) = img_elem.value().attr("src") {
                            if src.contains("/media/catalogue/") {
                                image_url = if src.starts_with("http") {
                                    src.to_string()
                                } else {
                                    format!("https://www.stampworld.com{}", src)
                                };
                                break;
                            }
                        }
                    }
                }
                println!("[DEBUG] STAMP: year={} context_above='{}' context_details='{}' image_url='{}'", year, context_above, context_details, image_url);
                let image_status = get_stampworld_image(&image_url, overwrite);
                let value_text = tds[1].text().collect::<String>();
                let mut split_idx = value_text.len();
                for (i, c) in value_text.chars().rev().enumerate() {
                    if !c.is_digit(10) && c != '\u{00bc}' && c != '\u{00bd}' && c != '\u{00be}' {
                        split_idx = value_text.len() - i - 1;
                        break;
                    }
                }
                let value_main = value_text[..split_idx + 1].to_string();
                let value_rest = value_text[split_idx + 1..].trim().to_string();
                let field14 = tds[5].text().collect::<String>();
                let field17 = tds[3].text().collect::<String>();
                let field18 = "0".to_string();
                let field19 = "0".to_string();
                let image_link = image_status.link;
                let image_exists = image_status.exists;
                let image_width = image_status.width.to_string();
                let image_height = image_status.height.to_string();
                let catalog_code = format!("Sw:{}", stamp_id);
                *sequencer += 1;
                let seq_str = format!("{:07}", *sequencer);
                let stamp_line = format!(
                    "STAMP{sep}{year}{sep}{desc}{sep}{context_above}{sep}{context_details}{sep}{seq}{sep}{field14}{sep}{value_main}{sep}{value_rest}{sep}{field17}{sep}{field18}{sep}{field19}{sep}{catalog_code}{sep}{image_link}{sep}{image_exists}{sep}{image_width}{sep}{image_height}",
                    sep=separator,
                    year=year,
                    desc=desc,
                    context_above=context_above,
                    context_details=context_details,
                    seq=seq_str,
                    field14=field14,
                    value_main=value_main,
                    value_rest=value_rest,
                    field17=field17,
                    field18=field18,
                    field19=field19,
                    catalog_code=catalog_code,
                    image_link=image_link,
                    image_exists=image_exists,
                    image_width=image_width,
                    image_height=image_height,
                );
                all_stamp_data.push(stamp_line);
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
