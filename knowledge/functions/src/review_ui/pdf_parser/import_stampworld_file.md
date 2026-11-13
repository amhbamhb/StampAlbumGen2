---
type: Rust Function
title: import_stampworld_file
resource: src/review_ui/pdf_parser/mod.rs#L8-L48
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/db_query/process_pdf_records
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_stampworld_file_import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/handle_stampworld_import_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/imports_real_pdfs_into_disk_database_and_skips_reimports
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/imports_denmark_pages_and_skips_all_266_on_second_pass
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/filters_years_imports_html_and_reports_invalid_inputs
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn import_stampworld_file( conn: &Connection, path: &Path, country: &str, start_year: &str, end_year: &str, ) -> Result<ImportSummary, String>`

# Calls

- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [parse_stampworld_pdf](../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf.md)
- [parse_stampworld_html](../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_html.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)
- [process_pdf_records](../../../../functions/src/review_ui/db_query/process_pdf_records.md)

# Called by

- [run_stampworld_file_import](../../../../functions/src/modes/run_stampworld_file_import.md)
- [handle_stampworld_import_request](../../../../functions/src/review_ui/http_handlers/handle_stampworld_import_request.md)
- [imports_real_pdfs_into_disk_database_and_skips_reimports](../../../../functions/tests/pdf_import_tool/imports_real_pdfs_into_disk_database_and_skips_reimports.md)
- [imports_denmark_pages_and_skips_all_266_on_second_pass](../../../../functions/tests/pdf_import_tool/imports_denmark_pages_and_skips_all_266_on_second_pass.md)
- [filters_years_imports_html_and_reports_invalid_inputs](../../../../functions/tests/pdf_import_tool/filters_years_imports_html_and_reports_invalid_inputs.md)