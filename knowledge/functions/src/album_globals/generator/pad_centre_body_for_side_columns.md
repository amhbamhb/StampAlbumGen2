---
type: Rust Function
title: pad_centre_body_for_side_columns
resource: src/album_globals/generator.rs#L328-L364
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator/count_nonempty_albumeasy_text_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/estimate_stamp_body_line_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pad_centre_body_for_side_columns( top_block: &str, body: &str, left: &str, right_first_line: &str, right_second_line: &str, ) -> String`

# Calls

- [count_nonempty_albumeasy_text_lines](../../../../functions/src/album_globals/generator/count_nonempty_albumeasy_text_lines.md)

# Called by

- [estimate_stamp_body_line_count](../../../../functions/src/album_globals/generator/estimate_stamp_body_line_count.md)
- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)