---
type: Rust Function
title: write_stamp_page_header
resource: src/album_globals/generator_new/layout.rs#L63-L79
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/start_stamp_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_stamp_page_header<W: Write>( out: &mut W, page_number: usize, config: &crate::config::Config, ) -> std::io::Result<()>`

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [start_stamp_page](../../../../../functions/src/album_globals/generator_new/layout/start_stamp_page.md)