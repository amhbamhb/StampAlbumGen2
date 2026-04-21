use super::*;
use rusqlite::{params, Connection};
use std::collections::HashMap;

pub(super) fn sync_processed_colnect_candidates(
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
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
    Ok(())
}

pub(super) fn mark_colnect_candidate_processed(
    conn: &Connection,
    colnect_id: &str,
) -> rusqlite::Result<usize> {
    if colnect_id.trim().is_empty() {
        return Ok(0);
    }

    conn.execute(
        "UPDATE colnect_stamps
         SET is_processed = 1
         WHERE TRIM(COALESCE(colnect_id, '')) = TRIM(?1)",
        params![colnect_id],
    )
}

pub(super) fn load_left_stamps(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    include_imageless: bool,
) -> Result<Vec<LeftStamp>, Box<dyn std::error::Error>> {
    let query = if include_imageless {
        "SELECT rowid,
                COALESCE(sg_number, ''),
            COALESCE(catalog_code, ''),
                COALESCE(year, 0),
                COALESCE(month_num, 0),
                COALESCE(day, ''),
                COALESCE(series, ''),
                COALESCE(image_path, ''),
                COALESCE(stampworld_image_url, ''),
                COALESCE(description, ''),
                COALESCE(watermark, ''),
                COALESCE(design, ''),
                COALESCE(engraving, ''),
                COALESCE(perforation, ''),
            COALESCE(context, ''),
            COALESCE(colnect_page_url, ''),
            COALESCE(colnect_image_url, ''),
            COALESCE(colnect_id, ''),
            COALESCE(colnect_name, ''),
            COALESCE(colnect_catalog_codes, ''),
            COALESCE(colnect_country, ''),
            COALESCE(colnect_width, ''),
            COALESCE(colnect_height, ''),
            COALESCE(colnect_format, ''),
            COALESCE(colnect_emission, ''),
            COALESCE(colnect_printing, ''),
            COALESCE(colnect_paper, ''),
            COALESCE(colnect_gum, ''),
            COALESCE(colnect_ord_mi, ''),
            COALESCE(colnect_ord_sn, ''),
            COALESCE(colnect_ord_yt, ''),
            COALESCE(colnect_face_value, ''),
            COALESCE(colnect_currency, ''),
            COALESCE(colnect_ord_sg, ''),
            COALESCE(colnect_expiry_date, ''),
            COALESCE(colnect_print_run, ''),
            COALESCE(colnect_colors, ''),
            COALESCE(colnect_parent_item, ''),
            COALESCE(colnect_variant, ''),
            COALESCE(colnect_overprint_variant, ''),
            COALESCE(colnect_score, ''),
            COALESCE(colnect_accuracy, ''),
            COALESCE(colnect_list, ''),
            COALESCE(colnect_quantity, ''),
            COALESCE(colnect_condition, ''),
            COALESCE(colnect_public_note, ''),
            COALESCE(colnect_private_note, '')
         FROM stamps
         ORDER BY year, sg_number"
    } else {
        "SELECT rowid,
                COALESCE(sg_number, ''),
            COALESCE(catalog_code, ''),
                COALESCE(year, 0),
                COALESCE(month_num, 0),
                COALESCE(day, ''),
                COALESCE(series, ''),
                COALESCE(image_path, ''),
                COALESCE(stampworld_image_url, ''),
                COALESCE(description, ''),
                COALESCE(watermark, ''),
                COALESCE(design, ''),
                COALESCE(engraving, ''),
                COALESCE(perforation, ''),
            COALESCE(context, ''),
            COALESCE(colnect_page_url, ''),
            COALESCE(colnect_image_url, ''),
            COALESCE(colnect_id, ''),
            COALESCE(colnect_name, ''),
            COALESCE(colnect_catalog_codes, ''),
            COALESCE(colnect_country, ''),
            COALESCE(colnect_width, ''),
            COALESCE(colnect_height, ''),
            COALESCE(colnect_format, ''),
            COALESCE(colnect_emission, ''),
            COALESCE(colnect_printing, ''),
            COALESCE(colnect_paper, ''),
            COALESCE(colnect_gum, ''),
            COALESCE(colnect_ord_mi, ''),
            COALESCE(colnect_ord_sn, ''),
            COALESCE(colnect_ord_yt, ''),
            COALESCE(colnect_face_value, ''),
            COALESCE(colnect_currency, ''),
            COALESCE(colnect_ord_sg, ''),
            COALESCE(colnect_expiry_date, ''),
            COALESCE(colnect_print_run, ''),
            COALESCE(colnect_colors, ''),
            COALESCE(colnect_parent_item, ''),
            COALESCE(colnect_variant, ''),
            COALESCE(colnect_overprint_variant, ''),
            COALESCE(colnect_score, ''),
            COALESCE(colnect_accuracy, ''),
            COALESCE(colnect_list, ''),
            COALESCE(colnect_quantity, ''),
            COALESCE(colnect_condition, ''),
            COALESCE(colnect_public_note, ''),
            COALESCE(colnect_private_note, '')
         FROM stamps
         WHERE TRIM(COALESCE(image_path, '')) != ''
           AND LOWER(COALESCE(image_path, '')) NOT LIKE 'images/no_image%'
         ORDER BY year, sg_number"
    };
    let mut stmt = conn.prepare(query)?;

    let rows = stmt.query_map([], |row| {
        let rel_image = row.get::<_, String>(7)?;
        Ok(LeftStamp {
            rowid: row.get(0)?,
            sg_number: row.get(1)?,
            catalog_code: row.get(2)?,
            year: row.get(3)?,
            month_num: row.get(4)?,
            day: row.get(5)?,
            series: row.get(6)?,
            image_path: rel_image.clone(),
            image_abs: resolve_image_abs(db_path, album_name, &rel_image),
            stampworld_image_url: row.get(8)?,
            description: row.get(9)?,
            watermark: row.get(10)?,
            design: row.get(11)?,
            engraving: row.get(12)?,
            perforation: row.get(13)?,
            context: row.get(14)?,
            colnect_page_url: row.get(15)?,
            colnect_image_url: row.get(16)?,
            colnect_id: row.get(17)?,
            colnect_name: row.get(18)?,
            colnect_catalog_codes: row.get(19)?,
            colnect_country: row.get(20)?,
            colnect_width: row.get(21)?,
            colnect_height: row.get(22)?,
            colnect_format: row.get(23)?,
            colnect_emission: row.get(24)?,
            colnect_printing: row.get(25)?,
            colnect_paper: row.get(26)?,
            colnect_gum: row.get(27)?,
            colnect_ord_mi: row.get(28)?,
            colnect_ord_sn: row.get(29)?,
            colnect_ord_yt: row.get(30)?,
            colnect_face_value: row.get(31)?,
            colnect_currency: row.get(32)?,
            colnect_ord_sg: row.get(33)?,
            colnect_expiry_date: row.get(34)?,
            colnect_print_run: row.get(35)?,
            colnect_colors: row.get(36)?,
            colnect_parent_item: row.get(37)?,
            colnect_variant: row.get(38)?,
            colnect_overprint_variant: row.get(39)?,
            colnect_score: row.get(40)?,
            colnect_accuracy: row.get(41)?,
            colnect_list: row.get(42)?,
            colnect_quantity: row.get(43)?,
            colnect_condition: row.get(44)?,
            colnect_public_note: row.get(45)?,
            colnect_private_note: row.get(46)?,
            all_db_fields: Vec::new(),
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    for stamp in &mut out {
        stamp.all_db_fields = load_stamp_all_fields(conn, stamp.rowid).unwrap_or_default();
    }
    Ok(out)
}

pub(super) fn load_right_candidates(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
) -> Result<Vec<RightCandidate>, Box<dyn std::error::Error>> {
    let mut code_stmt = conn.prepare(
        "SELECT COALESCE(colnect_id, ''), COALESCE(catalog_type, ''), COALESCE(code, '')
         FROM colnect_catalog_codes",
    )?;
    let code_rows = code_stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;
    let mut code_map: HashMap<String, Vec<CatalogCodePair>> = HashMap::new();
    for r in code_rows {
        let (cid, ctype, code) = r?;
        if cid.trim().is_empty() || code.trim().is_empty() {
            continue;
        }
        code_map.entry(cid).or_default().push(CatalogCodePair {
            catalog_type: ctype,
            code,
        });
    }

    let mut stmt = conn.prepare(
        "SELECT COALESCE(c.colnect_id, ''),
                COALESCE(c.sg_number, ''),
                CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER),
                COALESCE(c.series, ''),
                COALESCE(c.colnect_link, ''),
                COALESCE(c.description, ''),
                COALESCE(c.name, ''),
                COALESCE(c.watermark, ''),
                COALESCE(c.designers, ''),
                COALESCE(c.printers, ''),
                COALESCE(c.perforation, ''),
                COALESCE(c.themes, ''),
                COALESCE(c.catalog_codes, ''),
                COALESCE(c.country, ''),
                COALESCE(c.width, ''),
                COALESCE(c.height, ''),
                COALESCE(c.format, ''),
                COALESCE(c.emission, ''),
                COALESCE(c.printing, ''),
                COALESCE(c.paper, ''),
                COALESCE(c.gum, ''),
                COALESCE(c.ord_mi, ''),
                COALESCE(c.ord_sn, ''),
                COALESCE(c.ord_yt, ''),
                COALESCE(c.ord_sg, ''),
                COALESCE(c.expiry_date, ''),
                COALESCE(c.print_run, ''),
                COALESCE(c.face_value, ''),
                COALESCE(c.currency, ''),
                COALESCE(c.colors, ''),
                COALESCE(c.parent_item, ''),
                COALESCE(c.variant, ''),
                COALESCE(c.overprint_variant, ''),
                COALESCE(c.score, ''),
                COALESCE(c.accuracy, ''),
                COALESCE(c.list, ''),
                COALESCE(c.quantity, ''),
                COALESCE(c.condition, ''),
                COALESCE(c.public_note, ''),
                COALESCE(c.private_note, ''),
                COALESCE((
                    SELECT s.image_path
                    FROM stamps s
                    WHERE s.sg_number = c.sg_number
                      AND s.year = CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER)
                      AND TRIM(COALESCE(s.image_path, '')) != ''
                    LIMIT 1
                 ), '')
         FROM colnect_stamps c
         WHERE COALESCE(c.is_deleted, 0) = 0
                     AND COALESCE(c.is_processed, 0) = 0
         ORDER BY CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER), c.sg_number",
    )?;

    let rows = stmt.query_map([], |row| {
        let page_url = row.get::<_, String>(4)?;
        // Resolve Colnect image URLs lazily in the browser to avoid N network
        // requests during startup for every candidate.
        let image_url = String::new();
        let description = {
            let d = row.get::<_, String>(5)?;
            if d.trim().is_empty() {
                row.get::<_, String>(6)?
            } else {
                d
            }
        };
        let image_path = row.get::<_, String>(40)?;

        let colnect_id = row.get::<_, String>(0)?;

        Ok(RightCandidate {
            colnect_id: colnect_id.clone(),
            sg_number: row.get(1)?,
            year: row.get(2)?,
            series: row.get(3)?,
            page_url,
            image_url,
            image_path: image_path.clone(),
            image_abs: resolve_image_abs(db_path, album_name, &image_path),
            name: row.get(6)?,
            description,
            watermark: row.get(7)?,
            design: row.get(8)?,
            engraving: row.get(9)?,
            perforation: row.get(10)?,
            context: row.get(11)?,
            catalog_codes_text: row.get(12)?,
            country: row.get(13)?,
            width: row.get(14)?,
            height: row.get(15)?,
            format: row.get(16)?,
            emission: row.get(17)?,
            printing: row.get(18)?,
            paper: row.get(19)?,
            gum: row.get(20)?,
            ord_mi: row.get(21)?,
            ord_sn: row.get(22)?,
            ord_yt: row.get(23)?,
            ord_sg: row.get(24)?,
            expiry_date: row.get(25)?,
            print_run: row.get(26)?,
            face_value: row.get(27)?,
            currency: row.get(28)?,
            colors: row.get(29)?,
            parent_item: row.get(30)?,
            variant: row.get(31)?,
            overprint_variant: row.get(32)?,
            score: row.get(33)?,
            accuracy: row.get(34)?,
            list: row.get(35)?,
            quantity: row.get(36)?,
            condition: row.get(37)?,
            public_note: row.get(38)?,
            private_note: row.get(39)?,
            catalog_code_pairs: code_map.get(&colnect_id).cloned().unwrap_or_default(),
        })
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

pub(super) fn apply_matches(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    matches: Vec<SelectedMatch>,
    fields: FieldSelection,
    right_map: &HashMap<String, RightCandidate>,
) -> ApplyMatchesResponse {
    let default_catalog_code =
        load_album_config(db_path, album_name).effective_default_catalog_code();
    let mut updated_rows = 0usize;
    let mut skipped_rows = 0usize;
    let mut skipped_details = Vec::new();
    let mut processed_colnect_ids = Vec::new();
    let mut errors = Vec::new();

    for m in matches {
        let Some(c) = right_map.get(&m.colnect_id) else {
            skipped_rows += 1;
            skipped_details.push(format!(
                "skipped rowid={} colnect_id={}: candidate not found in current review session",
                m.left_rowid, m.colnect_id
            ));
            continue;
        };

        let selected_catalog_code =
            crate::catalog::normalize_catalog_number(&m.selected_catalog_code.unwrap_or_default());
        let selected_catalog_type = m
            .selected_catalog_type
            .as_deref()
            .map(crate::catalog::normalize_catalog_type)
            .unwrap_or_else(|| {
                crate::catalog::infer_catalog_type_for_number(
                    &c.catalog_codes_text,
                    &selected_catalog_code,
                    &default_catalog_code,
                )
            });
        let selected_width = m.width.as_deref().unwrap_or(c.width.as_str());
        let selected_height = m.height.as_deref().unwrap_or(c.height.as_str());

        let res = conn.execute(
            "UPDATE stamps
             SET sg_number = CASE WHEN ?1 = 1 AND TRIM(?2) != '' THEN ?2 ELSE sg_number END,
                 catalog_code = CASE WHEN ?1 = 1 AND TRIM(?2) != '' THEN ?3 ELSE catalog_code END,
                      series = CASE WHEN ?4 = 1 AND ?5 != '' THEN ?5 ELSE series END,
                 uniq_key = lower(coalesce(country,'')) || '|' ||
                                     CASE WHEN ?4 = 1 AND ?5 != '' THEN ?5 ELSE coalesce(series,'') END || '|' ||
                            CASE WHEN ?1 = 1 AND TRIM(?2) != '' THEN ?2 ELSE coalesce(sg_number,'') END,
                      description = CASE WHEN ?6 = 1 AND ?7 != '' THEN ?7 ELSE description END,
                      watermark = CASE WHEN ?8 = 1 AND ?9 != '' THEN ?9 ELSE watermark END,
                      design = CASE WHEN ?10 = 1 AND ?11 != '' THEN ?11 ELSE design END,
                      engraving = CASE WHEN ?12 = 1 AND ?13 != '' THEN ?13 ELSE engraving END,
                      perforation = CASE WHEN ?14 = 1 AND ?15 != '' THEN ?15 ELSE perforation END,
                      context = CASE WHEN ?16 = 1 AND ?17 != '' THEN ?17 ELSE context END,
                      colnect_page_url = CASE WHEN ?18 = 1 AND ?19 != '' THEN ?19 ELSE colnect_page_url END,
                      original_image_url = CASE WHEN ?18 = 1 AND ?19 != '' THEN ?19 ELSE original_image_url END,
                      colnect_image_url = CASE WHEN ?20 = 1 AND ?21 != '' THEN ?21 ELSE colnect_image_url END,
                 remote_image_url = CASE
                          WHEN (?20 = 1) AND (?21 != '') AND (TRIM(COALESCE(remote_image_url, '')) = '' OR remote_image_url LIKE '%colnect.com/stamps/stamp/%') THEN ?21
                    ELSE remote_image_url
                 END,
                      colnect_id = CASE WHEN ?22 = 1 THEN ?23 ELSE colnect_id END,
                      colnect_name = CASE WHEN ?22 = 1 THEN ?24 ELSE colnect_name END,
                      colnect_catalog_codes = CASE WHEN ?22 = 1 THEN ?25 ELSE colnect_catalog_codes END,
                      colnect_country = CASE WHEN ?22 = 1 THEN ?26 ELSE colnect_country END,
                      colnect_width = CASE WHEN ?22 = 1 THEN ?27 ELSE colnect_width END,
                      colnect_height = CASE WHEN ?22 = 1 THEN ?28 ELSE colnect_height END,
                      colnect_format = CASE WHEN ?22 = 1 THEN ?29 ELSE colnect_format END,
                      colnect_emission = CASE WHEN ?22 = 1 THEN ?30 ELSE colnect_emission END,
                      colnect_printing = CASE WHEN ?22 = 1 THEN ?31 ELSE colnect_printing END,
                      colnect_paper = CASE WHEN ?22 = 1 THEN ?32 ELSE colnect_paper END,
                      colnect_gum = CASE WHEN ?22 = 1 THEN ?33 ELSE colnect_gum END,
                      colnect_ord_mi = CASE WHEN ?22 = 1 THEN ?34 ELSE colnect_ord_mi END,
                      colnect_ord_sn = CASE WHEN ?22 = 1 THEN ?35 ELSE colnect_ord_sn END,
                      colnect_ord_yt = CASE WHEN ?22 = 1 THEN ?36 ELSE colnect_ord_yt END,
                      colnect_ord_sg = CASE WHEN ?22 = 1 THEN ?37 ELSE colnect_ord_sg END,
                      colnect_expiry_date = CASE WHEN ?22 = 1 THEN ?38 ELSE colnect_expiry_date END,
                      colnect_print_run = CASE WHEN ?22 = 1 THEN ?39 ELSE colnect_print_run END,
                      colnect_face_value = CASE WHEN ?22 = 1 THEN ?40 ELSE colnect_face_value END,
                      colnect_currency = CASE WHEN ?22 = 1 THEN ?41 ELSE colnect_currency END,
                      colnect_colors = CASE WHEN ?22 = 1 THEN ?42 ELSE colnect_colors END,
                      colnect_parent_item = CASE WHEN ?22 = 1 THEN ?43 ELSE colnect_parent_item END,
                      colnect_variant = CASE WHEN ?22 = 1 THEN ?44 ELSE colnect_variant END,
                      colnect_overprint_variant = CASE WHEN ?22 = 1 THEN ?45 ELSE colnect_overprint_variant END,
                      colnect_score = CASE WHEN ?22 = 1 THEN ?46 ELSE colnect_score END,
                      colnect_accuracy = CASE WHEN ?22 = 1 THEN ?47 ELSE colnect_accuracy END,
                      colnect_list = CASE WHEN ?22 = 1 THEN ?48 ELSE colnect_list END,
                      colnect_quantity = CASE WHEN ?22 = 1 THEN ?49 ELSE colnect_quantity END,
                      colnect_condition = CASE WHEN ?22 = 1 THEN ?50 ELSE colnect_condition END,
                      colnect_public_note = CASE WHEN ?22 = 1 THEN ?51 ELSE colnect_public_note END,
                      colnect_private_note = CASE WHEN ?22 = 1 THEN ?52 ELSE colnect_private_note END
                 WHERE rowid = ?53",
            params![
                if fields.replace_sg_number { 1 } else { 0 },
                selected_catalog_code,
                selected_catalog_type,
                if fields.update_series { 1 } else { 0 },
                c.series,
                if fields.update_description { 1 } else { 0 },
                c.description,
                if fields.update_watermark { 1 } else { 0 },
                c.watermark,
                if fields.update_design { 1 } else { 0 },
                c.design,
                if fields.update_engraving { 1 } else { 0 },
                c.engraving,
                if fields.update_perforation { 1 } else { 0 },
                c.perforation,
                if fields.update_context { 1 } else { 0 },
                c.context,
                if fields.update_colnect_links { 1 } else { 0 },
                c.page_url,
                if fields.update_colnect_image_url { 1 } else { 0 },
                c.image_url,
                if fields.update_colnect_metadata { 1 } else { 0 },
                c.colnect_id,
                c.name,
                c.catalog_codes_text,
                c.country,
                selected_width,
                selected_height,
                c.format,
                c.emission,
                c.printing,
                c.paper,
                c.gum,
                c.ord_mi,
                c.ord_sn,
                c.ord_yt,
                c.ord_sg,
                c.expiry_date,
                c.print_run,
                c.face_value,
                c.currency,
                c.colors,
                c.parent_item,
                c.variant,
                c.overprint_variant,
                c.score,
                c.accuracy,
                c.list,
                c.quantity,
                c.condition,
                c.public_note,
                c.private_note,
                m.left_rowid,
            ],
        );

        match res {
            Ok(n) if n > 0 => {
                updated_rows += 1;
                match mark_colnect_candidate_processed(conn, &m.colnect_id) {
                    Ok(_) => processed_colnect_ids.push(m.colnect_id.clone()),
                    Err(e) => errors.push(format!(
                        "updated rowid={} but failed to mark colnect_id={} processed: {}",
                        m.left_rowid, m.colnect_id, e
                    )),
                }
            }
            Ok(_) => {
                skipped_rows += 1;
                skipped_details.push(format!(
                    "skipped rowid={} colnect_id={}: no DB changes applied (selected fields may already match, or catalog number replace was not enabled)",
                    m.left_rowid, m.colnect_id
                ));
            }
            Err(e) => errors.push(format!(
                "failed to apply colnect_id={} to rowid={}: {}",
                m.colnect_id, m.left_rowid, e
            )),
        }
    }

    ApplyMatchesResponse {
        updated_rows,
        skipped_rows,
        skipped_details,
        processed_colnect_ids,
        errors,
    }
}

pub(super) fn load_left_stamp_by_rowid(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    rowid: i64,
) -> Result<Option<LeftStamp>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(
        "SELECT rowid,
                COALESCE(sg_number, ''),
            COALESCE(catalog_code, ''),
                COALESCE(year, 0),
                COALESCE(month_num, 0),
                COALESCE(day, ''),
                COALESCE(series, ''),
                COALESCE(image_path, ''),
                COALESCE(stampworld_image_url, ''),
                COALESCE(description, ''),
                COALESCE(watermark, ''),
                COALESCE(design, ''),
                COALESCE(engraving, ''),
                COALESCE(perforation, ''),
                COALESCE(context, ''),
                  COALESCE(colnect_page_url, ''),
                  COALESCE(colnect_image_url, ''),
                COALESCE(colnect_id, ''),
                COALESCE(colnect_name, ''),
                COALESCE(colnect_catalog_codes, ''),
                COALESCE(colnect_country, ''),
                  COALESCE(colnect_width, ''),
                  COALESCE(colnect_height, ''),
                  COALESCE(colnect_format, ''),
                  COALESCE(colnect_emission, ''),
                  COALESCE(colnect_printing, ''),
                  COALESCE(colnect_paper, ''),
                  COALESCE(colnect_gum, ''),
                  COALESCE(colnect_ord_mi, ''),
                  COALESCE(colnect_ord_sn, ''),
                  COALESCE(colnect_ord_yt, ''),
                COALESCE(colnect_face_value, ''),
                COALESCE(colnect_currency, ''),
                COALESCE(colnect_ord_sg, ''),
                  COALESCE(colnect_expiry_date, ''),
                COALESCE(colnect_print_run, ''),
                  COALESCE(colnect_colors, ''),
                  COALESCE(colnect_parent_item, ''),
                  COALESCE(colnect_variant, ''),
                  COALESCE(colnect_overprint_variant, ''),
                  COALESCE(colnect_score, ''),
                  COALESCE(colnect_accuracy, ''),
                  COALESCE(colnect_list, ''),
                  COALESCE(colnect_quantity, ''),
                  COALESCE(colnect_condition, ''),
                COALESCE(colnect_public_note, ''),
                COALESCE(colnect_private_note, '')
         FROM stamps
         WHERE rowid = ?1
         LIMIT 1",
    )?;

    let mut rows = stmt.query(params![rowid])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };

    let rel_image: String = row.get(7)?;
    let resolved_rowid: i64 = row.get(0)?;
    Ok(Some(LeftStamp {
        rowid: resolved_rowid,
        sg_number: row.get(1)?,
        catalog_code: row.get(2)?,
        year: row.get(3)?,
        month_num: row.get(4)?,
        day: row.get(5)?,
        series: row.get(6)?,
        image_path: rel_image.clone(),
        image_abs: resolve_image_abs(db_path, album_name, &rel_image),
        stampworld_image_url: row.get(8)?,
        description: row.get(9)?,
        watermark: row.get(10)?,
        design: row.get(11)?,
        engraving: row.get(12)?,
        perforation: row.get(13)?,
        context: row.get(14)?,
        colnect_page_url: row.get(15)?,
        colnect_image_url: row.get(16)?,
        colnect_id: row.get(17)?,
        colnect_name: row.get(18)?,
        colnect_catalog_codes: row.get(19)?,
        colnect_country: row.get(20)?,
        colnect_width: row.get(21)?,
        colnect_height: row.get(22)?,
        colnect_format: row.get(23)?,
        colnect_emission: row.get(24)?,
        colnect_printing: row.get(25)?,
        colnect_paper: row.get(26)?,
        colnect_gum: row.get(27)?,
        colnect_ord_mi: row.get(28)?,
        colnect_ord_sn: row.get(29)?,
        colnect_ord_yt: row.get(30)?,
        colnect_face_value: row.get(31)?,
        colnect_currency: row.get(32)?,
        colnect_ord_sg: row.get(33)?,
        colnect_expiry_date: row.get(34)?,
        colnect_print_run: row.get(35)?,
        colnect_colors: row.get(36)?,
        colnect_parent_item: row.get(37)?,
        colnect_variant: row.get(38)?,
        colnect_overprint_variant: row.get(39)?,
        colnect_score: row.get(40)?,
        colnect_accuracy: row.get(41)?,
        colnect_list: row.get(42)?,
        colnect_quantity: row.get(43)?,
        colnect_condition: row.get(44)?,
        colnect_public_note: row.get(45)?,
        colnect_private_note: row.get(46)?,
        all_db_fields: load_stamp_all_fields(conn, resolved_rowid).unwrap_or_default(),
    }))
}

pub(super) fn load_right_candidate_by_colnect_id(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    colnect_id: &str,
) -> Result<Option<RightCandidate>, Box<dyn std::error::Error>> {
    let mut code_stmt = conn.prepare(
        "SELECT COALESCE(catalog_type, ''), COALESCE(code, '')
         FROM colnect_catalog_codes
         WHERE COALESCE(colnect_id, '') = ?1",
    )?;
    let code_rows = code_stmt.query_map(params![colnect_id], |row| {
        Ok(CatalogCodePair {
            catalog_type: row.get(0)?,
            code: row.get(1)?,
        })
    })?;
    let mut code_pairs = Vec::new();
    for r in code_rows {
        let p = r?;
        if p.code.trim().is_empty() {
            continue;
        }
        code_pairs.push(p);
    }

    let mut stmt = conn.prepare(
        "SELECT COALESCE(c.colnect_id, ''),
                COALESCE(c.sg_number, ''),
                CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER),
                COALESCE(c.series, ''),
                COALESCE(c.colnect_link, ''),
                COALESCE(c.description, ''),
                COALESCE(c.name, ''),
                COALESCE(c.watermark, ''),
                COALESCE(c.designers, ''),
                COALESCE(c.printers, ''),
                COALESCE(c.perforation, ''),
                COALESCE(c.themes, ''),
                COALESCE(c.catalog_codes, ''),
                COALESCE(c.country, ''),
                COALESCE(c.width, ''),
                COALESCE(c.height, ''),
                COALESCE(c.format, ''),
                COALESCE(c.emission, ''),
                COALESCE(c.printing, ''),
                COALESCE(c.paper, ''),
                COALESCE(c.gum, ''),
                COALESCE(c.ord_mi, ''),
                COALESCE(c.ord_sn, ''),
                COALESCE(c.ord_yt, ''),
                COALESCE(c.ord_sg, ''),
                COALESCE(c.expiry_date, ''),
                COALESCE(c.print_run, ''),
                COALESCE(c.face_value, ''),
                COALESCE(c.currency, ''),
                COALESCE(c.colors, ''),
                COALESCE(c.parent_item, ''),
                COALESCE(c.variant, ''),
                COALESCE(c.overprint_variant, ''),
                COALESCE(c.score, ''),
                COALESCE(c.accuracy, ''),
                COALESCE(c.list, ''),
                COALESCE(c.quantity, ''),
                COALESCE(c.condition, ''),
                COALESCE(c.public_note, ''),
                COALESCE(c.private_note, ''),
                COALESCE((
                    SELECT s.image_path
                    FROM stamps s
                    WHERE s.sg_number = c.sg_number
                      AND s.year = CAST(substr(COALESCE(c.issued_on, ''), 1, 4) AS INTEGER)
                      AND TRIM(COALESCE(s.image_path, '')) != ''
                    LIMIT 1
                ), '')
                 FROM colnect_stamps c
                 WHERE COALESCE(c.colnect_id, '') = ?1
                     AND COALESCE(c.is_deleted, 0) = 0
                     AND COALESCE(c.is_processed, 0) = 0
         LIMIT 1",
    )?;

    let mut rows = stmt.query(params![colnect_id])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };

    let page_url: String = row.get(4)?;
    let image_url = if !page_url.trim().is_empty() {
        crate::colnect::image_fetcher::resolve_colnect_image_url(&page_url).unwrap_or_default()
    } else {
        String::new()
    };
    let description = {
        let d: String = row.get(5)?;
        if d.trim().is_empty() {
            row.get::<_, String>(6)?
        } else {
            d
        }
    };
    let image_path: String = row.get(40)?;

    Ok(Some(RightCandidate {
        colnect_id: row.get(0)?,
        sg_number: row.get(1)?,
        year: row.get(2)?,
        series: row.get(3)?,
        page_url,
        image_url,
        image_path: image_path.clone(),
        image_abs: resolve_image_abs(db_path, album_name, &image_path),
        description,
        watermark: row.get(7)?,
        design: row.get(8)?,
        engraving: row.get(9)?,
        perforation: row.get(10)?,
        context: row.get(11)?,
        name: row.get(6)?,
        catalog_codes_text: row.get(12)?,
        country: row.get(13)?,
        width: row.get(14)?,
        height: row.get(15)?,
        format: row.get(16)?,
        emission: row.get(17)?,
        printing: row.get(18)?,
        paper: row.get(19)?,
        gum: row.get(20)?,
        ord_mi: row.get(21)?,
        ord_sn: row.get(22)?,
        ord_yt: row.get(23)?,
        ord_sg: row.get(24)?,
        expiry_date: row.get(25)?,
        print_run: row.get(26)?,
        face_value: row.get(27)?,
        currency: row.get(28)?,
        colors: row.get(29)?,
        parent_item: row.get(30)?,
        variant: row.get(31)?,
        overprint_variant: row.get(32)?,
        score: row.get(33)?,
        accuracy: row.get(34)?,
        list: row.get(35)?,
        quantity: row.get(36)?,
        condition: row.get(37)?,
        public_note: row.get(38)?,
        private_note: row.get(39)?,
        catalog_code_pairs: code_pairs,
    }))
}

