---
type: Rust Function
title: process_pdf_records
resource: src/review_ui/db_query.rs#L24-L90
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/test_manual_import_single_record
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/insert_errors_roll_back_the_entire_file_and_are_not_counted_as_duplicates
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn process_pdf_records( conn: &Connection, records: Vec<PdfStampRecord>, country: &str, ) -> Result<ImportSummary, String>`

# Called by

- [import_stampworld_file](../../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)
- [test_manual_import_single_record](../../../../functions/tests/pdf_import_tool/test_manual_import_single_record.md)
- [duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata](../../../../functions/tests/pdf_import_tool/duplicates_are_country_scoped_and_do_not_overwrite_existing_metadata.md)
- [insert_errors_roll_back_the_entire_file_and_are_not_counted_as_duplicates](../../../../functions/tests/pdf_import_tool/insert_errors_roll_back_the_entire_file_and_are_not_counted_as_duplicates.md)