---
type: Rust Function
title: write_page_vspace_marker
resource: src/album_globals/generator_new/layout.rs#L23-L25
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator_new/layout/write_header_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_page_vspace_marker<W: Write>(out: &mut W, marker: &str) -> std::io::Result<()>`

# Called by

- [write_header_block](../../../../../functions/src/album_globals/generator_new/layout/write_header_block.md)