pub(super) fn add_colnect_stamp_to_stamps(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    req: AddColnectStampRequest,
) -> AddColnectStampResponse {
    if req.colnect_id.trim().is_empty() {
        return AddColnectStampResponse {
            ok: false,
            message: "colnect_id is required".to_string(),
            added_left: None,
            processed_colnect_id: None,
        };
    }

    let candidate =
        match load_right_candidate_by_colnect_id(conn, db_path, album_name, &req.colnect_id) {
            Ok(Some(c)) => c,
            Ok(None) => {
                return AddColnectStampResponse {
                    ok: false,
                    message: format!("colnect candidate not found for id={}", req.colnect_id),
                    added_left: None,
                    processed_colnect_id: None,
                }
            }
            Err(e) => {
                return AddColnectStampResponse {
                    ok: false,
                    message: format!("failed to load colnect candidate: {}", e),
                    added_left: None,
                    processed_colnect_id: None,
                }
            }
        };

    let existing = conn.query_row(
        "SELECT COUNT(*) FROM stamps WHERE TRIM(COALESCE(colnect_id, '')) = ?1",
        params![candidate.colnect_id.clone()],
        |row| row.get::<_, i64>(0),
    );
    if let Ok(count) = existing {
        if count > 0 {
            return AddColnectStampResponse {
                ok: false,
                message: format!(
                    "stamp with colnect_id={} already exists in stamps",
                    candidate.colnect_id
                ),
                added_left: None,
                processed_colnect_id: None,
            };
        }
    }

    let mut image_path = String::new();
    if req.associate_image {
        let mode = req
            .image_mode
            .as_deref()
            .unwrap_or("current_left")
            .to_ascii_lowercase();
        if mode == "current_left" {
            if let Some(left_rowid) = req.source_left_rowid {
                match conn.query_row(
                    "SELECT COALESCE(image_path, '') FROM stamps WHERE rowid = ?1",
                    params![left_rowid],
                    |row| row.get::<_, String>(0),
                ) {
                    Ok(v) if !v.trim().is_empty() => {
                        image_path = v;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        return AddColnectStampResponse {
                            ok: false,
                            message: format!(
                                "failed reading image from source row {}: {}",
                                left_rowid, e
                            ),
                            added_left: None,
                            processed_colnect_id: None,
                        };
                    }
                }
            }
        } else if mode == "upload" {
            let Some(upload_data_url) = req.upload_data_url.as_deref() else {
                return AddColnectStampResponse {
                    ok: false,
                    message: "upload mode selected but no upload data provided".to_string(),
                    added_left: None,
                    processed_colnect_id: None,
                };
            };
            match store_uploaded_image_from_data_url(
                db_path,
                album_name,
                &candidate.colnect_id,
                upload_data_url,
                req.upload_filename.as_deref(),
            ) {
                Ok(rel_path) => image_path = rel_path,
                Err(e) => {
                    return AddColnectStampResponse {
                        ok: false,
                        message: e,
                        added_left: None,
                        processed_colnect_id: None,
                    }
                }
            }
        }
    }

    let default_catalog_code =
        load_album_config(db_path, album_name).effective_default_catalog_code();
    let sg_number = crate::catalog::normalize_catalog_number(
        &req.target_sg_number
            .unwrap_or_else(|| candidate.sg_number.clone()),
    );
    let catalog_code = crate::catalog::infer_catalog_type_for_number(
        &candidate.catalog_codes_text,
        &sg_number,
        &default_catalog_code,
    );
    let year = req.target_year.unwrap_or(candidate.year);
    let series = req
        .target_series
        .unwrap_or_else(|| candidate.series.clone());

    let insert_res = conn.execute(
        "INSERT INTO stamps (
            country, series, year, sg_number, catalog_code, uniq_key,
            description, image_path, remote_image_url, original_image_url,
            colnect_page_url, colnect_image_url, colnect_id, colnect_name,
            colnect_catalog_codes, colnect_country, colnect_width, colnect_height,
            colnect_format, colnect_emission, colnect_printing, colnect_paper,
            colnect_gum, colnect_ord_mi, colnect_ord_sn, colnect_ord_yt,
            colnect_ord_sg, colnect_expiry_date, colnect_print_run, colnect_face_value,
            colnect_currency, colnect_colors, colnect_parent_item, colnect_variant,
            colnect_overprint_variant, colnect_score, colnect_accuracy, colnect_list,
            colnect_quantity, colnect_condition, colnect_public_note, colnect_private_note,
            watermark, design, engraving, perforation, context
         ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6,
                ?7, ?8, ?9, ?10,
                ?11, ?12, ?13, ?14,
                ?15, ?16, ?17, ?18,
                ?19, ?20, ?21, ?22,
                ?23, ?24, ?25, ?26,
                ?27, ?28, ?29, ?30,
                ?31, ?32, ?33, ?34,
                ?35, ?36, ?37, ?38,
                ?39, ?40, ?41, ?42,
                ?43, ?44, ?45, ?46, ?47
         )",
        params![
            candidate.country,
            series.clone(),
            year,
            sg_number.clone(),
            catalog_code,
            format!(
                "{}|{}|{}",
                candidate.country.trim().to_ascii_lowercase(),
                series,
                sg_number
            ),
            candidate.description,
            image_path,
            candidate.image_url,
            candidate.page_url,
            candidate.page_url,
            candidate.image_url,
            candidate.colnect_id,
            candidate.name,
            candidate.catalog_codes_text,
            candidate.country,
            candidate.width,
            candidate.height,
            candidate.format,
            candidate.emission,
            candidate.printing,
            candidate.paper,
            candidate.gum,
            candidate.ord_mi,
            candidate.ord_sn,
            candidate.ord_yt,
            candidate.ord_sg,
            candidate.expiry_date,
            candidate.print_run,
            candidate.face_value,
            candidate.currency,
            candidate.colors,
            candidate.parent_item,
            candidate.variant,
            candidate.overprint_variant,
            candidate.score,
            candidate.accuracy,
            candidate.list,
            candidate.quantity,
            candidate.condition,
            candidate.public_note,
            candidate.private_note,
            candidate.watermark,
            candidate.design,
            candidate.engraving,
            candidate.perforation,
            candidate.context,
        ],
    );

    if let Err(e) = insert_res {
        return AddColnectStampResponse {
            ok: false,
            message: format!("failed to insert into stamps: {}", e),
            added_left: None,
            processed_colnect_id: None,
        };
    }

    let rowid = conn.last_insert_rowid();
    let added_left = load_left_stamp_by_rowid(conn, db_path, album_name, rowid)
        .ok()
        .flatten();

    let processed_colnect_id = match mark_colnect_candidate_processed(conn, &req.colnect_id) {
        Ok(_) => Some(req.colnect_id.clone()),
        Err(_) => None,
    };

    let message = if processed_colnect_id.is_some() {
        format!(
            "added colnect_id={} into stamps rowid={}",
            req.colnect_id, rowid
        )
    } else {
        format!(
            "added colnect_id={} into stamps rowid={} (warning: candidate was not marked processed until next UI sync)",
            req.colnect_id, rowid
        )
    };

    AddColnectStampResponse {
        ok: true,
        message,
        added_left,
        processed_colnect_id,
    }
}

