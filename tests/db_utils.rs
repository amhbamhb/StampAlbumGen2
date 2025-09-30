use stamp_album_gen2::db_utils::{sort_stampstubs_inplace, StampStub, insert_stamp, migrate_fix_block_metadata};
use rusqlite::Connection;

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
fn test_numeric_prefix_ordering() {
    let mut v = vec![
        make_stub("10", 1948),
        make_stub("1", 1948),
        make_stub("2", 1948),
        make_stub("4AT", 1948),
        make_stub("4", 1948),
        make_stub("4*", 1948),
        make_stub("A1", 1948),
    ];
    sort_stampstubs_inplace(&mut v);
    let order: Vec<String> = v.into_iter().map(|s| s.sg_number).collect();
    assert_eq!(order, vec!["1", "2", "4", "4AT", "4*", "10", "A1"]);
}

#[test]
fn test_insert_stamp_backfill() {
    let conn = Connection::open_in_memory().expect("open in memory");
    conn.execute(
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
        [],
    ).unwrap();

    conn.execute("INSERT INTO stamps (country, series, sg_number, description, uniq_key) VALUES (?1,?2,?3,?4,?5)", rusqlite::params!["X","S","1","Existing desc","x|s|1"]).unwrap();

    let res = insert_stamp(&conn, "X", "S", 1900, "Jan", "1", "1", "1", "Type", "Denom", "Color", "New Desc", "1.234", "Mint", "Unused", "Used", "Letter", "USD", "images/1.jpg", "https://example.com/1.jpg", "orig.jpg", "WM", "Design", "Engr", "12", "Context");
    assert!(res.is_ok());

    let mut stmt = conn.prepare("SELECT description, stamps_issued, image_path, remote_image_url FROM stamps WHERE country=?1 AND series=?2 AND sg_number=?3").unwrap();
    let row = stmt.query_row(rusqlite::params!["X","S","1"], |r| Ok((r.get::<_,String>(0)?, r.get::<_,String>(1)?, r.get::<_,String>(2)?, r.get::<_,String>(3)?))).unwrap();
    assert_eq!(row.0, "Existing desc");
    assert_eq!(row.1, "1,234");
    assert_eq!(row.2, "images/1.jpg");
    assert_eq!(row.3, "https://example.com/1.jpg");
}

#[test]
fn test_migrate_fix_block_metadata_basic() {
    let conn = Connection::open_in_memory().expect("open in memory");
    conn.execute(
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
        [],
    ).unwrap();

    let bad_block = "Design: Herman Herbert Schwartz\nEngraving: Herman Herbert Schwartz\nPerforation: 12";
    conn.execute("INSERT INTO stamps (country, series, sg_number, design, engraving, perforation, context, currency, uniq_key) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        rusqlite::params!["Canada","S","183", bad_block, bad_block, bad_block, bad_block, "Sell", "canada|s|183"]).unwrap();

    let res = migrate_fix_block_metadata(&conn);
    assert!(res.is_ok());

    let row: (Option<String>, Option<String>, Option<String>, Option<String>) = conn.query_row(
        "SELECT watermark, design, engraving, perforation FROM stamps WHERE sg_number = '183'",
        [],
        |r| Ok((r.get::<_, Option<String>>(0)?, r.get::<_, Option<String>>(1)?, r.get::<_, Option<String>>(2)?, r.get::<_, Option<String>>(3)?)),
    ).unwrap();
    assert_eq!(row.1.unwrap_or_default(), "Herman Herbert Schwartz");
    assert_eq!(row.2.unwrap_or_default(), "Herman Herbert Schwartz");
    assert_eq!(row.3.unwrap_or_default(), "12");

    let cur: String = conn.query_row("SELECT currency FROM stamps WHERE sg_number = '183'", [], |r| r.get(0)).unwrap();
    assert_eq!(cur, "USD");
}
