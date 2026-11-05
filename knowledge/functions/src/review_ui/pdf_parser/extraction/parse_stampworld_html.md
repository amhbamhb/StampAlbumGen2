---
type: Rust Function
title: parse_stampworld_html
resource: src/review_ui/pdf_parser/extraction.rs#L295-L354
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/extraction/metadata
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/pdf_parser/import_stampworld_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_extraction_test/canonical_html_preserves_individual_rows_and_metadata
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_stampworld_html(path: &Path) -> Result<Vec<PdfStampRecord>, String>`

# Calls

- [metadata](../../../../../functions/src/review_ui/pdf_parser/extraction/metadata.md)

# Called by

- [import_stampworld_file](../../../../../functions/src/review_ui/pdf_parser/import_stampworld_file.md)
- [canonical_html_preserves_individual_rows_and_metadata](../../../../../functions/tests/pdf_extraction_test/canonical_html_preserves_individual_rows_and_metadata.md)