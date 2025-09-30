// Minimal, self-contained DB utilities moved out of the main crate.
use rusqlite::{params, Connection, Result};
use regex::Regex;
use log::{debug, error};

#[derive(Debug, Clone)]
pub struct StampStub {
    pub year: i32,
    pub month: String,
    pub month_num: i32,
    pub day: String,
    pub sg_number: String,
    pub r#type: String,
    pub stamp_type: String,
    pub denomination: String,
    pub color: String,
    pub description: String,
    pub desc: String,
    pub catalog_num: String,
    pub stamps_issued: String,
    pub mint_condition: String,
    pub unused: String,
    pub used: String,
    pub letter_fdc: String,
    pub currency: String,
    pub image_path: String,
    pub remote_image_url: String,
    pub original_image_url: String,
    pub watermark: String,
    pub design: String,
    pub engraving: String,
    pub perforation: String,
    pub series: String,
    pub context: String,
    pub multi_perf: bool,
}

pub fn get_all_series(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT DISTINCT series FROM stamps ORDER BY series")?;
    let iter = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;
    let mut res = Vec::new();
    for s in iter {
        res.push(s?);
    }
    Ok(res)
}

fn leading_int(s: &str) -> Option<i64> {
    let mut digits = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() { digits.push(ch); } else { break; }
    }
    if digits.is_empty() { None } else { digits.parse::<i64>().ok() }
}

fn suffix_key(s: &str) -> String {
    let mut chars = s.chars();
    while let Some(c) = chars.clone().next() {
        if c.is_ascii_digit() { chars.next(); } else { break; }
    }
    let rem: String = chars.collect();
    rem.chars().map(|c| if c.is_ascii_alphabetic() { c.to_ascii_lowercase() } else if c == '*' { '~' } else { c.to_ascii_lowercase() }).collect()
}

pub fn sort_stampstubs_inplace(v: &mut Vec<StampStub>) {
    v.sort_by(|a, b| {
        let year_cmp = a.year.cmp(&b.year);
        if year_cmp != std::cmp::Ordering::Equal { return year_cmp; }
        match (leading_int(&a.sg_number), leading_int(&b.sg_number)) {
            (Some(ai), Some(bi)) => {
                let cmp = ai.cmp(&bi);
                if cmp != std::cmp::Ordering::Equal { return cmp; }
                let a_suf = suffix_key(&a.sg_number);
                let b_suf = suffix_key(&b.sg_number);
                let a_empty = a_suf.is_empty();
                let b_empty = b_suf.is_empty();
                if a_empty != b_empty { return if a_empty { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }; }
                a_suf.cmp(&b_suf).then_with(|| a.sg_number.cmp(&b.sg_number))
            }
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.sg_number.cmp(&b.sg_number),
        }
    });
}

