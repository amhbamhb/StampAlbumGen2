---
type: Rust Function
title: estimate_stamp_heading_line_count
resource: src/album_globals/generator_new/emit.rs#L148-L167
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/emit/build_stamp_heading_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn estimate_stamp_heading_line_count( sh_label: &str, sgnum: &str, stamp_name: Option<&str>, perforation: Option<&str>, include_perforation: bool, emit_width: f64, sh_size: i32, ) -> usize`

# Calls

- [build_stamp_heading_lines](../../../../../functions/src/album_globals/generator_new/emit/build_stamp_heading_lines.md)

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)