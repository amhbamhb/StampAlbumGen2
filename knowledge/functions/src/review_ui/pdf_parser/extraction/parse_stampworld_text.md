---
type: Rust Function
title: parse_stampworld_text
resource: src/review_ui/pdf_parser/extraction.rs#L194-L287
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/extraction/metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/clear
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/pdf_extraction_test/malformed_rows_and_missing_headings_are_errors_not_empty_successes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn parse_stampworld_text(text: &str) -> Result<Vec<PdfStampRecord>, String>`

# Calls

- [metadata](../../../../../functions/src/review_ui/pdf_parser/extraction/metadata.md)
- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [clear](../../../../../functions/src/progress/ProgressIndicator/clear.md)

# Called by

- [parse_stampworld_pdf](../../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf.md)
- [malformed_rows_and_missing_headings_are_errors_not_empty_successes](../../../../../functions/tests/pdf_extraction_test/malformed_rows_and_missing_headings_are_errors_not_empty_successes.md)