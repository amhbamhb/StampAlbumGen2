---
type: Rust Function
title: emit_stamp_grid_row
resource: src/album_globals/generator_new/commemorative_layout.rs#L61-L131
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/commemorative_layout/clean_for_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn emit_stamp_grid_row<W: Write>( out: &mut W, stamp1: &Stamp, stamp2: Option<&Stamp>, ) -> std::io::Result<()>`

# Calls

- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [clean_for_output](../../../../../functions/src/album_globals/generator_new/commemorative_layout/clean_for_output.md)
- [as_str](../../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [emit_commemorative_page](../../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page.md)