---
type: Rust Function
title: minimal_payload
resource: src/review_ui/views/tests.rs#L4-L34
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/views/tests/compare_html_embeds_payload_and_core_shell
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/tests/compare_html_contains_backend_recovery_banner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/tests/compare_html_contains_runtime_log_hooks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/tests/compare_html_has_country_scoped_pdf_filters_only_on_review_screen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/tests/both_pull_screens_embed_saved_country_url_controls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/tests/both_pull_screens_explain_start_failures_and_offer_site_accounts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn minimal_payload() -> MatcherPayload`

# Called by

- [compare_html_embeds_payload_and_core_shell](../../../../../functions/src/review_ui/views/tests/compare_html_embeds_payload_and_core_shell.md)
- [compare_html_contains_backend_recovery_banner](../../../../../functions/src/review_ui/views/tests/compare_html_contains_backend_recovery_banner.md)
- [compare_html_contains_runtime_log_hooks](../../../../../functions/src/review_ui/views/tests/compare_html_contains_runtime_log_hooks.md)
- [compare_html_has_country_scoped_pdf_filters_only_on_review_screen](../../../../../functions/src/review_ui/views/tests/compare_html_has_country_scoped_pdf_filters_only_on_review_screen.md)
- [both_pull_screens_embed_saved_country_url_controls](../../../../../functions/src/review_ui/views/tests/both_pull_screens_embed_saved_country_url_controls.md)
- [both_pull_screens_explain_start_failures_and_offer_site_accounts](../../../../../functions/src/review_ui/views/tests/both_pull_screens_explain_start_failures_and_offer_site_accounts.md)