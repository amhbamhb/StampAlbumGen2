---
type: Rust Function
title: infer_and_fix_catalog_codes_for_colnect_rows
resource: src/modes.rs#L67-L105
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/catalog/infer_catalog_type_for_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/sync_missing_stamps_from_colnect
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn infer_and_fix_catalog_codes_for_colnect_rows( conn: &rusqlite::Connection, fallback_catalog_code: &str, ) -> rusqlite::Result<usize>`

# Calls

- [infer_catalog_type_for_number](../../../functions/src/catalog/infer_catalog_type_for_number.md)
- [normalize_catalog_type](../../../functions/src/catalog/normalize_catalog_type.md)

# Called by

- [sync_missing_stamps_from_colnect](../../../functions/src/modes/sync_missing_stamps_from_colnect.md)