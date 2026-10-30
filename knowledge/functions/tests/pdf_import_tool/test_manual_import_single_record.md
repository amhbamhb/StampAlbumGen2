---
type: Rust Function
title: test_manual_import_single_record
resource: tests/pdf_import_tool.rs#L33-L53
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/db_query/process_pdf_records
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_manual_import_single_record()`

# Calls

- [process_pdf_records](../../../functions/src/review_ui/db_query/process_pdf_records.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)