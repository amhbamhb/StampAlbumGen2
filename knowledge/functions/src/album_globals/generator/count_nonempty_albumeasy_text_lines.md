---
type: Rust Function
title: count_nonempty_albumeasy_text_lines
resource: src/album_globals/generator.rs#L236-L245
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/pad_centre_body_for_side_columns
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_nonempty_albumeasy_text_lines(s: &str) -> usize`

# Calls

- [count](../../../../functions/src/album_globals/selection/count.md)

# Called by

- [pad_centre_body_for_side_columns](../../../../functions/src/album_globals/generator/pad_centre_body_for_side_columns.md)