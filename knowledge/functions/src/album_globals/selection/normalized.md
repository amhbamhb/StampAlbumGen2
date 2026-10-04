---
type: Rust Function
title: normalized
resource: src/album_globals/selection.rs#L16-L18
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/selection/ColnectPdfFilters/matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/colnect_pdf_options
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn normalized(value: &str) -> String`

# Called by

- [matches](../../../../functions/src/album_globals/selection/ColnectPdfFilters/matches.md)
- [colnect_pdf_options](../../../../functions/src/album_globals/selection/colnect_pdf_options.md)