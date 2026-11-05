---
type: Rust Function
title: missing_corrupt_and_non_alpha_png_assets_fail_before_output
resource: tests/native_pdf_decorative_border.rs#L432-L460
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn missing_corrupt_and_non_alpha_png_assets_fail_before_output() -> Result<()>`

# Calls

- [save](../../../functions/src/review_ui/views/external_album/save.md)
- [policy](../../../functions/tests/native_pdf_decorative_border/policy.md)
- [err](../../../functions/src/external_album/err.md)