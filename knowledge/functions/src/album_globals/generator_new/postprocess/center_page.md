---
type: Rust Function
title: center_page
resource: src/album_globals/generator_new/postprocess.rs#L28-L68
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/compute_page_used_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/format_page_vspace_mm
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/postprocess/center_tmp_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/full_pages_covers_and_disabled_centering_get_no_extra_space
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn center_page( page: &[&str], config: &crate::config::Config, usable_height: f64, max_vspace: f64, output: &mut Vec<String>, )`

# Calls

- [count](../../../../../functions/src/album_globals/selection/count.md)
- [compute_page_used_height](../../../../../functions/src/album_globals/generator_new/postprocess/compute_page_used_height.md)
- [format_page_vspace_mm](../../../../../functions/src/album_globals/generator_new/layout/format_page_vspace_mm.md)

# Called by

- [center_tmp_pages](../../../../../functions/src/album_globals/generator_new/postprocess/center_tmp_pages.md)
- [full_pages_covers_and_disabled_centering_get_no_extra_space](../../../../../functions/src/album_globals/generator_new/postprocess/full_pages_covers_and_disabled_centering_get_no_extra_space.md)