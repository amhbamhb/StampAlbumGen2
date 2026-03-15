use rusqlite::{Connection, OptionalExtension, Result};

#[derive(Debug, Clone)]
pub struct ColnectInfo {
    pub name: Option<String>,
    pub watermark: Option<String>,
    pub format: Option<String>,
    pub emission: Option<String>,
    pub perforation: Option<String>,
    pub printing: Option<String>,
    pub paper: Option<String>,
    pub gum: Option<String>,
    pub themes: Option<String>,
    pub description: Option<String>,
    pub issued_on: Option<String>,
    pub expiry_date: Option<String>,
    pub print_run: Option<String>,
    pub face_value: Option<String>,
    pub currency: Option<String>,
    pub colors: Option<String>,
    pub designers: Option<String>,
    pub printers: Option<String>,
    pub score: Option<String>,
    pub accuracy: Option<String>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub catalog_codes: Vec<(String, String)>,
}

pub fn get_colnect_info(conn: &Connection, sg_number: &str) -> Result<Option<ColnectInfo>> {
    fn load_with_query(
        conn: &Connection,
        sql: &str,
        param: &str,
    ) -> Result<Option<(String, ColnectInfo)>> {
        let mut stmt = conn.prepare(sql)?;
        stmt.query_row([param], |row| {
            let colnect_id: String = row.get(0)?;
            Ok((
                colnect_id,
                ColnectInfo {
                    name: row.get(1)?,
                    watermark: row.get(2)?,
                    format: row.get(3)?,
                    emission: row.get(4)?,
                    perforation: row.get(5)?,
                    printing: row.get(6)?,
                    paper: row.get(7)?,
                    gum: row.get(8)?,
                    themes: row.get(9)?,
                    description: row.get(10)?,
                    issued_on: row.get(11)?,
                    expiry_date: row.get(12)?,
                    print_run: row.get(13)?,
                    face_value: row.get(14)?,
                    currency: row.get(15)?,
                    colors: row.get(16)?,
                    designers: row.get(17)?,
                    printers: row.get(18)?,
                    score: row.get(19)?,
                    accuracy: row.get(20)?,
                    width: row.get(21)?,
                    height: row.get(22)?,
                    catalog_codes: Vec::new(),
                },
            ))
        })
        .optional()
    }

    // Try exact SG match first.
    let exact_sql =
        "SELECT colnect_id, name, watermark, format, emission, perforation, printing, paper, gum, themes, description, issued_on, expiry_date, print_run, face_value, currency, colors, designers, printers, score, accuracy, width, height
         FROM colnect_stamps WHERE sg_number = ?1";
    let mut result = load_with_query(conn, exact_sql, sg_number)?;

    // Fallback for SG variants (e.g. requested `944`, Colnect has `944x`).
    // This is intentionally narrow: same numeric prefix and shortest suffix first.
    if result.is_none() {
        let variant_sql =
            "SELECT colnect_id, name, watermark, format, emission, perforation, printing, paper, gum, themes, description, issued_on, expiry_date, print_run, face_value, currency, colors, designers, printers, score, accuracy, width, height
             FROM colnect_stamps
             WHERE sg_number LIKE (?1 || '%')
               AND sg_number <> ?1
             ORDER BY LENGTH(sg_number) ASC
             LIMIT 1";
        result = load_with_query(conn, variant_sql, sg_number)?;
    }

    if let Some((colnect_id, mut info)) = result {
        let mut code_stmt = conn.prepare(
            "SELECT catalog_type, code FROM colnect_catalog_codes WHERE colnect_id = ?1",
        )?;
        let codes = code_stmt.query_map([colnect_id], |row| Ok((row.get(0)?, row.get(1)?)))?;

        for code in codes {
            info.catalog_codes.push(code?);
        }
        return Ok(Some(info));
    }

    Ok(None)
}
