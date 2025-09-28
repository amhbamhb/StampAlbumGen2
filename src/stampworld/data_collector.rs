use log::{debug, error, info};
use crate::types::ImageStatus;
use scraper::{Html, Selector};
use regex::Regex;
use std::collections::HashSet;
use rusqlite::params;

// Normalize stamps_issued values and provide small parser helpers used by
// both runtime code and unit tests.
pub(crate) fn normalize_stamps_issued(s: &str) -> String {
    let mut v = s.trim().to_string();
    if v.starts_with('(') && v.ends_with(')') {
        v = v[1..v.len()-1].to_string();
    }
    // Remove any characters except digits, dot and comma
    let re_non = Regex::new(r"[^0-9\.,]").unwrap();
    v = re_non.replace_all(&v, "").to_string();
    // If dots are used as thousand separators and no commas are present, replace them with commas
    if v.contains('.') && !v.contains(',') {
        v = v.replace('.', ",");
    }
    v
}

// Remove duplicate detail lines while preserving order.
pub(crate) fn dedupe_details(details: &[String]) -> Vec<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for d in details {
        let t = d.trim().to_string();
        if t.is_empty() { continue; }
        if !seen.contains(&t) {
            seen.insert(t.clone());
            out.push(t);
        }
    }
    out
}

// Extract common block metadata fields from a list of detail lines.
// Returns (watermark, design, engraving, perforation)
pub(crate) fn extract_block_metadata(details: &[String]) -> (String, String, String, String) {
    let mut watermark = String::new();
    let mut design = String::new();
    let mut engraving = String::new();
    let mut perforation = String::new();
    for d in details {
        let low = d.to_lowercase();
        if (low.contains("wm:") || low.contains("watermark")) && watermark.is_empty() {
            if let Some(pos) = d.find(':') { watermark = d[pos+1..].trim().to_string(); }
            else { watermark = d.replace("WM", "").replace("wm", "").trim().to_string(); }
        }
        if low.contains("design:") && design.is_empty() {
            if let Some(pos) = d.find(':') { design = d[pos+1..].trim().to_string(); }
        }
        if low.contains("engraving:") && engraving.is_empty() {
            if let Some(pos) = d.find(':') { engraving = d[pos+1..].trim().to_string(); }
        }
        if (low.contains("perforation:") || low.contains("perf:" )|| low.contains("perf")) && perforation.is_empty() {
            if let Some(pos) = d.find(':') { perforation = d[pos+1..].trim().to_string(); }
            else {
                // fallback: try to pick a token that starts with digits or 'perf'
                for tok in d.split_whitespace() {
                    let tl = tok.to_lowercase();
                    if tl.starts_with("perf") || tl.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                        perforation = tok.trim().to_string();
                        break;
                    }
                }
            }
        }
    }
    // Final trimming and normalization
    watermark = watermark.trim().trim_matches(|c: char| c == ':' || c == '.').to_string();
    design = design.trim().trim_matches(|c: char| c == ':' || c == '.').to_string();
    engraving = engraving.trim().trim_matches(|c: char| c == ':' || c == '.').to_string();
    perforation = perforation.trim().trim_matches(|c: char| c == ':' || c == '.').to_string();
    (watermark, design, engraving, perforation)
}

