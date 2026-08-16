---
type: Rust Function
title: format_page_vspace_mm
resource: src/album_globals/generator_new/layout.rs#L28-L34
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/center_tmp_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/center_tmp_pages
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_page_vspace_mm(mm: f64) -> String`

# Called by

- [center_tmp_pages](../../../../../functions/src/album_globals/generator/center_tmp_pages.md)
- [center_tmp_pages](../../../../../functions/src/album_globals/generator_new/postprocess/center_tmp_pages.md)