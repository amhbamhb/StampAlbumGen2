---
type: Rust Method
title: width_mm
resource: src/native_pdf/commands/page/album_pages_size.rs#L21-L23
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_crop_marks/AlbumPagesCropMarks/layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/resolve_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/copied_template_values_reach_every_existing_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/explicit_file_changes_preserve_precision_even_margins_and_native_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_config/legacy_root_and_section_precedence_are_explicit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn width_mm(self) -> f64`

# Called by

- [layout](../../../../../../../functions/src/native_pdf/commands/page/album_pages_crop_marks/AlbumPagesCropMarks/layout.md)
- [resolve_page](../../../../../../../functions/src/native_pdf/document/PageSettings/resolve_page.md)
- [write_proof](../../../../../../../functions/src/native_pdf/renderer/write_proof.md)
- [copied_template_values_reach_every_existing_command](../../../../../../../functions/tests/native_pdf_config/copied_template_values_reach_every_existing_command.md)
- [explicit_file_changes_preserve_precision_even_margins_and_native_output](../../../../../../../functions/tests/native_pdf_config/explicit_file_changes_preserve_precision_even_margins_and_native_output.md)
- [legacy_root_and_section_precedence_are_explicit](../../../../../../../functions/tests/native_pdf_config/legacy_root_and_section_precedence_are_explicit.md)
- [marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges](../../../../../../../functions/tests/native_pdf_crop_marks/marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges.md)