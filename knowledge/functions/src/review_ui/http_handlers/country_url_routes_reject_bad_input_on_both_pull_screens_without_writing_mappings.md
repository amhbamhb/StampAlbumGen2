---
type: Rust Function
title: country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings
resource: src/review_ui/http_handlers.rs#L2818-L2862
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/review_input
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/body
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings()`

# Calls

- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [review_input](../../../../functions/src/review_ui/http_handlers/review_input.md)
- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [body](../../../../functions/src/review_ui/external_album_api/body.md)
- [json](../../../../functions/src/external_album/json.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)