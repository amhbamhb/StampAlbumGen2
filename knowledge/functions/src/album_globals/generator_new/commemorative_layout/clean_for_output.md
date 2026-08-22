---
type: Rust Function
title: clean_for_output
resource: src/album_globals/generator_new/commemorative_layout.rs#L11-L19
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn clean_for_output(text: &str) -> String`

# Called by

- [emit_stamp_grid_row](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row.md)
- [build_stamp_description](../../../../../functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description.md)