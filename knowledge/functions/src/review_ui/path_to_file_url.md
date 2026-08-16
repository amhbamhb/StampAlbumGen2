---
type: Rust Function
title: path_to_file_url
resource: src/review_ui.rs#L1245-L1278
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/open_local_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn path_to_file_url(path: &Path) -> Option<String>`

# Called by

- [open_local_path](../../../functions/src/review_ui/open_local_path.md)