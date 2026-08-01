use rusqlite::Connection;
use std::env;
use std::path::{Path, PathBuf};

fn resolve_image_path(images_dir: &Path, image_path: &str) -> PathBuf {
    let trimmed = image_path.trim();
    let rest = if let Some(stripped) = trimmed.strip_prefix("images/") {
        stripped
    } else if let Some(stripped) = trimmed.strip_prefix("images\\") {
        stripped
    } else {
        trimmed
    };

    images_dir.join(rest.replace('\\', "/"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let db_path = args.next().ok_or("Usage: inspect_image_paths <db_path>")?;
    let db_path = PathBuf::from(db_path);
    let album_dir = db_path.parent().ok_or("DB path has no parent directory")?;
    let images_dir = album_dir.join("images");

    let conn = Connection::open(&db_path)?;
    let mut stmt = conn.prepare("SELECT COALESCE(image_path, '') FROM stamps ORDER BY rowid")?;

    let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;

    let mut total = 0usize;
    let mut empty = 0usize;
    let mut placeholder = 0usize;
    let mut backslash = 0usize;
    let mut slash = 0usize;
    let mut existing = 0usize;
    let mut missing = 0usize;
    let mut samples_missing: Vec<String> = Vec::new();
    let mut samples_existing: Vec<String> = Vec::new();

    for row in rows {
        let image_path = row?;
        total += 1;
        let trimmed = image_path.trim();
        if trimmed.is_empty() {
            empty += 1;
            continue;
        }
        if trimmed.to_ascii_lowercase().starts_with("images/no_image") {
            placeholder += 1;
        }
        if trimmed.contains('\\') {
            backslash += 1;
        }
        if trimmed.contains('/') {
            slash += 1;
        }

        let full = resolve_image_path(&images_dir, trimmed);
        if full.exists() {
            existing += 1;
            if samples_existing.len() < 5 {
                samples_existing.push(format!("{} => {}", trimmed, full.display()));
            }
        } else {
            missing += 1;
            if samples_missing.len() < 10 {
                samples_missing.push(format!("{} => {}", trimmed, full.display()));
            }
        }
    }

    println!("DB={}", db_path.display());
    println!("IMAGES_DIR={}", images_dir.display());
    println!("TOTAL_ROWS={}", total);
    println!("EMPTY_IMAGE_PATH={}", empty);
    println!("PLACEHOLDER_IMAGE_PATH={}", placeholder);
    println!("PATHS_WITH_BACKSLASH={}", backslash);
    println!("PATHS_WITH_SLASH={}", slash);
    println!("RESOLVE_EXISTING={}", existing);
    println!("RESOLVE_MISSING={}", missing);

    if !samples_existing.is_empty() {
        println!("EXISTING_SAMPLES:");
        for sample in samples_existing {
            println!("  {}", sample);
        }
    }

    if !samples_missing.is_empty() {
        println!("MISSING_SAMPLES:");
        for sample in samples_missing {
            println!("  {}", sample);
        }
    }

    Ok(())
}
