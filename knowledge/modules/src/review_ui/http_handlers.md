---
type: Rust Module
title: http_handlers
resource: src/review_ui/http_handlers.rs#L1-L2932
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-review-ui-db-query-best-effort-prepare-images-for-pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-review-ui-pdf-generation-review-pdf-error
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-review-ui-ui-state-bootstraphandlersstate-reviewhandlersstate
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [bundled_favicon_png](../../../functions/src/review_ui/http_handlers/bundled_favicon_png.md)
- [ping_response_json](../../../functions/src/review_ui/http_handlers/ping_response_json.md)
- [handle_stampworld_import_request](../../../functions/src/review_ui/http_handlers/handle_stampworld_import_request.md)
- [handle_site_account_request](../../../functions/src/review_ui/http_handlers/handle_site_account_request.md)
- [handle_country_url_request](../../../functions/src/review_ui/http_handlers/handle_country_url_request.md)
- [SaveRequest](../../../classes/src/review_ui/http_handlers/SaveRequest.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [ReviewHandlersStateRequests](../../../classes/src/review_ui/http_handlers/ReviewHandlersStateRequests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [review_input](../../../functions/src/review_ui/http_handlers/review_input.md)
- [stampworld_file_import_route_reports_invalid_requests_in_both_loops](../../../functions/src/review_ui/http_handlers/stampworld_file_import_route_reports_invalid_requests_in_both_loops.md)
- [review_pdf_filters_routes_validate_requests_and_report_empty_selection](../../../functions/src/review_ui/http_handlers/review_pdf_filters_routes_validate_requests_and_report_empty_selection.md)
- [country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings](../../../functions/src/review_ui/http_handlers/country_url_routes_reject_bad_input_on_both_pull_screens_without_writing_mappings.md)
- [site_accounts_routes_return_json_in_bootstrap_and_review_modes](../../../functions/src/review_ui/http_handlers/site_accounts_routes_return_json_in_bootstrap_and_review_modes.md)

# Imports

- `super::*`
- `crate::review_ui::db_query::best_effort_prepare_images_for_pdf`
- `crate::review_ui::pdf_generation::review_pdf_error`
- `crate::review_ui::ui_state::{BootstrapHandlersState, ReviewHandlersState}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)