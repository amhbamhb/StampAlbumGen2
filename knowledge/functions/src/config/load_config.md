---
type: Rust Function
title: load_config
resource: src/config.rs#L1757-L2560
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/detect_duplicate_keys
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/with_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_config(path: &str) -> Config`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [detect_duplicate_keys](../../../functions/src/config/detect_duplicate_keys.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [with_defaults](../../../functions/src/config/Config/with_defaults.md)
- [ensure_config_and_template_files](../../../functions/src/config/ensure_config_and_template_files.md)