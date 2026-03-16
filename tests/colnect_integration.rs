use stamp_album_gen2::colnect::db::get_colnect_info;
use stamp_album_gen2::colnect::ingest::ingest_colnect_data;
use stamp_album_gen2::colnect::parser::parse_csv;
use stamp_album_gen2::db_utils::init_db;
use std::path::Path;

#[test]
fn test_colnect_integration() {
    let db_path = "test_colnect.db";
    let _ = std::fs::remove_file(db_path);

    let conn = init_db(db_path).expect("init_db");

    let csv_path = "en_stamps_csv_list_country_206-Switzerland_year_1972.csv";
    if !Path::new(csv_path).exists() {
        // Skip test if file not found (e.g. in CI)
        println!("Skipping test_colnect_integration: CSV file not found");
        return;
    }

    let stamps = parse_csv(csv_path).expect("parse_csv");
    assert!(!stamps.is_empty());
    println!("Parsed {} stamps", stamps.len());

    ingest_colnect_data(&conn, &stamps).expect("ingest");

    // Verify data
    // One stamp has Sg:CH 824 -> normalized 824
    // Let's check the CSV content for "Sg:CH 824"
    // "Breakdown Service",Switzerland,"Automobile Associations","Mi:CH 964, Sn:CH 540, Yt:CH 895, Sg:CH 824, ...

    let info = get_colnect_info(&conn, "824").expect("get_colnect_info");

    if info.is_none() {
        // Maybe normalization failed or stored differently?
        // Try "CH 824"
        let info2 = get_colnect_info(&conn, "CH 824").expect("get_colnect_info 2");
        if info2.is_some() {
            println!("Found as CH 824");
        } else {
            println!("Not found as 824 or CH 824");
            // Debug: list all sg_numbers
            let mut stmt = conn
                .prepare("SELECT sg_number FROM colnect_stamps")
                .unwrap();
            let rows = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();
            for r in rows {
                println!("Stored SG: {}", r.unwrap());
            }
        }
    }

    assert!(info.is_some(), "Should find stamp with SG 824");
    let info = info.unwrap();
    println!("Fetched info: {:?}", info);

    // Check description
    // "75th anniv. of the touring and automobile clubs of Switzerland: Wrench, Road Sign, Club Emblems  "
    assert!(info.description.unwrap().contains("75th anniv"));

    let _ = std::fs::remove_file(db_path);
}
