---
type: Rust Function
title: load_flag_map_from_fs
resource: src/flagdb.rs#L32-L76
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/default_flag_map
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

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [default_flag_map](../../../functions/src/flagdb/default_flag_map.md)

# Called by

- [country_to_code](../../../functions/src/flagdb/country_to_code.md)
- [embedded_flags_writeback_and_load](../../../functions/tests/embedded_flags/embedded_flags_writeback_and_load.md)