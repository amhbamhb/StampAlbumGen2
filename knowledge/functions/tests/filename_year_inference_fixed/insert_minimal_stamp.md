---
type: Rust Function
title: insert_minimal_stamp
resource: tests/filename_year_inference_fixed.rs#L9-L24
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/filename_year_inference_fixed/filename_infers_db_min_max_when_none_fixed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/filename_year_inference_fixed/filename_uses_passed_start_and_db_max_when_end_missing_fixed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/filename_year_inference_fixed/filename_uses_db_min_and_passed_end_when_start_missing_fixed
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn insert_minimal_stamp(conn: &rusqlite::Connection, year: i32, sg_number: &str)`

# Called by

- [filename_infers_db_min_max_when_none_fixed](../../../functions/tests/filename_year_inference_fixed/filename_infers_db_min_max_when_none_fixed.md)
- [filename_uses_passed_start_and_db_max_when_end_missing_fixed](../../../functions/tests/filename_year_inference_fixed/filename_uses_passed_start_and_db_max_when_end_missing_fixed.md)
- [filename_uses_db_min_and_passed_end_when_start_missing_fixed](../../../functions/tests/filename_year_inference_fixed/filename_uses_db_min_and_passed_end_when_start_missing_fixed.md)