---
type: Rust Method
title: set
resource: src/album_globals/structs.rs#L20-L22
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/build/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/set_attr_if_non_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/set_attr_if_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set(&mut self, key: &str, value: &str)`

# Called by

- [main](../../../../../functions/build/main.md)
- [set_attr_if_non_empty](../../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_non_empty.md)
- [set_attr_if_missing](../../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_missing.md)
- [fetch_and_filter_stamps](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)