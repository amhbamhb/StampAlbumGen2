---
type: Rust Function
title: imports_denmark_pages_and_skips_all_266_on_second_pass
resource: tests/pdf_import_tool.rs#L98-L142
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn imports_denmark_pages_and_skips_all_266_on_second_pass()`

# Calls

- [parse_stampworld_pdf](../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [import_stampworld_file](../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)