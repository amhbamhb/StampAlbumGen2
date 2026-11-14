use super::{Stamp, generator_new::helpers::fetch_and_filter_stamps_for_selection};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

/// PDF-only metadata selection. Empty lists mean All; an empty value means unknown.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ColnectPdfFilters {
    #[serde(default)]
    pub emission: Vec<String>,
    #[serde(default)]
    pub format: Vec<String>,
}

fn normalized(value: &str) -> String {
    value.trim().to_lowercase()
}

impl ColnectPdfFilters {
    /// Reject excessively large selections before loading country data.
    pub fn validate(&self) -> Result<(), String> {
        for values in [&self.emission, &self.format] {
            if values.len() > 100 || values.iter().any(|value| value.len() > 512) {
                return Err(
                    "PDF filters support at most 100 values per field, each at most 512 bytes."
                        .into(),
                );
            }
        }
        Ok(())
    }

    /// Whether both fields include all records.
    pub fn is_all(&self) -> bool {
        self.emission.is_empty() && self.format.is_empty()
    }

    /// Match OR within a field and AND across fields, independent of display toggles.
    pub fn matches(&self, stamp: &Stamp) -> bool {
        [
            (&self.emission, "colnect_emission"),
            (&self.format, "colnect_format"),
        ]
        .iter()
        .all(|(values, key)| {
            let actual = normalized(stamp.attrs.get(key).map(String::as_str).unwrap_or(""));
            values.is_empty() || values.iter().any(|value| normalized(value) == actual)
        })
    }
}

/// Dynamic values resolved from records eligible for this country's album output.
#[derive(Debug, Serialize)]
pub struct ColnectPdfOptions {
    pub emission: Vec<String>,
    pub format: Vec<String>,
}

pub(crate) type PreparedPdfStamps = HashMap<String, Vec<Stamp>>;

