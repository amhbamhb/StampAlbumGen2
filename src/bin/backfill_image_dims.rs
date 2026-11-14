use flexi_logger::{Duplicate, FileSpec, Logger};
use log::{debug, error, info, warn};
use std::env;
use std::path::{Path, PathBuf};

fn print_usage(prog: &str) {
    eprintln!("Usage: {} [--apply] [--albums-dir DIR] [--verbose]", prog);
    eprintln!("  --apply       : actually write updates to DB (default: dry-run)");
    eprintln!("  --albums-dir  : directory containing Albums (default: Albums)");
    eprintln!("  --verbose     : emit debug logs");
}

fn resolve_dimensions(images_dir: &Path, image_path: &str) -> Option<(u32, u32)> {
    if image_path.is_empty() {
        return None;
    }
    // try as-given first
    if let Some(d) = stamp_album_gen2::image_utils::get_image_dimensions(image_path) {
        return Some(d);
    }
    // Try relative to album images dir
    let rest = image_path.strip_prefix("images/").unwrap_or(image_path);
    let alt = images_dir.join(rest);
    stamp_album_gen2::image_utils::get_image_dimensions(alt)
}

fn process_db(db_path: &Path, apply: bool) -> Result<usize, Box<dyn std::error::Error>> {
    info!("Processing DB: {}", db_path.display());
    let db_path_str = db_path.to_str().ok_or("Invalid UTF-8 path")?;
    let conn = stamp_album_gen2::db_utils::init_db(db_path_str)?;
    let album_dir = db_path.parent().unwrap_or_else(|| Path::new("."));
    let images_dir = album_dir.join("images");

    let mut stmt = conn.prepare(
        "SELECT country, series, sg_number, image_path FROM stamps WHERE image_path IS NOT NULL AND image_path != '' AND (image_width_px IS NULL OR image_height_px IS NULL)",
    )?;
    let mut rows = stmt.query([])?;
    let mut updated = 0usize;
    while let Some(r) = rows.next()? {
        let country: String = r.get(0)?;
        let series: String = r.get(1)?;
        let sg: String = r.get(2)?;
        let image_path: String = r.get(3)?;
        debug!(
            "Found NULL-dim row: {} | {} | {} -> {}",
            country, series, sg, image_path
        );
        if let Some((w, h)) = resolve_dimensions(&images_dir, &image_path) {
            info!("Resolved {} -> {}x{}", image_path, w, h);
            if apply {
                match stamp_album_gen2::db_utils::update_image_dimensions(
                    &conn,
                    &country,
                    &series,
                    &sg,
                    Some(w as i32),
                    Some(h as i32),
                ) {
                    Ok(n) => {
                        debug!("DB update affected {} rows", n);
                        updated += 1;
                    }
                    Err(e) => {
                        error!("Failed to update {} {} {}: {}", country, series, sg, e);
                    }
                }
            } else {
                info!(
                    "Dry-run: would update {}|{}|{} -> {}x{}",
                    country, series, sg, w, h
                );
            }
        } else {
            warn!(
                "Could not resolve image dimensions for {} (album images dir: {})",
                image_path,
                images_dir.display()
            );
        }
    }
    Ok(updated)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args
        .first()
        .map(|s| s.as_str())
        .unwrap_or("backfill_image_dims");
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage(prog);
        return;
    }
    let apply = args.iter().any(|a| a == "--apply");
    let verbose = args.iter().any(|a| a == "--verbose");
    let mut albums_dir = PathBuf::from("Albums");
    if let Some(pos) = args.iter().position(|a| a == "--albums-dir") {
        if let Some(v) = args.get(pos + 1) {
            albums_dir = PathBuf::from(v);
        } else {
            print_usage(prog);
            return;
        }
    }

    if verbose {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }
    }
    // Initialize logger to write debug/info to a file under Logs and only duplicate errors to stderr
    let ts = stamp_album_gen2::time_utils::formatted_now("%Y%m%d-%H%M%S");
    let basename = format!("backfill-{}", ts);
    match Logger::try_with_env_or_str(std::env::var("RUST_LOG").as_deref().unwrap_or("info")) {
        Ok(l) => {
            if let Err(e) = l
                .log_to_file(
                    FileSpec::default()
                        .directory("Logs")
                        .basename(&basename)
                        .suffix("log"),
                )
                .duplicate_to_stderr(Duplicate::Error)
                .start()
            {
                eprintln!("Failed to start logger: {}", e);
            }
        }
        Err(e) => eprintln!("Logger init failed: {}", e),
    }

    info!(
        "Backfill runner starting (albums_dir={}, apply={})",
        albums_dir.display(),
        apply
    );

    let mut total_updated = 0usize;
    if !albums_dir.exists() {
        error!("Albums dir does not exist: {}", albums_dir.display());
        std::process::exit(1);
    }

    if let Ok(dir_entries) = std::fs::read_dir(&albums_dir) {
        for entry in dir_entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Expect DB at Albums/<name>/<name>.db
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let db_path = path.join(format!("{}.db", name));
                    if db_path.exists() {
                        match process_db(&db_path, apply) {
                            Ok(n) => total_updated += n,
                            Err(e) => error!("Error processing {}: {}", db_path.display(), e),
                        }
                    } else {
                        debug!(
                            "No DB found for album {} at expected path {}",
                            name,
                            db_path.display()
                        );
                    }
                }
            }
        }
    } else {
        error!("Failed to read Albums dir {}", albums_dir.display());
    }

    info!("Backfill completed. total_updated={}", total_updated);
}
