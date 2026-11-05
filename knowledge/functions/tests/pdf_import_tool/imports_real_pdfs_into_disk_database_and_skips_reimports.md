---
type: Rust Function
title: imports_real_pdfs_into_disk_database_and_skips_reimports
resource: tests/pdf_import_tool.rs#L56-L96
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn imports_real_pdfs_into_disk_database_and_skips_reimports()`

# Calls

- [import_stampworld_file](../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)
- [open](../../../functions/src/external_album/open.md)