---
type: Rust Function
title: finalize_pdf_document
resource: src/review/pdf_generation.rs#L51-L54
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review/pdf_generation/generate_pdf_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn finalize_pdf_document(pdf: &mut Vec<u8>, title: &str) -> Result<(), String>`

# Called by

- [generate_pdf_page](../../../../functions/src/review/pdf_generation/generate_pdf_page.md)