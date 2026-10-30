---
type: Rust Function
title: new_page
resource: src/album_globals/page_functions.rs#L48-L70
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/layout/write_page_start
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/page_functions/new_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_row
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/album_creation/create_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn new_page<W: std::io::Write>( writer: &mut W, rows_written: &mut i32, stamps_written: &mut i32, used_width: &mut i32, used_height: &mut i32, max_row_height: &mut i32, )`

# Calls

- [write_page_start](../../../../functions/src/album_globals/generator_new/layout/write_page_start.md)

# Called by

- [new_series](../../../../functions/src/album_globals/page_functions/new_series.md)
- [new_row](../../../../functions/src/album_globals/page_functions/new_row.md)
- [create_album](../../../../functions/src/commands_file_generator/album_creation/create_album.md)