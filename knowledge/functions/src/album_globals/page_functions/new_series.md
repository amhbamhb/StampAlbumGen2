---
type: Rust Function
title: new_series
resource: src/album_globals/page_functions.rs#L17-L45
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/page_functions/new_page
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/commands_file_generator/album_creation/create_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn new_series<W: std::io::Write>( writer: &mut W, ctx: &SeriesContext<'_>, rows_written: &mut i32, stamps_written: &mut i32, used_width: &mut i32, used_height: &mut i32, max_row_height: &mut i32, )`

# Calls

- [new_page](../../../../functions/src/album_globals/page_functions/new_page.md)

# Called by

- [create_album](../../../../functions/src/commands_file_generator/album_creation/create_album.md)