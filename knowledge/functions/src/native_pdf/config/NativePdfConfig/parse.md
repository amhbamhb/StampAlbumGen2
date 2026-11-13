---
type: Rust Method
title: parse
resource: src/native_pdf/config.rs#L74-L616
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/config/string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/config/table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/config/boolean
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_decorative_border
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_crop_marks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_footer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_footer_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_footer_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_header_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_header_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_title
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_border_line_colours
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_even_margins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/capture_generation_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/resolve_with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/proof/GeometryProofSettings/validate_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/config/collect_keys
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse(source: &str, base: Option<&Path>) -> Result<Self>`

# Calls

- [string](../../../../../functions/src/native_pdf/config/string.md)
- [table](../../../../../functions/src/native_pdf/config/table.md)
- [boolean](../../../../../functions/src/native_pdf/config/boolean.md)
- [set_decorative_border](../../../../../functions/src/native_pdf/document/PageSettings/set_decorative_border.md)
- [set_crop_marks](../../../../../functions/src/native_pdf/document/PageSettings/set_crop_marks.md)
- [set_footer](../../../../../functions/src/native_pdf/document/PageSettings/set_footer.md)
- [set_footer_date](../../../../../functions/src/native_pdf/document/PageSettings/set_footer_date.md)
- [set_footer_number](../../../../../functions/src/native_pdf/document/PageSettings/set_footer_number.md)
- [set_header](../../../../../functions/src/native_pdf/document/PageSettings/set_header.md)
- [set_header_number](../../../../../functions/src/native_pdf/document/PageSettings/set_header_number.md)
- [set_header_date](../../../../../functions/src/native_pdf/document/PageSettings/set_header_date.md)
- [set_title](../../../../../functions/src/native_pdf/document/PageSettings/set_title.md)
- [set_border_line_colours](../../../../../functions/src/native_pdf/document/PageSettings/set_border_line_colours.md)
- [set_even_margins](../../../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)
- [capture_generation_date](../../../../../functions/src/native_pdf/document/PageSettings/capture_generation_date.md)
- [resolve_with_context](../../../../../functions/src/native_pdf/document/PageSettings/resolve_with_context.md)
- [validate_layout](../../../../../functions/src/native_pdf/proof/GeometryProofSettings/validate_layout.md)
- [collect_keys](../../../../../functions/src/native_pdf/config/collect_keys.md)