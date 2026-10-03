---
type: Rust Function
title: review_input
resource: src/review_ui/http_handlers.rs#L221-L250
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/http_handlers/review_pdf_filters_routes_validate_requests_and_report_empty_selection
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
- [site_accounts_routes_return_json_in_bootstrap_and_review_modes](../../../../functions/src/review_ui/http_handlers/site_accounts_routes_return_json_in_bootstrap_and_review_modes.md)