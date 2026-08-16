---
type: Rust Function
title: ingest_colnect_data_with_default_catalog
resource: src/colnect/ingest.rs#L10-L16
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_mode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/colnect/ingest/ingest_colnect_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ingest_colnect_data_with_default_catalog( conn: &Connection, stamps: &[ColnectStamp], default_catalog_code: &str, ) -> Result<()>`

# Calls

- [ingest_colnect_data_with_mode](../../../../functions/src/colnect/ingest/ingest_colnect_data_with_mode.md)

# Called by

- [ingest_colnect_data](../../../../functions/src/colnect/ingest/ingest_colnect_data.md)
- [run_database_mode](../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)