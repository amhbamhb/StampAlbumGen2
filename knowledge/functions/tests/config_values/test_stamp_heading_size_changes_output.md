---
type: Rust Function
title: test_stamp_heading_size_changes_output
resource: tests/config_values.rs#L46-L110
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/set_overwrite_album_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values/read_stamp_heading_size
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_stamp_heading_size_changes_output()`

# Calls

- [set_overwrite_album_from_cli](../../../functions/src/file_utils/set_overwrite_album_from_cli.md)
- [read_stamp_heading_size](../../../functions/tests/config_values/read_stamp_heading_size.md)