pub(super) fn restore_colnect_candidate_to_queue(
    conn: &Connection,
    colnect_id: &str,
) -> rusqlite::Result<usize> {
    if colnect_id.trim().is_empty() {
        return Ok(0);
    }

    conn.execute(
        "UPDATE colnect_stamps
         SET is_processed = 0
         WHERE TRIM(COALESCE(colnect_id, '')) = TRIM(?1)",
        params![colnect_id],
    )
}

pub(super) fn query_stamp_image_path(conn: &Connection, rowid: i64) -> Result<String, String> {
    conn.query_row(
        "SELECT COALESCE(image_path, '') FROM stamps WHERE rowid = ?1",
        params![rowid],
        |row| row.get::<_, String>(0),
    )
    .map_err(|e| format!("failed to read image path from row {}: {}", rowid, e))
}

pub(super) fn resolve_left_editor_image_path(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    image_mode: Option<&str>,
    source_rowid: Option<i64>,
    current_image_path: Option<&str>,
    upload_data_url: Option<&str>,
    upload_filename: Option<&str>,
    file_token: &str,
) -> Result<String, String> {
    let mode = image_mode
        .unwrap_or(if current_image_path.is_some() {
            "keep"
        } else {
            "current_left"
        })
        .trim()
        .to_ascii_lowercase();

    match mode.as_str() {
        "keep" => Ok(current_image_path.unwrap_or("").trim().to_string()),
        "current_left" => {
            let Some(rowid) = source_rowid else {
                return Err("current-left image mode requires a source row".to_string());
            };
            let image_path = query_stamp_image_path(conn, rowid)?;
            if image_path.trim().is_empty() {
                return Err(format!("source row {} has no image to reuse", rowid));
            }
            Ok(image_path)
        }
        "upload" => {
            let Some(data_url) = upload_data_url else {
                return Err("upload image mode requires an uploaded file".to_string());
            };
            store_uploaded_image_from_data_url(
                db_path,
                album_name,
                file_token,
                data_url,
                upload_filename,
            )
        }
        other => Err(format!("unsupported image mode: {}", other)),
    }
}

