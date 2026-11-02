---
type: Rust Function
title: extract_stampworld_pdf_text
resource: src/review_ui/pdf_parser/extraction.rs#L147-L159
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/country_url/StampWorldCountryUrl/load
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_content
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn extract_stampworld_pdf_text<P: AsRef<Path>>(path: P) -> Result<String, String>`

# Calls

- [load](../../../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/load.md)

# Called by

- [parse_stampworld_pdf](../../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf.md)
- [extract_stampworld_pdf_content](../../../../../functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_content.md)