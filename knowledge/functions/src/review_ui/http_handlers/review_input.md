---
type: Rust Function
title: review_input
resource: src/review_ui/http_handlers.rs#L277-L306
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/http_handlers/review_pdf_filters_routes_validate_requests_and_report_empty_selection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/site_accounts_routes_return_json_in_bootstrap_and_review_modes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn review_input() -> ReviewHandlersStateRequests`

# Called by

- [review_pdf_filters_routes_validate_requests_and_report_empty_selection](../../../../functions/src/review_ui/http_handlers/review_pdf_filters_routes_validate_requests_and_report_empty_selection.md)
- [country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings](../../../../functions/src/review_ui/http_handlers/country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings.md)
- [site_accounts_routes_return_json_in_bootstrap_and_review_modes](../../../../functions/src/review_ui/http_handlers/site_accounts_routes_return_json_in_bootstrap_and_review_modes.md)