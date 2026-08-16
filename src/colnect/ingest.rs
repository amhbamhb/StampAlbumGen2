use crate::colnect::parser::ColnectStamp;
use log::info;
use regex::Regex;
use rusqlite::{params, Connection, Result};

pub fn ingest_colnect_data(conn: &Connection, stamps: &[ColnectStamp]) -> Result<()> {
    ingest_colnect_data_with_default_catalog(conn, stamps, "sg")
}

pub fn ingest_colnect_data_with_default_catalog(
    conn: &Connection,
    stamps: &[ColnectStamp],
    default_catalog_code: &str,
) -> Result<()> {
    ingest_colnect_data_with_mode(conn, stamps, default_catalog_code, true)
}

pub fn ingest_colnect_data_with_mode(
    conn: &Connection,
    stamps: &[ColnectStamp],
    default_catalog_code: &str,
    clear_existing: bool,
) -> Result<()> {
    if clear_existing {
        // Single-file mode: start with a fresh staging set.
        conn.execute("DELETE FROM colnect_catalog_codes", [])?;
        conn.execute("DELETE FROM colnect_stamps", [])?;
    }

    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO colnect_stamps (
            sg_number, colnect_id, name, catalog_codes, country, width, height, watermark, format, emission,
            perforation, printing, paper, gum, ord_mi, ord_sn, ord_yt, ord_sg, themes, colnect_link, description, issued_on, expiry_date,
            print_run, face_value, currency, colors, designers, printers, series,
            parent_item, variant, overprint_variant, score, accuracy, list, quantity, condition, public_note, private_note
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40)",
    )?;

    let mut code_stmt = conn.prepare(
        "INSERT OR REPLACE INTO colnect_catalog_codes (colnect_id, catalog_type, code) VALUES (?1, ?2, ?3)",
    )?;

    let mut count = 0;
    for stamp in stamps {
        let catalog_number =
            crate::catalog::preferred_catalog_number(&stamp.catalog_codes, default_catalog_code)
                .unwrap_or_else(|| {
                    extract_colnect_code(&stamp.catalog_codes).unwrap_or_else(|| stamp.link.clone())
                });

        // Use Colnect code as ID, fallback to Link
        let colnect_id =
            extract_colnect_code(&stamp.catalog_codes).unwrap_or_else(|| stamp.link.clone());

        // Convert Option<String> to Option<&str> for rusqlite
        let width_ref = stamp.width.as_deref();
        let height_ref = stamp.height.as_deref();
        let watermark_ref = stamp.watermark.as_deref();
        let format_ref = stamp.format.as_deref();
        let emission_ref = stamp.emission.as_deref();
        let perforation_ref = stamp.perforation.as_deref();
        let printing_ref = stamp.printing.as_deref();
        let paper_ref = stamp.paper.as_deref();
        let gum_ref = stamp.gum.as_deref();
        let ord_mi_ref = stamp.ord_mi.as_deref();
        let ord_sn_ref = stamp.ord_sn.as_deref();
        let ord_yt_ref = stamp.ord_yt.as_deref();
        let ord_sg_ref = stamp.ord_sg.as_deref();
        let themes_ref = stamp.themes.as_deref();
        let description_ref = stamp.description.as_deref();
        let expiry_date_ref = stamp.expiry_date.as_deref();
        let print_run_ref = stamp.print_run.as_deref();
        let face_value_ref = stamp.face_value.as_deref();
        let currency_ref = stamp.currency.as_deref();
        let colors_ref = stamp.colors.as_deref();
        let designers_ref = stamp.designers.as_deref();
        let printers_ref = stamp.printers.as_deref();
        let parent_item_ref = stamp.parent_item.as_deref();
        let variant_ref = stamp.variant.as_deref();
        let overprint_variant_ref = stamp.overprint_variant.as_deref();
        let score_ref = stamp.score.as_deref();
        let accuracy_ref = stamp.accuracy.as_deref();
        let list_ref = stamp.list.as_deref();
        let quantity_ref = stamp.quantity.as_deref();
        let condition_ref = stamp.condition.as_deref();
        let public_note_ref = stamp.public_note.as_deref();
        let private_note_ref = stamp.private_note.as_deref();

        stmt.execute(params![
            catalog_number,
            colnect_id,
            stamp.name,
            stamp.catalog_codes,
            stamp.country,
            width_ref,
            height_ref,
            watermark_ref,
            format_ref,
            emission_ref,
            perforation_ref,
            printing_ref,
            paper_ref,
            gum_ref,
            ord_mi_ref,
            ord_sn_ref,
            ord_yt_ref,
            ord_sg_ref,
            themes_ref,
            stamp.link,
            description_ref,
            stamp.issued_on,
            expiry_date_ref,
            print_run_ref,
            face_value_ref,
            currency_ref,
            colors_ref,
            designers_ref,
            printers_ref,
            stamp.series,
            parent_item_ref,
            variant_ref,
            overprint_variant_ref,
            score_ref,
            accuracy_ref,
            list_ref,
            quantity_ref,
            condition_ref,
            public_note_ref,
            private_note_ref,
        ])?;

        // Insert individual catalog codes
        for part in stamp.catalog_codes.split(',') {
            let part = part.trim();
            if let Some((type_part, code_part)) = part.split_once(':') {
                let cat_type = type_part.trim();
                let code = code_part.trim();
                code_stmt.execute(params![colnect_id, cat_type, code])?;
            }
        }

        count += 1;
    }
    info!("Ingested {} Colnect stamps", count);

    conn.execute(
        "UPDATE colnect_stamps
         SET is_processed = CASE
             WHEN TRIM(COALESCE(colnect_id, '')) != ''
              AND EXISTS (
                  SELECT 1
                  FROM stamps s
                  WHERE TRIM(COALESCE(s.colnect_id, '')) = TRIM(COALESCE(colnect_stamps.colnect_id, ''))
                    AND TRIM(COALESCE(s.colnect_id, '')) != ''
              ) THEN 1
             ELSE 0
         END",
        [],
    )?;

    // Skip link_colnect_data for now as it has issues with NULL handling
    // TODO: Fix link_colnect_data to handle NULL values properly
    // link_colnect_data(conn)?;

    Ok(())
}