pub fn get_stamps_by_series(conn: &Connection, series: &str, year: &str) -> Result<Vec<StampStub>> {
    let mut result: Vec<StampStub> = Vec::new();
    if year.trim().is_empty() || year == "0" {
        let mut stmt = conn.prepare("SELECT year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc FROM stamps WHERE series = ?1 ORDER BY year, sg_number")?;
        let rows = stmt.query_map([series], |row| {
            Ok(StampStub {
                year: row.get::<_, i32>(0)?,
                month: row.get(1)?,
                month_num: row.get::<_, i32>(2)?,
                day: row.get(3)?,
                sg_number: row.get(4)?,
                r#type: row.get(5)?,
                stamp_type: row.get(5)?,
                denomination: row.get(6)?,
                color: row.get(7)?,
                description: row.get::<_, String>(8)?,
                desc: row.get(8)?,
                catalog_num: row.get(4)?,
                stamps_issued: row.get(9)?,
                image_path: row.get(10)?,
                remote_image_url: row.get(11)?,
                original_image_url: row.get(12)?,
                watermark: row.get(13)?,
                design: row.get(14)?,
                engraving: row.get(15)?,
                perforation: row.get(16)?,
                series: row.get(17)?,
                currency: row.get(18)?,
                context: row.get(19)?,
                mint_condition: row.get(20)?,
                unused: row.get(21)?,
                used: row.get(22)?,
                letter_fdc: row.get(23)?,
                multi_perf: false,
            })
        })?;
        for s in rows { result.push(s?); }
    } else {
        let mut stmt = conn.prepare("SELECT year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc FROM stamps WHERE series = ?1 AND year = ?2 ORDER BY sg_number")?;
        let rows = stmt.query_map([series, year], |row| {
            Ok(StampStub {
                year: row.get::<_, i32>(0)?,
                month: row.get(1)?,
                month_num: row.get::<_, i32>(2)?,
                day: row.get(3)?,
                sg_number: row.get(4)?,
                r#type: row.get(5)?,
                stamp_type: row.get(5)?,
                denomination: row.get(6)?,
                color: row.get(7)?,
                description: row.get::<_, String>(8)?,
                desc: row.get(8)?,
                catalog_num: row.get(4)?,
                stamps_issued: row.get(9)?,
                image_path: row.get(10)?,
                remote_image_url: row.get(11)?,
                original_image_url: row.get(12)?,
                watermark: row.get(13)?,
                design: row.get(14)?,
                engraving: row.get(15)?,
                perforation: row.get(16)?,
                series: row.get(17)?,
                currency: row.get(18)?,
                context: row.get(19)?,
                mint_condition: row.get(20)?,
                unused: row.get(21)?,
                used: row.get(22)?,
                letter_fdc: row.get(23)?,
                multi_perf: false,
            })
        })?;
        for s in rows { result.push(s?); }
    }
    sort_stampstubs_inplace(&mut result);
    Ok(result)
}

pub fn init_db(db_path: &str) -> Result<Connection> {
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    let conn = Connection::open(db_path)?;
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
            context TEXT,
            uniq_key TEXT
        )",
        [],
    )?;
    Ok(conn)
}

