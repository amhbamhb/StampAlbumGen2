---
type: Rust Function
title: add_stamp
resource: src/album_globals/page_functions.rs#L144-L160
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/commands_file_generator/album_creation/create_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn add_stamp<W: std::io::Write>(writer: &mut W, ctx: &StampContext<'_>)`

# Called by

- [create_album](../../../../functions/src/commands_file_generator/album_creation/create_album.md)