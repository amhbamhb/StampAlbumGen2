---
type: Rust Function
title: estimate_stamp_body_line_count
resource: src/album_globals/generator.rs#L432-L495
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator/estimate_chars_per_line_for_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/wrap_left_for_estimate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/wrap_description_for_estimate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/pad_centre_body_for_side_columns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/count_albumeasy_text_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn estimate_stamp_body_line_count( raw_left: &str, raw_stamps_issued: &str, raw_description: &str, raw_color: &str, raw_colnect: &str, right_first_line: &str, right_second_line: &str, emit_width: f64, emit_height: f64, ) -> usize`

# Calls

- [estimate_chars_per_line_for_stamp](../../../../functions/src/album_globals/generator/estimate_chars_per_line_for_stamp.md)
- [wrap_left_for_estimate](../../../../functions/src/album_globals/generator/wrap_left_for_estimate.md)
- [wrap_description_for_estimate](../../../../functions/src/album_globals/generator/wrap_description_for_estimate.md)
- [pad_centre_body_for_side_columns](../../../../functions/src/album_globals/generator/pad_centre_body_for_side_columns.md)
- [count_albumeasy_text_lines](../../../../functions/src/album_globals/generator/count_albumeasy_text_lines.md)

# Called by

- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)