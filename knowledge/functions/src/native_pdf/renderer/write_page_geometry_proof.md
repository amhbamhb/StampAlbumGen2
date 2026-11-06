---
type: Rust Function
title: write_page_geometry_proof
resource: src/native_pdf/renderer.rs#L11-L18
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/PageSettings/capture_generation_date
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_page_geometry_proof( settings: &PageSettings, proof: &GeometryProofSettings, output: &Path, ) -> Result<()>`

# Calls

- [capture_generation_date](../../../../functions/src/native_pdf/document/PageSettings/capture_generation_date.md)
- [write_page_geometry_proof_with_date](../../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)