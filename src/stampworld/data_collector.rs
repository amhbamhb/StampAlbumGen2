use crate::types::ImageStatus;
use log::{debug, error, info};
use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::params;
use scraper::{Html, Selector};
use std::collections::HashSet;

// Precompiled regexes used across the module
pub static RANGE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*\d+\s*\p{Pd}\s*\d+\s*$").expect("invalid range regex"));

pub static DAY_MONTH_RE: Lazy<Regex> = Lazy::new(|| {
    // Matches "1. January", "12. Feb", " 3. März", case-insensitive, accepts Unicode letters and hyphens
    Regex::new(r"(?i)^\s*(\d{1,2})\.\s*([\p{L}\-]+)\s*$").expect("invalid day-month regex")
});

// Import parser helpers from the extracted parser module. These helpers were
// previously defined here; they have been moved to `stampworld::parser` so
// they can be unit tested and reused independently from the large collector.
use crate::stampworld::parser::{
    compress_context, dedupe_details, extract_block_metadata, normalize_stamps_issued,
};

/// Consolidated StampWorld collector implementing the key heuristics:
/// - Prefer images scoped to a stamp row; fall back to block/global images only when needed
/// - Stop after N consecutive pages that add zero new stamps (`stop_after_no_hits`)
/// - In-run dedupe using an in-memory HashSet keyed by country||series||sg_number
/// - Enforce requested year range strictly
pub fn collect_stampworld_data_with_fetcher(
    fetcher: &dyn crate::stampworld::fetch::Fetcher,
    sequencer: &mut i32,
    _separator: &str,
    site_name: &str,
    images_directory: &str,
    month_number: &dyn Fn(&str) -> String,
    get_stampworld_image: &dyn Fn(&str, &str, bool) -> ImageStatus,
    base_url: &str,
    start_year: i32,
    end_year: i32,
    overwrite_images: bool,
    page_callback: Option<&dyn Fn(usize)>,
    max_pages: i32,
    stop_after_no_hits: i32,
) -> Vec<String> {
    info!(
        "collect_stampworld_data called for {} base_url={} range={}..{}",
        site_name, base_url, start_year, end_year
    );

    // Load config and check whether the user requested detailed diagnostics
    // which we will emit at INFO level to avoid requiring RUST_LOG=debug.
    let cfg = crate::config::load_config("config.toml");
    let emit_row_diag = cfg.detailed_page_fit_diagnostics.unwrap_or(false);

    // Use the injected fetcher for network requests. Call sites that do not
    // pass a fetcher can use the wrapper `collect_stampworld_data` below.

    // Initialize DB: prefer album-scoped DB name (parent folder basename),
    // falling back to the country/site-based name if we can't determine it.
    // Previously this used the site name which produced `Albums/<Album>/Switzerland.db`.
    // For per-album runs we want `Albums/<Album>/<Album>.db` so the generator and
    // collector operate on the same file.
    let images_parent = std::path::Path::new(images_directory)
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| std::path::PathBuf::from(images_directory));
    let country_db_name = site_name.replace("/", "-");
    let album_basename = images_parent
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| country_db_name.clone());
    let db_path = images_parent
        .join(format!("{}.db", album_basename))
        .to_string_lossy()
        .to_string();
    let conn = match crate::db_utils::init_db(&db_path) {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to initialize DB {}: {}", db_path, e);
            return Vec::new();
        }
    };
    // Log the DB path we opened so it's straightforward to confirm the
    // scraper and manual sqlite queries target the same file.
    info!("Using DB at: {}", db_path);

    // (Image dimension resolution is now centralized via `image_utils::get_image_dimensions`)

    let mut all_stamp_data: Vec<String> = Vec::new();
    let mut done = false;
    let mut page_num: i32 = 1;
    let mut consecutive_no_hits: i32 = 0;
    let mut seen_stamps: HashSet<String> = HashSet::new();

    // Helper selectors used across pages/blocks
    let content_selector = Selector::parse("div.container-fluid.content_table")
        .unwrap_or_else(|_| Selector::parse("div.content_table").unwrap());
    let table_selector =
        Selector::parse("tr.stamp_tr").unwrap_or_else(|_| Selector::parse("tr").unwrap());

    // Precompile regexes. Use look-arounds so we only match standalone 4-digit
    // years (e.g. 1976) and avoid matching them when they're embedded inside
    // larger numeric tokens such as stamps-issued counters like "13662000".
    // Use word boundaries to avoid matching years embedded inside larger numbers.
    let year_re = Regex::new(r"\b(?:18|19|20)\d{2}\b").unwrap();

    // allow a small set of deterministic slug-fallback attempts when the
    // first request for page 1 returns 404. We keep a mutable
    // `working_base_url` and a queue `slug_variants` so we can try up to a
    // couple of reasonable alternatives (insert '!' before first hyphen,
    // or before last hyphen) without guessing lots of permutations.
    let mut working_base_url = base_url.to_string();
    let mut slug_variants: Vec<String> = Vec::new();

    while !done {
        // Respect max_pages if set
        if max_pages > 0 && page_num > max_pages {
            debug!("Reached max_pages={} stopping", max_pages);
            break;
        }

        // Fetch page using the current working base URL
        let url = format!("{}{}", working_base_url, page_num);
        // Temporary info: log exact request URL we'll call (use info so it appears without RUST_LOG=debug)
        info!("Request URL: {}", url);
        let (status, body) = match fetcher.fetch_text(&url) {
            Ok((s, b)) => (s, b),
            Err(e) => {
                error!("Problem retrieving page {}: {}", url, e);
                break;
            }
        };
        // Extra stdout-level diagnostics to ensure automated runs capture
        // the fetch result even when logging is redirected. These lines are
        // intentionally printed to stdout so CI or wrappers can see them.
        // println!(
        //     "[STAMPWORLD DEBUG] Fetched URL: {} status={} bytes={}",
        //     url,
        //     status.as_u16(),
        //     body.len()
        // );
        // Log short HTTP response info to help diagnose site errors (info-level to avoid huge debug dumps)
        // info!("HTTP status for {} -> {}", url, status);
        // If page 1 returns 404, prepare and/or consume deterministic
        // slug_variants (insert '!' before first/last hyphen) and retry them
        // one-by-one. We only populate the queue once per run to avoid
        // recomputing variants on every 404.
        if status.as_u16() == 404 && page_num == 1 {
            // Populate variants if not already done and the site_name doesn't
            // already contain an '!'. Use a small encoder similar to main.rs.
            if slug_variants.is_empty() && !site_name.contains('!') {
                // prefer hyphen-separated slugs for fallback variants
                let encode_site =
                    |s: &str| s.replace(' ', "-").replace('!', "%21").replace('/', "%2F");
                // first-hyphen insertion
                if let Some((first, rest)) = site_name.split_once('-') {
                    let variant = format!("{}!{}", first, rest);
                    if variant != site_name {
                        let encoded = encode_site(&variant);
                        let alt_base = format!(
                            "https://www.stampworld.com/en/stamps/{}/Postage%20stamps/{}-{}?page=",
                            encoded, start_year, end_year
                        );
                        slug_variants.push(alt_base);
                    }
                }
                // last-hyphen insertion
                if let Some(pos) = site_name.rfind('-') {
                    // only build if different from first insertion
                    let (head, tail) = site_name.split_at(pos);
                    // tail starts with '-', drop it then build
                    if tail.len() > 1 {
                        let tail_rest = &tail[1..];
                        let variant = format!("{}!{}", head, tail_rest);
                        if variant != site_name {
                            let encode = |s: &str| {
                                s.replace(' ', "-").replace('!', "%21").replace('/', "%2F")
                            };
                            let encoded = encode(&variant);
                            let alt_base = format!("https://www.stampworld.com/en/stamps/{}/Postage%20stamps/{}-{}?page=", encoded, start_year, end_year);
                            // avoid duplicates
                            if !slug_variants.contains(&alt_base) {
                                slug_variants.push(alt_base);
                            }
                        }
                    }
                }
            }

            // If we have prepared variants, consume the next one and retry
            if !slug_variants.is_empty() {
                let alt_base = slug_variants.remove(0);
                info!(
                    "Page 1 returned 404; retrying with slug-variant URL: {}",
                    alt_base
                );
                working_base_url = alt_base;
                // reset page counter so we re-request page 1 for the new slug
                page_num = 1;
                // continue to try the alternate URL immediately
                continue;
            }
        }
        // Emit first ~2KB of body to aid debugging (avoid flood) - chars taken as approximate
        // let snippet: String = body.chars().take(2048).collect();
        // if !snippet.is_empty() {
        //     info!("HTTP body snippet for {}:\n{}", url, snippet);
        // }
        if let Some(cb) = page_callback {
            cb(page_num as usize);
        }

        let document = Html::parse_document(&body);

        let mut new_stamps_this_page: i32 = 0;
        // Debug: count content blocks found on this page
        let content_blocks: Vec<_> = document.select(&content_selector).collect();
        info!(
            "Found {} content blocks on page {}",
            content_blocks.len(),
            page_num
        );
        // println!(
        //     "[STAMPWORLD DEBUG] Found {} content blocks on page {}",
        //     content_blocks.len(),
        //     page_num
        // );

        for (block_idx, content_div) in content_blocks.into_iter().enumerate() {
            // Print a small snippet of the block HTML to help diagnose why
            // row selectors may not match in certain environments.
            let _block_html_snippet: String = content_div.html().chars().take(500).collect();
            // println!(
            //     "[STAMPWORLD DEBUG] Block {} HTML snippet: {}",
            //     block_idx, block_html_snippet
            // );
            // Extract block-level details
            let p_selector = Selector::parse("p").unwrap();
            let mut details_vec: Vec<String> = Vec::new();
            for p in content_div.select(&p_selector) {
                let t = p.text().collect::<String>();
                if !t.trim().is_empty() {
                    details_vec.push(t);
                }
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
                debug!("Extracted series_desc from header: '{}'", series_desc);
                if let Some(caps) = Regex::new(r"(\d{4})(?:\s*-\s*(\d{4}))?")
                    .unwrap()
                    .captures(&text)
                {
                    block_year_int = caps
                        .get(1)
                        .map_or("0", |m| m.as_str())
                        .parse::<i32>()
                        .unwrap_or(0);
                }
                // (no-op) header debug removed — header_texts will be logged later when available
            }

            // If block start year is well beyond requested end_year, stop to avoid unnecessary scraping
            if block_year_int > 0 && block_year_int > end_year + 10 {
                debug!(
                    "Block year {} significantly > endYear {} — stopping",
                    block_year_int, end_year
                );
                done = true;
                break;
            }

            // Collect block/global images (fallback)
            let images_selector = Selector::parse(".images_container img")
                .unwrap_or_else(|_| Selector::parse("img").unwrap());
            let mut global_images: Vec<(String, String)> = Vec::new(); // (url, alt)
            for img in content_div.select(&images_selector) {
                if let Some(src) = img.value().attr("src") {
                    let url = if src.starts_with("http") {
                        src.to_string()
                    } else if src.starts_with("//") {
                        format!("https:{}", src)
                    } else {
                        format!("https://www.stampworld.com{}", src)
                    };
                    let alt = img.value().attr("alt").unwrap_or("").to_string();
                    // Filter out obvious icons and gifs
                    if url.to_lowercase().contains(".gif")
                        || url.to_lowercase().contains("layout")
                        || url.to_lowercase().contains("static")
                    {
                        continue;
                    }
                    global_images.push((url, alt));
                }
            }

            // Build a header -> column index map for this table so we can robustly
            // extract fields like "Stamps Issued", "Mint Condition", "Unused", "Used", "Letter/FDC"
            use std::collections::HashMap;
            let mut col_map: HashMap<String, usize> = HashMap::new();
            let mut header_texts: Vec<String> = Vec::new();
            if let Some(table_node) = content_div
                .select(&Selector::parse("table").unwrap())
                .next()
            {
                if let Some(thead) = table_node.select(&Selector::parse("thead").unwrap()).next() {
                    for th in thead.select(&Selector::parse("th").unwrap()) {
                        let mut txt = th.text().collect::<String>();
                        txt = txt.trim().to_string();
                        if let Some(img) = th.select(&Selector::parse("img").unwrap()).next() {
                            if let Some(alt) = img.value().attr("alt") {
                                let a = alt.trim();
                                if !a.is_empty() {
                                    txt = a.to_string();
                                }
                            }
                        }
                        header_texts.push(txt.clone());
                    }
                    // Map names to header indices (we'll align these with row cells including th)
                    for (i, txt) in header_texts.iter().enumerate() {
                        let low = txt.to_lowercase();
                        if low.contains("stamps issued") || low.contains("released") {
                            col_map.insert("stamps_issued".to_string(), i);
                        }
                        if low.contains("mint condition") || low.contains("mint") {
                            col_map.insert("mint_condition".to_string(), i);
                        }
                        if low.contains("unused") {
                            col_map.insert("unused".to_string(), i);
                        }
                        if low.contains("used") {
                            col_map.insert("used".to_string(), i);
                        }
                        if low.contains("letter") || low.contains("fdc") {
                            col_map.insert("letter_fdc".to_string(), i);
                        }
                        if low.contains("currency") || low.contains("usd") {
                            col_map.insert("currency".to_string(), i);
                        }
                    }
                }
            }
            if !header_texts.is_empty() {
                // info!("Block {} headers: {:?}", block_idx, header_texts);
            }

            // Associate images with specific rows first
            let mut image_idx = 0usize;
            let rows: Vec<_> = content_div.select(&table_selector).collect();
            info!(
                "Block {} has {} table rows matching selector",
                block_idx,
                rows.len()
            );
            // println!(
            //     "[STAMPWORLD DEBUG] Block {} has {} table rows matching selector",
            //     block_idx,
            //     rows.len()
            // );
            if !rows.is_empty() {
                // Log a short snippet of the first row to help diagnose selector mismatches
                let sample_html = rows[0].inner_html();
                let sample_snippet: String = sample_html.chars().take(300).collect();
                debug!("Block {} first row snippet: {}", block_idx, sample_snippet);
                // println!(
                //     "[STAMPWORLD DEBUG] Block {} first row snippet: {}",
                //     block_idx, sample_snippet
                // );
            }
            for (ridx, tr) in rows.into_iter().enumerate() {
                // Basic row parsing: build a flattened row_cells vector so we can
                // handle tables where the catalog number is either in a <th>
                // or in the first <td>. This makes parsing tolerant of slight
                // variations in the markup observed across StampWorld pages.
                let th_selector = Selector::parse("th").unwrap();
                let td_selector = Selector::parse("td").unwrap();
                let ths: Vec<_> = tr.select(&th_selector).collect();
                let tds: Vec<_> = tr.select(&td_selector).collect();

                // Build a row_cells vector that begins with the catalog number
                // (sg_number) followed by the row's td cell texts. If the row
                // used a <th> for the catalog number, include it first; otherwise
                // assume the first <td> is the catalog number.
                let mut row_cells: Vec<String> = Vec::new();
                if let Some(th_row) = ths.first() {
                    row_cells.push(th_row.text().collect::<String>().trim().to_string());
                    for td in &tds {
                        row_cells.push(td.text().collect::<String>().trim().to_string());
                    }
                } else {
                    // No <th>: use tds directly. If there are at least one td,
                    // the first is likely the sg_number.
                    for td in &tds {
                        row_cells.push(td.text().collect::<String>().trim().to_string());
                    }
                }

                // Require at least a catalog number plus a few cells to be useful.
                if row_cells.is_empty() {
                    if emit_row_diag {
                        info!("Block {} Row {} skipped: empty row_cells", block_idx, ridx);
                    }
                    continue;
                }
                if ridx < 3 {
                    if emit_row_diag {
                        info!(
                            "Block {} Row {} cells sample: {:?}",
                            block_idx, ridx, row_cells
                        );
                    } else {
                        debug!(
                            "Block {} Row {} cells sample: {:?}",
                            block_idx, ridx, row_cells
                        );
                    }
                }

                let sg_number = row_cells
                    .first()
                    .map(|s| s.trim().to_string())
                    .unwrap_or_default();
                debug!(
                    "Processing row with sg_number: '{}', cells: {}",
                    sg_number,
                    row_cells.len()
                );
                // Pull a couple of fields early so grouped-range handling can
                // inspect the description cell for a year fallback. Observed
                // pages commonly have the description at index 6 (0=sg), so
                // prefer that when available; fall back to index 5 for older
                // or alternate layouts. Stamps issued is usually one column
                // after the description (index 7) when present.
                let description = if row_cells.len() > 6 {
                    row_cells.get(6).cloned().unwrap_or_default()
                } else {
                    row_cells.get(5).cloned().unwrap_or_default()
                };
                let _stamps_issued = if row_cells.len() > 7 {
                    row_cells.get(7).cloned().unwrap_or_default()
                } else {
                    row_cells.get(6).cloned().unwrap_or_default()
                };
                // Handle grouped-range tokens like "1067-1068": expand into
                // placeholders if the config requests it. We compute stamp_year
                // here (fallback to block_year_int) so inserted placeholders
                // have a reasonable year value.
                if RANGE_RE.is_match(&sg_number) {
                    // Determine stamp_year for the row (fallback to block year)
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

                    // Normalize common Unicode hyphen variants to ASCII
                    let mut norm = sg_number.clone();
                    norm = norm.replace(['\u{2011}', '\u{2013}', '\u{2012}', '\u{2010}'], "-");
                    if emit_row_diag {
                        info!(
                            "Encountered grouped-range token='{}' (normalized='{}')",
                            sg_number, norm
                        );
                    }

                    if let Some(expand) = cfg.expand_grouped_ranges.clone() {
                        let parts: Vec<&str> = norm
                            .split('-')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect();
                        if parts.len() == 2 {
                            if let (Ok(a), Ok(b)) =
                                (parts[0].parse::<i32>(), parts[1].parse::<i32>())
                            {
                                let start = a.min(b);
                                let end = a.max(b);

                                // If user requested detail_fetch, attempt to fetch group/detail pages
                                // and extract per-stamp rows. Fall back to placeholders when
                                // fetching/parsing fails. Limit fetches per-range to avoid heavy loads.
                                if expand == "detail_fetch" {
                                    // Try to find a canonical group base URL inside the block HTML
                                    let group_url_re = Regex::new(
                                        r#"https?://[^"']+/stamps/[^"']+/Postage[- ]stamps/g\d+/"#,
                                    )
                                    .unwrap();
                                    let block_html = content_div.html();
                                    let maybe_group_base = group_url_re
                                        .find(&block_html)
                                        .map(|m| m.as_str().to_string());

                                    // Safe per-range limit
                                    let max_fetch_per_range: i32 = 50;
                                    let mut fetch_count: i32 = 0;

                                    for n in start..=end {
                                        let s = n.to_string();
                                        // If we've already inserted or seen this stamp in-run, skip
                                        let keyp = format!("{}||{}||{}", site_name, series_desc, s);
                                        if seen_stamps.contains(&keyp) {
                                            continue;
                                        }

                                        if fetch_count >= max_fetch_per_range {
                                            debug!("Reached per-range fetch limit {} for grouped-range {}-{}; inserting placeholders for remaining", max_fetch_per_range, start, end);
                                            // insert placeholders for remaining numbers
                                            for m in n..=end {
                                                let ms = m.to_string();
                                                if let Ok(inserted) =
                                                    crate::db_utils::insert_stamp_placeholder(
                                                        &conn,
                                                        site_name,
                                                        &series_desc,
                                                        stamp_year,
                                                        &ms,
                                                    )
                                                {
                                                    if inserted {
                                                        let _ = conn.execute(
                                                            "UPDATE stamps
                                                             SET catalog_code = 'sw'
                                                             WHERE country = ?1
                                                               AND series = ?2
                                                               AND year = ?3
                                                               AND sg_number = ?4
                                                               AND TRIM(COALESCE(catalog_code, '')) = ''",
                                                            params![
                                                                site_name,
                                                                &series_desc,
                                                                stamp_year,
                                                                &ms
                                                            ],
                                                        );
                                                        seen_stamps.insert(format!(
                                                            "{}||{}||{}",
                                                            site_name, series_desc, ms
                                                        ));
                                                        new_stamps_this_page += 1;
                                                        all_stamp_data.push(format!(
                                                            "STAMP;{};{};{};{};{};{};{}",
                                                            stamp_year,
                                                            "",
                                                            "",
                                                            series_desc,
                                                            ms,
                                                            "PLACEHOLDER",
                                                            ""
                                                        ));
                                                    }
                                                }
                                            }
                                            break;
                                        }

                                        let mut did_insert = false;
                                        if let Some(group_base) = maybe_group_base.clone() {
                                            // Construct detail URL (group page + fragment). The fragment doesn't change server response
                                            // but it helps us locate the per-stamp element in the returned HTML when present.
                                            let detail_url = format!(
                                                "{}#{}",
                                                group_base.trim_end_matches('/'),
                                                s
                                            );
                                            debug!(
                                                "Attempting detail fetch for {} -> {}",
                                                s, detail_url
                                            );
                                            if let Ok((_status, text)) =
                                                fetcher.fetch_text(&detail_url)
                                            {
                                                let doc = Html::parse_document(&text);
                                                // Search table rows and try to find the row containing this sg_number anchor/id
                                                let trs: Vec<_> =
                                                    doc.select(&table_selector).collect();
                                                for tr_doc in trs.into_iter() {
                                                    // Quick check: does this tr contain an anchor or id for this sg number?
                                                    if tr_doc.html().contains(&format!("a_s_{}", s))
                                                        || tr_doc
                                                            .text()
                                                            .collect::<String>()
                                                            .contains(&s)
                                                    {
                                                        // Reconstruct flattened row_cells similar to main parsing
                                                        let td_sel = Selector::parse("td").unwrap();
                                                        let th_sel = Selector::parse("th").unwrap();
                                                        let ths2: Vec<_> =
                                                            tr_doc.select(&th_sel).collect();
                                                        let tds2: Vec<_> =
                                                            tr_doc.select(&td_sel).collect();
                                                        // Build row_cells
                                                        let mut row_cells2: Vec<String> =
                                                            Vec::new();
                                                        if let Some(th_row) = ths2.first() {
                                                            row_cells2.push(
                                                                th_row
                                                                    .text()
                                                                    .collect::<String>()
                                                                    .trim()
                                                                    .to_string(),
                                                            );
                                                            for td in &tds2 {
                                                                row_cells2.push(
                                                                    td.text()
                                                                        .collect::<String>()
                                                                        .trim()
                                                                        .to_string(),
                                                                );
                                                            }
                                                        } else {
                                                            for td in &tds2 {
                                                                row_cells2.push(
                                                                    td.text()
                                                                        .collect::<String>()
                                                                        .trim()
                                                                        .to_string(),
                                                                );
                                                            }
                                                        }
                                                        if row_cells2.is_empty() {
                                                            continue;
                                                        }
                                                        // Map fields (reuse same indices as main parsing)
                                                        let sg_number_f = row_cells2
                                                            .first()
                                                            .map(|s| s.trim().to_string())
                                                            .unwrap_or_default();
                                                        let type_f = row_cells2
                                                            .get(1)
                                                            .cloned()
                                                            .unwrap_or_default();
                                                        let denomination_f = row_cells2
                                                            .get(2)
                                                            .cloned()
                                                            .unwrap_or_default();
                                                        let color_f = row_cells2
                                                            .get(4)
                                                            .cloned()
                                                            .unwrap_or_default();
                                                        let description_f = if row_cells2.len() > 6
                                                        {
                                                            row_cells2
                                                                .get(6)
                                                                .cloned()
                                                                .unwrap_or_default()
                                                        } else {
                                                            row_cells2
                                                                .get(5)
                                                                .cloned()
                                                                .unwrap_or_default()
                                                        };
                                                        // Extract month/day heuristics per-line
                                                        let mut month_f = String::new();
                                                        let mut day_f = String::new();
                                                        for line in row_cells2.join(" \n").lines() {
                                                            if let Some(cap) =
                                                                DAY_MONTH_RE.captures(line)
                                                            {
                                                                day_f = cap
                                                                    .get(1)
                                                                    .map_or("", |m| m.as_str())
                                                                    .to_string();
                                                                month_f = cap
                                                                    .get(2)
                                                                    .map_or("", |m| m.as_str())
                                                                    .to_string();
                                                                break;
                                                            }
                                                        }
                                                        let month_num_f = if month_f.is_empty() {
                                                            "0".to_string()
                                                        } else {
                                                            month_number(&month_f)
                                                        };
                                                        // Call centralized insert
                                                        match crate::db_utils::insert_stamp(
                                                            &conn,
                                                            site_name,
                                                            &series_desc,
                                                            stamp_year,
                                                            &month_f,
                                                            &month_num_f,
                                                            &day_f,
                                                            &sg_number_f,
                                                            &type_f,
                                                            &denomination_f,
                                                            &color_f,
                                                            &description_f,
                                                            "", // stamps_issued
                                                            "", // mint_condition
                                                            "", // unused
                                                            "", // used
                                                            "", // letter_fdc
                                                            "", // currency
                                                            "", // image_path
                                                            "", // remote_image_url
                                                            "", // original_image_url
                                                            "", // watermark
                                                            "", // design
                                                            "", // engraving
                                                            "", // perforation
                                                            "", // context
                                                        ) {
                                                            Ok(inserted) => {
                                                                if inserted {
                                                                    let _ = conn.execute(
                                                                        "UPDATE stamps
                                                                         SET catalog_code = 'sw'
                                                                         WHERE country = ?1
                                                                           AND series = ?2
                                                                           AND year = ?3
                                                                           AND sg_number = ?4
                                                                           AND TRIM(COALESCE(catalog_code, '')) = ''",
                                                                        params![
                                                                            site_name,
                                                                            &series_desc,
                                                                            stamp_year,
                                                                            &sg_number_f
                                                                        ],
                                                                    );
                                                                    if emit_row_diag {
                                                                        info!("Detail-fetch inserted stamp {} {}", series_desc, sg_number_f);
                                                                    }
                                                                    seen_stamps
                                                                        .insert(keyp.clone());
                                                                    new_stamps_this_page += 1;
                                                                    all_stamp_data.push(format!("STAMP;{};{};{};{};{};{};{}", stamp_year, month_f, day_f, series_desc, sg_number_f, type_f, description_f));
                                                                    did_insert = true;
                                                                }
                                                            }
                                                            Err(e) => {
                                                                error!("Detail-fetch DB insert failed for {}: {}", sg_number_f, e);
                                                            }
                                                        }
                                                        break;
                                                    }
                                                }
                                                if did_insert {
                                                    fetch_count += 1;
                                                    continue;
                                                }
                                            } else {
                                                debug!(
                                                    "Detail fetch error for {}: {}",
                                                    detail_url, "fetch failed"
                                                );
                                            }
                                        }

                                        // If we couldn't fetch/parse/insert, fall back to inserting a placeholder row
                                        if !did_insert {
                                            match crate::db_utils::insert_stamp_placeholder(
                                                &conn,
                                                site_name,
                                                &series_desc,
                                                stamp_year,
                                                &s,
                                            ) {
                                                Ok(inserted) => {
                                                    if inserted {
                                                        let _ = conn.execute(
                                                            "UPDATE stamps
                                                             SET catalog_code = 'sw'
                                                             WHERE country = ?1
                                                               AND series = ?2
                                                               AND year = ?3
                                                               AND sg_number = ?4
                                                               AND TRIM(COALESCE(catalog_code, '')) = ''",
                                                            params![
                                                                site_name,
                                                                &series_desc,
                                                                stamp_year,
                                                                &s
                                                            ],
                                                        );
                                                        if emit_row_diag {
                                                            info!("Inserted placeholder for {} {} (fallback)", series_desc, s);
                                                        }
                                                        seen_stamps.insert(keyp.clone());
                                                        new_stamps_this_page += 1;
                                                        all_stamp_data.push(format!(
                                                            "STAMP;{};{};{};{};{};{};{}",
                                                            stamp_year,
                                                            "",
                                                            "",
                                                            series_desc,
                                                            s,
                                                            "PLACEHOLDER",
                                                            ""
                                                        ));
                                                    }
                                                }
                                                Err(e) => {
                                                    error!("Placeholder helper failed (fallback) for {}: {}", s, e);
                                                }
                                            }
                                        }
                                    }
                                    // After processing range (either fetched or placeholder) continue outer row loop
                                    continue;
                                } else if expand == "placeholders" {
                                    for n in start..=end {
                                        let s = n.to_string();
                                        match crate::db_utils::insert_stamp_placeholder(
                                            &conn,
                                            site_name,
                                            &series_desc,
                                            stamp_year,
                                            &s,
                                        ) {
                                            Ok(inserted) => {
                                                if inserted {
                                                    let _ = conn.execute(
                                                        "UPDATE stamps
                                                         SET catalog_code = 'sw'
                                                         WHERE country = ?1
                                                           AND series = ?2
                                                           AND year = ?3
                                                           AND sg_number = ?4
                                                           AND TRIM(COALESCE(catalog_code, '')) = ''",
                                                        params![site_name, &series_desc, stamp_year, &s],
                                                    );
                                                    if emit_row_diag {
                                                        info!(
                                                            "Inserted placeholder for {} {}",
                                                            series_desc, s
                                                        );
                                                    }
                                                    let keyp = format!(
                                                        "{}||{}||{}",
                                                        site_name, series_desc, s
                                                    );
                                                    seen_stamps.insert(keyp);
                                                    new_stamps_this_page += 1;
                                                    let stamp_line = format!(
                                                        "STAMP;{};{};{};{};{};{};{}",
                                                        stamp_year,
                                                        "",
                                                        "",
                                                        series_desc,
                                                        s,
                                                        "PLACEHOLDER",
                                                        ""
                                                    );
                                                    all_stamp_data.push(stamp_line);
                                                } else if emit_row_diag {
                                                    info!("Placeholder skipped, already exists: {} {}", series_desc, s);
                                                }
                                            }
                                            Err(e) => {
                                                error!(
                                                    "Placeholder helper failed for {}: {}",
                                                    s, e
                                                );
                                            }
                                        }
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    if emit_row_diag {
                        info!(
                            "Skipping sg_number='{}' because it looks like a range/group",
                            sg_number
                        );
                    }
                    continue;
                }

                // Map baseline fields from the flattened row_cells. The expected
                // layout (when a <th> is present) is: [sg, type, denom, img?, color, desc, stamps_issued]
                // When no <th> and sg is in the first td the positions are the same
                // (sg at index 0). Use safe getters to avoid panics on short rows.
                let type_ = row_cells.get(1).cloned().unwrap_or_default();
                let denomination = row_cells.get(2).cloned().unwrap_or_default();
                let color = row_cells.get(4).cloned().unwrap_or_default();
                // Observed real pages commonly place description at index 6 (0=sg), so prefer that when available.
                let description = if row_cells.len() > 6 {
                    row_cells.get(6).cloned().unwrap_or_default()
                } else {
                    row_cells.get(5).cloned().unwrap_or_default()
                };
                let stamps_issued = if row_cells.len() > 7 {
                    row_cells.get(7).cloned().unwrap_or_default()
                } else {
                    row_cells.get(6).cloned().unwrap_or_default()
                };

                // Prefer images scoped inside the row
                let mut selected_image_url = String::new();
                let mut _selected_alt = String::new();
                // row-local images
                for img in tr.select(&Selector::parse("img").unwrap()) {
                    if let Some(src) = img.value().attr("src") {
                        let url = if src.starts_with("http") {
                            src.to_string()
                        } else if src.starts_with("//") {
                            format!("https:{}", src)
                        } else {
                            format!("https://www.stampworld.com{}", src)
                        };
                        let alt = img.value().attr("alt").unwrap_or("").to_string();
                        if url.to_lowercase().contains(".gif")
                            || url.to_lowercase().contains("layout")
                        {
                            continue;
                        }
                        selected_image_url = url;
                        _selected_alt = alt;
                        break;
                    }
                }

                // Fallback to global images by order
                if selected_image_url.is_empty() && image_idx < global_images.len() {
                    selected_image_url = global_images[image_idx].0.clone();
                    _selected_alt = global_images[image_idx].1.clone();
                    image_idx += 1;
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
                if stamp_year < start_year {
                    if emit_row_diag {
                        info!(
                            "Skipping {}: year {} < start_year {}",
                            sg_number, stamp_year, start_year
                        );
                    } else {
                        debug!("Skipping {}: year {} < start", sg_number, stamp_year);
                    }
                    continue;
                }
                if stamp_year > end_year {
                    if emit_row_diag {
                        info!(
                            "Skipping {}: year {} > end_year {}",
                            sg_number, stamp_year, end_year
                        );
                    } else {
                        debug!(
                            "Skipping {}: year {} > endYear {}",
                            sg_number, stamp_year, end_year
                        );
                    }
                    // Do not halt the entire scrape on encountering a single
                    // out-of-range row; pages can contain mixed years. Skip
                    // this row and continue scanning the rest of the block.
                    continue;
                }

                // Build unique key and dedupe in-run
                let mut series_name = if !series_desc.is_empty() {
                    series_desc.clone()
                } else {
                    description.clone()
                };
                if series_name.is_empty() {
                    series_name = site_name.to_string();
                    debug!("Using site_name as fallback: '{}'", series_name);
                }
                debug!(
                    "series_name set to: '{}' (series_desc: '{}', description: '{}')",
                    series_name, series_desc, description
                );
                let key = format!("{}||{}||{}", site_name, series_name, sg_number);
                if seen_stamps.contains(&key) {
                    if emit_row_diag {
                        info!("Duplicate in-run stamp skipped: {}", key);
                    } else {
                        debug!("Duplicate in-run stamp skipped: {}", key);
                    }
                    continue;
                }

                // Determine local image path after download (if any)
                let mut local_image_path = String::new();
                let mut remote_image_url = String::new();
                let mut original_image_url = String::new();
                if !selected_image_url.is_empty() {
                    // Use downloader closure; it may accept an id (sg) or full URL
                    // Pass the sanitized sg_number so the downloader can produce a
                    // deterministic per-row filename (avoids duplicates and makes
                    // filenames predictable).
                    let sanitized_sg = crate::album_utils::sanitize_filename(
                        &sg_number.replace(['\u{2011}', '\u{2013}'], "-"),
                    );
                    let image_res =
                        get_stampworld_image(&selected_image_url, &sanitized_sg, overwrite_images);
                    debug!(
                        "Downloader returned: exists={} filename='{}'",
                        image_res.exists, image_res.filename
                    );
                    if image_res.exists {
                        // create per-row filename to reduce accidental reuse
                        let sanitized_sg = crate::album_utils::sanitize_filename(
                            &sg_number.replace(['\u{2011}', '\u{2013}'], "-"),
                        );
                        // If the downloader didn't provide a filename, fall back to the
                        // last path segment of the remote URL so we still record an
                        // original filename for display purposes.
                        let fallback_filename = selected_image_url
                            .rsplit('/')
                            .next()
                            .and_then(|s| s.split('?').next())
                            .unwrap_or("image.jpg")
                            .to_string();
                        let target_name = format!("{}-{}", sanitized_sg, image_res.filename);
                        let downloaded_path =
                            format!("{}/{}", images_directory, image_res.filename);
                        let target_path = format!("{}/{}", images_directory, target_name);

                        let downloaded_exists = std::path::Path::new(&downloaded_path).exists();
                        let target_exists = std::path::Path::new(&target_path).exists();
                        if downloaded_exists && downloaded_path != target_path {
                            if target_exists {
                                // Canonical target already exists; drop duplicate source file.
                                let _ = std::fs::remove_file(&downloaded_path);
                            } else {
                                match std::fs::rename(&downloaded_path, &target_path) {
                                    Ok(_) => {}
                                    Err(rename_err) => {
                                        match std::fs::copy(&downloaded_path, &target_path) {
                                            Ok(_) => {
                                                let _ = std::fs::remove_file(&downloaded_path);
                                            }
                                            Err(copy_err) => {
                                                debug!(
                                                    "Failed to move {} to {} (rename={}, copy={})",
                                                    downloaded_path,
                                                    target_path,
                                                    rename_err,
                                                    copy_err
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if std::path::Path::new(&target_path).exists() {
                            local_image_path = format!("images/{}", target_name);
                        } else {
                            // Fallback if downloader already saved to a different final filename.
                            local_image_path = format!("images/{}", image_res.filename);
                        }
                        remote_image_url = selected_image_url.clone();
                        original_image_url = if image_res.filename.trim().is_empty() {
                            fallback_filename.clone()
                        } else {
                            image_res.filename.clone()
                        };
                    }
                }

                // Month/day extraction - scan deduplicated detail lines and
                // each logical line so patterns like "27. November" inside a
                // multi-line <p> are detected.
                let mut month = String::new();
                let mut day = String::new();
                'md_out: for d in &deduped {
                    for line in d.lines() {
                        if let Some(cap) = DAY_MONTH_RE.captures(line) {
                            day = cap.get(1).map_or("", |m| m.as_str()).to_string();
                            month = cap.get(2).map_or("", |m| m.as_str()).to_string();
                            break 'md_out;
                        }
                    }
                }
                let month_num = if month.is_empty() {
                    "0".to_string()
                } else {
                    month_number(&month)
                };

                // Extract block-level metadata using the helper and produce a cleaned context
                let (watermark, design, engraving, perforation) = extract_block_metadata(&deduped);
                let raw_context = deduped
                    .iter()
                    .filter(|l| !l.trim().is_empty() && l.trim() != "\u{200b}")
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(" | ");
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
                            if first.contains("no") || first.contains("no.") || first.contains("#")
                            {
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
                    if series_desc.contains("1860 -1862 Queen Victoria - Not Watermarked")
                        && sg_number == "2"
                    {
                        debug!(
                            "[debug-row] sg_number={} row_cells={:?}",
                            sg_number, row_cells
                        );
                    }
                    stamps_issued = get_col("stamps_issued");
                    mint_condition = get_col("mint_condition");
                    unused = get_col("unused");
                    used = get_col("used");
                    letter_fdc = get_col("letter_fdc");
                    currency = get_col("currency");
                    if currency.is_empty() {
                        // fallback: last cell
                        if let Some(last) = row_cells.last() {
                            currency = last.clone();
                        }
                        // secondary fallback: scan row cells (end-first) for currency-like tokens
                        if currency.trim().is_empty() {
                            for cell in row_cells.iter().rev() {
                                let c = cell.trim();
                                if c.is_empty() {
                                    continue;
                                }
                                let low = c.to_lowercase();
                                if low.contains("usd")
                                    || low == "$"
                                    || low == "sell"
                                    || low.contains("us$")
                                    || low.contains("eur")
                                    || low.contains("€")
                                    || low.contains("gbp")
                                    || low.contains("£")
                                {
                                    currency = c.to_string();
                                    break;
                                }
                            }
                        }
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
                        mint_condition = tds
                            .get(9)
                            .map(|n| n.text().collect::<String>().trim().to_string())
                            .unwrap_or_default();
                        currency = tds
                            .last()
                            .map(|n| n.text().collect::<String>().trim().to_string())
                            .unwrap_or_default();
                        // fallback scan for currency tokens inside tds if last cell is not the currency
                        if currency.trim().is_empty() {
                            for td in tds.iter().rev() {
                                let c = td.text().collect::<String>().trim().to_string();
                                if c.is_empty() {
                                    continue;
                                }
                                let low = c.to_lowercase();
                                if low.contains("usd")
                                    || low == "$"
                                    || low == "sell"
                                    || low.contains("us$")
                                    || low.contains("eur")
                                    || low.contains("€")
                                    || low.contains("gbp")
                                    || low.contains("£")
                                {
                                    currency = c;
                                    break;
                                }
                            }
                        }
                    }
                }

                // Normalize currency tokens (map known site tokens like 'SELL' to USD)
                fn normalize_currency_token(s: &str) -> String {
                    let t = s.trim();
                    if t.is_empty() {
                        return String::new();
                    }
                    let low = t.to_lowercase();
                    // Common mappings
                    if low == "sell"
                        || low == "usd"
                        || low == "$"
                        || low == "us$"
                        || low.contains("usd")
                        || low.contains("us$")
                    {
                        return "USD".to_string();
                    }
                    if low == "eur" || low.contains("€") {
                        return "EUR".to_string();
                    }
                    if low == "gbp" || low.contains("£") {
                        return "GBP".to_string();
                    }
                    // fallback: uppercase and trim
                    t.to_uppercase()
                }
                let currency_norm = normalize_currency_token(&currency);

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
                    &currency_norm,
                    &local_image_path,
                    &remote_image_url,
                    &original_image_url,
                    &watermark,
                    &design,
                    &engraving,
                    &perforation,
                    &cleaned_context_details,
                );
                // Diagnostic: log the raw result from the DB layer so we can see
                // whether the call inserted/updated (Ok(true)), was a no-op
                // (Ok(false)), or returned an error (Err).
                info!(
                    "db_utils::insert_stamp result for {}|{}|{} => {:?}",
                    site_name, series_name, sg_number, res
                );
                match res {
                    Ok(inserted) => {
                        if inserted {
                            let _ = conn.execute(
                                "UPDATE stamps
                                 SET catalog_code = 'sw'
                                 WHERE country = ?1
                                   AND series = ?2
                                   AND year = ?3
                                   AND sg_number = ?4
                                   AND TRIM(COALESCE(catalog_code, '')) = ''",
                                params![site_name, &series_name, stamp_year, &sg_number],
                            );
                            // info!("Inserted new stamp: {} | {} | {}", site_name, series_name, sg_number);
                            new_stamps_this_page += 1;
                            seen_stamps.insert(key.clone());
                            let stamp_line = format!("STAMP;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                                stamp_year, month, day, series_name, sg_number, type_, denomination, color, description, stamps_issued, "", "", "", "", "", local_image_path, remote_image_url, original_image_url, "", "", "", "", cleaned_context_details);
                            all_stamp_data.push(stamp_line);
                            // Persist image pixel dimensions when available (read from local file if present)
                            if !local_image_path.is_empty() {
                                if let Err(e) = crate::db_utils::persist_image_dimensions_from_file(
                                    &conn,
                                    site_name,
                                    &series_name,
                                    &sg_number,
                                    &local_image_path,
                                ) {
                                    error!(
                                        "Failed to persist image dimensions for {} {} {}: {}",
                                        site_name, series_name, sg_number, e
                                    );
                                }
                            }
                        } else if overwrite_images {
                            // The row already exists and wasn't updated by insert_stamp (no placeholder).
                            // If --overwrite-images was requested, perform a targeted UPDATE to replace image fields
                            // and fill any missing metadata fields (but do not overwrite existing non-empty values).
                            // info!("--overwrite-images set; attempting conditional UPDATE for {} | {} | {}", site_name, series_name, sg_number);
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
                                perforation = CASE WHEN (perforation IS NULL OR perforation = '') AND (?14 != '') THEN ?14 ELSE perforation END,
                                catalog_code = CASE WHEN (catalog_code IS NULL OR catalog_code = '') THEN 'sw' ELSE catalog_code END
                                WHERE country = ?15 AND series = ?16 AND sg_number = ?17";
                            let params_vec = params![
                                local_image_path,
                                remote_image_url,
                                original_image_url,
                                description,
                                stamps_issued,
                                mint_condition,
                                unused,
                                used,
                                letter_fdc,
                                currency,
                                watermark,
                                design,
                                engraving,
                                perforation,
                                site_name,
                                series_name,
                                sg_number
                            ];
                            match conn.execute(update_sql, params_vec) {
                                Ok(rows) => {
                                    if rows > 0 {
                                        // info!("Updated images/metadata for {} | {} | {} (rows={})", site_name, series_name, sg_number, rows);
                                        // Treat this as an update event: emit a STAMP line so downstream tools see the refreshed image/metadata
                                        new_stamps_this_page += 1;
                                        seen_stamps.insert(key.clone());
                                        let stamp_line = format!("STAMP;{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{};{}",
                                            stamp_year, month, day, series_name, sg_number, type_, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, local_image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, cleaned_context_details);
                                        all_stamp_data.push(stamp_line);
                                        // Persist image pixel dimensions when available (read from local file if present)
                                        if !local_image_path.is_empty() {
                                            if let Err(e) =
                                                crate::db_utils::persist_image_dimensions_from_file(
                                                    &conn,
                                                    site_name,
                                                    &series_name,
                                                    &sg_number,
                                                    &local_image_path,
                                                )
                                            {
                                                error!("Failed to persist image dimensions for {} {} {}: {}", site_name, series_name, sg_number, e);
                                            }
                                        }
                                    } else {
                                        debug!("--overwrite-images set but no DB row matched for update: {} | {} | {}", site_name, series_name, sg_number);
                                    }
                                }
                                Err(e) => {
                                    error!(
                                        "DB update error for {} {} {}: {}",
                                        site_name, series_name, sg_number, e
                                    );
                                }
                            }
                        } else {
                            // Row already existed and we are not overwriting images.
                            // Ensure we backfill image pixel dimensions when a local image file
                            // is available but the DB columns are still NULL.
                            if !local_image_path.is_empty() {
                                // Check whether dimensions are already set
                                match conn.query_row(
                                    "SELECT image_width_px, image_height_px FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3",
                                    rusqlite::params![site_name, &series_name, &sg_number],
                                    |r| Ok((r.get::<_, Option<i32>>(0)?, r.get::<_, Option<i32>>(1)?)),
                                ) {
                                    Ok((maybe_w, maybe_h)) => {
                                        if maybe_w.is_none() || maybe_h.is_none() {
                                            if let Err(e) = crate::db_utils::persist_image_dimensions_from_file(
                                                &conn,
                                                site_name,
                                                &series_name,
                                                &sg_number,
                                                &local_image_path,
                                            ) {
                                                error!(
                                                    "Failed to persist backfilled image dimensions for {} {} {}: {}",
                                                    site_name, series_name, sg_number, e
                                                );
                                            } else {
                                                if let Some((w_px, h_px)) =
                                                    crate::image_utils::get_image_dimensions(&local_image_path)
                                                {
                                                    debug!("Backfilled image dimensions for {} {} {} -> {}x{}", site_name, series_name, sg_number, w_px, h_px);
                                                }
                                            }

                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to read existing dimensions for {} {} {}: {}", site_name, series_name, sg_number, e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "DB insert error for {} {} {}: {}",
                            site_name, series_name, sg_number, e
                        );
                    }
                }

                if *sequencer < 0 {
                    *sequencer = 0;
                } // dummy use to silence unused warnings when needed
            } // end row loop

            if done {
                break;
            }
        } // end content_div loop

        if new_stamps_this_page == 0 {
            consecutive_no_hits += 1;
        } else {
            consecutive_no_hits = 0;
        }
        if stop_after_no_hits > 0 && consecutive_no_hits >= stop_after_no_hits {
            debug!(
                "Stopping due to {} consecutive pages with no new stamps",
                consecutive_no_hits
            );
            break;
        }

        // Move to the next page unless done
        if done {
            break;
        }
        page_num += 1;
    }

    debug!("Total stamp data lines collected: {}", all_stamp_data.len());
    all_stamp_data
}

/// Backwards-compatible wrapper which constructs the default ReqwestFetcher and
/// calls the injected-fetcher implementation. Callers that wish to inject a
/// test double should call `collect_stampworld_data_with_fetcher` directly.
pub fn collect_stampworld_data(
    sequencer: &mut i32,
    _separator: &str,
    site_name: &str,
    images_directory: &str,
    month_number: &dyn Fn(&str) -> String,
    get_stampworld_image: &dyn Fn(&str, &str, bool) -> ImageStatus,
    base_url: &str,
    start_year: i32,
    end_year: i32,
    overwrite_images: bool,
    page_callback: Option<&dyn Fn(usize)>,
    max_pages: i32,
    stop_after_no_hits: i32,
) -> Vec<String> {
    info!("stampworld::collect_stampworld_data wrapper invoked");
    let fetcher = crate::stampworld::fetch::ReqwestFetcher::new();
    collect_stampworld_data_with_fetcher(
        &fetcher,
        sequencer,
        _separator,
        site_name,
        images_directory,
        month_number,
        get_stampworld_image,
        base_url,
        start_year,
        end_year,
        overwrite_images,
        page_callback,
        max_pages,
        stop_after_no_hits,
    )
}