// Stubs for heavy migrations; keep compatibility but avoid depending on main crate internals
pub fn migrate_stamps_table(_conn: &Connection) -> Result<()> { Ok(()) }
pub fn migrate_fix_block_metadata(conn: &Connection) -> Result<()> {
    debug!("Starting migration: fix block metadata and normalize currency (db crate)");
    conn.execute_batch("BEGIN IMMEDIATE;")?;

    // Local extractor that looks for lines like "Design: ...", "Engraving: ...", "Perforation: ..."
    fn extract_block_metadata_local(details: &[String]) -> (String, String, String, String) {
        let combined = details.join("\n");
        let mut design = String::new();
        let mut engraving = String::new();
        let mut perforation = String::new();
        let mut watermark = String::new();

        let re_design = Regex::new(r"(?i)design:\s*(.+)").ok();
        let re_engr = Regex::new(r"(?i)engraving:\s*(.+)").ok();
        let re_perf = Regex::new(r"(?i)perforation:\s*([0-9]+)").ok();
        let re_wm = Regex::new(r"(?i)watermark:\s*(.+)").ok();

        if let Some(re) = re_design {
            if let Some(cap) = re.captures(&combined) {
                if let Some(m) = cap.get(1) { design = m.as_str().trim().to_string(); }
            }
        }
        if let Some(re) = re_engr {
            if let Some(cap) = re.captures(&combined) {
                if let Some(m) = cap.get(1) { engraving = m.as_str().trim().to_string(); }
            }
        }
        if let Some(re) = re_perf {
            if let Some(cap) = re.captures(&combined) {
                if let Some(m) = cap.get(1) { perforation = m.as_str().trim().to_string(); }
            }
        }
        if let Some(re) = re_wm {
            if let Some(cap) = re.captures(&combined) {
                if let Some(m) = cap.get(1) { watermark = m.as_str().trim().to_string(); }
            }
        }

        (watermark, design, engraving, perforation)
    }

    fn normalize_currency_token_inner(s: &str) -> String {
        let t = s.trim();
        if t.is_empty() { return String::new(); }
        let low = t.to_lowercase();
        if low == "sell" || low == "usd" || low == "$" || low == "us$" || low.contains("usd") || low.contains("us$") { return "USD".to_string(); }
        if low == "eur" || low.contains("€") { return "EUR".to_string(); }
        if low == "gbp" || low.contains("£") { return "GBP".to_string(); }
        t.to_uppercase()
    }

    // Scan rows and find suspects
    let mut stmt = conn.prepare("SELECT id, design, engraving, perforation, context, currency FROM stamps")?;
    let mut rows = stmt.query([])?;
    let mut updates: Vec<(i64, String, String, String, String, String)> = Vec::new();
    while let Ok(Some(r)) = rows.next() {
        let id: i64 = r.get(0)?;
        let design: String = r.get::<_, Option<String>>(1)?.unwrap_or_default();
        let engraving: String = r.get::<_, Option<String>>(2)?.unwrap_or_default();
        let perforation: String = r.get::<_, Option<String>>(3)?.unwrap_or_default();
        let context: String = r.get::<_, Option<String>>(4)?.unwrap_or_default();
        let currency: String = r.get::<_, Option<String>>(5)?.unwrap_or_default();

        let suspect = |s: &str| {
            let low = s.to_lowercase();
            low.contains("design:") || low.contains("engraving:") || low.contains("perforation:") || s.contains('\n') || s.contains('\r')
        };
        if suspect(&design) || suspect(&engraving) || suspect(&perforation) {
            let mut details: Vec<String> = Vec::new();
            if !context.trim().is_empty() { details.push(context.clone()); }
            if !design.trim().is_empty() { details.push(design.clone()); }
            if !engraving.trim().is_empty() { details.push(engraving.clone()); }
            if !perforation.trim().is_empty() { details.push(perforation.clone()); }
            let (wm_new, design_new, engr_new, perf_new) = extract_block_metadata_local(&details);
            let currency_new = normalize_currency_token_inner(&currency);
            updates.push((id, wm_new, design_new, engr_new, perf_new, currency_new));
        }
    }

    let mut updated_count: i64 = 0;
    for (id, _, _, _, _, _) in updates.iter() {
        let id = *id;
        let row: (String, String, String, String, String) = conn.query_row(
            "SELECT watermark, design, engraving, perforation, currency FROM stamps WHERE id = ?1",
            params![id],
            |r| Ok((
                r.get::<_, Option<String>>(0)?.unwrap_or_default(),
                r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                r.get::<_, Option<String>>(3)?.unwrap_or_default(),
                r.get::<_, Option<String>>(4)?.unwrap_or_default(),
            )),
        )?;
        let combined = vec![row.0.clone(), row.1.clone(), row.2.clone(), row.3.clone()];
        let (wm_new, design_new, engr_new, perf_new) = extract_block_metadata_local(&combined);
        let currency_new = normalize_currency_token_inner(&row.4);

        let mut set_clauses: Vec<String> = Vec::new();
        let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
        if !wm_new.is_empty() { set_clauses.push("watermark = ?".to_string()); values.push(Box::new(wm_new)); }
        if !design_new.is_empty() { set_clauses.push("design = ?".to_string()); values.push(Box::new(design_new)); }
        if !engr_new.is_empty() { set_clauses.push("engraving = ?".to_string()); values.push(Box::new(engr_new)); }
        if !perf_new.is_empty() { set_clauses.push("perforation = ?".to_string()); values.push(Box::new(perf_new)); }
        if !currency_new.is_empty() { set_clauses.push("currency = ?".to_string()); values.push(Box::new(currency_new)); }

        if !set_clauses.is_empty() {
            let sql = format!("UPDATE stamps SET {} WHERE id = ?", set_clauses.join(", "));
            values.push(Box::new(id));
            let refs: Vec<&dyn rusqlite::ToSql> = values.iter().map(|b| &**b as &dyn rusqlite::ToSql).collect();
            if let Ok(rows) = conn.execute(&sql, rusqlite::params_from_iter(refs)) {
                updated_count += rows as i64;
            }
        }
    }

    conn.execute_batch("COMMIT;")?;
    debug!("Completed migration: fix block metadata; updated {} rows", updated_count);
    Ok(())
}