pub(super) fn save_left_stamp(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    req: SaveLeftStampRequest,
) -> SaveLeftStampResponse {
    if req.rowid <= 0 {
        return SaveLeftStampResponse {
            ok: false,
            message: "rowid is required".to_string(),
            updated_left: None,
        };
    }
    if req.sg_number.trim().is_empty() {
        return SaveLeftStampResponse {
            ok: false,
            message: "Catalog number / reference code is required".to_string(),
            updated_left: None,
        };
    }

    let existing_image_path = match query_stamp_image_path(conn, req.rowid) {
        Ok(value) => value,
        Err(message) => {
            return SaveLeftStampResponse {
                ok: false,
                message,
                updated_left: None,
            }
        }
    };

    let image_path = match resolve_left_editor_image_path(
        conn,
        db_path,
        album_name,
        req.image_mode.as_deref(),
        Some(req.rowid),
        Some(existing_image_path.as_str()),
        req.upload_data_url.as_deref(),
        req.upload_filename.as_deref(),
        &req.sg_number,
    ) {
        Ok(value) => value,
        Err(message) => {
            return SaveLeftStampResponse {
                ok: false,
                message,
                updated_left: None,
            }
        }
    };

    let resolved_colnect_width = if req.colnect_width.trim().is_empty() {
        req.width.trim().to_string()
    } else {
        req.colnect_width.trim().to_string()
    };
    let resolved_colnect_height = if req.colnect_height.trim().is_empty() {
        req.height.trim().to_string()
    } else {
        req.colnect_height.trim().to_string()
    };

    let update_res = conn.execute(
        "UPDATE stamps
         SET sg_number = ?1,
             catalog_code = ?2,
             year = ?3,
             series = ?4,
             description = ?5,
             watermark = ?6,
             design = ?7,
             engraving = ?8,
             perforation = ?9,
             context = ?10,
             image_path = ?11,
             colnect_page_url = ?12,
             colnect_image_url = ?13,
             colnect_id = ?14,
             colnect_name = ?15,
             colnect_catalog_codes = ?16,
             colnect_country = ?17,
             colnect_width = ?18,
             colnect_height = ?19,
             colnect_format = ?20,
             colnect_emission = ?21,
             colnect_printing = ?22,
             colnect_paper = ?23,
             colnect_gum = ?24,
             colnect_ord_mi = ?25,
             colnect_ord_sn = ?26,
             colnect_ord_yt = ?27,
             colnect_face_value = ?28,
             colnect_currency = ?29,
             colnect_ord_sg = ?30,
             colnect_expiry_date = ?31,
             colnect_print_run = ?32,
             colnect_colors = ?33,
             colnect_parent_item = ?34,
             colnect_variant = ?35,
             colnect_overprint_variant = ?36,
             colnect_score = ?37,
             colnect_accuracy = ?38,
             colnect_list = ?39,
             colnect_quantity = ?40,
             colnect_condition = ?41,
             colnect_public_note = ?42,
             colnect_private_note = ?43,
             uniq_key = lower(coalesce(country, '')) || '|' || ?4 || '|' || ?1
         WHERE rowid = ?44",
        params![
            req.sg_number.trim(),
            crate::catalog::normalize_catalog_type(&req.catalog_code),
            req.year,
            req.series.trim(),
            req.description.trim(),
            req.watermark.trim(),
            req.design.trim(),
            req.engraving.trim(),
            req.perforation.trim(),
            req.context.trim(),
            image_path,
            req.colnect_page_url.trim(),
            req.colnect_image_url.trim(),
            req.colnect_id.trim(),
            req.colnect_name.trim(),
            req.colnect_catalog_codes.trim(),
            req.colnect_country.trim(),
            resolved_colnect_width,
            resolved_colnect_height,
            req.colnect_format.trim(),
            req.colnect_emission.trim(),
            req.colnect_printing.trim(),
            req.colnect_paper.trim(),
            req.colnect_gum.trim(),
            req.colnect_ord_mi.trim(),
            req.colnect_ord_sn.trim(),
            req.colnect_ord_yt.trim(),
            req.colnect_face_value.trim(),
            req.colnect_currency.trim(),
            req.colnect_ord_sg.trim(),
            req.colnect_expiry_date.trim(),
            req.colnect_print_run.trim(),
            req.colnect_colors.trim(),
            req.colnect_parent_item.trim(),
            req.colnect_variant.trim(),
            req.colnect_overprint_variant.trim(),
            req.colnect_score.trim(),
            req.colnect_accuracy.trim(),
            req.colnect_list.trim(),
            req.colnect_quantity.trim(),
            req.colnect_condition.trim(),
            req.colnect_public_note.trim(),
            req.colnect_private_note.trim(),
            req.rowid,
        ],
    );

    match update_res {
        Ok(0) => SaveLeftStampResponse {
            ok: false,
            message: format!("no stamp row found for rowid={}", req.rowid),
            updated_left: None,
        },
        Ok(_) => SaveLeftStampResponse {
            ok: true,
            message: format!("updated base stamp rowid={}", req.rowid),
            updated_left: load_left_stamp_by_rowid(conn, db_path, album_name, req.rowid)
                .ok()
                .flatten(),
        },
        Err(e) => SaveLeftStampResponse {
            ok: false,
            message: format!("failed to update rowid={}: {}", req.rowid, e),
            updated_left: None,
        },
    }
}

