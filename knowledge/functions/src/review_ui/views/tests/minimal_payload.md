---
type: Rust Function
title: minimal_payload
resource: src/review_ui/views/tests.rs#L4-L32
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
---

# Signature

`fn minimal_payload() -> MatcherPayload`

# Called by

- [compare_html_embeds_payload_and_core_shell](../../../../../functions/src/review_ui/views/tests/compare_html_embeds_payload_and_core_shell.md)
- [compare_html_contains_backend_recovery_banner](../../../../../functions/src/review_ui/views/tests/compare_html_contains_backend_recovery_banner.md)
- [compare_html_contains_runtime_log_hooks](../../../../../functions/src/review_ui/views/tests/compare_html_contains_runtime_log_hooks.md)