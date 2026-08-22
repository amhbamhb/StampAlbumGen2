---
type: Rust Function
title: count_albumeasy_text_lines
resource: src/album_globals/generator.rs#L227-L231
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/estimate_stamp_body_line_count
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_albumeasy_text_lines(s: &str) -> usize`

# Called by

- [estimate_stamp_body_line_count](../../../../functions/src/album_globals/generator/estimate_stamp_body_line_count.md)