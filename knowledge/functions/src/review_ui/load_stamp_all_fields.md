---
type: Rust Function
title: load_stamp_all_fields
resource: src/review_ui.rs#L2543-L2574
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/column_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/value_ref_to_string
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/matcher_service/load_left_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_left_stamp_by_rowid
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_stamp_all_fields( conn: &Connection, rowid: i64, ) -> Result<Vec<DbField>, Box<dyn std::error::Error>>`

# Calls

- [column_names](../../../functions/src/db_utils/migrations/column_names.md)
- [value_ref_to_string](../../../functions/src/review_ui/value_ref_to_string.md)

# Called by

- [load_left_stamps](../../../functions/src/review_ui/matcher_service/load_left_stamps.md)
- [load_left_stamp_by_rowid](../../../functions/src/review_ui/matcher_service/load_left_stamp_by_rowid.md)