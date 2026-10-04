---
type: Rust Function
title: compute_page_used_height
resource: src/album_globals/generator_new/postprocess.rs#L73-L89
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator_new/postprocess/center_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compute_page_used_height(page: &[&str], _config: &crate::config::Config) -> f64`

# Called by

- [center_page](../../../../../functions/src/album_globals/generator_new/postprocess/center_page.md)