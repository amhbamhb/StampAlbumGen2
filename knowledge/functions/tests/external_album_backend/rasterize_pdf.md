---
type: Rust Function
title: rasterize_pdf
resource: tests/external_album_backend.rs#L1048-L1072
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rasterize_pdf(tool: &Path, pdf: &Path, prefix: &Path)`

# Calls

- [args](../../../functions/tests/native_pdf_page_settings/args.md)

# Called by

- [brazil_real_albumeasy_import_approval_and_page_edit](../../../functions/tests/external_album_backend/brazil_real_albumeasy_import_approval_and_page_edit.md)