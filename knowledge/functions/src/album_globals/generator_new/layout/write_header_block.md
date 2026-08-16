---
type: Rust Function
title: write_header_block
resource: src/album_globals/generator_new/layout.rs#L112-L146
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/layout/write_page_vspace_marker
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_page_vspace
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_header_block<W: Write>( out: &mut W, pending_lines: &[(String, f64)], v_before: f64, v_after: f64, _config: &crate::config::Config, ) -> std::io::Result<f64>`

# Calls

- [write_page_vspace_marker](../../../../../functions/src/album_globals/generator_new/layout/write_page_vspace_marker.md)
- [write_page_vspace](../../../../../functions/src/album_globals/generator_new/layout/write_page_vspace.md)

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)