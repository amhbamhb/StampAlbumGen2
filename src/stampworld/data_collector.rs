// StampWorld data collection module
use log::{debug, error, info};
use crate::types::ImageStatus;
use crate::album_utils::clean_text;
use crate::album_utils::sanitize_filename;
use crate::file_utils::get_db_path_from_cli;
use crate::placeholder_image;

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
    // Create a client with browser-like headers to reduce the chance of being served
    // an error/placeholder page by StampWorld when requests appear to come from a bot.
    let mut default_headers = reqwest::header::HeaderMap::new();
    default_headers.insert(
        reqwest::header::USER_AGENT,
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36".parse().unwrap(),
    );
    default_headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        "en-US,en;q=0.9".parse().unwrap(),
    );
    default_headers.insert(
        reqwest::header::REFERER,
        "https://www.google.com/".parse().unwrap(),
    );
    // Add a few more common headers to mimic a real browser
    default_headers.insert(
        reqwest::header::ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".parse().unwrap(),
    );
    // Intentionally do not set an explicit Accept-Encoding header here so
    // reqwest can advertise the encodings it actually supports and will
    // transparently decode responses (e.g., gzip/deflate). Setting an
    // explicit header including 'br' can lead to servers returning brotli
    // compressed bodies when reqwest doesn't support brotli, producing
    // binary files on disk.
    // 'Connection: keep-alive' header—reqwest may set it automatically, but include for completeness
    default_headers.insert(
        reqwest::header::HeaderName::from_static("connection"),
        "keep-alive".parse().unwrap(),
    );

    // Build client with cookie store enabled and redirects allowed to follow server's redirects
    let client = reqwest::blocking::Client::builder()
        .default_headers(default_headers)
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .expect("Failed to build reqwest client");

    // Initialize SQLite database in the album directory for the country
    // Use the CLI-provided db_path for consistency
    let db_path = get_db_path_from_cli();
    let conn = crate::db_utils::init_db(&db_path).expect("Failed to initialize database");
    
    // If overwrite is true, clear existing data
    if overwrite_images {
        debug!("Overwrite flag is true, clearing existing stamp data");
        match conn.execute("DELETE FROM stamps", []) {
            Ok(affected) => info!("Cleared existing stamp data: {} rows removed", affected),
            Err(e) => error!("Failed to clear existing data: {}", e),
        }
    }

    while !done {
        if page_num > MAX_PAGES {
            debug!("Reached maximum pages limit: {}", MAX_PAGES);
            break;
        }
        // Normalize base_url and ensure the page query is formed as '/stamps/?page=N' when appropriate.
        // Common incoming shapes: 'https://.../stamps/', 'https://.../stamps', or already containing '?page='
        let url = if base_url.contains("?page=") || base_url.contains("&page=") {
            // User already supplied a URL with paging; append page_num directly if there's a placeholder
            if base_url.contains("{page}") {
                base_url.replace("{page}", &page_num.to_string())
            } else {
                // If URL already has page=, assume caller wants explicit page parameter control; try to append if missing
                format!("{}{}", base_url, page_num)
            }
        } else {
            // Ensure there's a trailing slash before '?page=' for sites that expect '/stamps/?page='
            if base_url.ends_with('/') {
                format!("{}?page={}", base_url, page_num)
            } else {
                // If path ends without slash and looks like a directory (no query), add '/?page='
                // But avoid duplicating when base_url already contains a query string
                if base_url.contains('?') {
                    format!("{}&page={}", base_url, page_num)
                } else {
                    // Prefer the canonical '/.../stamps/?page=N' that StampWorld redirects to
                    format!("{}/?page={}", base_url, page_num)
                }
            }
        };
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
        // Log Content-Encoding to help debug whether the server returned a
        // compressed body that reqwest decoded or not.
        let content_encoding = resp.headers().get(reqwest::header::CONTENT_ENCODING)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("(none)");
        debug!("Response Content-Encoding: {}", content_encoding);

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
        // Try a list of possible selectors to locate the content blocks. Sites change
        // structure frequently, so prefer the first selector that returns results.
        let possible_selectors = vec![
            "div.catalogue-container",
            "div.content_table",
            "table.table.data_table",
            "table.data_table",
            "tr.stamp_tr",
            "div.catalogue-list",
            "div.catalogue",
            ".catalogue-list",
            ".stamp-list",
            ".stampContainer",
            "div.search_results",
            "[class*='stamp']",
            "[class*='catalogue']",
            "[class*='listing']",
        ];

        let mut chosen_selector_str = "table.data_table";
        for sel_str in &possible_selectors {
            if let Ok(sel) = scraper::Selector::parse(sel_str) {
                if document.select(&sel).count() > 0 {
                    chosen_selector_str = sel_str;
                    break;
                }
            }
        }

        let content_selector = scraper::Selector::parse(chosen_selector_str).unwrap();
        info!("[stampworld] Using content selector: '{}'", chosen_selector_str);
        let content_count = document.select(&content_selector).count();
        info!("[stampworld] Found {} content blocks with selector '{}'", content_count, chosen_selector_str);
        if content_count == 0 {
            info!("[stampworld] No content blocks found with any of the tried selectors. Checking a short list of alternates at info level:");
            let alternates = vec![
                "div.catalogue-list",
                ".stamp-list",
                ".stampContainer",
                "div.search_results",
                "table.data_table",
            ];
            for alt in alternates.iter() {
                if let Ok(sel2) = scraper::Selector::parse(alt) {
                    let c2 = document.select(&sel2).count();
                    info!("[stampworld] Alternate selector '{}' matched {} blocks", alt, c2);
                }
            }
            // Save the raw HTML for debugging so we can inspect the DOM and tune selectors
            let debug_path = format!("{}/stampworld_debug_{}_page_{}.html", std::env::current_dir().unwrap().display(), _site_name.replace(' ', "_").to_lowercase(), page_num);
            // Quick substring counts for well-known markers so diagnostics are readable
            let found_tr_stamp_tr = body.matches("tr class=\"stamp_tr").count();
            let found_data_table = body.matches("data_table").count();
            info!("[stampworld] Raw marker counts: tr.stamp_tr={}, occurrences of 'data_table'={}", found_tr_stamp_tr, found_data_table);
            // Force-save the decoded text body (overwrite previous file) so it's readable for debugging
            match std::fs::write(&debug_path, body.as_bytes()) {
                Ok(_) => info!("[stampworld] Saved debug HTML (text) to: {}", debug_path),
                Err(e) => error!("[stampworld] Failed to save debug HTML: {}", e),
            }
            debug!("No content blocks found with any of the tried selectors. Listing table classes for debugging:");
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

                    if !found_images.is_empty() {
                        for (url, alt) in found_images {
                            images.push(url);
                            image_alt_texts.push(alt);
                        }
                        break;
                    }
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
            for tr in stamp_rows {
                stamp_row_count += 1;
                let th_selector = scraper::Selector::parse("th").unwrap();
                let ths: Vec<_> = tr.select(&th_selector).collect();
                let td_selector = scraper::Selector::parse("td").unwrap();
                let tds: Vec<_> = tr.select(&td_selector).collect();
                if ths.is_empty() || tds.len() < 13 { 
                    continue; 
                }
                let sg_number = ths[0].text().collect::<String>().trim().to_string();

                // If the SG cell represents a numeric range or group (e.g., "23-25", "45‑52", "29‑44")
                // then skip this row entirely — these are set/group placeholders, not individual stamps.
                // Match patterns like: digits hyphen digits, possibly using different hyphen characters.
                let range_re = regex::Regex::new(r"^\s*\d+\s*[-\u{2011}\u{2013}\u{2012}\u{2010}]\s*\d+\s*$").unwrap();
                if range_re.is_match(&sg_number) {
                    debug!("Skipping SG range/group row: sg_number='{}'", sg_number);
                    continue;
                }
                let stamp_type_attr = tr.value().attr("data-stamp-type").unwrap_or("");
                let type_ = if !stamp_type_attr.is_empty() {
                    stamp_type_attr.to_string()
                } else {
                    tds[0].text().collect::<String>().trim().to_string()
                };
                let denomination = tds[1].text().collect::<String>().trim().to_string();
                let color = tds[3].text().collect::<String>().trim().to_string();
                let description = tds[5].text().collect::<String>().trim().to_string();
                let stamps_issued = tds[6].text().collect::<String>().trim().to_string();
                let stamps_issued = stamps_issued.trim_start_matches('(').trim_end_matches(')').to_string();
                debug!("[stampworld] td[5] (desc): '{}' | td[6] (stamps_issued): '{}'", description, stamps_issued);
                let mint_condition = tds[8].text().collect::<String>().trim().to_string();
                let unused = tds[9].text().collect::<String>().trim().to_string();
                let used = tds[10].text().collect::<String>().trim().to_string();
                let letter_fdc = tds[11].text().collect::<String>().trim().to_string();
                let currency_raw = if tds.len() > 12 { tds[12].text().collect::<String>().trim().to_string() } else { String::new() };
                let mut currency = currency_raw.clone();
                if currency.is_empty() {
                    let denomination_upper = denomination.to_uppercase();
                    if denomination_upper.contains("$") || denomination_upper.contains("USD") || denomination_upper.contains("DOLLAR") {
                        currency = "USD".to_string();
                    } else if denomination_upper.contains("£") || denomination_upper.contains("GBP") || denomination_upper.contains("POUND") {
                        currency = "GBP".to_string();
                    } else if denomination_upper.contains("€") || denomination_upper.contains("EUR") || denomination_upper.contains("EURO") {
                        currency = "EUR".to_string();
                    } else if denomination_upper.contains("C") && !denomination_upper.contains("C.") && !denomination_upper.contains("CT") {
                        currency = "CAD".to_string();
                    } else if denomination_upper.contains("¢") || denomination_upper.contains("CENT") {
                        currency = "USD".to_string();
                    }
                }
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

                // Prefer images found within this specific row and try to match by stamp type
                let mut selected_image_url = String::new();
                let mut selected_alt = String::new();

                let mut row_imgs: Vec<(String, String)> = Vec::new();
                for selector_str in &["img", ".stamp_img img", ".stamp_img_js img"] {
                    if let Ok(selector) = scraper::Selector::parse(selector_str) {
                        for img in tr.select(&selector) {
                            if let Some(src) = img.value().attr("src") {
                                let alt = img.value().attr("alt").unwrap_or("").to_string();
                                let full_url = if src.starts_with("http") {
                                    src.to_string()
                                } else if src.starts_with("//") {
                                    format!("https:{}", src)
                                } else if src.starts_with("/") {
                                    format!("https://www.stampworld.com{}", src)
                                } else {
                                    format!("https://www.stampworld.com/{}", src)
                                };
                                if !full_url.is_empty() && (full_url.contains("stamps") || full_url.contains("Stamps")) {
                                    row_imgs.push((full_url, alt));
                                }
                            }
                        }
                    }
                }

                if !row_imgs.is_empty() {
                    let type_upper = type_.to_uppercase();
                    let mut found = None;
                    for (u, a) in &row_imgs {
                        let a_upper = a.to_uppercase();
                        if a_upper.contains(&type_upper) || a_upper.contains(&format!("TYPE {}", type_upper)) {
                            found = Some((u.clone(), a.clone()));
                            break;
                        }
                    }
                    if let Some((u,a)) = found {
                        selected_image_url = u;
                        selected_alt = a;
                    } else {
                        selected_image_url = row_imgs[0].0.clone();
                        selected_alt = row_imgs[0].1.clone();
                    }
                } else if image_idx < images.len() {
                    // Try to pick a better-matching global image when available.
                    // Problem: the global images vector ordering doesn't always match stamp rows.
                    // Heuristic: if the current series (proper_series) or type_ mentions specific
                    // tokens (e.g. "Maple Leaf" / type P), prefer a global image whose alt or
                    // filename contains those tokens. Otherwise fall back to the next image.
                    let mut chosen_index: Option<usize> = None;
                    let series_upper = proper_series.to_uppercase();
                    let type_upper = type_.to_uppercase();

                    for idx in image_idx..images.len() {
                        let alt = image_alt_texts.get(idx).cloned().unwrap_or_default().to_uppercase();
                        let url = images.get(idx).cloned().unwrap_or_default().to_uppercase();

                        // Match by obvious series keywords first
                        if series_upper.contains("MAPLE") || series_upper.contains("MAPLE LEAF") {
                            if alt.contains("MAPLE") || url.contains("/P-") || url.contains("/P1-") || url.contains("/P2-") || url.contains("/P3-") {
                                chosen_index = Some(idx);
                                break;
                            }
                        }

                        // Match by stamp type token if present (e.g. type 'P' or 'P1')
                        if !type_upper.is_empty() {
                            // match 'TYPE P' in alt or '/P' in filename
                            if alt.contains(&type_upper) || url.contains(&format!("/{}-", type_upper)) {
                                chosen_index = Some(idx);
                                break;
                            }
                        }
                    }

                    if let Some(fi) = chosen_index {
                        selected_image_url = images[fi].clone();
                        selected_alt = image_alt_texts.get(fi).unwrap_or(&String::new()).clone();
                        // mark as consumed so future rows don't accidentally reuse the same slot
                        images[fi] = String::new();
                        image_alt_texts[fi] = String::new();
                        image_idx = fi + 1;
                    } else {
                        selected_image_url = images[image_idx].clone();
                        selected_alt = image_alt_texts.get(image_idx).unwrap_or(&String::new()).clone();
                        image_idx += 1;
                    }
                } else {
                    debug!("[stampworld] No image available for stamp {} (row={}, image_idx={}, images.len()={})", sg_number, stamp_row_count, image_idx, images.len());
                }

                let (image_url, alt_text) = (selected_image_url.clone(), selected_alt.clone());
                // Diagnostic audit: if we're processing StampWorld page 2, append a CSV line
                // with the chosen image and available candidates to help diagnose wrong-image selection.
                if page_num == 2 {
                    use std::fs::OpenOptions;
                    use std::io::Write;
                    let audit_path = "/tmp/stampworld_page2_audit.csv";
                    let need_header = !std::path::Path::new(audit_path).exists();
                    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(audit_path) {
                        if need_header {
                            let _ = writeln!(f, "page,table_index,sg_number,selected_image_url,selected_alt,row_images,global_images,image_idx");
                        }
                        // Serialize row_images and images vectors into small strings
                        let row_imgs_serialized = {
                            // gather any row-local images we found (row_imgs is in scope)
                            let mut parts: Vec<String> = Vec::new();
                            for (_ridx, (u,a)) in row_imgs.iter().enumerate() {
                                let u_s = u.replace(',', ";");
                                let a_s = a.replace(',', ";");
                                parts.push(format!("{}|{}", u_s, a_s));
                            }
                            parts.join(";;")
                        };
                        let global_imgs_serialized = {
                            let mut parts: Vec<String> = Vec::new();
                            for (i, u) in images.iter().enumerate() {
                                let u_s = u.replace(',', ";");
                                let alt = image_alt_texts.get(i).cloned().unwrap_or_default().replace(',', ";");
                                parts.push(format!("{}|{}", u_s, alt));
                            }
                            parts.join(";;")
                        };
                        let sel_u = image_url.replace(',', ";");
                        let sel_a = alt_text.replace(',', ";");
                        let _ = writeln!(f, "{},{},{},{},{},{},{},{}", page_num, table_index, sg_number.replace(',', ";"), sel_u, sel_a, row_imgs_serialized, global_imgs_serialized, image_idx);
                    }
                }
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
                                
                                // Simple behavior: use the sanitized base filename as-is and rely on the
                                // downloader's overwrite/checksum behavior to avoid unwanted overwrites.
                                // Do not attempt to generate versioned filenames or rename files on disk.
                                let base_filename = sanitized_image_name.to_string();
                                let image_already_exists = original_image_exists && !overwrite_images;

                                if image_already_exists {
                                    debug!("[stampworld] Skipping image download for stamp {} - image already exists: {}", sg_number, image_path);
                                    final_image_name = base_filename.clone();
                                } else {
                                    // Download with the original filename (downloader will handle checksum/overwrite)
                                    let image_status = get_stampworld_image(&image_url, overwrite_images);

                                    // Create a per-row unique filename by prefixing sanitized sg_number
                                    if image_status.exists {
                                        // sanitize sg_number to use in filenames
                                        let sg_safe = sanitize_filename(&sg_number.replace('\u{2011}', "-").replace('\u{2013}', "-").replace(' ', "_"));
                                        let target_name = format!("{}-{}", sg_safe, image_status.filename);
                                        let target_path = format!("{}/{}", _images_directory, target_name);
                                        let downloaded_path = format!("{}/{}", _images_directory, image_status.filename);
                                        // If target doesn't exist, copy the downloaded file to the target unique name
                                        if !std::path::Path::new(&target_path).exists() {
                                            match std::fs::copy(&downloaded_path, &target_path) {
                                                Ok(_) => {
                                                    debug!("Copied downloaded image to per-row filename: {}", target_name);
                                                    final_image_name = target_name;
                                                }
                                                Err(e) => {
                                                    error!("Failed to copy downloaded image {} to {}: {}", downloaded_path, target_path, e);
                                                    final_image_name = image_status.filename;
                                                }
                                            }
                                        } else {
                                            // already exists, use it
                                            final_image_name = target_name;
                                        }
                                    } else {
                                        final_image_name = image_status.filename;
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

                // If this block or stamp row is a "Set" placeholder (e.g. "Set (* Stamp not included in this set)")
                // skip inserting these rows into the DB. They are not real stamps.
                if cleaned_context_details.contains("Set (") || description.contains("Set (") {
                    info!("Skipping insertion for placeholder 'Set' row: sg_number='{}' series='{}'", sg_number, proper_series);
                    continue;
                }

                debug!("[stampworld] PRE-INSERT: sg_number='{}' year='{}' watermark='{}' design='{}' engraving='{}' perforation='{}'", sg_number, stamp_year, watermark, design, engraving, perforation);

                debug!("INSERTING: sg_number='{}' type='{}' series='{}' (from desc: '{}') local_image_path='{}' remote_image_url='{}'", sg_number, type_, proper_series, desc, local_image_path, remote_image_url);

                let result = crate::db_utils::insert_stamp(
                    &conn,
                    _site_name, // country
                    &proper_series,      // series (fixed)
                    stamp_year, // year as i32
                    &month,     // month
                    &_month_number(&month), // month_num (derived)
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
