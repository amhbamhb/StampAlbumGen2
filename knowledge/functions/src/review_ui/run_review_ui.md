---
type: Rust Function
title: run_review_ui
resource: src/review_ui.rs#L3045-L3286
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/is_imported
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/sync_processed_colnect_candidates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/build_matcher_payload
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/bind_review_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/already_running_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/open_review_ui_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn run_review_ui( db_path: &str, album_name_opt: Option<&str>, prebound_server: Option<(Server, String)>, bind_addr_opt: Option<&str>, open_browser: bool, opts: &MatcherPayloadOptions<'_>, ) -> Result<ReviewUiRunOutcome, Box<dyn std::error::Error>>`

# Calls

- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)
- [is_imported](../../../functions/src/external_album/is_imported.md)
- [sync_processed_colnect_candidates](../../../functions/src/review_ui/matcher_service/sync_processed_colnect_candidates.md)
- [build_matcher_payload](../../../functions/src/review_ui/build_matcher_payload.md)
- [bind_review_server](../../../functions/src/review_ui/bind_review_server.md)
- [already_running_url](../../../functions/src/review_ui/already_running_url.md)
- [open_review_ui_url](../../../functions/src/review_ui/open_review_ui_url.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)

# Called by

- [main](../../../functions/src/main.md)