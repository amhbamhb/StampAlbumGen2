---
type: Rust Function
title: review_pdf_filters_routes_validate_requests_and_report_empty_selection
resource: src/review_ui/http_handlers.rs#L2747-L2815
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/http_handlers/review_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/body
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn review_pdf_filters_routes_validate_requests_and_report_empty_selection()`

# Calls

- [review_input](../../../../functions/src/review_ui/http_handlers/review_input.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [json](../../../../functions/src/external_album/json.md)
- [body](../../../../functions/src/review_ui/external_album_api/body.md)