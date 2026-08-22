---
type: Rust Function
title: wrap_description_for_estimate
resource: src/album_globals/generator.rs#L261-L284
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

`fn wrap_description_for_estimate(s: &str, max_chars: usize) -> String`

# Called by

- [estimate_stamp_body_line_count](../../../../functions/src/album_globals/generator/estimate_stamp_body_line_count.md)