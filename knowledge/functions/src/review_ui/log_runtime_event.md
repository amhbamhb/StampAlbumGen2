---
type: Rust Function
title: log_runtime_event
resource: src/review_ui.rs#L70-L79
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/sanitize_credentials_in_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/runtime_log_buffer
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/open_review_ui_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_startup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/create_bootstrap_country_or_theme
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/colnect_import_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/run_review_ui_bootstrap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/run_review_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/start_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/store_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/handle
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/respond
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/review_pdf_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_site_account_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_country_url_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn log_runtime_event(message: &str)`

# Calls

- [sanitize_credentials_in_text](../../../functions/src/review_ui/sanitize_credentials_in_text.md)
- [runtime_log_buffer](../../../functions/src/review_ui/runtime_log_buffer.md)

# Called by

- [main](../../../functions/src/main.md)
- [start_bootstrap_stampworld_job](../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [open_review_ui_url](../../../functions/src/review_ui/open_review_ui_url.md)
- [log_startup](../../../functions/src/review_ui/log_startup.md)
- [scan_bootstrap_countries](../../../functions/src/review_ui/scan_bootstrap_countries.md)
- [create_bootstrap_country_or_theme](../../../functions/src/review_ui/create_bootstrap_country_or_theme.md)
- [colnect_import_data](../../../functions/src/review_ui/colnect_import_data.md)
- [run_review_ui_bootstrap](../../../functions/src/review_ui/run_review_ui_bootstrap.md)
- [run_review_ui](../../../functions/src/review_ui/run_review_ui.md)
- [start_job](../../../functions/src/review_ui/external_album_api/start_job.md)
- [store_image](../../../functions/src/review_ui/external_album_api/store_image.md)
- [handle](../../../functions/src/review_ui/external_album_api/handle.md)
- [respond](../../../functions/src/review_ui/external_album_api/respond.md)
- [review_pdf_error](../../../functions/src/review_ui/http_handlers/review_pdf_error.md)
- [handle_site_account_request](../../../functions/src/review_ui/http_handlers/handle_site_account_request.md)
- [handle_country_url_request](../../../functions/src/review_ui/http_handlers/handle_country_url_request.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)