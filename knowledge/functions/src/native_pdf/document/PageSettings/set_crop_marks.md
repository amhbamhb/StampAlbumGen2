---
type: Rust Method
title: set_crop_marks
resource: src/native_pdf/document.rs#L133-L135
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/invalid_layout_creates_no_pdf_and_preserves_existing_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_crop_marks(&mut self, marks: AlbumPagesCropMarks)`

# Called by

- [parse](../../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [invalid_layout_creates_no_pdf_and_preserves_existing_output](../../../../../functions/tests/native_pdf_crop_marks/invalid_layout_creates_no_pdf_and_preserves_existing_output.md)
- [actual_native_paths_match_all_repeated_reference_cases](../../../../../functions/tests/native_pdf_crop_marks/actual_native_paths_match_all_repeated_reference_cases.md)