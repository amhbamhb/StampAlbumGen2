---
type: Rust Function
title: default_flag_map
resource: src/flagdb.rs#L15-L29
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/flagdb/load_flag_map_from_fs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/flagdb/test_load_builtin_map
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn default_flag_map() -> HashMap<String, String>`

# Called by

- [load_flag_map_from_fs](../../../functions/src/flagdb/load_flag_map_from_fs.md)
- [test_load_builtin_map](../../../functions/tests/flagdb/test_load_builtin_map.md)