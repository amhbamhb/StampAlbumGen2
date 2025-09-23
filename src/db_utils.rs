#![allow(dead_code)]
// Returns a list of distinct series names (ordered by series name)
pub fn get_all_series(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT series FROM stamps ORDER BY series")?;
    let series_iter = stmt.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    })?;
    let mut result = Vec::new();
    for s in series_iter {
        result.push(s?);
    }
    Ok(result)
}

// Returns a list of stamps for a given series (across all years), sorted by SG number
pub fn get_stamps_by_series_only(conn: &rusqlite::Connection, series: &str) -> rusqlite::Result<Vec<StampStub>> {
    let mut stmt = conn.prepare("SELECT year, month, day, sg_number, type, denomination, color, month_num, description, stamps_issued, image_path, remote_image_url, watermark, design, engraving, perforation, series, currency, context FROM stamps WHERE series = ?1 ORDER BY sg_number")?;
    let stamp_iter = stmt.query_map([series], |row| {
            Ok(StampStub {
                year: row.get::<_, i32>(0)?,
                month: row.get(1)?,
                day: row.get(2)?,
                sg_number: row.get(3)?,
                stamp_type: row.get(4)?,
                denomination: row.get(5)?,
                color: row.get(6)?,
                multi_perf: false,
                desc: row.get(8)?,  // description is at index 8
                catalog_num: row.get(3)?,  // Use sg_number for catalog_num
                stamps_issued: row.get(9)?,
                image_path: row.get(10)?,
                remote_image_url: row.get(11)?,
                watermark: row.get(12)?,
                design: row.get(13)?,
                engraving: row.get(14)?,
                perforation: row.get(15)?,
                series: row.get(16)?,
                currency: row.get(17)?,
                context: row.get(18)?,
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
    pub month: String,
    pub day: String,
    pub sg_number: String,
    pub stamp_type: String,
    pub denomination: String,
    pub color: String,
    pub multi_perf: bool,
    pub desc: String,
    pub catalog_num: String,
    pub stamps_issued: String,
    pub image_path: String,
    pub remote_image_url: String,
    pub watermark: String,
    pub design: String,
    pub engraving: String,
    pub perforation: String,
    pub series: String,
    pub currency: String,
    pub context: String,
}
use rusqlite::{params, Connection, Result};
use log::{debug, error};

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
            original_image_url TEXT,
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
            if col_count > 0 && col_count != 26 {
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
                original_image_url TEXT,
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
    original_image_url: &str,
    watermark: &str,
    design: &str,
    engraving: &str,
    perforation: &str,
    context: &str,
) -> Result<()> {
    // Check if stamp already exists
    let exists_sql = "SELECT COUNT(*), image_path, remote_image_url FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3";
    let mut stmt = conn.prepare(exists_sql)?;
    let (count, existing_image_path, existing_remote_url): (i64, String, String) = stmt.query_row(params![country, series, sg_number], |row| {
        Ok((row.get(0)?, row.get(1).unwrap_or(String::new()), row.get(2).unwrap_or(String::new())))
    })?;
    
    if count > 0 {
        // Check if we should update this existing stamp
    // Consider any placeholder that starts with images/no_image_ (including per-stamp placeholders)
    let has_placeholder = existing_image_path.is_empty() || existing_image_path.starts_with("images/no_image");
        let has_new_remote_url = !remote_image_url.is_empty() && remote_image_url != existing_remote_url;
        
        if has_placeholder && has_new_remote_url {
            debug!("Stamp exists but has placeholder image (country='{}', series='{}', sg_number='{}'), updating with new image data", country, series, sg_number);
            
            // Update the existing record with new image information
            let update_sql = "UPDATE stamps SET 
                image_path = ?1, 
                remote_image_url = ?2, 
                original_image_url = ?3,
                year = ?4,
                month = ?5,
                month_num = ?6,
                day = ?7,
                type = ?8,
                denomination = ?9,
                color = ?10,
                description = ?11,
                stamps_issued = ?12,
                mint_condition = ?13,
                unused = ?14,
                used = ?15,
                letter_fdc = ?16,
                currency = ?17,
                watermark = ?18,
                design = ?19,
                engraving = ?20,
                perforation = ?21,
                context = ?22
                WHERE country = ?23 AND series = ?24 AND sg_number = ?25";
            
            let month_num = match month.trim().to_lowercase().as_str() {
                "wm" | "" => 0,
                "january" | "jan" => 1,
                "february" | "feb" => 2,
                "march" | "mar" => 3,
                "april" | "apr" => 4,
                "may" => 5,
                "june" | "jun" => 6,
                "july" | "jul" => 7,
                "august" | "aug" => 8,
                "september" | "sep" | "sept" => 9,
                "october" | "oct" => 10,
                "november" | "nov" => 11,
                "december" | "dec" => 12,
                _ => 0,
            };
            
            match conn.execute(update_sql, params![
                image_path, remote_image_url, original_image_url,
                year, month, month_num, day, type_, denomination, color, description,
                stamps_issued, mint_condition, unused, used, letter_fdc, currency,
                watermark, design, engraving, perforation, context,
                country, series, sg_number
            ]) {
                Ok(_) => {
                    debug!("Successfully updated existing stamp: {} {} {}", country, series, sg_number);
                    return Ok(());
                }
                Err(e) => {
                    error!("Failed to update existing stamp: {} {} {} | Error: {}", country, series, sg_number, e);
                    return Err(e.into());
                }
            }
        } else {
            debug!("Stamp already exists with valid data (country='{}', series='{}', sg_number='{}'), skipping", country, series, sg_number);
            return Ok(());
        }
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
    let sql = "INSERT INTO stamps (country, series, year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25)";
    let params_vec = vec![
        country.to_string(), series.to_string(), year.to_string(), month_clean.to_string(), month_num.to_string(), day.to_string(), sg_number.to_string(), type_.to_string(), denomination.to_string(), color.to_string(), description.to_string(), stamps_issued.to_string(), mint_condition.to_string(), unused.to_string(), used.to_string(), letter_fdc.to_string(), currency.to_string(), image_path.to_string(), remote_image_url.to_string(), original_image_url.to_string(), watermark.to_string(), design.to_string(), engraving.to_string(), perforation.to_string(), context.to_string()
    ];
        debug!("SQL: {}", sql);
        debug!("Params: {:?}", params_vec);
    match conn.execute(sql, params![country, series, year, month_clean, month_num, day, sg_number, type_, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context]) {
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
