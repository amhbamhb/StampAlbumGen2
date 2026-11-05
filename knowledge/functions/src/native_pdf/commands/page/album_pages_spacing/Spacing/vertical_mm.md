---
type: Rust Method
title: vertical_mm
resource: src/native_pdf/commands/page/album_pages_spacing.rs#L22-L24
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
  - target: functions/tests/native_pdf_page_settings/spacing_state_matches_reference_content_anchors_and_gaps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn vertical_mm(self) -> f64`

# Called by

- [resolve_page](../../../../../../../functions/src/native_pdf/document/PageSettings/resolve_page.md)
- [copied_template_values_reach_every_existing_command](../../../../../../../functions/tests/native_pdf_config/copied_template_values_reach_every_existing_command.md)
- [spacing_state_matches_reference_content_anchors_and_gaps](../../../../../../../functions/tests/native_pdf_page_settings/spacing_state_matches_reference_content_anchors_and_gaps.md)