pub(super) fn add_left_stamp(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    req: AddLeftStampRequest,
) -> AddLeftStampResponse {
    if req.sg_number.trim().is_empty() {
        return AddLeftStampResponse {
            ok: false,
            message: "Catalog number / reference code is required".to_string(),
            added_left: None,
        };
    }

    let country = req
        .source_rowid
        .and_then(|rowid| {
            conn.query_row(
                "SELECT COALESCE(country, '') FROM stamps WHERE rowid = ?1",
                params![rowid],
                |row| row.get::<_, String>(0),
            )
            .ok()
        })
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| album_name.to_string());

    let image_path = match resolve_left_editor_image_path(
        conn,
        db_path,
        album_name,
        req.image_mode.as_deref(),
        req.source_rowid,
        None,
        req.upload_data_url.as_deref(),
        req.upload_filename.as_deref(),
        &req.sg_number,
    ) {
        Ok(value) => value,
        Err(message) => {
            return AddLeftStampResponse {
                ok: false,
                message,
                added_left: None,
            }
        }
    };

    let catalog_code = {
        let requested = crate::catalog::normalize_catalog_type(&req.catalog_code);
        if requested.is_empty() {
            load_album_config(db_path, album_name).effective_default_catalog_code()
        } else {
            requested
        }
    };
    let series = req.series.trim().to_string();
    let sg_number = req.sg_number.trim().to_string();
    let uniq_key = format!(
        "{}|{}|{}",
        country.trim().to_ascii_lowercase(),
        series,
        sg_number
    );

    let resolved_colnect_width = if req.colnect_width.trim().is_empty() {
        req.width.trim().to_string()
    } else {
        req.colnect_width.trim().to_string()
    };
    let resolved_colnect_height = if req.colnect_height.trim().is_empty() {
        req.height.trim().to_string()
    } else {
        req.colnect_height.trim().to_string()
    };

    let insert_res = conn.execute(
        "INSERT INTO stamps (
            country, series, year, sg_number, catalog_code, uniq_key,
            description, image_path, watermark, design, engraving, perforation, context,
            colnect_page_url, colnect_image_url, colnect_id, colnect_name,
            colnect_catalog_codes, colnect_country, colnect_width, colnect_height,
            colnect_format, colnect_emission, colnect_printing, colnect_paper,
            colnect_gum, colnect_ord_mi, colnect_ord_sn, colnect_ord_yt,
            colnect_face_value, colnect_currency, colnect_ord_sg, colnect_expiry_date,
            colnect_print_run, colnect_colors, colnect_parent_item, colnect_variant,
            colnect_overprint_variant, colnect_score, colnect_accuracy, colnect_list,
            colnect_quantity, colnect_condition, colnect_public_note, colnect_private_note
         ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13,
            ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21,
            ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29,
            ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37,
                ?38, ?39, ?40, ?41, ?42, ?43, ?44, ?45
         )",
        params![
            country,
            series,
            req.year,
            sg_number,
            catalog_code,
            uniq_key,
            req.description.trim(),
            image_path,
            req.watermark.trim(),
            req.design.trim(),
            req.engraving.trim(),
            req.perforation.trim(),
            req.context.trim(),
            req.colnect_page_url.trim(),
            req.colnect_image_url.trim(),
            req.colnect_id.trim(),
            req.colnect_name.trim(),
            req.colnect_catalog_codes.trim(),
            req.colnect_country.trim(),
            resolved_colnect_width,
            resolved_colnect_height,
            req.colnect_format.trim(),
            req.colnect_emission.trim(),
            req.colnect_printing.trim(),
            req.colnect_paper.trim(),
            req.colnect_gum.trim(),
            req.colnect_ord_mi.trim(),
            req.colnect_ord_sn.trim(),
            req.colnect_ord_yt.trim(),
            req.colnect_face_value.trim(),
            req.colnect_currency.trim(),
            req.colnect_ord_sg.trim(),
            req.colnect_expiry_date.trim(),
            req.colnect_print_run.trim(),
            req.colnect_colors.trim(),
            req.colnect_parent_item.trim(),
            req.colnect_variant.trim(),
            req.colnect_overprint_variant.trim(),
            req.colnect_score.trim(),
            req.colnect_accuracy.trim(),
            req.colnect_list.trim(),
            req.colnect_quantity.trim(),
            req.colnect_condition.trim(),
            req.colnect_public_note.trim(),
            req.colnect_private_note.trim(),
        ],
    );

    match insert_res {
        Ok(_) => {
            let rowid = conn.last_insert_rowid();
            AddLeftStampResponse {
                ok: true,
                message: format!("added base/reference item rowid={}", rowid),
                added_left: load_left_stamp_by_rowid(conn, db_path, album_name, rowid)
                    .ok()
                    .flatten(),
            }
        }
        Err(e) => AddLeftStampResponse {
            ok: false,
            message: format!("failed to add base/reference item: {}", e),
            added_left: None,
        },
    }
}

