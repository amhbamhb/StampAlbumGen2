use stamp_album_gen2::colnect::ingest::ingest_colnect_data;
use stamp_album_gen2::colnect::parser::parse_csv;
use stamp_album_gen2::db_utils::init_db;
use std::path::Path;

#[test]
fn ingest_switzerland_1972() {
    let db_path = "Albums/Switzerland/Switzerland.db";
    if !Path::new(db_path).exists() {
        println!("Skipping ingest: DB not found at {}", db_path);
        return;
    }

    let conn = init_db(db_path).expect("init_db");

    let csv_path = "en_stamps_csv_list_country_206-Switzerland_year_1972.csv";
    if !Path::new(csv_path).exists() {
        println!("Skipping ingest: CSV not found at {}", csv_path);
        return;
    }

    let stamps = parse_csv(csv_path).expect("parse_csv");
    println!("Parsed {} stamps", stamps.len());

    ingest_colnect_data(&conn, &stamps).expect("ingest");
    println!("Successfully ingested Colnect data into {}", db_path);
}
