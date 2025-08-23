// Stampworld data source handler
use crate::album_globals::{ImageStatus};

pub fn collect_stampworld_data(
    sequencer: &mut i32,
    separator: &str,
    _site_name: &str,
    _images_directory: &str,
    month_number: &dyn Fn(&str) -> String,
    get_stampworld_image: &dyn Fn(&str) -> ImageStatus,
    base_url: &str,
) -> Vec<String> {
    println!("[DEBUG] collect_stampworld_data called with base_url: {}", base_url);
    let mut all_stamp_data = Vec::new();
    let mut done = false;
    let mut page_num = 1;
    let client = reqwest::blocking::Client::new();
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
        // Check for page count logic
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
        // Improved image scraping: find all <img> tags with src containing /media/catalogue/
        let img_selector = scraper::Selector::parse("img").unwrap();
        for img_elem in document.select(&img_selector) {
            if let Some(src) = img_elem.value().attr("src") {
                if src.contains("/media/catalogue/") {
                    let image_url = if src.starts_with("http") {
                        src.to_string()
                    } else {
                        format!("https://www.stampworld.com{}", src)
                    };
                    let image_status = get_stampworld_image(&image_url);
                    // Optionally, you can add more metadata here if needed
                    let stamp_line = format!("STAMP{sep}{image_link}{sep}{image_exists}{sep}{image_width}{sep}{image_height}",
                        sep=separator,
                        image_link=image_status.link,
                        image_exists=image_status.exists,
                        image_width=image_status.width,
                        image_height=image_status.height,
                    );
                    all_stamp_data.push(stamp_line);
                }
            }
        }
        if last_page {
            done = true;
        } else {
            page_num += 1;
        }
    }
    all_stamp_data
}
