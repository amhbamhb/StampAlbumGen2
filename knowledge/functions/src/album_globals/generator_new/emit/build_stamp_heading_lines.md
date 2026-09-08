---
type: Rust Function
title: build_stamp_heading_lines
resource: src/album_globals/generator_new/emit.rs#L91-L123
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/emit/heading_max_chars_per_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/wrap_preserving_breaks_keep_empty
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/emit/format_stamp_heading
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/estimate_stamp_heading_line_count
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_stamp_heading_lines( sh_label: &str, sgnum: &str, stamp_name: Option<&str>, perforation: Option<&str>, include_perforation: bool, emit_width: f64, sh_size: i32, ) -> Vec<String>`

# Calls

- [heading_max_chars_per_line](../../../../../functions/src/album_globals/generator_new/emit/heading_max_chars_per_line.md)
- [wrap_preserving_breaks_keep_empty](../../../../../functions/src/album_globals/generator_new/emit/wrap_preserving_breaks_keep_empty.md)

# Called by

- [format_stamp_heading](../../../../../functions/src/album_globals/generator_new/emit/format_stamp_heading.md)
- [estimate_stamp_heading_line_count](../../../../../functions/src/album_globals/generator_new/emit/estimate_stamp_heading_line_count.md)