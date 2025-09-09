// Returns a list of series (name, year, month, day, is_perforated, perforation)
pub fn get_all_series(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<SeriesStub>> {
    let mut stmt = conn.prepare("SELECT DISTINCT series, year, month, day, perforation FROM stamps ORDER BY year, series")?;
    let series_iter = stmt.query_map([], |row| {
        Ok(SeriesStub {
            name: row.get(0)?,
            year: row.get::<_, i32>(1)?,
            month: row.get(2)?,
            day: row.get(3)?,
            is_perforated: false, // TODO: parse from data if available
            perforation: row.get(4)?,
        })
    })?;
    let mut result = Vec::new();
    for s in series_iter {
        result.push(s?);
    }
    Ok(result)
}

#[derive(Debug)]
pub struct SeriesStub {
    pub name: String,
    pub year: i32,
    pub month: String,
    pub day: String,
    pub is_perforated: bool,
    pub perforation: String,
}

// Returns a list of stamps for a given series and year
pub fn get_stamps_by_series(conn: &rusqlite::Connection, series: &str, year: &str) -> rusqlite::Result<Vec<StampStub>> {
    let mut stmt = conn.prepare("SELECT year, sg_number, type, denomination, color, 0, description, sg_number, '', image_path, watermark, design, engraving FROM stamps WHERE series = ?1 AND year = ?2 ORDER BY sg_number")?;
    let stamp_iter = stmt.query_map([series, year], |row| {
            let multi_perf_int: i32 = row.get(5)?;
            Ok(StampStub {
                year: row.get::<_, i32>(0)?,
                field1: row.get(1)?,
                field2: row.get(2)?,
                field3: row.get(3)?,
                color: row.get(4)?,
                multi_perf: multi_perf_int != 0,
                desc: row.get(6)?,
                catalog_num: row.get(7)?,
                field4: row.get(8)?,
                image_path: row.get(9)?,
                watermark: row.get(10)?,
                design: row.get(11)?,
                engraving: row.get(12)?,
            })
    })?;
    let mut result = Vec::new();
    for s in stamp_iter {
        result.push(s?);
    }
    Ok(result)
}

#[derive(Debug)]
pub struct StampStub {
    pub year: i32,
    pub field1: String,
    pub field2: String,
    pub field3: String,
    pub color: String,
    pub multi_perf: bool,
    pub desc: String,
    pub catalog_num: String,
    pub field4: String,
    pub image_path: String,
    pub watermark: String,
    pub design: String,
    pub engraving: String,
}
use rusqlite::{params, Connection, Result};
use log::{debug, info, error};

pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    debug!("Opened database at {}", db_path);
    // Always create the table first
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stamps (
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
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT
        )",
        [],
    )?;
    // Print column names and count after creation
    {
        let pragma = conn.prepare("PRAGMA table_info(stamps)");
        if let Ok(mut stmt) = pragma {
            let mut rows = stmt.query([])?;
            let mut col_names = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                let name: String = row.get(1)?;
                col_names.push(name);
            }
            // eprintln!("[DEBUG] Stamps table columns after creation: {:?} (count: {})", col_names, col_names.len());
        }
    }
    // Now check column count and drop/recreate if needed
    let mut needs_recreate = false;
    let mut col_count = 0;
    {
        let pragma = conn.prepare("PRAGMA table_info(stamps)");
        if let Ok(mut stmt) = pragma {
            let mut rows = stmt.query([])?;
            let mut col_names = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                let name: String = row.get(1)?;
                col_names.push(name.clone());
                col_count += 1;
            }
            debug!("Stamps table columns before possible drop: {:?} (count: {})", col_names, col_count);
            if col_count > 0 && col_count != 25 {
                needs_recreate = true;
            }
        }
    }
    if needs_recreate {
        debug!("Dropping outdated stamps table ({} columns)...", col_count);
        conn.execute("DROP TABLE IF EXISTS stamps", [])?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS stamps (
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
                watermark TEXT,
                design TEXT,
                engraving TEXT,
                perforation TEXT,
                context TEXT
            )",
            [],
        )?;
            debug!("Stamps table recreated.");
        // Print column names and count after recreation
        let pragma = conn.prepare("PRAGMA table_info(stamps)");
        if let Ok(mut stmt) = pragma {
            let mut rows = stmt.query([])?;
            let mut col_names = Vec::new();
            while let Ok(Some(row)) = rows.next() {
                let name: String = row.get(1)?;
                col_names.push(name);
            }
                debug!("Stamps table columns after recreation: {:?} (count: {})", col_names, col_names.len());
        }
    }
    // Set this to true to enable debug output, false to disable
    const DEBUG: bool = true;
    if DEBUG {
            debug!("Stamps table ensured.");
    }
    Ok(conn)
}

pub fn create_stamps_table(conn: &Connection) -> Result<()> {
        debug!("Ensuring stamps table exists...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stamps (
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
            watermark TEXT,
            design TEXT,
            engraving TEXT,
            perforation TEXT,
            context TEXT
        )",
        [],
    )?;
        debug!("Stamps table ensured.");
    Ok(())
}

