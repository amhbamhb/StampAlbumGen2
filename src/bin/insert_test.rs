use stamp_album_gen2::db_utils::{init_db, insert_stamp};
use std::env;

fn main() {
    // Allow optional DB path via first arg for flexibility
    let args: Vec<String> = env::args().collect();
    let db_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "Albums/Israel/Israel.db".to_string()
    };

    println!("Test insert: opening DB at {}", db_path);
    let conn = match init_db(&db_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to open DB: {}", e);
            std::process::exit(1);
        }
    };

    // Insert a small synthetic row using the same insert_stamp signature
    let res = insert_stamp(
        &conn,
        "Israel",
        "RUST-TEST-SERIES",
        2025,
        "",
        "0",
        "01",
        "RUST-TEST-1",
        "ST",
        "0",
        "TestColor",
        "Rust test description",
        "0",
        "",
        "",
        "",
        "",
        "USD",
        "images/no_image_placeholder.png",
        "",
        "",
        "",
        "",
        "",
        "",
        "testcontext",
    );

    match res {
        Ok(_) => println!("insert_test: insert returned Ok"),
        Err(e) => eprintln!("insert_test: insert failed: {}", e),
    }
}
