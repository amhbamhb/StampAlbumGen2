---
type: Rust Function
title: config
resource: tests/native_pdf_crop_marks.rs#L61-L63
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/NativePdfConfig/from_toml
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/native_pdf_crop_marks/marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/missing_invalid_disabled_keys_redaction_and_unknown_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/rendered_native_colour_is_independent_of_album_border
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn config(crop: &str) -> Result<NativePdfConfig>`

# Calls

- [from_toml](../../../functions/src/native_pdf/config/NativePdfConfig/from_toml.md)

# Called by

- [marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges](../../../functions/tests/native_pdf_crop_marks/marks_do_not_resize_shift_or_reserve_content_and_follow_physical_edges.md)
- [missing_invalid_disabled_keys_redaction_and_unknown_keys](../../../functions/tests/native_pdf_crop_marks/missing_invalid_disabled_keys_redaction_and_unknown_keys.md)
- [rendered_native_colour_is_independent_of_album_border](../../../functions/tests/native_pdf_crop_marks/rendered_native_colour_is_independent_of_album_border.md)