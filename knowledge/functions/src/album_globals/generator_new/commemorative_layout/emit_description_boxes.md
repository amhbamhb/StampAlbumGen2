---
type: Rust Function
title: emit_description_boxes
resource: src/album_globals/generator_new/commemorative_layout.rs#L134-L168
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_description_boxes<W: Write>( out: &mut W, stamp1: &Stamp, stamp2: Option<&Stamp>, ) -> std::io::Result<()>`

# Calls

- [build_stamp_description](../../../../../functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description.md)

# Called by

- [emit_commemorative_page](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page.md)