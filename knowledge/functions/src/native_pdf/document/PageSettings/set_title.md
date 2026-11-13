---
type: Rust Method
title: set_title
resource: src/native_pdf/document.rs#L141-L143
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_first_run_installation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_title/title_fit_boundaries_and_hint_preserve_measured_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_title(&mut self, title: AlbumPagesTitle)`

# Called by

- [ensure_config_and_template_files](../../../../../functions/src/config/ensure_config_and_template_files.md)
- [parse](../../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [ensure_first_run_installation](../../../../../functions/src/setup/ensure_first_run_installation.md)
- [title_fit_boundaries_and_hint_preserve_measured_text](../../../../../functions/tests/native_pdf_page_title/title_fit_boundaries_and_hint_preserve_measured_text.md)