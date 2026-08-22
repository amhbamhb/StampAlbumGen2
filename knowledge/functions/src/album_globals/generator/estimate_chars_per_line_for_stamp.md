---
type: Rust Function
title: estimate_chars_per_line_for_stamp
resource: src/album_globals/generator.rs#L233-L259
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

`fn estimate_chars_per_line_for_stamp(emit_width: f64, emit_height: f64) -> usize`

# Called by

- [estimate_stamp_body_line_count](../../../../functions/src/album_globals/generator/estimate_stamp_body_line_count.md)