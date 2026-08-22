---
type: Rust Function
title: config_values_simple_no_placeholder
resource: tests/config_values_simple.rs#L77-L159
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/config_values_simple/make_test_db_simple
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/set_overwrite_album_from_cli
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn config_values_simple_no_placeholder()`

# Calls

- [make_test_db_simple](../../../functions/tests/config_values_simple/make_test_db_simple.md)
- [set_overwrite_album_from_cli](../../../functions/src/file_utils/set_overwrite_album_from_cli.md)