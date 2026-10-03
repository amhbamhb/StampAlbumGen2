---
type: Rust Function
title: emit_commemorative_page
resource: src/album_globals/generator_new/commemorative_layout.rs#L265-L297
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_description_boxes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page_end
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_commemorative_page<W: Write>( out: &mut W, page_number: usize, title: &str, stamps: &[Stamp], ) -> std::io::Result<()>`

# Calls

- [emit_commemorative_page_start](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page_start.md)
- [emit_stamp_grid_row](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row.md)
- [emit_description_boxes](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_description_boxes.md)
- [emit_commemorative_page_end](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page_end.md)