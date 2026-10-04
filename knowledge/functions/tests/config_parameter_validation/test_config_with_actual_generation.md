---
type: Rust Function
title: test_config_with_actual_generation
resource: tests/config_parameter_validation.rs#L433-L510
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/set_overwrite_album_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_parameter_validation/make_minimal_test_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_config_with_actual_generation()`

# Calls

- [set_overwrite_album_from_cli](../../../functions/src/file_utils/set_overwrite_album_from_cli.md)
- [make_minimal_test_db](../../../functions/tests/config_parameter_validation/make_minimal_test_db.md)