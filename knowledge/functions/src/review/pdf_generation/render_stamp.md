---
type: Rust Function
title: render_stamp
resource: src/review/pdf_generation.rs#L44-L48
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review/pdf_generation/generate_pdf_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn render_stamp(stamp: &StampEntry) -> Result<Vec<u8>, String>`

# Called by

- [generate_pdf_page](../../../../functions/src/review/pdf_generation/generate_pdf_page.md)