---
type: Rust Function
title: preferred_catalog_number
resource: src/catalog.rs#L146-L149
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/catalog/preferred_catalog_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_number
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/colnect/ingest/ingest_colnect_data_with_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn preferred_catalog_number(raw: &str, default_catalog_code: &str) -> Option<String>`

# Calls

- [preferred_catalog_entry](../../../functions/src/catalog/preferred_catalog_entry.md)
- [normalize_catalog_number](../../../functions/src/catalog/normalize_catalog_number.md)

# Called by

- [ingest_colnect_data_with_mode](../../../functions/src/colnect/ingest/ingest_colnect_data_with_mode.md)