---
type: Rust Function
title: extract_stampworld_pdf_content
resource: src/review_ui/pdf_parser/extraction.rs#L108-L114
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_images
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn extract_stampworld_pdf_content<P: AsRef<Path>>( path: P, ) -> Result<(String, Vec<oxidize_pdf::operations::ExtractedImage>), String>`

# Calls

- [extract_stampworld_pdf_text](../../../../../functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_text.md)
- [extract_stampworld_pdf_images](../../../../../functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_images.md)