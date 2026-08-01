use rusqlite::{Connection, OptionalExtension, Result};

#[derive(Debug, Clone)]
pub struct ColnectInfo {
    pub country: Option<String>,
    pub colnect_link: Option<String>,
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
    pub ord_mi: Option<String>,
    pub ord_sn: Option<String>,
    pub ord_yt: Option<String>,
    pub ord_sg: Option<String>,
    pub parent_item: Option<String>,
    pub variant: Option<String>,
    pub overprint_variant: Option<String>,
    pub list: Option<String>,
    pub quantity: Option<String>,
    pub condition: Option<String>,
    pub public_note: Option<String>,
    pub private_note: Option<String>,
    pub catalog_codes: Vec<(String, String)>,
}

fn load_colnect_info_with_query(
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
                country: row.get(1)?,
                colnect_link: row.get(2)?,
                name: row.get(3)?,
                watermark: row.get(4)?,
                format: row.get(5)?,
                emission: row.get(6)?,
                perforation: row.get(7)?,
                printing: row.get(8)?,
                paper: row.get(9)?,
                gum: row.get(10)?,
                themes: row.get(11)?,
                description: row.get(12)?,
                issued_on: row.get(13)?,
                expiry_date: row.get(14)?,
                print_run: row.get(15)?,
                face_value: row.get(16)?,
                currency: row.get(17)?,
                colors: row.get(18)?,
                designers: row.get(19)?,
                printers: row.get(20)?,
                score: row.get(21)?,
                accuracy: row.get(22)?,
                width: row.get(23)?,
                height: row.get(24)?,
                ord_mi: row.get(25)?,
                ord_sn: row.get(26)?,
                ord_yt: row.get(27)?,
                ord_sg: row.get(28)?,
                parent_item: row.get(29)?,
                variant: row.get(30)?,
                overprint_variant: row.get(31)?,
                list: row.get(32)?,
                quantity: row.get(33)?,
                condition: row.get(34)?,
                public_note: row.get(35)?,
                private_note: row.get(36)?,
                catalog_codes: Vec::new(),
            },
        ))
    })
    .optional()
}

fn hydrate_catalog_codes(
    conn: &Connection,
    colnect_id: String,
    mut info: ColnectInfo,
) -> Result<Option<ColnectInfo>> {
    let mut code_stmt =
        conn.prepare("SELECT catalog_type, code FROM colnect_catalog_codes WHERE colnect_id = ?1")?;
    let codes = code_stmt.query_map([colnect_id], |row| Ok((row.get(0)?, row.get(1)?)))?;

    for code in codes {
        info.catalog_codes.push(code?);
    }

    Ok(Some(info))
}

