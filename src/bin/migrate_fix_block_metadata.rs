use std::env;
use std::path::Path;
use chrono::Local;
fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: migrate_fix_block_metadata <path-to-db>");
        std::process::exit(2);
    }
    let db = &args[1];
    if !Path::new(db).exists() {
        eprintln!("DB path does not exist: {}", db);
        std::process::exit(3);
    }
    let ts = Local::now().format("%Y%m%d%H%M%S").to_string();
    let bak = format!("{}.bak.{}", db, ts);
    match std::fs::copy(db, &bak) {
        Ok(_) => eprintln!("Backup created: {}", bak),
        Err(e) => { eprintln!("Failed to create backup {}: {}", bak, e); std::process::exit(4); }
    }
    // open DB and run migration
    match rusqlite::Connection::open(db) {
        Ok(conn) => {
            match stamp_album_gen2::db_utils::migrate_fix_block_metadata(&conn) {
                Ok(_) => eprintln!("Migration completed successfully"),
                Err(e) => { eprintln!("Migration failed: {}", e); std::process::exit(5); }
            }
        }
        Err(e) => { eprintln!("Failed to open DB {}: {}", db, e); std::process::exit(6); }
    }
}
