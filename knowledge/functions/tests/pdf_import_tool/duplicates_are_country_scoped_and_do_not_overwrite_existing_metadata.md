---
type: Rust Function
title: duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata
resource: tests/pdf_import_tool.rs#L147-L165
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/db_query/process_pdf_records
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/sample_record
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata()`

# Calls

- [process_pdf_records](../../../functions/src/review_ui/db_query/process_pdf_records.md)
- [sample_record](../../../functions/tests/pdf_import_tool/sample_record.md)