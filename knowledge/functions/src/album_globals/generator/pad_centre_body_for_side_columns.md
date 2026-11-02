---
type: Rust Function
title: pad_centre_body_for_side_columns
resource: src/album_globals/generator.rs#L248-L284
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator/count_nonempty_albumeasy_text_lines
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pad_centre_body_for_side_columns( top_block: &str, body: &str, left: &str, right_first_line: &str, right_second_line: &str, ) -> String`

# Calls

- [count_nonempty_albumeasy_text_lines](../../../../functions/src/album_globals/generator/count_nonempty_albumeasy_text_lines.md)

# Called by

- [build_stamp_text](../../../../functions/src/album_globals/generator/build_stamp_text.md)