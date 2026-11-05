---
type: Rust Method
title: next_top
resource: src/native_pdf/commands/page/album_pages_spacing.rs#L27-L33
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/native_pdf_config/explicit_file_changes_preserve_precision_even_margins_and_native_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/spacing_state_matches_reference_content_anchors_and_gaps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn next_top(self, previous_bottom_mm: f64) -> Result<f64>`

# Called by

- [explicit_file_changes_preserve_precision_even_margins_and_native_output](../../../../../../../functions/tests/native_pdf_config/explicit_file_changes_preserve_precision_even_margins_and_native_output.md)
- [spacing_state_matches_reference_content_anchors_and_gaps](../../../../../../../functions/tests/native_pdf_page_settings/spacing_state_matches_reference_content_anchors_and_gaps.md)