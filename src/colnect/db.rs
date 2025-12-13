use rusqlite::{Connection, Result, OptionalExtension};

#[derive(Debug, Clone)]
pub struct ColnectInfo {
    pub printing: Option<String>,
    pub paper: Option<String>,
    pub gum: Option<String>,
    pub themes: Option<String>,
    pub description: Option<String>,
    pub issued_on: Option<String>,
    pub print_run: Option<String>,
    pub face_value: Option<String>,
    pub currency: Option<String>,
    pub colors: Option<String>,
    pub designers: Option<String>,
    pub printers: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub catalog_codes: Vec<(String, String)>,
}

pub fn get_colnect_info(conn: &Connection, sg_number: &str) -> Result<Option<ColnectInfo>> {
    // Try exact match first
    let mut stmt = conn.prepare(
        "SELECT colnect_id, printing, paper, gum, themes, description, issued_on, print_run, face_value, currency, colors, designers, printers, width, height 
         FROM colnect_stamps WHERE sg_number = ?1"
    )?;
    
    let result = stmt.query_row([sg_number], |row| {
        let colnect_id: String = row.get(0)?;
        Ok((colnect_id, ColnectInfo {
            printing: row.get(1)?,
            paper: row.get(2)?,
            gum: row.get(3)?,
            themes: row.get(4)?,
            description: row.get(5)?,
            issued_on: row.get(6)?,
            print_run: row.get(7)?,
            face_value: row.get(8)?,
            currency: row.get(9)?,
            colors: row.get(10)?,
            designers: row.get(11)?,
            printers: row.get(12)?,
            width: row.get(13)?,
            height: row.get(14)?,
            catalog_codes: Vec::new(),
        }))
    }).optional()?;

    if let Some((colnect_id, mut info)) = result {
        let mut code_stmt = conn.prepare(
            "SELECT catalog_type, code FROM colnect_catalog_codes WHERE colnect_id = ?1"
        )?;
        let codes = code_stmt.query_map([colnect_id], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        
        for code in codes {
            info.catalog_codes.push(code?);
        }
        return Ok(Some(info));
    }

    Ok(None)
}
