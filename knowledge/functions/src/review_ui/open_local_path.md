---
type: Rust Function
title: open_local_path
resource: src/review_ui.rs#L1504-L1520
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/path_to_file_url
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn open_local_path(path: &Path) -> bool`

# Calls

- [path_to_file_url](../../../functions/src/review_ui/path_to_file_url.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)