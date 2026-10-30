---
type: Rust Function
title: parse_stampworld_pdf
resource: src/review_ui/pdf_parser/extraction.rs#L89-L91
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_text
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_extraction_test/stampworld_pdf_extraction_layout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_extraction_test/all_supplied_stampworld_pdfs_have_individual_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_extraction_test/supplied_denmark_pages_have_266_individual_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_import_tool/imports_denmark_pages_and_skips_all_266_on_second_pass
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_stampworld_pdf<P: AsRef<Path>>(path: P) -> Result<Vec<PdfStampRecord>, String>`

# Calls

- [parse_stampworld_text](../../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text.md)
- [extract_stampworld_pdf_text](../../../../../functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_text.md)

# Called by

- [import_stampworld_file](../../../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)
- [stampworld_pdf_extraction_layout](../../../../../functions/tests/pdf_extraction_test/stampworld_pdf_extraction_layout.md)
- [all_supplied_stampworld_pdfs_have_individual_rows](../../../../../functions/tests/pdf_extraction_test/all_supplied_stampworld_pdfs_have_individual_rows.md)
- [supplied_denmark_pages_have_266_individual_stamps](../../../../../functions/tests/pdf_extraction_test/supplied_denmark_pages_have_266_individual_stamps.md)
- [imports_denmark_pages_and_skips_all_266_on_second_pass](../../../../../functions/tests/pdf_import_tool/imports_denmark_pages_and_skips_all_266_on_second_pass.md)