---
type: Rust Function
title: settings
resource: tests/native_pdf_page_settings.rs#L41-L56
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_even_margins
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn settings(source: &str) -> Result<PageSettings>`

# Calls

- [args](../../../functions/tests/native_pdf_page_settings/args.md)
- [set_even_margins](../../../functions/src/native_pdf/document/PageSettings/set_even_margins.md)