//! Archived copy of the original `src/db_utils.rs` prior to cleanup.
```rust
// Full archived copy of original `src/db_utils.rs` prior to the refactor.
// This is a verbatim snapshot of the file as it existed before it was
// removed from `src/` and replaced by a thin wrapper.
//
// NOTE: keep this file read-only archival material. Do not import it from
// the crate root. If you wish to restore functionality, copy relevant parts
// back into `src/` or the new `crates/stamp_album_db` crate instead.

//! Thin compatibility wrapper for DB helpers.
//!
//! This file delegates heavy DB operations to `crates/stamp_album_db` while
//! preserving a small compatibility surface used by the rest of the main
//! crate. The goal is a minimal, compile-safe change for the first refactor
//! step.

pub use stamp_album_db::StampStub;
use rusqlite::{Connection, Result};

/// Initialize DB using the DB crate implementation
pub fn init_db(db_path: &str) -> Result<Connection> {
	stamp_album_db::init_db(db_path)
}

/// Run stamps table migration (delegates to DB crate)
pub fn migrate_stamps_table(conn: &Connection) -> Result<()> {
	stamp_album_db::migrate_stamps_table(conn)
}

/// Run block metadata migration (delegates to DB crate)
pub fn migrate_fix_block_metadata(conn: &Connection) -> Result<()> {
	stamp_album_db::migrate_fix_block_metadata(conn)
}

/// Delegate insert_stamp to the DB crate (keeps same signature)
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
	stamp_album_db::insert_stamp(
		conn,
		country,
		series,
		year,
		month,
		_month_num,
		day,
		sg_number,
		type_,
		denomination,
		color,
		description,
		stamps_issued,
		mint_condition,
		unused,
		used,
		letter_fdc,
		currency,
		image_path,
		remote_image_url,
		original_image_url,
		watermark,
		design,
		engraving,
		perforation,
		context,
	)
}

/// Delegate get_stamps_by_series to crate
pub fn get_stamps_by_series(conn: &Connection, series: &str, year: &str) -> Result<Vec<StampStub>> {
	stamp_album_db::get_stamps_by_series(conn, series, year)
}

/// Delegate get_stamps_by_country_series to crate
pub fn get_stamps_by_country_series(
	conn: &Connection,
	country: &str,
	series: &str,
) -> Result<Vec<(i32, String, String, i32, String, i32, String, String, String, String, String, String)>> {
	stamp_album_db::get_stamps_by_country_series(conn, country, series)
}

/// Keep local implementation for formatting stamps_issued (was used widely)
pub fn format_stamps_issued(s: &str) -> String {
	// Ported from the original implementation to keep compatibility
	let t = s.trim();
	if t.is_empty() {
		return String::new();
	}

	// Remove surrounding parentheses
	let mut v = t.to_string();
	if v.starts_with('(') && v.ends_with(')') {
		v = v[1..v.len() - 1].to_string();
	}

	// Keep only digits, dot, comma
	let re_keep = regex::Regex::new(r"[^0-9\.,]").unwrap();
	v = re_keep.replace_all(&v, "").to_string();
	if v.is_empty() {
		return String::new();
	}

	// Determine decimal separator using heuristics:
	let dot_count = v.matches('.').count();
	let comma_count = v.matches(',').count();
	let (int_part, frac_part_opt) = if dot_count > 0 && comma_count > 0 {
		let last_dot = v.rfind('.').unwrap();
		let last_comma = v.rfind(',').unwrap();
		if last_dot > last_comma {
			(
				v[..last_dot].to_string(),
				Some(v[last_dot + 1..].to_string()),
			)
		} else {
			(
				v[..last_comma].to_string(),
				Some(v[last_comma + 1..].to_string()),
			)
		}
	} else if dot_count > 0 {
		if dot_count > 1 {
			(v.replace('.', ""), None)
		} else {
			let idx = v.rfind('.').unwrap();
			let right_len = v.len() - idx - 1;
			if right_len == 3 {
				(v.replace('.', ""), None)
			} else {
				(v[..idx].to_string(), Some(v[idx + 1..].to_string()))
			}
		}
	} else if comma_count > 0 {
		if comma_count > 1 {
			(v.replace(',', ""), None)
		} else {
			let idx = v.rfind(',').unwrap();
			let right_len = v.len() - idx - 1;
			if right_len == 3 {
				(v.replace(',', ""), None)
			} else {
				(v[..idx].to_string(), Some(v[idx + 1..].to_string()))
			}
		}
	} else {
		(v.clone(), None)
	};

	let re_digits = regex::Regex::new(r"\D").unwrap();
	let int_digits = re_digits.replace_all(&int_part, "").to_string();
	if int_digits.is_empty() {
		return String::new();
	}

	let rev: Vec<char> = int_digits.chars().rev().collect();
	let mut grouped = String::new();
	for (i, c) in rev.iter().enumerate() {
		if i > 0 && i % 3 == 0 {
			grouped.push(',');
		}
		grouped.push(*c);
	}
	let int_fmt: String = grouped.chars().rev().collect();

	if let Some(frac_raw) = frac_part_opt {
		let frac_digits = re_digits.replace_all(&frac_raw, "").to_string();
		if frac_digits.is_empty() {
			return int_fmt;
		}
		return format!("{}.{}", int_fmt, frac_digits);
	}

	int_fmt
}

// Returns a list of stamps for a given series and year
pub fn get_stamps_by_series(
	conn: &rusqlite::Connection,
	series: &str,
	year: &str,
) -> rusqlite::Result<Vec<StampStub>> {
	// Decide which query to run and collect rows into a Vec<StampStub>
	let mut result: Vec<StampStub> = Vec::new();
	if year.trim().is_empty() || year == "0" {
		let mut stmt = conn.prepare("SELECT year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc, image_width_px, image_height_px FROM stamps WHERE series = ?1 ORDER BY year, sg_number")?;
		// Note: we expect image_width_px and image_height_px to be present in the table; if not, the PRAGMA-based migration
		// should have added them. Use named retrieval below when mapping rows.
		let rows = stmt.query_map(
			[series],
			|row| -> std::result::Result<StampStub, rusqlite::Error> {
				Ok(StampStub {
					year: row.get::<_, i32>(0)?,
					month: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
					month_num: row.get::<_, i32>(2)?,
					day: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
					sg_number: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
					r#type: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
					stamp_type: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
					denomination: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
					color: row.get::<_, Option<String>>(7)?.unwrap_or_default(),
					description: row.get::<_, Option<String>>(8)?.unwrap_or_default(),
					desc: row.get::<_, Option<String>>(8)?.unwrap_or_default(),
					catalog_num: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
					stamps_issued: crate::db_utils::format_stamps_issued(
						&row.get::<_, Option<String>>(9)?.unwrap_or_default(),
					),
					image_path: row.get::<_, Option<String>>(10)?.unwrap_or_default(),
					remote_image_url: row.get::<_, Option<String>>(11)?.unwrap_or_default(),
					original_image_url: row.get::<_, Option<String>>(12)?.unwrap_or_default(),
					watermark: row.get::<_, Option<String>>(13)?.unwrap_or_default(),
					design: row.get::<_, Option<String>>(14)?.unwrap_or_default(),
					engraving: row.get::<_, Option<String>>(15)?.unwrap_or_default(),
					perforation: row.get::<_, Option<String>>(16)?.unwrap_or_default(),
					series: row.get::<_, Option<String>>(17)?.unwrap_or_default(),
					currency: row.get::<_, Option<String>>(18)?.unwrap_or_default(),
					context: row.get::<_, Option<String>>(19)?.unwrap_or_default(),
					mint_condition: row.get::<_, Option<String>>(20)?.unwrap_or_default(),
					unused: row.get::<_, Option<String>>(21)?.unwrap_or_default(),
					used: row.get::<_, Option<String>>(22)?.unwrap_or_default(),
					letter_fdc: row.get::<_, Option<String>>(23)?.unwrap_or_default(),
					image_width_px: row.get::<_, Option<i64>>(24)?.map(|v| v as i32),
					image_height_px: row.get::<_, Option<i64>>(25)?.map(|v| v as i32),
					multi_perf: false,
				})
			},
		)?;
		for s in rows {
			result.push(s?);
		}
	} else {
		let mut stmt = conn.prepare("SELECT year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc, image_width_px, image_height_px FROM stamps WHERE series = ?1 AND year = ?2 ORDER BY sg_number")?;
		let rows = stmt.query_map(
			[series, year],
			|row| -> std::result::Result<StampStub, rusqlite::Error> {
				Ok(StampStub {
					year: row.get::<_, i32>(0)?,
					month: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
					month_num: row.get::<_, i32>(2)?,
					day: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
					sg_number: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
					mint_condition: row.get::<_, Option<String>>(20)?.unwrap_or_default(),
					unused: row.get::<_, Option<String>>(21)?.unwrap_or_default(),
					used: row.get::<_, Option<String>>(22)?.unwrap_or_default(),
					letter_fdc: row.get::<_, Option<String>>(23)?.unwrap_or_default(),
					image_width_px: row.get::<_, Option<i64>>(24)?.map(|v| v as i32),
					image_height_px: row.get::<_, Option<i64>>(25)?.map(|v| v as i32),
					multi_perf: false,
				})
			},
		)?;
		for s in rows {
			result.push(s?);
		}
	} else {
		let mut stmt = conn.prepare("SELECT year, month, month_num, day, sg_number, type, denomination, color, description, stamps_issued, image_path, remote_image_url, original_image_url, watermark, design, engraving, perforation, series, currency, context, mint_condition, unused, used, letter_fdc, image_width_px, image_height_px FROM stamps WHERE series = ?1 AND year = ?2 ORDER BY sg_number")?;
		let rows = stmt.query_map(
			[series, year],
			|row| -> std::result::Result<StampStub, rusqlite::Error> {
				Ok(StampStub {
					year: row.get::<_, i32>(0)?,
					month: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
					month_num: row.get::<_, i32>(2)?,
					day: row.get::<_, Option<String>>(3)?.unwrap_or_default(),
					sg_number: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
					r#type: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
					stamp_type: row.get::<_, Option<String>>(5)?.unwrap_or_default(),
					denomination: row.get::<_, Option<String>>(6)?.unwrap_or_default(),
					color: row.get::<_, Option<String>>(7)?.unwrap_or_default(),
					description: row.get::<_, Option<String>>(8)?.unwrap_or_default(),
					desc: row.get::<_, Option<String>>(8)?.unwrap_or_default(),
					catalog_num: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
					stamps_issued: crate::db_utils::format_stamps_issued(
						&row.get::<_, Option<String>>(9)?.unwrap_or_default(),
					),
					image_path: row.get::<_, Option<String>>(10)?.unwrap_or_default(),
					remote_image_url: row.get::<_, Option<String>>(11)?.unwrap_or_default(),
					original_image_url: row.get::<_, Option<String>>(12)?.unwrap_or_default(),
					watermark: row.get::<_, Option<String>>(13)?.unwrap_or_default(),
					design: row.get::<_, Option<String>>(14)?.unwrap_or_default(),
					engraving: row.get::<_, Option<String>>(15)?.unwrap_or_default(),
					perforation: row.get::<_, Option<String>>(16)?.unwrap_or_default(),
					series: row.get::<_, Option<String>>(17)?.unwrap_or_default(),
					currency: row.get::<_, Option<String>>(18)?.unwrap_or_default(),
					context: row.get::<_, Option<String>>(19)?.unwrap_or_default(),
					mint_condition: row.get::<_, Option<String>>(20)?.unwrap_or_default(),
					unused: row.get::<_, Option<String>>(21)?.unwrap_or_default(),
					used: row.get::<_, Option<String>>(22)?.unwrap_or_default(),
					letter_fdc: row.get::<_, Option<String>>(23)?.unwrap_or_default(),
					image_width_px: row.get::<_, Option<i64>>(24)?.map(|v| v as i32),
					image_height_px: row.get::<_, Option<i64>>(25)?.map(|v| v as i32),
					multi_perf: false,
				})
			},
		)?;
		for s in rows {
			result.push(s?);
		}
	}
	// apply sorting to the result vector (module-level helper)
	sort_stampstubs_inplace(&mut result);

	Ok(result)
}

#[derive(Debug, Clone)]
pub struct StampStub {
	// Superset of fields used across modules. Some code refers to alternate names;
	// providing both helps during the merge and keeps compatibility.
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
	pub image_width_px: Option<i32>,
	pub image_height_px: Option<i32>,
	pub watermark: String,
	pub design: String,
	pub engraving: String,
	pub perforation: String,
	pub series: String,
	pub context: String,
	pub multi_perf: bool,
}

use log::{debug, error, info};
use regex::Regex;
use rusqlite::{params, Connection, Result};

// Robust migration helper: recreate `stamps` table with canonical schema while preserving data.
pub fn migrate_stamps_table(conn: &Connection) -> Result<()> {
	debug!("Starting stamps table migration");
	// We'll execute statements inside an explicit BEGIN/COMMIT to avoid needing
	// a mutable transaction object borrowed from `conn`.
	conn.execute_batch("BEGIN IMMEDIATE;")?;

	// Desired final schema columns and types (same as earlier, plus uniq_key)
	// include image_width_px and image_height_px so the canonical schema contains them
	let desired_cols = vec![
		"id INTEGER PRIMARY KEY",
		"country TEXT",
		"series TEXT",
		"year INTEGER",
		"month TEXT",
		"month_num INTEGER",
		"day TEXT",
		"sg_number TEXT",
		"type TEXT",
		"denomination TEXT",
		"color TEXT",
		"description TEXT",
		"stamps_issued TEXT",
		"mint_condition TEXT",
		"unused TEXT",
		"used TEXT",
		"letter_fdc TEXT",
		"currency TEXT",
		"image_path TEXT",
		"remote_image_url TEXT",
		"original_image_url TEXT",
		"image_width_px INTEGER",
		"image_height_px INTEGER",
		"watermark TEXT",
		"design TEXT",
		"engraving TEXT",
		"perforation TEXT",
		"context TEXT",
		"uniq_key TEXT",
	];

	// Also keep a simple list of desired column NAMES for matching existing columns.
	let desired_col_names = vec![
		"id",
		"country",
		"series",
		"year",
		"month",
		"month_num",
		"day",
		"sg_number",
		"type",
		"denomination",
		"color",
		"description",
		"stamps_issued",
		"mint_condition",
		"unused",
		"used",
		"letter_fdc",
		"currency",
		"image_path",
		"remote_image_url",
		"original_image_url",
		"image_width_px",
		"image_height_px",
		"watermark",
		"design",
		"engraving",
		"perforation",
		"context",
		"uniq_key",
	];

	// Create new table
	let create_sql = format!(
		"CREATE TABLE IF NOT EXISTS stamps_new ({})",
		desired_cols.join(",")
	);
	conn.execute(&create_sql, [])?;

	// Get existing columns from old table
	let mut stmt = conn.prepare("PRAGMA table_info(stamps)")?;
	let mut rows = stmt.query([])?;
	let mut existing_cols: Vec<String> = Vec::new();
	while let Ok(Some(r)) = rows.next() {
		let name: String = r.get(1)?;
		existing_cols.push(name);
	}
	debug!("Existing stamps columns: {:?}", existing_cols);
	// Determine which columns we can copy
	// Determine which columns exist in both the old table and our desired schema
	let common_cols: Vec<String> = existing_cols
		.iter()
		.filter(|c| desired_col_names.contains(&c.as_str()))
		.cloned()
		.collect();
	// always include id if present and available
	let copy_cols = if existing_cols.contains(&"id".to_string()) {
		// ensure id is first
		let mut v = vec!["id".to_string()];
		for col in &common_cols {
			if col != "id" {
				v.push(col.clone());
			}
		}
		v
	} else {
		common_cols.clone()
	};
	debug!("Copying columns into stamps_new: {:?}", copy_cols);

	// Determine how many rows exist in the original stamps table before attempting copy.
	// If the original table is empty, it's safe to continue the migration even if
	// the copy produces 0 rows (there was nothing to copy).
	let original_count: i64 = conn.query_row("SELECT COUNT(*) FROM stamps", [], |r| r.get(0))?;

	if !copy_cols.is_empty() {
		let cols_list = copy_cols.join(",");
		let insert_sql = format!(
			"INSERT INTO stamps_new ({0}) SELECT {0} FROM stamps",
			cols_list
		);
		conn.execute(&insert_sql, [])?;
		debug!("Data copied into stamps_new");
	}

	// Safety check: if stamps_new is empty after the copy, abort the migration
	// to avoid accidentally dropping the live `stamps` table and losing data.
	let new_count: i64 = conn.query_row("SELECT COUNT(*) FROM stamps_new", [], |r| r.get(0))?;
	if new_count == 0 && original_count > 0 {
		// If stamps_new ended up empty but the original table actually had rows,
		// abort to avoid data loss. If the original was empty, proceed.
		error!("Migration aborted: stamps_new contains 0 rows after attempted copy. Rolling back to avoid data loss.");
		// Roll back the explicit transaction we started earlier and return an error
		let _ = conn.execute_batch("ROLLBACK;");
		return Err(rusqlite::Error::InvalidQuery);
	}

	// For rows where uniq_key is null/empty, backfill it
	conn.execute("UPDATE stamps_new SET uniq_key = lower(coalesce(country,'')) || '|' || coalesce(series,'') || '|' || coalesce(sg_number,'') WHERE uniq_key IS NULL OR uniq_key = ''", [])?;

	// Check for duplicates on uniq_key
	let dup_groups: i64 = conn.query_row("SELECT COUNT(*) FROM (SELECT uniq_key FROM stamps_new GROUP BY uniq_key HAVING COUNT(*) > 1)", [], |r| r.get(0))?;
	if dup_groups == 0 {
		conn.execute(
			"CREATE UNIQUE INDEX IF NOT EXISTS stamps_unique_uniq_key ON stamps_new(uniq_key)",
			[],
		)?;
		debug!("Created UNIQUE index on stamps_new.uniq_key");
	} else {
		error!(
			"Migration found {} duplicate uniq_key groups. UNIQUE index not created.",
			dup_groups
		);
	}

	// Drop old table and rename
	conn.execute("DROP TABLE IF EXISTS stamps", [])?;
	conn.execute("ALTER TABLE stamps_new RENAME TO stamps", [])?;

	conn.execute_batch("COMMIT;")?;
	debug!("Stamps table migration completed successfully");
	Ok(())
}

/// Migration to fix rows where block metadata (design/engraving/perforation)
/// were stored as concatenated block text. This function heuristically
/// detects suspect rows, parses their `context` / combined text using the
/// collector helper `extract_block_metadata`, normalizes currency tokens and
/// updates only the missing or obviously-wrong columns. It's safe to run
/// multiple times and uses a transaction.
pub fn migrate_fix_block_metadata(conn: &Connection) -> Result<()> {
	debug!("Starting migration: fix block metadata and normalize currency");
	conn.execute_batch("BEGIN IMMEDIATE;")?;

	// Small helper: normalize currency tokens similarly to the collector
	fn normalize_currency_token_inner(s: &str) -> String {
		let t = s.trim();
		if t.is_empty() {
			return String::new();
		}
		let low = t.to_lowercase();
		if low == "sell"
			|| low == "usd"
			|| low == "$"
			|| low == "us$"
			|| low.contains("usd")
			|| low.contains("us$")
		{
			return "USD".to_string();
		}
		if low == "eur" || low.contains("€") {
			return "EUR".to_string();
		}
		if low == "gbp" || low.contains("£") {
			return "GBP".to_string();
		}
		t.to_uppercase()
	}

	// Find candidate rows: those where design/engraving/perforation contain labels
	let mut stmt =
		conn.prepare("SELECT id, design, engraving, perforation, context, currency FROM stamps")?;
	let mut rows = stmt.query([])?;
	let mut updates: Vec<(i64, String, String, String, String, String)> = Vec::new();
	while let Ok(Some(r)) = rows.next() {
		let id: i64 = r.get(0)?;
		let design: String = r.get::<_, Option<String>>(1)?.unwrap_or_default();
		let engraving: String = r.get::<_, Option<String>>(2)?.unwrap_or_default();
		let perforation: String = r.get::<_, Option<String>>(3)?.unwrap_or_default();
		let context: String = r.get::<_, Option<String>>(4)?.unwrap_or_default();
		let currency: String = r.get::<_, Option<String>>(5)?.unwrap_or_default();

		// Heuristic: if any of the design/engraving/perforation fields contain the words
		// 'Design:' or 'Engraving:' or 'Perforation:' or contain newlines, treat as suspect
		let suspect = |s: &str| {
			let low = s.to_lowercase();
			low.contains("design:")
				|| low.contains("engraving:")
				|| low.contains("perforation:")
				|| s.contains('\n')
				|| s.contains('\r')
		};
		if suspect(&design) || suspect(&engraving) || suspect(&perforation) {
			// Build a details vector by splitting context plus the fields to give the parser enough material
			let mut details: Vec<String> = Vec::new();
			if !context.trim().is_empty() {
				details.push(context.clone());
			}
			if !design.trim().is_empty() {
				details.push(design.clone());
			}
			if !engraving.trim().is_empty() {
				details.push(engraving.clone());
			}
			if !perforation.trim().is_empty() {
				details.push(perforation.clone());
			}
			// Call the collector helper (it's pub(crate) so available in the crate)
			let (wm_new, design_new, engr_new, perf_new) =
				crate::stampworld::data_collector::extract_block_metadata(&details);
			let currency_new = normalize_currency_token_inner(&currency);
			updates.push((id, wm_new, design_new, engr_new, perf_new, currency_new));
		}
	}

	// The above push used a temporary tuple; convert and apply updates
	let mut updated_count: i64 = 0;
	for (id, _, _, _, _, _) in updates.iter() {
		// Re-fetch per-row to ensure we operate on the latest values
		let id = *id;
		let row: (String, String, String, String, String) = conn.query_row(
			"SELECT watermark, design, engraving, perforation, currency FROM stamps WHERE id = ?1",
			params![id],
			|r| {
				Ok((
					r.get::<_, Option<String>>(0)?.unwrap_or_default(),
					r.get::<_, Option<String>>(1)?.unwrap_or_default(),
					r.get::<_, Option<String>>(2)?.unwrap_or_default(),
					r.get::<_, Option<String>>(3)?.unwrap_or_default(),
					r.get::<_, Option<String>>(4)?.unwrap_or_default(),
				))
			},
		)?;
		let combined = vec![row.0.clone(), row.1.clone(), row.2.clone(), row.3.clone()];
		let (wm_new, design_new, engr_new, perf_new) =
			crate::stampworld::data_collector::extract_block_metadata(&combined);
		let currency_new = normalize_currency_token_inner(&row.4);

		// Build update statement and params
		let mut set_clauses: Vec<String> = Vec::new();
		let mut values: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();
		if !wm_new.is_empty() {
			set_clauses.push("watermark = ?".to_string());
			values.push(Box::new(wm_new));
		}
		if !design_new.is_empty() {
			set_clauses.push("design = ?".to_string());
			values.push(Box::new(design_new));
		}
		if !engr_new.is_empty() {
			set_clauses.push("engraving = ?".to_string());
			values.push(Box::new(engr_new));
		}
		if !perf_new.is_empty() {
			set_clauses.push("perforation = ?".to_string());
			values.push(Box::new(perf_new));
		}
		if !currency_new.is_empty() {
			set_clauses.push("currency = ?".to_string());
			values.push(Box::new(currency_new));
		}

		if !set_clauses.is_empty() {
			let sql = format!("UPDATE stamps SET {} WHERE id = ?", set_clauses.join(", "));
			values.push(Box::new(id));
			// Create a slice of &dyn ToSql
			let refs: Vec<&dyn rusqlite::ToSql> = values
				.iter()
				.map(|b| &**b as &dyn rusqlite::ToSql)
				.collect();
			if let Ok(rows) = conn.execute(&sql, rusqlite::params_from_iter(refs)) {
				updated_count += rows as i64;
			}
		}
	}

	conn.execute_batch("COMMIT;")?;
	info!(
		"Migration fixed block metadata; updated {} row(s)",
		updated_count
	);
	debug!("Completed migration: fix block metadata and normalize currency");
	Ok(())
}

pub fn init_db(db_path: &str) -> Result<Connection> {
	// Ensure parent directory for the database exists so SQLite can create the file
	if let Some(parent) = std::path::Path::new(db_path).parent() {
		if !parent.as_os_str().is_empty() {
			if let Err(e) = std::fs::create_dir_all(parent) {
				error!(
					"Failed to create parent directory for DB {}: {}",
					parent.display(),
					e
				);
				// Continue and attempt to open the DB - the original behaviour was to fail here;
				// returning the error from Connection::open will surface the problem to callers.
			}
		}
	}
	let conn = Connection::open(db_path)?;
	// Log the path opened and some filesystem metadata to help debug DB path mismatches
	let canonical =
		std::fs::canonicalize(db_path).unwrap_or_else(|_| std::path::PathBuf::from(db_path));
	debug!(
		"Opened database at {} (canonical: {})",
		db_path,
		canonical.display()
	);
	match std::fs::metadata(&canonical) {
		Ok(md) => debug!(
			"Database file exists. size={} bytes, readonly={}",
			md.len(),
			md.permissions().readonly()
		),
		Err(_) => debug!("Database file does not exist yet: {}", canonical.display()),
	}
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
			image_width_px INTEGER,
			image_height_px INTEGER,
			watermark TEXT,
			design TEXT,
			engraving TEXT,
			perforation TEXT,
			context TEXT
		)",
		[],
	)?;
