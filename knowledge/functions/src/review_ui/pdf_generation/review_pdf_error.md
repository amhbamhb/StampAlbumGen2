---
type: Rust Function
title: review_pdf_error
resource: src/review_ui/pdf_generation.rs#L15-L29
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn review_pdf_error(message: &str, status: u16) -> Response<std::io::Cursor<Vec<u8>>>`

# Calls

- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)