// Compress a context string by removing consecutive duplicate tokens separated by '|'
// and truncate if it exceeds a reasonable length.
pub(crate) fn compress_context(s: &str, max_len: usize) -> String {
    let parts: Vec<&str> = s.split('|').map(|p| p.trim()).filter(|p| !p.is_empty()).collect();
    let mut out_parts: Vec<&str> = Vec::new();
    let mut last: Option<&str> = None;
    for p in parts.iter() {
        if last.map(|l| l == *p).unwrap_or(false) {
            continue;
        }
        out_parts.push(*p);
        last = Some(*p);
        // Quick break if we already exceeded max parts
        if out_parts.len() >= 20 { break; }
    }
    let mut out = out_parts.join(" | ");
    if out.len() > max_len {
        out.truncate(max_len);
        // avoid trailing partial token
        if let Some(pos) = out.rfind('|') { out.truncate(pos); }
    }
    out
}

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
            let p_selector = Selector::parse("p").unwrap();
            let mut details_vec: Vec<String> = Vec::new();
            for p in content_div.select(&p_selector) {
                let t = p.text().collect::<String>();
                if !t.trim().is_empty() { details_vec.push(t); }
            }
            // Deduplicate detail lines and produce a compact context string
            let deduped = dedupe_details(&details_vec);

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
                // (no-op) header debug removed — header_texts will be logged later when available
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

            // Build a header -> column index map for this table so we can robustly
            // extract fields like "Stamps Issued", "Mint Condition", "Unused", "Used", "Letter/FDC"
            use std::collections::HashMap;
            let mut col_map: HashMap<String, usize> = HashMap::new();
            let mut header_texts: Vec<String> = Vec::new();
            if let Some(table_node) = content_div.select(&Selector::parse("table").unwrap()).next() {
                if let Some(thead) = table_node.select(&Selector::parse("thead").unwrap()).next() {
                    for th in thead.select(&Selector::parse("th").unwrap()) {
                        let mut txt = th.text().collect::<String>();
                        txt = txt.trim().to_string();
                        if let Some(img) = th.select(&Selector::parse("img").unwrap()).next() {
                            if let Some(alt) = img.value().attr("alt") {
                                let a = alt.trim();
                                if !a.is_empty() { txt = a.to_string(); }
                            }
                        }
                        header_texts.push(txt.clone());
                    }
                    // Map names to header indices (we'll align these with row cells including th)
                    for (i, txt) in header_texts.iter().enumerate() {
                        let low = txt.to_lowercase();
                        if low.contains("stamps issued") || low.contains("released") { col_map.insert("stamps_issued".to_string(), i); }
                        if low.contains("mint condition") || low.contains("mint") { col_map.insert("mint_condition".to_string(), i); }
                        if low.contains("unused") { col_map.insert("unused".to_string(), i); }
                        if low.contains("used") { col_map.insert("used".to_string(), i); }
                        if low.contains("letter") || low.contains("fdc") { col_map.insert("letter_fdc".to_string(), i); }
                        if low.contains("currency") || low.contains("usd") { col_map.insert("currency".to_string(), i); }
                    }
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

                // Extract block-level metadata using the helper and produce a cleaned context
                let (watermark, design, engraving, perforation) = extract_block_metadata(&deduped);
                let raw_context = deduped.iter().filter(|l| !l.trim().is_empty() && l.trim() != "\u{200b}").map(|s| s.as_str()).collect::<Vec<_>>().join(" | ");
                let cleaned_context_details = compress_context(&raw_context, 1024);

                // Per-row metadata: description already captured; try to extract
                // stamps_issued, mint_condition, unused, used, letter_fdc, currency
                let mut mint_condition = String::new();
                let mut unused = String::new();
                let mut used = String::new();
                let mut letter_fdc = String::new();
                let mut currency = String::new();
                let mut stamps_issued = stamps_issued.clone();
                // Use header mapping when available to extract per-column values reliably.
                // The table rows include a leading <th> for the 'No.' column; to align headers
                // with row cells, build a flattened row_cells vector that starts with the row's th
                // (if present) followed by the row's tds. Then header index i maps to row_cells[i].
                if !col_map.is_empty() {
                    let mut row_cells: Vec<String> = Vec::new();
                    // include the row's th (catalog number) as first cell if present
                    if let Some(th_row) = tr.select(&Selector::parse("th").unwrap()).next() {
                        row_cells.push(th_row.text().collect::<String>().trim().to_string());
                    } else {
                        // ensure alignment: if header begins with No., push an empty placeholder for this row
                        if !header_texts.is_empty() {
                            let first = header_texts[0].to_lowercase();
                            if first.contains("no") || first.contains("no.") || first.contains("#") {
                                row_cells.push(String::new());
                            }
                        }
                    }
                    for td in &tds {
                        row_cells.push(td.text().collect::<String>().trim().to_string());
                    }
                    let get_col = |name: &str| -> String {
                        if let Some(&idx) = col_map.get(name) {
                            if idx < row_cells.len() {
                                return row_cells[idx].clone();
                            }
                        }
                        String::new()
                    };
                    if series_desc.contains("1860 -1862 Queen Victoria - Not Watermarked") && sg_number == "2" {
                        debug!("[debug-row] sg_number={} row_cells={:?}", sg_number, row_cells);
                    }
                    stamps_issued = get_col("stamps_issued");
                    mint_condition = get_col("mint_condition");
                    unused = get_col("unused");
                    used = get_col("used");
                    letter_fdc = get_col("letter_fdc");
                    currency = get_col("currency");
                    if currency.is_empty() {
                        // fallback: last cell
                        if let Some(last) = row_cells.last() { currency = last.clone(); }
                    }
                } else {
                    // fallback heuristics when header mapping isn't present
                    if tds.len() > 13 {
                        mint_condition = tds[9].text().collect::<String>().trim().to_string();
                        unused = tds[10].text().collect::<String>().trim().to_string();
                        used = tds[11].text().collect::<String>().trim().to_string();
                        letter_fdc = tds[12].text().collect::<String>().trim().to_string();
                        currency = tds[13].text().collect::<String>().trim().to_string();
                    } else if tds.len() > 9 {
                        mint_condition = tds.get(9).map(|n| n.text().collect::<String>().trim().to_string()).unwrap_or_default();
                        currency = tds.last().map(|n| n.text().collect::<String>().trim().to_string()).unwrap_or_default();
                    }
                }

                // Normalize stamps_issued: strip parentheses and non-digit/sep chars,
                // and convert '.' thousand separators to ',' per user request.
                let stamps_issued_norm = normalize_stamps_issued(&stamps_issued);

                // Call DB insert following the existing callers' parameter order
                info!("DB insert: country='{}' series='{}' sg='{}' year={} local_image='{}' remote='{}'",
                    site_name, series_name, sg_number, stamp_year, local_image_path, remote_image_url);
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
                    &stamps_issued_norm,
                    &mint_condition,
                    &unused,
                    &used,
                    &letter_fdc,
                    &currency,
                    &local_image_path,
                    &remote_image_url,
                    &original_image_url,
                    &watermark,
                    &design,
                    &engraving,
                    &perforation,
                    &cleaned_context_details,
                );
                match res {
                    Ok(inserted) => {
                        if inserted {
                            info!("Inserted new stamp: {} | {} | {}", site_name, series_name, sg_number);
                            new_stamps_this_page += 1;
                            seen_stamps.insert(key.clone());
                            let stamp_line = format!("STAMP;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                                stamp_year, month, day, series_name, sg_number, type_, denomination, color, description, stamps_issued, "", "", "", "", "", local_image_path, remote_image_url, original_image_url, "", "", "", "", cleaned_context_details);
                            all_stamp_data.push(stamp_line);
                        } else if overwrite_images {
                            // The row already exists and wasn't updated by insert_stamp (no placeholder).
                            // If --overwrite-images was requested, perform a targeted UPDATE to replace image fields
                            // and fill any missing metadata fields (but do not overwrite existing non-empty values).
                            info!("--overwrite-images set; attempting conditional UPDATE for {} | {} | {}", site_name, series_name, sg_number);
                            let update_sql = "UPDATE stamps SET
                                image_path = CASE WHEN (?1 != '') THEN ?1 ELSE image_path END,
                                remote_image_url = CASE WHEN (?2 != '') THEN ?2 ELSE remote_image_url END,
                                original_image_url = CASE WHEN (?3 != '') THEN ?3 ELSE original_image_url END,
                                description = CASE WHEN (description IS NULL OR description = '') AND (?4 != '') THEN ?4 ELSE description END,
                                stamps_issued = CASE WHEN (stamps_issued IS NULL OR stamps_issued = '') AND (?5 != '') THEN ?5 ELSE stamps_issued END,
                                mint_condition = CASE WHEN (mint_condition IS NULL OR mint_condition = '') AND (?6 != '') THEN ?6 ELSE mint_condition END,
                                unused = CASE WHEN (unused IS NULL OR unused = '') AND (?7 != '') THEN ?7 ELSE unused END,
                                used = CASE WHEN (used IS NULL OR used = '') AND (?8 != '') THEN ?8 ELSE used END,
                                letter_fdc = CASE WHEN (letter_fdc IS NULL OR letter_fdc = '') AND (?9 != '') THEN ?9 ELSE letter_fdc END,
                                currency = CASE WHEN (currency IS NULL OR currency = '') AND (?10 != '') THEN ?10 ELSE currency END,
                                watermark = CASE WHEN (watermark IS NULL OR watermark = '') AND (?11 != '') THEN ?11 ELSE watermark END,
                                design = CASE WHEN (design IS NULL OR design = '') AND (?12 != '') THEN ?12 ELSE design END,
                                engraving = CASE WHEN (engraving IS NULL OR engraving = '') AND (?13 != '') THEN ?13 ELSE engraving END,
                                perforation = CASE WHEN (perforation IS NULL OR perforation = '') AND (?14 != '') THEN ?14 ELSE perforation END
                                WHERE country = ?15 AND series = ?16 AND sg_number = ?17";
                            let params_vec = params![
                                local_image_path, remote_image_url, original_image_url,
                                description, stamps_issued, mint_condition, unused, used, letter_fdc, currency,
                                watermark, design, engraving, perforation,
                                site_name, series_name, sg_number
                            ];
                            match conn.execute(update_sql, params_vec) {
                                Ok(rows) => {
                                    if rows > 0 {
                                        info!("Updated images/metadata for {} | {} | {} (rows={})", site_name, series_name, sg_number, rows);
                                        // Treat this as an update event: emit a STAMP line so downstream tools see the refreshed image/metadata
                                        new_stamps_this_page += 1;
                                        seen_stamps.insert(key.clone());
                                        let stamp_line = format!("STAMP;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                                            stamp_year, month, day, series_name, sg_number, type_, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, local_image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, cleaned_context_details);
                                        all_stamp_data.push(stamp_line);
                                    } else {
                                        debug!("--overwrite-images set but no DB row matched for update: {} | {} | {}", site_name, series_name, sg_number);
                                    }
                                }
                                Err(e) => { error!("DB update error for {} {} {}: {}", site_name, series_name, sg_number, e); }
                            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_stamps_issued_basic() {
        assert_eq!(normalize_stamps_issued("1.234.567"), "1,234,567");
        assert_eq!(normalize_stamps_issued("(1234)"), "1234");
        assert_eq!(normalize_stamps_issued("1,234"), "1,234");
        assert_eq!(normalize_stamps_issued("N/A"), "");
    }

    #[test]
    fn test_dedupe_details_preserves_order() {
        let src = vec!["WM: None".to_string(), "Perforation: 12".to_string(), "WM: None".to_string(), "Note".to_string()];
        let out = dedupe_details(&src);
        assert_eq!(out, vec!["WM: None".to_string(), "Perforation: 12".to_string(), "Note".to_string()]);
    }

    #[test]
    fn test_extract_block_metadata() {
        let src = vec!["WM: None".to_string(), "Design: King George".to_string(), "Engraving: Recess".to_string(), "Perf: 12".to_string()];
        let (wm, design, engr, perf) = extract_block_metadata(&src);
        assert_eq!(wm, "None");
        assert_eq!(design, "King George");
        assert_eq!(engr, "Recess");
        assert_eq!(perf, "12");
    }
}
