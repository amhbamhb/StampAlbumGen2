---
type: Rust Function
title: is_commemorative_for_year
resource: src/album_globals/commemorative_generator.rs#L23-L38
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/commemorative_generator/get_commemorative_range
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_commemorative_for_year(stamp: &Stamp, year: i32) -> bool`

# Calls

- [get_commemorative_range](../../../../functions/src/album_globals/commemorative_generator/get_commemorative_range.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)