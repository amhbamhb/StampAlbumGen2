---
type: Rust Function
title: center_tmp_pages
resource: src/album_globals/generator_new/postprocess.rs#L9-L296
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/layout/format_page_vspace_mm
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn center_tmp_pages( tmp_path: &Path, config: &crate::config::Config, usable_height: f64, ) -> std::io::Result<()>`

# Calls

- [format_page_vspace_mm](../../../../../functions/src/album_globals/generator_new/layout/format_page_vspace_mm.md)