---
type: Rust Function
title: extract_stampworld_pdf_text
resource: src/review_ui/pdf_parser/extraction.rs#L93-L105
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
---

# Signature

`pub fn extract_stampworld_pdf_text<P: AsRef<Path>>(path: P) -> Result<String, String>`

# Calls

- [load](../../../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/load.md)

# Called by

- [parse_stampworld_pdf](../../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_pdf.md)