pub fn insert_stamp(
    conn: &Connection,
    country: &str,
    series: &str,
    year: i32,
    month: &str,
    _month_num: &str,
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
) -> Result<bool> {
    let mut stamps_issued_normalized = stamps_issued.trim().to_string();
    if stamps_issued_normalized.starts_with('(') && stamps_issued_normalized.ends_with(')') {
        stamps_issued_normalized = stamps_issued_normalized[1..stamps_issued_normalized.len()-1].to_string();
    }
    let re_non = Regex::new(r"[^0-9\.,]").unwrap();
    stamps_issued_normalized = re_non.replace_all(&stamps_issued_normalized, "").to_string();
    if stamps_issued_normalized.contains('.') && !stamps_issued_normalized.contains(',') {
        stamps_issued_normalized = stamps_issued_normalized.replace('.', ",");
    }

    let exists_sql = "SELECT COUNT(*), image_path, remote_image_url, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, watermark, design, engraving, perforation, context FROM stamps WHERE country = ?1 AND series = ?2 AND sg_number = ?3";
    let mut stmt = conn.prepare(exists_sql)?;
    let (count, existing_image_path, existing_remote_url, existing_description, existing_stamps_issued, existing_mint_condition, existing_unused, existing_used, existing_letter_fdc, existing_currency, existing_watermark, existing_design, existing_engraving, existing_perforation, existing_context): (i64, String, String, String, String, String, String, String, String, String, String, String, String, String, String) = stmt.query_row(params![country, series, sg_number], |row| {
        Ok((
            row.get(0)?,
            row.get(1).unwrap_or(String::new()),
            row.get(2).unwrap_or(String::new()),
            row.get(3).unwrap_or(String::new()),
            row.get(4).unwrap_or(String::new()),
            row.get(5).unwrap_or(String::new()),
            row.get(6).unwrap_or(String::new()),
            row.get(7).unwrap_or(String::new()),
            row.get(8).unwrap_or(String::new()),
            row.get(9).unwrap_or(String::new()),
            row.get(10).unwrap_or(String::new()),
            row.get(11).unwrap_or(String::new()),
            row.get(12).unwrap_or(String::new()),
            row.get(13).unwrap_or(String::new()),
            row.get(14).unwrap_or(String::new()),
        ))
    })?;

    if count > 0 {
        let is_empty = |s: &str| s.trim().is_empty();
        let mut needs_update = false;
        let has_new_remote_url = !remote_image_url.trim().is_empty() && remote_image_url != existing_remote_url;
        if has_new_remote_url { needs_update = true; }
        if !image_path.trim().is_empty() {
            if is_empty(&existing_image_path) || existing_image_path.starts_with("images/no_image") {
                needs_update = true;
            }
        }
        if is_empty(&existing_description) && !description.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_stamps_issued) && !stamps_issued.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_mint_condition) && !mint_condition.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_unused) && !unused.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_used) && !used.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_letter_fdc) && !letter_fdc.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_currency) && !currency.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_watermark) && !watermark.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_design) && !design.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_engraving) && !engraving.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_perforation) && !perforation.trim().is_empty() { needs_update = true; }
        if is_empty(&existing_context) && !context.trim().is_empty() { needs_update = true; }

        if needs_update {
            debug!("Stamp exists and backfill/update required (country='{}', series='{}', sg_number='{}')", country, series, sg_number);
            let update_sql = "UPDATE stamps SET
                image_path = CASE WHEN (?1 != '') AND (image_path IS NULL OR image_path = '' OR image_path LIKE 'images/no_image%') THEN ?1 ELSE image_path END,
                remote_image_url = CASE WHEN (?2 != '') THEN ?2 ELSE remote_image_url END,
                original_image_url = CASE WHEN (?3 != '') THEN ?3 ELSE original_image_url END,
                description = CASE WHEN (description IS NULL OR description = '') AND (?4 != '') THEN ?4 ELSE description END,
                stamps_issued = CASE WHEN (stamps_issued IS NULL OR stamps_issued = '') AND (?5 != '') THEN ?5 ELSE stamps_issued END,
                mint_condition = CASE WHEN (mint_condition IS NULL OR mint_condition = '') AND (?6 != '') THEN ?6 ELSE mint_condition END,
                unused = CASE WHEN (unused IS NULL OR unused = '') AND (?7 != '') THEN ?7 ELSE unused END,
                used = CASE WHEN (used IS NULL OR used = '') AND (?8 != '') THEN ?8 ELSE used END,
                letter_fdc = CASE WHEN (letter_fdc IS NULL OR letter_fdc = '') AND (?9 != '') THEN ?9 ELSE letter_fdc END,
                currency = CASE WHEN (currency IS NULL OR currency = '') AND (?10 != '') THEN ?10 ELSE currency END,
                watermark = CASE WHEN (watermark IS NULL OR watermark = '') AND (?11 != '') THEN ?11 ELSE watermark END,
                design = CASE WHEN (design IS NULL OR design = '') AND (?12 != '') THEN ?12 ELSE design END,
                engraving = CASE WHEN (engraving IS NULL OR engraving = '') AND (?13 != '') THEN ?13 ELSE engraving END,
                perforation = CASE WHEN (perforation IS NULL OR perforation = '') AND (?14 != '') THEN ?14 ELSE perforation END,
                context = CASE WHEN (context IS NULL OR context = '') AND (?15 != '') THEN ?15 ELSE context END
                WHERE country = ?16 AND series = ?17 AND sg_number = ?18";

            let params_vec = params![
                image_path, remote_image_url, original_image_url,
                description, stamps_issued_normalized, mint_condition, unused, used, letter_fdc, currency,
                watermark, design, engraving, perforation, context,
                country, series, sg_number
            ];
            match conn.execute(update_sql, params_vec) {
                Ok(rows) => {
                    if rows > 0 {
                        debug!("Updated existing stamp metadata/image for {} | {} | {} (rows={})", country, series, sg_number, rows);
                        return Ok(true);
                    } else {
                        debug!("Backfill attempted but no DB row matched for update: {} | {} | {}", country, series, sg_number);
                    }
                }
                Err(e) => { error!("DB update error for {} {} {}: {}", country, series, sg_number, e); }
            }
        }

        debug!("Stamp already exists with no new fillable data (country='{}', series='{}', sg_number='{}'), skipping", country, series, sg_number);
        return Ok(false);
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
        country.to_string(),
        series.to_string(),
        year.to_string(),
        month_clean.to_string(),
        month_num.to_string(),
        day.to_string(),
        sg_number.to_string(),
        type_.to_string(),
        denomination.to_string(),
        color.to_string(),
        description.to_string(),
        stamps_issued.to_string(),
        mint_condition.to_string(),
        unused.to_string(),
        used.to_string(),
        letter_fdc.to_string(),
        currency.to_string(),
        image_path.to_string(),
        remote_image_url.to_string(),
        original_image_url.to_string(),
        watermark.to_string(),
        design.to_string(),
        engraving.to_string(),
        perforation.to_string(),
        context.to_string()
    ];
    debug!("SQL: {}", sql);
    debug!("Params: {:?}", params_vec);
    let mut attempts = 0;
    loop {
        attempts += 1;
        match conn.execute(sql, params![country, series, year, month_clean, month_num, day, sg_number, type_, denomination, color, description, stamps_issued, mint_condition, unused, used, letter_fdc, currency, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, context]) {
            Ok(_) => {
                debug!("Insert succeeded on attempt {}", attempts);
                let last_id = conn.last_insert_rowid();
                debug!("Last insert rowid: {}", last_id);
                break;
            }
            Err(e) => {
                error!("Insert failed on attempt {}: {}", attempts, e);
                if attempts >= 2 {
                    error!("Giving up after {} attempts. Params: {:?}", attempts, params_vec);
                    break;
                } else {
                    debug!("Retrying insert...");
                }
            }
        }
    }
    Ok(true)
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
            row.get::<_, i32>(5)?, // month_num
            row.get(6)?, // day
            row.get::<_, String>(7)?, // description
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
