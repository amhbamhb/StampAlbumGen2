// use chrono::Local;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Require the DB path as the first argument to avoid hard-coded values
    let db_path = std::env::args()
        .nth(1)
        .expect("Usage: migrate_stamps <db_path>");

    // Remove any existing backup files matching the pattern (keep only one backup)
    let parent = Path::new(&db_path)
        .parent()
        .ok_or("DB path has no parent")?
        .to_path_buf();
    let file_name = Path::new(&db_path)
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("Invalid DB path filename")?
        .to_string();
    let bak_prefix = format!("{}.bak", file_name);
    for entry in fs::read_dir(&parent)? {
        let entry = entry?;
        let p = entry.path();
        if let Some(fname) = p.file_name().and_then(|s| s.to_str()) {
            if fname.starts_with(&bak_prefix) {
                let full = parent.join(fname);
                println!("Removing existing backup: {}", full.display());
                let _ = fs::remove_file(full);
            }
        }
    }

    // Create new timestamped backup
    let ts = stamp_album_gen2::time_utils::formatted_now("%Y%m%d%H%M%S");
    let backup_name = format!("{}.bak.{}", file_name, ts);
    let backup_path = parent.join(&backup_name);
    println!("Creating backup {} -> {}", db_path, backup_path.display());
    fs::copy(&db_path, &backup_path)?;

    // Open DB via the library init_db so any pre-init work happens
    let conn = stamp_album_gen2::db_utils::init_db(&db_path)?;

    // Run migration
    println!("Running migrate_stamps_table...");
    match stamp_album_gen2::db_utils::migrate_stamps_table(&conn) {
        Ok(_) => println!("Migration completed OK"),
        Err(e) => println!("Migration failed: {}", e),
    }

    // Print summary info
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM stamps", [], |r| r.get(0))?;
    println!("Stamps row count after migration: {}", count);

    // List columns
    let mut stmt = conn.prepare("PRAGMA table_info(stamps)")?;
    let mut rows = stmt.query([])?;
    println!("Stamps columns:");
    while let Ok(Some(r)) = rows.next() {
        let cid: i64 = r.get(0)?;
        let name: String = r.get(1)?;
        let ty: String = r.get(2)?;
        println!("  {}: {} ({})", cid, name, ty);
    }

    // List indexes
    let mut idx_stmt = conn.prepare("PRAGMA index_list(stamps)")?;
    let mut idx_rows = idx_stmt.query([])?;
    println!("Indexes on stamps:");
    while let Ok(Some(r)) = idx_rows.next() {
        let seq: i64 = r.get(0)?;
        let name: String = r.get(1)?;
        let unique: i64 = r.get(2)?;
        println!("  {}: {} (unique={})", seq, name, unique);
    }

    Ok(())
}
