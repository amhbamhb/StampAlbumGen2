---
type: Rust Function
title: write_page_geometry_proof_with_date
resource: src/native_pdf/renderer.rs#L22-L29
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/renderer/write_proof
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_page_geometry_proof_with_date( settings: &PageSettings, proof: &GeometryProofSettings, output: &Path, generation_date: Option<chrono::NaiveDate>, ) -> Result<()>`

# Calls

- [write_proof](../../../../functions/src/native_pdf/renderer/write_proof.md)

# Called by

- [write_page_geometry_proof](../../../../functions/src/native_pdf/renderer/write_page_geometry_proof.md)
- [header_and_footer_share_one_generation_date_with_independent_fixed_overrides](../../../../functions/tests/native_pdf_footer_date/header_and_footer_share_one_generation_date_with_independent_fixed_overrides.md)
- [generation_date_requires_one_explicit_document_context](../../../../functions/tests/native_pdf_header_date/generation_date_requires_one_explicit_document_context.md)