---
type: Rust Method
title: validate_layout
resource: src/native_pdf/proof.rs#L27-L35
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/document/Bounds/inset
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/config/NativePdfConfig/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/renderer/write_page_geometry_proof_with_date
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_layout(&self, layout: &PageLayout) -> Result<()>`

# Calls

- [inset](../../../../../functions/src/native_pdf/document/Bounds/inset.md)

# Called by

- [parse](../../../../../functions/src/native_pdf/config/NativePdfConfig/parse.md)
- [write_page_geometry_proof_with_date](../../../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)