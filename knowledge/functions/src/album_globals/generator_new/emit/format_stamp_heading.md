---
type: Rust Function
title: format_stamp_heading
resource: src/album_globals/generator_new/emit.rs#L125-L147
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/emit/build_stamp_heading_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/emit/emit_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn format_stamp_heading( sh_label: &str, sgnum: &str, stamp_name: Option<&str>, perforation: Option<&str>, include_perforation: bool, emit_width: f64, sh_size: i32, ) -> Option<String>`

# Calls

- [build_stamp_heading_lines](../../../../../functions/src/album_globals/generator_new/emit/build_stamp_heading_lines.md)

# Called by

- [emit_stamp](../../../../../functions/src/album_globals/generator_new/emit/emit_stamp.md)