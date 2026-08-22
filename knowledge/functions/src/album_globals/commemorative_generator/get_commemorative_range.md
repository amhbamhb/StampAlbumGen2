---
type: Rust Function
title: get_commemorative_range
resource: src/album_globals/commemorative_generator.rs#L8-L20
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/commemorative_generator/is_commemorative_for_year
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn get_commemorative_range(year: i32) -> Option<(u32, u32)>`

# Called by

- [is_commemorative_for_year](../../../../functions/src/album_globals/commemorative_generator/is_commemorative_for_year.md)