pub fn insert_stamp(
    conn: &Connection,
    country: &str,
    series: &str,
    year: i32,
    month: &str,
    day: &str,
    sg_number: &str,
    type_: &str,
    denomination: &str,
    color: &str,
    description: &str,
    stamps_issued: &str,
    mint_condition: &str,
    unused: &str,
    used: &str,
    letter_fdc: &str,
    currency: &str,
    image_path: &str,
    remote_image_url: &str,
    watermark: &str,
    design: &str,
    engraving: &str,
    perforation: &str,
    context: &str,
) -> Result<()> {
    // Check if stamp already exists
    let exists_sql = "SELECT COUNT(*) FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3";
    let mut stmt = conn.prepare(exists_sql)?;
    let count: i64 = stmt.query_row(params![country, series, sg_number], |row| row.get(0))?;
    
    if count > 0 {
        debug!("Stamp already exists (country='{}', series='{}', sg_number='{}'), skipping insert", country, series, sg_number);
        return Ok(());
    }
    
    debug!("--- STAMP INSERT ATTEMPT ---");
    debug!("[insert_stamp] country='{}'", country);
    debug!("[insert_stamp] series='{}'", series);
    debug!("[insert_stamp] year={}", year);
    debug!("[insert_stamp] month='{}'", month);
    debug!("[insert_stamp] day='{}'", day);
    debug!("[insert_stamp] sg_number='{}'", sg_number);
    debug!("[insert_stamp] type_='{}'", type_);
    debug!("[insert_stamp] denomination='{}'", denomination);
    debug!("[insert_stamp] color='{}'", color);
    debug!("[insert_stamp] description='{}'", description);
    debug!("[insert_stamp] stamps_issued='{}'", stamps_issued);
    debug!("[insert_stamp] mint_condition='{}'", mint_condition);
    debug!("[insert_stamp] unused='{}'", unused);
    debug!("[insert_stamp] used='{}'", used);
    debug!("[insert_stamp] letter_fdc='{}'", letter_fdc);
    debug!("[insert_stamp] currency='{}'", currency);
    debug!("[insert_stamp] image_path='{}'", image_path);
    debug!("[insert_stamp] remote_image_url='{}'", remote_image_url);
    debug!("[insert_stamp] watermark='{}'", watermark);
    debug!("[insert_stamp] design='{}'", design);
    debug!("[insert_stamp] engraving='{}'", engraving);
    debug!("[insert_stamp] perforation='{}'", perforation);
    debug!("[insert_stamp] context='{}'", context);
    let month_clean = match month.trim().to_lowercase().as_str() {
        "wm" | "" => "",
        "january" | "jan" => "January",
        "february" | "feb" => "February",
        "march" | "mar" => "March",
        "april" | "apr" => "April",
        "may" => "May",
        "june" | "jun" => "June",
        "july" | "jul" => "July",
        "august" | "aug" => "August",
        "september" | "sep" | "sept" => "September",
        "october" | "oct" => "October",
        "november" | "nov" => "November",
        "december" | "dec" => "December",
        _ => "",
    };
    let month_num = match month_clean {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => 0,
    };
    let sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, watermark, design, engraving, perforation, context) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24)";
    let params_vec = vec![
        country.to_string(), series.to_string(), year.to_string(), month_clean.to_string(), month_num.to_string(), day.to_string(), sg_number.to_string(), type_.to_string(), denomination.to_string(), color.to_string(), description.to_string(), stamps_issued.to_string(), mint_condition.to_string(), unused.to_string(), used.to_string(), letter_fdc.to_string(), currency.to_string(), image_path.to_string(), remote_image_url.to_string(), watermark.to_string(), design.to_string(), engraving.to_string(), perforation.to_string(), context.to_string()
    ];
        debug!("SQL: {}", sql);
        debug!("Params: {:?}", params_vec);
    match conn.execute(sql, params![country, series, year, month_clean, month_num, day, sg_number, type_, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, watermark, design, engraving, perforation, context]) {
        Ok(_) => {
                debug!("Insert succeeded");
        }
        Err(e) => {
                error!("Insert failed: {}", e);
                error!("Params: {:?}", params_vec);
        }
    }
    Ok(())
}

pub fn get_stamps_by_country_series(conn: &Connection, country: &str, series: &str) -> Result<Vec<(i32, String, String, i32, String, i32, String, String, String, String, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, country, series, year, month, month_num, day, description, image_path, remote_image_url, perforation, context FROM stamps WHERE country = ?1 AND series = ?2 ORDER BY year"
    )?;
    let stamp_iter = stmt.query_map(params![country, series], |row| {
        Ok((
            row.get(0)?, // id
            row.get(1)?, // country
            row.get(2)?, // series
            row.get(3)?, // year
            row.get(4)?, // month
            row.get(5)?, // month_num
            row.get(6)?, // day
            row.get(7)?, // description
            row.get(8)?, // image_path
            row.get(9)?, // remote_image_url
            row.get(10)?, // perforation
            row.get(11)? // context
        ))
    })?;
    let mut results = Vec::new();
    for stamp in stamp_iter {
        results.push(stamp?);
    }
    Ok(results)
}
