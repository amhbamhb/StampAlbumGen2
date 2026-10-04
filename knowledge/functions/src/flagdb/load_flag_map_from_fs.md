---
type: Rust Function
title: load_flag_map_from_fs
resource: src/flagdb.rs#L28-L45
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/default_flag_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/flag_directories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/countries/merge_overrides
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/country_to_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/embedded_flags/embedded_flags_writeback_and_load
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn load_flag_map_from_fs() -> HashMap<String, String>`

# Calls

- [default_flag_map](../../../functions/src/flagdb/default_flag_map.md)
- [flag_directories](../../../functions/src/flagdb/assets/flag_directories.md)
- [merge_overrides](../../../functions/src/flagdb/countries/merge_overrides.md)

# Called by

- [country_to_code](../../../functions/src/flagdb/country_to_code.md)
- [embedded_flags_writeback_and_load](../../../functions/tests/embedded_flags/embedded_flags_writeback_and_load.md)