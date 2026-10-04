---
type: Rust Function
title: open_folder_in_file_manager
resource: src/review_ui.rs#L2594-L2632
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn open_folder_in_file_manager(path: &str) -> bool`

# Called by

- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)