use rusqlite::params;
use std::env;
use std::path::Path;

fn print_usage() {
    eprintln!(
        "Usage: backfill_image_dims --db-path <DB_PATH> [--images-dir <IMAGES_DIR>] [--apply]"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        std::process::exit(1);
    }
    let mut db_path = String::new();
    let mut images_dir: Option<String> = None;
    let mut apply = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--db-path" => {
                i += 1;
                if i < args.len() {
                    db_path = args[i].clone();
                }
            }
            "--images-dir" => {
                i += 1;
                if i < args.len() {
                    images_dir = Some(args[i].clone());
                }
            }
            "--apply" => {
                apply = true;
            }
            _ => {}
        }
        i += 1;
    }
    if db_path.is_empty() {
        eprintln!("--db-path is required");
        print_usage();
        std::process::exit(2);
    }

    println!("Opening DB: {}", db_path);
    let conn = match rusqlite::Connection::open(&db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open DB {}: {}", db_path, e);
            std::process::exit(3);
        }
    };

    // Run migration from stamp_album_db crate to ensure columns exist
    match stamp_album_db::migrate_stamps_table(&conn) {
        Ok(_) => println!("Migration complete."),
        Err(e) => {
            eprintln!("Migration failed: {}", e);
            std::process::exit(4);
        }
    }

    // Query rows needing dimensions
    let mut stmt = conn.prepare("SELECT id, image_path FROM stamps WHERE image_path IS NOT NULL AND image_path != '' AND (image_width_px IS NULL OR image_height_px IS NULL)").expect("prepare select");
    let mut rows = stmt.query([]).expect("query");
    let mut count = 0;
    while let Ok(Some(r)) = rows.next() {
        let id: i64 = r.get(0).unwrap_or(0);
        let image_path: String = r.get(1).unwrap_or_default();
        let candidates = if let Some(ref dir) = images_dir {
            vec![
                image_path.clone(),
                format!("{}/{}", dir, image_path),
                format!(
                    "{}/{}",
                    dir,
                    Path::new(&image_path)
                        .file_name()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_default()
                ),
            ]
        } else {
            vec![
                image_path.clone(),
                format!("images/{}", image_path),
                image_path.clone(),
            ]
        };
        let mut found: Option<(i32, i32)> = None;
        for p in candidates {
            if p.is_empty() {
                continue;
            }
            if Path::new(&p).exists() {
                if let Some((w, h)) = stamp_album_gen2::image_utils::get_image_dimensions(&p) {
                    found = Some((w as i32, h as i32));
                    break;
                }
            }
        }
        if let Some((w, h)) = found {
            println!(
                "Found dims for id={} path='{}' => {}x{}",
                id, image_path, w, h
            );
            if apply {
                match conn.execute(
                    "UPDATE stamps SET image_width_px = ?1, image_height_px = ?2 WHERE id = ?3",
                    params![w, h, id],
                ) {
                    Ok(rows_updated) => println!("Updated id {} (rows {})", id, rows_updated),
                    Err(e) => eprintln!("Failed to update id {}: {}", id, e),
                }
            }
            count += 1;
        } else {
            println!("No local image found for id={} path='{}'", id, image_path);
        }
    }
    println!(
        "Backfill scan complete; {} rows with found dims (apply={})",
        count, apply
    );
}
