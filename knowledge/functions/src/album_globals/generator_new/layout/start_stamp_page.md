---
type: Rust Function
title: start_stamp_page
resource: src/album_globals/generator_new/layout.rs#L84-L97
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/layout/write_stamp_page_header
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn start_stamp_page<W: Write>( out: &mut W, page_counter: &mut usize, config: &crate::config::Config, ) -> std::io::Result<()>`

# Calls

- [write_stamp_page_header](../../../../../functions/src/album_globals/generator_new/layout/write_stamp_page_header.md)

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)