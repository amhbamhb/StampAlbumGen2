use rusqlite::{params, Connection, Result};
use crate::colnect::parser::ColnectStamp;
use log::{info, warn};
use regex::Regex;

pub fn ingest_colnect_data(conn: &Connection, stamps: &[ColnectStamp]) -> Result<()> {
    let mut stmt = conn.prepare(
        "INSERT OR REPLACE INTO colnect_stamps (
            sg_number, colnect_id, catalog_codes, width, height, printing, paper, gum, themes,
            colnect_link, description, issued_on, print_run, face_value, currency, colors, designers, printers, series
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
    )?;

    let mut code_stmt = conn.prepare(
        "INSERT OR REPLACE INTO colnect_catalog_codes (colnect_id, catalog_type, code) VALUES (?1, ?2, ?3)"
    )?;

    let mut count = 0;
    for stamp in stamps {
        let sg_number = extract_sg_number(&stamp.catalog_codes);
        
        // Use Colnect code as ID, fallback to Link
        let colnect_id = extract_colnect_code(&stamp.catalog_codes).unwrap_or_else(|| stamp.link.clone());

        if let Some(sg) = sg_number {
            // Normalize SG number: remove country prefix if present
            let normalized_sg = normalize_sg(&sg);

            stmt.execute(params![
                normalized_sg,
                colnect_id,
                stamp.catalog_codes,
                stamp.width,
                stamp.height,
                stamp.printing,
                stamp.paper,
                stamp.gum,
                stamp.themes,
                stamp.link,
                stamp.description,
                stamp.issued_on,
                stamp.print_run,
                stamp.face_value,
                stamp.currency,
                stamp.colors,
                stamp.designers,
                stamp.printers,
                stamp.series,
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
        } else {
            warn!("No SG number found for stamp: {}", stamp.name);
        }
    }
    info!("Ingested {} Colnect stamps", count);
    
    // Attempt to link stamps that have mismatched SG numbers
    link_colnect_data(conn)?;
    
    Ok(())
}

fn link_colnect_data(conn: &Connection) -> Result<()> {
    info!("Attempting to link Colnect stamps by Date and Face Value...");
    
    // Select all Colnect stamps
    let mut stmt = conn.prepare(
        "SELECT colnect_id, issued_on, face_value, sg_number, series FROM colnect_stamps"
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

    let mut update_stmt = conn.prepare(
        "UPDATE colnect_stamps SET sg_number = ?1 WHERE colnect_id = ?2"
    )?;

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
                     WHERE year = ?1 AND month_num = ?2"
                )?;
                
                let candidates = candidates_stmt.query_map(params![year, month], |r| {
                    Ok((
                        r.get::<_, String>(0)?, 
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?
                    ))
                })?;

                let mut matches = Vec::new();

                // Normalize Colnect face value numbers
                let col_nums: Vec<String> = re_nums.find_iter(&fv)
                    .map(|m| m.as_str().replace(',', ".").trim_end_matches('.').to_string())
                    .collect();

                for cand in candidates {
                    let (cand_sg, cand_denom, cand_series, cand_desc) = cand?;
                    
                    // Normalize StampWorld denomination numbers
                    let cand_nums: Vec<String> = re_nums.find_iter(&cand_denom)
                        .map(|m| m.as_str().replace(',', ".").trim_end_matches('.').to_string())
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
                            if s_ser_lower.contains(&c_ser_lower) || c_ser_lower.contains(&s_ser_lower) {
                                score += 0.3;
                            }
                            if s_desc_lower.contains(&c_ser_lower) || c_ser_lower.contains(&s_desc_lower) {
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
                        info!("Linking Colnect {} (SG {}) -> StampWorld SG {}", colnect_id, current_sg, new_sg);
                        update_stmt.execute(params![new_sg, colnect_id])?;
                        linked_count += 1;
                    }
                }
            }
        }
    }
    
    info!("Linked {} Colnect stamps by Date+FaceValue+Series", linked_count);
    Ok(())
}


fn extract_sg_number(codes: &str) -> Option<String> {
    // codes example: "Mi:CH 964, Sn:CH 540, Yt:CH 895, Sg:CH 824, AFA:CH 959..."
    for part in codes.split(',') {
        let part = part.trim();
        if part.starts_with("Sg:") {
            return Some(part["Sg:".len()..].trim().to_string());
        }
    }
    None
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

fn normalize_sg(sg: &str) -> String {
    // "CH 824" -> "824"
    // "824" -> "824"
    // "GB 123" -> "123"
    // Simple heuristic: if there is a space, and the first part is letters, remove it.
    if let Some((prefix, number)) = sg.split_once(' ') {
        if prefix.chars().all(|c| c.is_alphabetic()) {
            return number.to_string();
        }
    }
    sg.to_string()
}