pub(super) fn delete_left_stamp(
    conn: &Connection,
    req: DeleteLeftStampRequest,
) -> DeleteLeftStampResponse {
    if req.rowid <= 0 {
        return DeleteLeftStampResponse {
            ok: false,
            message: "rowid is required".to_string(),
            deleted_rowid: None,
            restored_colnect_id: None,
        };
    }

    let linked_colnect_id = conn
        .query_row(
            "SELECT COALESCE(colnect_id, '') FROM stamps WHERE rowid = ?1",
            params![req.rowid],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_default();

    match conn.execute("DELETE FROM stamps WHERE rowid = ?1", params![req.rowid]) {
        Ok(0) => DeleteLeftStampResponse {
            ok: false,
            message: format!("no stamp row found for rowid={}", req.rowid),
            deleted_rowid: None,
            restored_colnect_id: None,
        },
        Ok(_) => {
            let restored_colnect_id = if linked_colnect_id.trim().is_empty() {
                None
            } else {
                let _ = restore_colnect_candidate_to_queue(conn, &linked_colnect_id);
                Some(linked_colnect_id.clone())
            };
            DeleteLeftStampResponse {
                ok: true,
                message: format!("deleted base stamp rowid={}", req.rowid),
                deleted_rowid: Some(req.rowid),
                restored_colnect_id,
            }
        }
        Err(e) => DeleteLeftStampResponse {
            ok: false,
            message: format!("failed to delete rowid={}: {}", req.rowid, e),
            deleted_rowid: None,
            restored_colnect_id: None,
        },
    }
}

pub(super) fn clear_left_colnect_link(
    conn: &Connection,
    db_path: &str,
    album_name: &str,
    req: ClearLeftColnectLinkRequest,
) -> ClearLeftColnectLinkResponse {
    if req.rowid <= 0 {
        return ClearLeftColnectLinkResponse {
            ok: false,
            message: "rowid is required".to_string(),
            updated_left: None,
            restored_colnect_id: None,
        };
    }

    let linked_colnect_id = conn
        .query_row(
            "SELECT COALESCE(colnect_id, '') FROM stamps WHERE rowid = ?1",
            params![req.rowid],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_default();

    let update_res = conn.execute(
        "UPDATE stamps
         SET colnect_page_url = '',
             colnect_image_url = '',
             colnect_id = '',
             colnect_name = '',
             colnect_catalog_codes = '',
             colnect_country = '',
             colnect_width = '',
             colnect_height = '',
             colnect_format = '',
             colnect_emission = '',
             colnect_printing = '',
             colnect_paper = '',
             colnect_gum = '',
             colnect_ord_mi = '',
             colnect_ord_sn = '',
             colnect_ord_yt = '',
             colnect_face_value = '',
             colnect_currency = '',
             colnect_ord_sg = '',
             colnect_expiry_date = '',
             colnect_print_run = '',
             colnect_colors = '',
             colnect_parent_item = '',
             colnect_variant = '',
             colnect_overprint_variant = '',
             colnect_score = '',
             colnect_accuracy = '',
             colnect_list = '',
             colnect_quantity = '',
             colnect_condition = '',
             colnect_public_note = '',
             colnect_private_note = ''
         WHERE rowid = ?1",
        params![req.rowid],
    );

    match update_res {
        Ok(0) => ClearLeftColnectLinkResponse {
            ok: false,
            message: format!("no stamp row found for rowid={}", req.rowid),
            updated_left: None,
            restored_colnect_id: None,
        },
        Ok(_) => {
            let restored_colnect_id = if linked_colnect_id.trim().is_empty() {
                None
            } else {
                let _ = restore_colnect_candidate_to_queue(conn, &linked_colnect_id);
                Some(linked_colnect_id.clone())
            };
            ClearLeftColnectLinkResponse {
                ok: true,
                message: if let Some(colnect_id) = restored_colnect_id.as_ref() {
                    format!(
                        "cleared linked Colnect data from rowid={} and returned {} to the candidate list",
                        req.rowid, colnect_id
                    )
                } else {
                    format!("rowid={} had no linked Colnect data to clear", req.rowid)
                },
                updated_left: load_left_stamp_by_rowid(conn, db_path, album_name, req.rowid)
                    .ok()
                    .flatten(),
                restored_colnect_id,
            }
        }
        Err(e) => ClearLeftColnectLinkResponse {
            ok: false,
            message: format!(
                "failed clearing Colnect data for rowid={}: {}",
                req.rowid, e
            ),
            updated_left: None,
            restored_colnect_id: None,
        },
    }
}

pub(super) fn delete_colnect_candidate(
    conn: &Connection,
    req: DeleteColnectCandidateRequest,
) -> DeleteColnectCandidateResponse {
    if req.colnect_id.trim().is_empty() {
        return DeleteColnectCandidateResponse {
            ok: false,
            message: "colnect_id is required".to_string(),
            deleted_stamp_rows: 0,
            deleted_code_rows: 0,
        };
    }

    let deleted_stamp_rows = match conn.execute(
        "UPDATE colnect_stamps
         SET is_deleted = 1,
             deleted_at = ?2
         WHERE COALESCE(colnect_id, '') = ?1
           AND COALESCE(is_deleted, 0) = 0",
        params![req.colnect_id.clone(), chrono::Utc::now().to_rfc3339()],
    ) {
        Ok(n) => n,
        Err(e) => {
            return DeleteColnectCandidateResponse {
                ok: false,
                message: format!("failed soft-deleting colnect candidate: {}", e),
                deleted_stamp_rows: 0,
                deleted_code_rows: 0,
            };
        }
    };

    DeleteColnectCandidateResponse {
        ok: true,
        message: format!(
            "soft-deleted colnect_id={} (rows={})",
            req.colnect_id, deleted_stamp_rows
        ),
        deleted_stamp_rows,
        deleted_code_rows: 0,
    }
}

pub(super) fn undo_colnect_delete(
    conn: &Connection,
    req: UndoColnectDeleteRequest,
) -> UndoColnectDeleteResponse {
    if req.colnect_id.trim().is_empty() {
        return UndoColnectDeleteResponse {
            ok: false,
            message: "colnect_id is required".to_string(),
            restored_rows: 0,
        };
    }

    let restored_rows = match conn.execute(
        "UPDATE colnect_stamps
         SET is_deleted = 0,
             deleted_at = ''
         WHERE COALESCE(colnect_id, '') = ?1
           AND COALESCE(is_deleted, 0) = 1",
        params![req.colnect_id.clone()],
    ) {
        Ok(n) => n,
        Err(e) => {
            return UndoColnectDeleteResponse {
                ok: false,
                message: format!("failed undoing soft delete: {}", e),
                restored_rows: 0,
            };
        }
    };

    UndoColnectDeleteResponse {
        ok: true,
        message: format!(
            "restored colnect_id={} (rows={})",
            req.colnect_id, restored_rows
        ),
        restored_rows,
    }
}
