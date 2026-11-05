---
type: Rust Method
title: from_toml_in
resource: src/native_pdf/config.rs#L70-L72
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/load
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/loaded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/explicit_policy_validation_and_definition_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_decorative_border/actual_native_strokes_images_alpha_and_content_match_references
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn from_toml_in(source: &str, base: &Path) -> Result<Self>`

# Called by

- [load](../../../../../functions/src/native_pdf/config/NativePdfConfig/load.md)
- [loaded](../../../../../functions/tests/native_pdf_decorative_border/loaded.md)
- [explicit_policy_validation_and_definition_overrides](../../../../../functions/tests/native_pdf_decorative_border/explicit_policy_validation_and_definition_overrides.md)
- [explicit_config_disabled_compatibility_and_templates](../../../../../functions/tests/native_pdf_decorative_border/explicit_config_disabled_compatibility_and_templates.md)
- [actual_native_strokes_images_alpha_and_content_match_references](../../../../../functions/tests/native_pdf_decorative_border/actual_native_strokes_images_alpha_and_content_match_references.md)