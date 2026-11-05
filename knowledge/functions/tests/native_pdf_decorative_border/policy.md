---
type: Rust Function
title: policy
resource: tests/native_pdf_decorative_border.rs#L66-L71
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/native_pdf_decorative_border/mirrored_corner_bounds_and_fixed_slot_line_spacing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/strict_unsupported_invalid_and_redacted_definition_errors
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/missing_corrupt_and_non_alpha_png_assets_fail_before_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn policy() -> Result<DecorativeBorderPolicy>`

# Called by

- [mirrored_corner_bounds_and_fixed_slot_line_spacing](../../../functions/tests/native_pdf_decorative_border/mirrored_corner_bounds_and_fixed_slot_line_spacing.md)
- [strict_unsupported_invalid_and_redacted_definition_errors](../../../functions/tests/native_pdf_decorative_border/strict_unsupported_invalid_and_redacted_definition_errors.md)
- [missing_corrupt_and_non_alpha_png_assets_fail_before_output](../../../functions/tests/native_pdf_decorative_border/missing_corrupt_and_non_alpha_png_assets_fail_before_output.md)