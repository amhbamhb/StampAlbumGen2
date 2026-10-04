---
type: Rust Function
title: flag_directories
resource: src/flagdb/assets.rs#L10-L34
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/load_flag_map_from_fs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_flag_image_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn flag_directories() -> Vec<PathBuf>`

# Calls

- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [load_flag_map_from_fs](../../../../functions/src/flagdb/load_flag_map_from_fs.md)
- [ensure_flag_image_with_fetcher](../../../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)