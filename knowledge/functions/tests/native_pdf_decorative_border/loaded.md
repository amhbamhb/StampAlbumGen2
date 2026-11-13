---
type: Rust Function
title: loaded
resource: tests/native_pdf_decorative_border.rs#L75-L86
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml_in
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/native_pdf_decorative_border/edge_only_and_mixed_layouts_match_margin_and_padding_semantics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/explicit_policy_validation_and_definition_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/actual_decorative_pixels_and_transparency_match
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn loaded(name: &str) -> Result<NativePdfConfig>`

# Calls

- [from_toml_in](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml_in.md)

# Called by

- [edge_only_and_mixed_layouts_match_margin_and_padding_semantics](../../../functions/tests/native_pdf_decorative_border/edge_only_and_mixed_layouts_match_margin_and_padding_semantics.md)
- [explicit_policy_validation_and_definition_overrides](../../../functions/tests/native_pdf_decorative_border/explicit_policy_validation_and_definition_overrides.md)
- [numeric_content_is_unchanged_and_invalid_geometry_preserves_output](../../../functions/tests/native_pdf_decorative_border/numeric_content_is_unchanged_and_invalid_geometry_preserves_output.md)
- [actual_decorative_pixels_and_transparency_match](../../../functions/tests/native_pdf_decorative_border/actual_decorative_pixels_and_transparency_match.md)