pub fn get_colnect_info(conn: &Connection, sg_number: &str) -> Result<Option<ColnectInfo>> {
    // Try exact SG match first.
    let exact_sql =
        "SELECT colnect_id, country, colnect_link, name, watermark, format, emission, perforation, printing, paper, gum, themes, description, issued_on, expiry_date, print_run, face_value, currency, colors, designers, printers, score, accuracy, width, height, ord_mi, ord_sn, ord_yt, ord_sg, parent_item, variant, overprint_variant, list, quantity, condition, public_note, private_note
         FROM colnect_stamps WHERE sg_number = ?1";
    let mut result = load_colnect_info_with_query(conn, exact_sql, sg_number)?;

    // Fallback for SG variants (e.g. requested `944`, Colnect has `944x`).
    // The suffix after the numeric base must be non-digit to avoid false matches
    // like sg="4" matching "468d" (a completely different stamp number). Valid
    // variants: "944a", "944*", "944x"; invalid: "4" → "468d", "5" → "586".
    if result.is_none() {
        let variant_sql =
                        "SELECT colnect_id, country, colnect_link, name, watermark, format, emission, perforation, printing, paper, gum, themes, description, issued_on, expiry_date, print_run, face_value, currency, colors, designers, printers, score, accuracy, width, height, ord_mi, ord_sn, ord_yt, ord_sg, parent_item, variant, overprint_variant, list, quantity, condition, public_note, private_note
             FROM colnect_stamps
             WHERE sg_number LIKE (?1 || '%')
               AND sg_number <> ?1
             ORDER BY LENGTH(sg_number) ASC
             LIMIT 20";
        let candidates: Vec<(String, ColnectInfo)> = {
            let mut stmt = conn.prepare(variant_sql)?;
            let rows = stmt.query_map([sg_number], |row| {
                let colnect_id: String = row.get(0)?;
                Ok((
                    colnect_id,
                    ColnectInfo {
                        country: row.get(1)?,
                        colnect_link: row.get(2)?,
                        name: row.get(3)?,
                        watermark: row.get(4)?,
                        format: row.get(5)?,
                        emission: row.get(6)?,
                        perforation: row.get(7)?,
                        printing: row.get(8)?,
                        paper: row.get(9)?,
                        gum: row.get(10)?,
                        themes: row.get(11)?,
                        description: row.get(12)?,
                        issued_on: row.get(13)?,
                        expiry_date: row.get(14)?,
                        print_run: row.get(15)?,
                        face_value: row.get(16)?,
                        currency: row.get(17)?,
                        colors: row.get(18)?,
                        designers: row.get(19)?,
                        printers: row.get(20)?,
                        score: row.get(21)?,
                        accuracy: row.get(22)?,
                        width: row.get(23)?,
                        height: row.get(24)?,
                        ord_mi: row.get(25)?,
                        ord_sn: row.get(26)?,
                        ord_yt: row.get(27)?,
                        ord_sg: row.get(28)?,
                        parent_item: row.get(29)?,
                        variant: row.get(30)?,
                        overprint_variant: row.get(31)?,
                        list: row.get(32)?,
                        quantity: row.get(33)?,
                        condition: row.get(34)?,
                        public_note: row.get(35)?,
                        private_note: row.get(36)?,
                        catalog_codes: Vec::new(),
                    },
                ))
            })?;
            rows.filter_map(|r| r.ok()).collect()
        };
        // Only accept a candidate if its sg_number is a genuine variant:
        // the suffix after the query prefix must start with a non-digit character
        // (e.g. "944" → "944a" is valid; "4" → "468d" is not).
        for (colnect_id, info) in candidates {
            // Re-query the sg_number for this colnect_id to validate it.
            let candidate_sg: Option<String> = conn
                .query_row(
                    "SELECT sg_number FROM colnect_stamps WHERE colnect_id = ?1 LIMIT 1",
                    [&colnect_id],
                    |row| row.get(0),
                )
                .optional()?;
            if let Some(ref csg) = candidate_sg {
                let suffix = &csg[sg_number.len().min(csg.len())..];
                if suffix.starts_with(|c: char| !c.is_ascii_digit()) {
                    result = Some((colnect_id, info));
                    break;
                }
            }
        }
    }

    if let Some((colnect_id, info)) = result {
        return hydrate_catalog_codes(conn, colnect_id, info);
    }

    Ok(None)
}

pub fn get_colnect_info_by_id(conn: &Connection, colnect_id: &str) -> Result<Option<ColnectInfo>> {
    let by_id_sql =
        "SELECT colnect_id, country, colnect_link, name, watermark, format, emission, perforation, printing, paper, gum, themes, description, issued_on, expiry_date, print_run, face_value, currency, colors, designers, printers, score, accuracy, width, height, ord_mi, ord_sn, ord_yt, ord_sg, parent_item, variant, overprint_variant, list, quantity, condition, public_note, private_note
         FROM colnect_stamps WHERE colnect_id = ?1 LIMIT 1";

    if let Some((resolved_id, info)) = load_colnect_info_with_query(conn, by_id_sql, colnect_id)? {
        return hydrate_catalog_codes(conn, resolved_id, info);
    }

    Ok(None)
}
