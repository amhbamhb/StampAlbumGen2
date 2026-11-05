---
type: Rust Function
title: mime_from_path
resource: src/review_ui.rs#L1600-L1613
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn mime_from_path(path: &Path) -> &'static str`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)