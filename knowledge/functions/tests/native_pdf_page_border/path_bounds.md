---
type: Rust Function
title: path_bounds
resource: tests/native_pdf_page_border.rs#L180-L207
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/units/points_to_mm
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/native_pdf_page_border/rendered_border_paths_match_frozen_references
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn path_bounds(path: roxmltree::Node<'_, '_>) -> Result<[f64; 4]>`

# Calls

- [points_to_mm](../../../functions/src/native_pdf/units/points_to_mm.md)

# Called by

- [rendered_border_paths_match_frozen_references](../../../functions/tests/native_pdf_page_border/rendered_border_paths_match_frozen_references.md)