fn load_stamps(
    conn: &Connection,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> rusqlite::Result<PreparedPdfStamps> {
    let mut prepared = HashMap::new();
    for series in crate::db_utils::get_all_series(conn, None)? {
        let stamps = fetch_and_filter_stamps_for_selection(conn, &series, start_year, end_year)?;
        prepared.insert(series, stamps);
    }
    Ok(prepared)
}

/// Look up current-country choices using the same metadata precedence as generation.
pub fn colnect_pdf_options(conn: &Connection) -> rusqlite::Result<ColnectPdfOptions> {
    let prepared = load_stamps(conn, None, None)?;
    let collect = |key: &str| {
        let mut values = BTreeMap::new();
        for stamp in prepared.values().flatten() {
            let value = stamp.attrs.get(key).map(|v| v.trim()).unwrap_or("");
            let entry = values
                .entry(normalized(value))
                .or_insert_with(|| value.to_string());
            // Stable display spelling even when source records differ only in case.
            if value < entry.as_str() {
                *entry = value.to_string();
            }
        }
        values.into_values().collect()
    };
    Ok(ColnectPdfOptions {
        emission: collect("colnect_emission"),
        format: collect("colnect_format"),
    })
}

pub(crate) fn prepare_pdf_stamps(
    conn: &Connection,
    filters: &ColnectPdfFilters,
    start_year: Option<i32>,
    end_year: Option<i32>,
) -> Result<PreparedPdfStamps, String> {
    filters.validate()?;
    let mut prepared = load_stamps(conn, start_year, end_year)
        .map_err(|error| format!("Failed to load PDF selection: {error}"))?;
    for stamps in prepared.values_mut() {
        stamps.retain(|stamp| filters.matches(stamp));
    }
    if prepared.values().all(Vec::is_empty) {
        return Err(
            "No stamps match these PDF filters and years. Previous output was not changed.".into(),
        );
    }
    Ok(prepared)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_utils::init_db;

    fn fixture() -> Connection {
        let conn = init_db(":memory:").expect("selection database");
        for (number, year, emission, format, series) in [
            ("1001", 1975, "Official", "Stamp", "Regular"),
            ("1002", 1976, "Definitive", "Stamp", "Regular"),
            ("1003", 1976, "Official", "Miniature Sheet", "Sheets"),
            ("1004", 1976, "", "", "Unknown"),
            ("1005", 1977, "", "", "Fallback"),
            ("1006", 1976, " official ", " stamp ", "Regular"),
        ] {
            conn.execute(
                "INSERT INTO stamps (country, series, year, sg_number, type, image_path,
                 colnect_emission, colnect_format, uniq_key)
                 VALUES ('Testland', ?1, ?2, ?3, 'Postage', '', ?4, ?5, ?3)",
                rusqlite::params![series, year, number, emission, format],
            )
            .expect("fixture stamp");
        }
        conn.execute(
            "INSERT INTO colnect_stamps (colnect_id, sg_number, emission, format)
             VALUES ('ref-1005', '1005', 'Revenue', 'Stamp')",
            [],
        )
        .expect("reference metadata");
        conn.execute(
            "INSERT INTO colnect_stamps (colnect_id, sg_number, emission, format)
             VALUES ('ref-1002', '1002', 'Official', 'Miniature Sheet')",
            [],
        )
        .expect("conflicting reference");
        conn.execute(
            "INSERT INTO colnect_stamps (colnect_id, sg_number, emission, format)
             VALUES ('candidate-only', '9000', 'Candidate Only', 'Booklet')",
            [],
        )
        .expect("unmatched candidate");
        conn
    }

    fn count(prepared: &PreparedPdfStamps) -> usize {
        prepared.values().map(Vec::len).sum()
    }

    #[test]
    fn dynamic_options_use_resolved_album_records_and_keep_countries_separate() {
        let conn = fixture();
        let options = colnect_pdf_options(&conn).expect("options");
        assert_eq!(options.emission, ["", "Definitive", "Official", "Revenue"]);
        assert_eq!(options.format, ["", "Miniature Sheet", "Stamp"]);
        let other_country = init_db(":memory:").expect("another country's database");
        assert!(
            colnect_pdf_options(&other_country)
                .expect("empty country")
                .emission
                .is_empty()
        );
        conn.execute(
            "UPDATE stamps SET colnect_emission = 'New Category' WHERE sg_number = '1001'",
            [],
        )
        .expect("new category");
        assert!(
            colnect_pdf_options(&conn)
                .expect("updated options")
                .emission
                .contains(&"New Category".into())
        );
    }

    #[test]
    fn selection_composes_or_and_years_and_preserves_metadata_precedence() {
        let conn = fixture();
        let all =
            prepare_pdf_stamps(&conn, &ColnectPdfFilters::default(), None, None).expect("All");
        assert_eq!(count(&all), 6);
        let filters = ColnectPdfFilters {
            emission: vec!["Official".into(), "Definitive".into()],
            format: vec!["Stamp".into()],
        };
        let selected =
            prepare_pdf_stamps(&conn, &filters, Some(1976), Some(1976)).expect("selected");
        assert_eq!(count(&selected), 2);
        assert!(selected["Sheets"].is_empty());
        assert!(selected["Regular"].iter().any(|stamp| {
            stamp
                .attrs
                .get("sg_number")
                .is_some_and(|value| value == "1002")
        }));
        let fallback = ColnectPdfFilters {
            emission: vec!["revenue".into()],
            format: vec![],
        };
        assert_eq!(
            count(&prepare_pdf_stamps(&conn, &fallback, Some(1977), None).expect("fallback")),
            1
        );
        let unknown = ColnectPdfFilters {
            emission: vec!["".into()],
            format: vec!["".into()],
        };
        assert_eq!(
            count(&prepare_pdf_stamps(&conn, &unknown, None, None).expect("unknown")),
            1
        );
    }

    #[test]
    fn selection_reports_empty_results_and_database_errors() {
        let conn = fixture();
        let filters = ColnectPdfFilters {
            emission: vec!["Not present".into()],
            format: vec![],
        };
        assert!(
            prepare_pdf_stamps(&conn, &filters, None, None)
                .expect_err("empty")
                .contains("No stamps match")
        );
        conn.execute("DROP TABLE colnect_catalog_codes", [])
            .expect("broken reference schema");
        assert!(colnect_pdf_options(&conn).is_err());
        assert!(
            prepare_pdf_stamps(&conn, &filters, None, None)
                .expect_err("broken database")
                .contains("Failed to load PDF selection")
        );
    }

    #[test]
    fn selection_allows_country_without_reference_tables() {
        let conn = fixture();
        conn.execute("DROP TABLE colnect_stamps", [])
            .expect("legacy country");
        let options = colnect_pdf_options(&conn).expect("stored metadata only");
        assert!(options.emission.contains(&"Official".into()));
        assert!(!options.emission.contains(&"Revenue".into()));
    }
}
