---
type: Rust Function
title: make_test_db
resource: tests/config_values.rs#L13-L24
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/operations/insert_stamp_placeholder
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_test_db(db_path: &str)`

# Calls

- [insert_stamp_placeholder](../../../functions/src/db_utils/operations/insert_stamp_placeholder.md)