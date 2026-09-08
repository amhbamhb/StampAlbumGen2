use stamp_album_gen2::db_utils::{init_db, insert_stamp_from_struct, StampInsert};
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
    let si = StampInsert {
        country: "Israel".to_string(),
        series: "RUST-TEST-SERIES".to_string(),
        year: 2025,
        month: "".to_string(),
        month_num: "0".to_string(),
        day: "01".to_string(),
        sg_number: "RUST-TEST-1".to_string(),
        type_: "ST".to_string(),
        denomination: "0".to_string(),
        color: "TestColor".to_string(),
        description: "Rust test description".to_string(),
        stamps_issued: "0".to_string(),
        mint_condition: "".to_string(),
        unused: "".to_string(),
        used: "".to_string(),
        letter_fdc: "".to_string(),
        currency: "USD".to_string(),
        image_path: "images/no_image_placeholder.png".to_string(),
        remote_image_url: "".to_string(),
        original_image_url: "".to_string(),
        watermark: "".to_string(),
        design: "".to_string(),
        engraving: "".to_string(),
        perforation: "".to_string(),
        context: "testcontext".to_string(),
    };
    let res = insert_stamp_from_struct(&conn, &si);

    match res {
        Ok(_) => println!("insert_test: insert returned Ok"),
        Err(e) => eprintln!("insert_test: insert failed: {}", e),
    }
}
