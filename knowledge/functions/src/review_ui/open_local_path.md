---
type: Rust Function
title: open_local_path
resource: src/review_ui.rs#L1515-L1531
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/path_to_file_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn open_local_path(path: &Path) -> bool`

# Calls

- [path_to_file_url](../../../functions/src/review_ui/path_to_file_url.md)
- [open](../../../functions/src/external_album/open.md)