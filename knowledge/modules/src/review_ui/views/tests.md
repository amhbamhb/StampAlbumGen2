---
type: Rust Module
title: tests
resource: src/review_ui/views/tests.rs#L1-L126
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-render-bootstrap-html-render-html
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-review-ui-matcherpayload
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [minimal_payload](../../../../functions/src/review_ui/views/tests/minimal_payload.md)
- [bootstrap_html_uses_default_inbox_hint_when_missing](../../../../functions/src/review_ui/views/tests/bootstrap_html_uses_default_inbox_hint_when_missing.md)
- [bootstrap_html_uses_custom_inbox_hint_when_present](../../../../functions/src/review_ui/views/tests/bootstrap_html_uses_custom_inbox_hint_when_present.md)
- [bootstrap_html_contains_health_and_runtime_log_hooks](../../../../functions/src/review_ui/views/tests/bootstrap_html_contains_health_and_runtime_log_hooks.md)
- [bootstrap_html_exposes_edit_config_action](../../../../functions/src/review_ui/views/tests/bootstrap_html_exposes_edit_config_action.md)
- [compare_html_embeds_payload_and_core_shell](../../../../functions/src/review_ui/views/tests/compare_html_embeds_payload_and_core_shell.md)
- [compare_html_contains_backend_recovery_banner](../../../../functions/src/review_ui/views/tests/compare_html_contains_backend_recovery_banner.md)
- [compare_html_contains_runtime_log_hooks](../../../../functions/src/review_ui/views/tests/compare_html_contains_runtime_log_hooks.md)
- [compare_html_has_country_scoped_pdf_filters_only_on_review_screen](../../../../functions/src/review_ui/views/tests/compare_html_has_country_scoped_pdf_filters_only_on_review_screen.md)
- [both_pull_screens_embed_saved_country_url_controls](../../../../functions/src/review_ui/views/tests/both_pull_screens_embed_saved_country_url_controls.md)
- [both_pull_screens_explain_start_failures_and_offer_site_accounts](../../../../functions/src/review_ui/views/tests/both_pull_screens_explain_start_failures_and_offer_site_accounts.md)

# Imports

- `super::{render_bootstrap_html, render_html}`
- `crate::review_ui::MatcherPayload`

# Member of

- [stamp_album_gen2](../../../../packages/stamp_album_gen2.md)