#[allow(dead_code)]
fn link_colnect_data(conn: &Connection) -> Result<()> {
    info!("Attempting to link Colnect stamps by Date and Face Value...");

    // Select all Colnect stamps
    let mut stmt = conn.prepare(
        "SELECT colnect_id, issued_on, face_value, sg_number, series FROM colnect_stamps",
    )?;

    let colnect_rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, Option<String>>(1)?,
            row.get::<_, Option<String>>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, Option<String>>(4)?,
        ))
    })?;

    let mut update_stmt =
        conn.prepare("UPDATE colnect_stamps SET sg_number = ?1 WHERE colnect_id = ?2")?;

    let mut linked_count = 0;
    let re_nums = Regex::new(r"[\d\.,]+").unwrap();

    for row in colnect_rows {
        let (colnect_id, issued_on, face_value, current_sg, col_series) = row?;

        if let (Some(date_str), Some(fv)) = (issued_on, face_value) {
            // Parse date YYYY-MM-DD
            if let Ok(date) = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                let year = date.format("%Y").to_string();
                let month = date.format("%-m").to_string(); // 1-12
                                                            // let day = date.format("%-d").to_string();   // 1-31 (Unused for now)

                // Find candidates in main stamps table
                // Note: stamps table has month_num as integer, day as text (sometimes "05" sometimes "5")
                let mut candidates_stmt = conn.prepare(
                    "SELECT sg_number, denomination, series, description FROM stamps 
                     WHERE year = ?1 AND month_num = ?2",
                )?;

                let candidates = candidates_stmt.query_map(params![year, month], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                    ))
                })?;

                let mut matches = Vec::new();

                // Normalize Colnect face value numbers
                let col_nums: Vec<String> = re_nums
                    .find_iter(&fv)
                    .map(|m| {
                        m.as_str()
                            .replace(',', ".")
                            .trim_end_matches('.')
                            .to_string()
                    })
                    .collect();

                for cand in candidates {
                    let (cand_sg, cand_denom, cand_series, cand_desc) = cand?;

                    // Normalize StampWorld denomination numbers
                    let cand_nums: Vec<String> = re_nums
                        .find_iter(&cand_denom)
                        .map(|m| {
                            m.as_str()
                                .replace(',', ".")
                                .trim_end_matches('.')
                                .to_string()
                        })
                        .collect();

                    // Compare number sequences
                    if col_nums == cand_nums {
                        matches.push((cand_sg, cand_series, cand_desc));
                    }
                }

                let best_match = if matches.len() == 1 {
                    Some(matches[0].0.clone())
                } else if matches.len() > 1 {
                    // Disambiguate using series/description similarity
                    if let Some(c_series) = &col_series {
                        let mut best_score = 0.0;
                        let mut best_sg = None;

                        for (sg, s_series, s_desc) in &matches {
                            let c_ser_lower = c_series.to_lowercase();
                            let s_ser_lower = s_series.to_lowercase();
                            let s_desc_lower = s_desc.to_lowercase();

                            let mut score = strsim::jaro_winkler(c_series, s_series);
                            let score_desc = strsim::jaro_winkler(c_series, s_desc);
                            score = score.max(score_desc);

                            // Boost score if one contains the other
                            if s_ser_lower.contains(&c_ser_lower)
                                || c_ser_lower.contains(&s_ser_lower)
                            {
                                score += 0.3;
                            }
                            if s_desc_lower.contains(&c_ser_lower)
                                || c_ser_lower.contains(&s_desc_lower)
                            {
                                score += 0.2;
                            }

                            if score > best_score {
                                best_score = score;
                                best_sg = Some(sg.clone());
                            }
                        }

                        // Only accept if score is reasonably high or significantly better?
                        // For now, just take the best one if > 0.5
                        if best_score > 0.5 {
                            best_sg
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(new_sg) = best_match {
                    if new_sg != current_sg {
                        info!(
                            "Linking Colnect {} (SG {}) -> StampWorld SG {}",
                            colnect_id, current_sg, new_sg
                        );
                        update_stmt.execute(params![new_sg, colnect_id])?;
                        linked_count += 1;
                    }
                }
            }
        }
    }

    info!(
        "Linked {} Colnect stamps by Date+FaceValue+Series",
        linked_count
    );
    Ok(())
}

fn extract_colnect_code(codes: &str) -> Option<String> {
    for part in codes.split(',') {
        let part = part.trim();
        if part.starts_with("Col:") {
            return Some(part["Col:".len()..].trim().to_string());
        }
    }
    None
}
