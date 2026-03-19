use log::{debug, info, warn};
use rusqlite::{params, Connection};
use scraper::{Html, Selector};
use std::path::Path;
use image::GenericImageView;

#[derive(Debug, Clone, Copy, Default)]
pub struct ColnectImageFetchStats {
    pub attempted: usize,
    pub downloaded: usize,
    pub failed: usize,
    pub skipped_existing: usize,
}

fn extract_file_extension(url: &str) -> &'static str {
    let lower = url.to_ascii_lowercase();
    for ext in [".jpg", ".jpeg", ".png", ".webp"] {
        if lower.contains(ext) {
            return &ext[1..];
        }
    }
    "jpg"
}

fn sanitize_for_filename(v: &str) -> String {
    let mut out = String::with_capacity(v.len());
    for ch in v.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch);
        } else {
            out.push('_');
        }
    }
    out
}

fn is_likely_stamp_image(url: &str) -> bool {
    let u = url.to_ascii_lowercase();
    if u.contains("none-stamps") {
        return false;
    }
    // Skip obvious non-stamp assets.
    if u.contains("/logo") || u.contains("/flag") || u.contains("/icon") || u.contains("sprite") {
        return false;
    }
    // Prefer URLs that look like image files and have stamp-ish path hints.
    let has_image_ext = [".jpg", ".jpeg", ".png", ".webp"].iter().any(|e| u.contains(e));
    let stamp_hint = u.contains("stamp") || u.contains("stamps") || u.contains("philately");
    has_image_ext && stamp_hint
}

fn resolve_url(base: &str, candidate: &str) -> Option<String> {
    let cand = candidate.trim();
    if cand.is_empty() {
        return None;
    }
    if cand.starts_with("http://") || cand.starts_with("https://") {
        return Some(cand.to_string());
    }
    let base_url = reqwest::Url::parse(base).ok()?;
    base_url.join(cand).ok().map(|u| u.to_string())
}

fn extract_colnect_image_url(page_url: &str, html: &str) -> Option<String> {
    let doc = Html::parse_document(html);

    // Highest priority: social meta image tags.
    for selector_str in [
        "meta[property='og:image']",
        "meta[name='twitter:image']",
        "meta[property='twitter:image']",
        "link[rel='image_src']",
    ] {
        let Ok(sel) = Selector::parse(selector_str) else {
            continue;
        };
        for node in doc.select(&sel) {
            let value = node
                .value()
                .attr("content")
                .or_else(|| node.value().attr("href"));
            if let Some(v) = value.and_then(|x| resolve_url(page_url, x)) {
                if is_likely_stamp_image(&v) {
                    return Some(v);
                }
            }
        }
    }

    // Fallback: scan img/src and source/srcset candidates.
    let img_sel = Selector::parse("img").ok();
    if let Some(sel) = img_sel {
        for node in doc.select(&sel) {
            if let Some(src) = node.value().attr("src").and_then(|x| resolve_url(page_url, x)) {
                if is_likely_stamp_image(&src) {
                    return Some(src);
                }
            }
            if let Some(srcset) = node.value().attr("srcset") {
                for part in srcset.split(',') {
                    let first = part.split_whitespace().next().unwrap_or("");
                    if let Some(src) = resolve_url(page_url, first) {
                        if is_likely_stamp_image(&src) {
                            return Some(src);
                        }
                    }
                }
            }
        }
    }

    // Last resort: regex scan for absolute image URLs in HTML/script blocks.
    if let Ok(re_abs) = regex::Regex::new(r#"https?://[^\s\"']+\.(?:jpg|jpeg|png|webp)(?:\?[^\s\"']*)?"#) {
        for m in re_abs.find_iter(html) {
            let candidate = m.as_str();
            if is_likely_stamp_image(candidate) {
                return Some(candidate.to_string());
            }
        }
    }

    // Also try relative image paths and resolve against the page URL.
    if let Ok(re_rel) = regex::Regex::new(r#"(?:/|\./)[^\s\"']+\.(?:jpg|jpeg|png|webp)(?:\?[^\s\"']*)?"#) {
        for m in re_rel.find_iter(html) {
            if let Some(candidate) = resolve_url(page_url, m.as_str()) {
                if is_likely_stamp_image(&candidate) {
                    return Some(candidate);
                }
            }
        }
    }

    None
}

pub fn resolve_colnect_image_url(page_url: &str) -> Option<String> {
    let client = crate::stampworld::fetch::build_client();
    let (_, html) = crate::stampworld::fetch::fetch_text(&client, page_url).ok()?;
    extract_colnect_image_url(page_url, &html)
}

