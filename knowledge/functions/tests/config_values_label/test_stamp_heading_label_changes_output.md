---
type: Rust Function
title: test_stamp_heading_label_changes_output
resource: tests/config_values_label.rs#L40-L99
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/set_overwrite_album_from_cli
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_values_label/read_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_stamp_heading_label_changes_output()`

# Calls

- [set_overwrite_album_from_cli](../../../functions/src/file_utils/set_overwrite_album_from_cli.md)
- [read_stamp_heading_label](../../../functions/tests/config_values_label/read_stamp_heading_label.md)