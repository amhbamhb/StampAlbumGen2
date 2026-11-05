---
type: Rust Method
title: horizontal_mm
resource: src/native_pdf/commands/page/album_pages_spacing.rs#L18-L20
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/document/PageSettings/resolve_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/copied_template_values_reach_every_existing_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn horizontal_mm(self) -> f64`

# Called by

- [resolve_page](../../../../../../../functions/src/native_pdf/document/PageSettings/resolve_page.md)
- [copied_template_values_reach_every_existing_command](../../../../../../../functions/tests/native_pdf_config/copied_template_values_reach_every_existing_command.md)
- [rendered_paths_match_margin_and_spacing_fixtures](../../../../../../../functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures.md)