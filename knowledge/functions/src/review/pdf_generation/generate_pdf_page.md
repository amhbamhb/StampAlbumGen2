---
type: Rust Function
title: generate_pdf_page
resource: src/review/pdf_generation.rs#L22-L41
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review/pdf_generation/render_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review/pdf_generation/finalize_pdf_document
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_pdf_page(page: &PdfPage) -> Result<Vec<u8>, String>`

# Calls

- [render_stamp](../../../../functions/src/review/pdf_generation/render_stamp.md)
- [finalize_pdf_document](../../../../functions/src/review/pdf_generation/finalize_pdf_document.md)