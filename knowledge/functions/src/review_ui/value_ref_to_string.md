---
type: Rust Function
title: value_ref_to_string
resource: src/review_ui.rs#L2533-L2541
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/load_stamp_all_fields
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn value_ref_to_string(v: rusqlite::types::ValueRef<'_>) -> String`

# Called by

- [load_stamp_all_fields](../../../functions/src/review_ui/load_stamp_all_fields.md)