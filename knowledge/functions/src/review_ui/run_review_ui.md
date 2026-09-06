---
type: Rust Function
title: run_review_ui
resource: src/review_ui.rs#L2689-L2907
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/log_runtime_event
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

`pub fn run_review_ui( db_path: &str, album_name_opt: Option<&str>, prebound_server: Option<(Server, String)>, bind_addr_opt: Option<&str>, open_browser: bool, default_catalog_code: &str, default_sort_mode: &str, odd_row_color: &str, even_row_color: &str, first_column_color: &str, second_column_color: &str, series_default_checked: bool, description_default_checked: bool, watermark_default_checked: bool, design_default_checked: bool, engraving_default_checked: bool, perforation_default_checked: bool, context_default_checked: bool, colnect_links_default_checked: bool, colnect_image_url_default_checked: bool, colnect_metadata_default_checked: bool, replace_sg_number_default_checked: bool, right_unmatched_only_default_checked: bool, albumeasy_auto_generate_pdf_default: bool, albumeasy_cli_configured: bool, catalog_family_options: &[String], ) -> Result<ReviewUiRunOutcome, Box<dyn std::error::Error>>`

# Calls

- [log_runtime_event](../../../functions/src/review_ui/log_runtime_event.md)
- [sync_processed_colnect_candidates](../../../functions/src/review_ui/matcher_service/sync_processed_colnect_candidates.md)
- [build_matcher_payload](../../../functions/src/review_ui/build_matcher_payload.md)
- [bind_review_server](../../../functions/src/review_ui/bind_review_server.md)
- [already_running_url](../../../functions/src/review_ui/already_running_url.md)
- [open_review_ui_url](../../../functions/src/review_ui/open_review_ui_url.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)

# Called by

- [main](../../../functions/src/main.md)