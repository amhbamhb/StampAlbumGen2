---
type: Rust Function
title: write_page_vspace
resource: src/album_globals/generator_new/layout.rs#L11-L17
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_header_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_page_vspace<W: Write>(out: &mut W, mm: f64) -> std::io::Result<()>`

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [write_header_block](../../../../../functions/src/album_globals/generator_new/layout/write_header_block.md)
- [write_globals](../../../../../functions/src/album_utils/globals/write_globals.md)
- [add_wikipedia_section](../../../../../functions/src/album_utils/globals/add_wikipedia_section.md)