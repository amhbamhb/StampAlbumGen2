---
type: Rust Function
title: ingest_colnect_data_with_mode
resource: src/colnect/ingest.rs#L18-L166
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/catalog/preferred_catalog_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/ingest/extract_colnect_code
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_default_catalog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ingest_colnect_data_with_mode( conn: &Connection, stamps: &[ColnectStamp], default_catalog_code: &str, clear_existing: bool, ) -> Result<()>`

# Calls

- [preferred_catalog_number](../../../../functions/src/catalog/preferred_catalog_number.md)
- [extract_colnect_code](../../../../functions/src/colnect/ingest/extract_colnect_code.md)

# Called by

- [ingest_colnect_data_with_default_catalog](../../../../functions/src/colnect/ingest/ingest_colnect_data_with_default_catalog.md)
- [run_colnect_inbox_mode](../../../../functions/src/modes/run_colnect_inbox_mode.md)
- [colnect_import_data](../../../../functions/src/review_ui/colnect_import_data.md)