pub fn populate_missing_images_from_colnect_pages(
    conn: &Connection,
    album_name: &str,
) -> Result<ColnectImageFetchStats, Box<dyn std::error::Error>> {
    let mut stats = ColnectImageFetchStats::default();

    let mut stmt = conn.prepare(
        "SELECT rowid,
                COALESCE(sg_number, ''),
                COALESCE(year, 0),
                COALESCE(image_path, ''),
                COALESCE(remote_image_url, ''),
                                COALESCE(original_image_url, ''),
                                COALESCE(colnect_page_url, '')
         FROM stamps
         WHERE (TRIM(COALESCE(image_path, '')) = '' OR COALESCE(image_path, '') LIKE 'images/no_image%')
           AND (
             COALESCE(remote_image_url, '') LIKE '%colnect.com/stamps/stamp/%'
             OR COALESCE(original_image_url, '') LIKE '%colnect.com/stamps/stamp/%'
                         OR COALESCE(colnect_page_url, '') LIKE '%colnect.com/stamps/stamp/%'
           )",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i32>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
        ))
    })?;

    for row in rows {
        let (
            rowid,
            sg_number,
            year,
            image_path,
            remote_image_url,
            original_image_url,
            colnect_page_url,
        ) = row?;

        let page_url = if colnect_page_url.contains("colnect.com/stamps/stamp/") {
            colnect_page_url.clone()
        } else if remote_image_url.contains("colnect.com/stamps/stamp/") {
            remote_image_url.clone()
        } else if original_image_url.contains("colnect.com/stamps/stamp/") {
            original_image_url.clone()
        } else {
            continue;
        };

        let existing_full = if image_path.trim().is_empty() {
            None
        } else {
            Some(format!(
                "Albums/{}/{}",
                album_name.replace(' ', "-"),
                image_path.trim()
            ))
        };
        if let Some(full) = existing_full {
            if Path::new(&full).exists() && !image_path.starts_with("images/no_image") {
                stats.skipped_existing += 1;
                continue;
            }
        }

        stats.attempted += 1;

        let client = crate::stampworld::fetch::build_client();
        let (_, html) = match crate::stampworld::fetch::fetch_text(&client, &page_url) {
            Ok(v) => v,
            Err(e) => {
                stats.failed += 1;
                eprintln!(
                    "[COLNECT_IMG_FALLBACK] fetch page failed sg={} url='{}': {}",
                    sg_number, page_url, e
                );
                warn!(
                    "Failed to fetch Colnect page for sg={} url='{}': {}",
                    sg_number, page_url, e
                );
                continue;
            }
        };

        let Some(image_url) = extract_colnect_image_url(&page_url, &html) else {
            stats.failed += 1;
            eprintln!(
                "[COLNECT_IMG_FALLBACK] no image URL found sg={} page='{}'",
                sg_number, page_url
            );
            warn!(
                "No usable image URL found on Colnect page for sg={} url='{}'",
                sg_number, page_url
            );
            continue;
        };

        let safe_sg = sanitize_for_filename(&sg_number);
        let ext = extract_file_extension(&image_url);
        let file_name = format!("{}-{}-colnect.{}", safe_sg, year, ext);
        let rel_path = format!("images/{}", file_name);
        let full_path = Path::new("Albums")
            .join(album_name.replace(' ', "-"))
            .join("images")
            .join(&file_name);

        if let Err(e) = crate::image_downloader::download_image(&image_url, &full_path, &sg_number) {
            stats.failed += 1;
            eprintln!(
                "[COLNECT_IMG_FALLBACK] download failed sg={} img='{}': {}",
                sg_number, image_url, e
            );
            warn!(
                "Failed to download Colnect-resolved image for sg={} url='{}': {}",
                sg_number, image_url, e
            );
            continue;
        }

        let (width_px, height_px) = match image::open(&full_path) {
            Ok(img) => {
                let (w, h) = img.dimensions();
                (Some(w as i32), Some(h as i32))
            }
            Err(_) => (None, None),
        };

        conn.execute(
            "UPDATE stamps
             SET image_path = ?1,
                 remote_image_url = CASE
                     WHEN TRIM(COALESCE(remote_image_url, '')) = '' OR remote_image_url LIKE '%colnect.com/stamps/stamp/%' THEN ?2
                     ELSE remote_image_url
                 END,
                 original_image_url = CASE
                     WHEN TRIM(COALESCE(original_image_url, '')) = '' THEN ?3
                     ELSE original_image_url
                 END,
                 colnect_page_url = CASE
                     WHEN TRIM(COALESCE(colnect_page_url, '')) = '' THEN ?3
                     ELSE colnect_page_url
                 END,
                 colnect_image_url = CASE
                     WHEN TRIM(COALESCE(colnect_image_url, '')) = '' THEN ?2
                     ELSE colnect_image_url
                 END,
                 image_width_px = COALESCE(?4, image_width_px),
                 image_height_px = COALESCE(?5, image_height_px)
             WHERE rowid = ?6",
            params![
                rel_path,
                image_url,
                page_url,
                width_px,
                height_px,
                rowid
            ],
        )?;

        stats.downloaded += 1;
        eprintln!(
            "[COLNECT_IMG_FALLBACK] downloaded sg={} -> {}",
            sg_number,
            full_path.display()
        );
        debug!(
            "Downloaded Colnect fallback image for sg={} -> {}",
            sg_number,
            full_path.display()
        );
    }

    info!(
        "Colnect fallback image fetch done for album '{}': attempted={}, downloaded={}, failed={}, skipped_existing={}",
        album_name, stats.attempted, stats.downloaded, stats.failed, stats.skipped_existing
    );
    eprintln!(
        "[COLNECT_IMG_FALLBACK] album='{}' attempted={} downloaded={} failed={} skipped_existing={}",
        album_name, stats.attempted, stats.downloaded, stats.failed, stats.skipped_existing
    );

    Ok(stats)
}
