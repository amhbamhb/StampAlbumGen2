---
type: Rust Function
title: margin_and_even_page_bounds_match_frozen_expectations
resource: tests/native_pdf_page_settings.rs#L98-L124
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/Bounds/inset
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn margin_and_even_page_bounds_match_frozen_expectations() -> Result<()>`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [inset](../../../functions/src/native_pdf/document/Bounds/inset.md)