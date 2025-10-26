use rusqlite::Connection;
use stamp_album_db::{get_all_series, get_stamps_by_series, sort_stampstubs_inplace, insert_stamp, migrate_fix_block_metadata, StampStub};

fn make_stub(sg: &str, year: i32) -> StampStub {
    StampStub {
        year,
        month: String::new(),
        month_num: 0,
        day: String::new(),
        sg_number: sg.to_string(),
        r#type: String::new(),
        stamp_type: String::new(),
        denomination: String::new(),
        color: String::new(),
        description: String::new(),
        desc: String::new(),
        catalog_num: sg.to_string(),
        stamps_issued: String::new(),
        mint_condition: String::new(),
        unused: String::new(),
        used: String::new(),
        letter_fdc: String::new(),
        currency: String::new(),
        image_path: String::new(),
        remote_image_url: String::new(),
        original_image_url: String::new(),
        image_width_px: None,
        image_height_px: None,
        watermark: String::new(),
        design: String::new(),
        engraving: String::new(),
        perforation: String::new(),
        series: String::new(),
        context: String::new(),
        multi_perf: false,
    }
}

#[test]
fn test_sort_stampstubs_inplace_basic() {
    let mut v = vec![
        make_stub("10", 1950),
        make_stub("1", 1950),
        make_stub("2", 1950),
        make_stub("4AT", 1950),
        make_stub("4", 1950),
    ];
    sort_stampstubs_inplace(&mut v);
    let order: Vec<String> = v.into_iter().map(|s| s.sg_number).collect();
    assert_eq!(order, vec!["1", "2", "4", "4AT", "10"]);
}

#[test]
fn test_get_all_series_and_get_stamps_by_series() {
    let conn = Connection::open_in_memory().expect("open");
    conn.execute_batch(
        "CREATE TABLE stamps (
            id INTEGER PRIMARY KEY,
            country TEXT,
            series TEXT,
            year INTEGER,
            month TEXT,
            month_num INTEGER,
            day TEXT,
            sg_number TEXT,
            type TEXT,
            denomination TEXT,
            color TEXT,
            description TEXT,
            stamps_issued TEXT,
            mint_condition TEXT,
            unused TEXT,
            used TEXT,
            letter_fdc TEXT,
            currency TEXT,
            image_path TEXT,
            remote_image_url TEXT,
            original_image_url TEXT,
            image_width_px INTEGER,
            image_height_px INTEGER,
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT,
            uniq_key TEXT
        )",
    ).unwrap();

    conn.execute(
        "INSERT INTO stamps (series, year, sg_number, month, month_num, day, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, currency, context, mint_condition, unused, used, letter_fdc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)",
        rusqlite::params!["S1", 1950, "1", "", 0, "", "", "", "", "Desc S1", "1", "", "", "", "", "", "", "", "USD", "", "", "", "", ""],
    ).unwrap();
    conn.execute(
        "INSERT INTO stamps (series, year, sg_number, month, month_num, day, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, currency, context, mint_condition, unused, used, letter_fdc) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)",
        rusqlite::params!["S2", 1951, "1", "", 0, "", "", "", "", "Desc S2", "1", "", "", "", "", "", "", "", "USD", "", "", "", "", ""],
    ).unwrap();

    let series = get_all_series(&conn).expect("get_all_series");
    assert!(series.contains(&"S1".to_string()));
    assert!(series.contains(&"S2".to_string()));

    let stamps = get_stamps_by_series(&conn, "S1", "0").expect("get_stamps_by_series");
    assert_eq!(stamps.len(), 1);
    assert_eq!(stamps[0].sg_number, "1");
}

#[test]
fn test_insert_stamp_and_backfill() {
    let conn = Connection::open_in_memory().expect("open");
    conn.execute_batch(
        "CREATE TABLE stamps (
            id INTEGER PRIMARY KEY,
            country TEXT,
            series TEXT,
            year INTEGER,
            month TEXT,
            month_num INTEGER,
            day TEXT,
            sg_number TEXT,
            type TEXT,
            denomination TEXT,
            color TEXT,
            description TEXT,
            stamps_issued TEXT,
            mint_condition TEXT,
            unused TEXT,
            used TEXT,
            letter_fdc TEXT,
            currency TEXT,
            image_path TEXT,
            remote_image_url TEXT,
            original_image_url TEXT,
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT,
            uniq_key TEXT
        )",
    ).unwrap();

    conn.execute("INSERT INTO stamps (country, series, sg_number, description, uniq_key) VALUES (?1,?2,?3,?4,?5)", rusqlite::params!["X","S","1","Existing desc","x|s|1"]).unwrap();

    let res = insert_stamp(&conn, "X", "S", 1900, "Jan", "1", "1", "1", "Type", "Denom", "Color", "New Desc", "1.234", "Mint", "Unused", "Used", "Letter", "USD", "images/1.jpg", "https://example.com/1.jpg", "orig.jpg", "WM", "Design", "Engr", "12", "Context").unwrap();
    assert!(res);

    let row: (String, String, String, String) = conn.query_row("SELECT description, stamps_issued, image_path, remote_image_url FROM stamps WHERE country=?1 AND series=?2 AND sg_number=?3", rusqlite::params!["X","S","1"], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))).unwrap();
    assert_eq!(row.0, "Existing desc");
    assert_eq!(row.1, "1,234");
    assert_eq!(row.2, "images/1.jpg");
    assert_eq!(row.3, "https://example.com/1.jpg");
}

#[test]
fn test_migrate_fix_block_metadata_local() {
    let conn = Connection::open_in_memory().expect("open");
    conn.execute_batch(
        "CREATE TABLE stamps (id INTEGER PRIMARY KEY, country TEXT, series TEXT, year INTEGER, sg_number TEXT, watermark TEXT, design TEXT, engraving TEXT, perforation TEXT, context TEXT, currency TEXT, uniq_key TEXT)",
    ).unwrap();

    let bad_block = "Design: John Doe\nEngraving: John Doe\nPerforation: 14";
    conn.execute("INSERT INTO stamps (country, series, sg_number, design, engraving, perforation, context, currency, uniq_key) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)", rusqlite::params!["C","S","1", bad_block, bad_block, bad_block, bad_block, "Sell", "c|s|1"]).unwrap();

    let res = migrate_fix_block_metadata(&conn);
    if let Err(e) = &res {
        eprintln!("migrate_fix_block_metadata error: {:?}", e);
    }
    assert!(res.is_ok(), "migration failed");

    let row: (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>) = conn.query_row(
        "SELECT watermark, design, engraving, perforation, currency FROM stamps WHERE sg_number = '1'",
        [],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
    ).unwrap();
    // design/engraving should have been normalized to just the name; perforation to number; currency normalized to USD
    assert_eq!(row.1.unwrap_or_default(), "John Doe");
    assert_eq!(row.2.unwrap_or_default(), "John Doe");
    assert_eq!(row.3.unwrap_or_default(), "14");
    assert_eq!(row.4.unwrap_or_default(), "USD");
}
