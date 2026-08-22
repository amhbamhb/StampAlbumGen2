---
type: Rust Function
title: resolve_runtime_albums_root
resource: src/file_utils.rs#L10-L28
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_bootstrap_albums_root_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn resolve_runtime_albums_root() -> PathBuf`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [load_config](../../../functions/src/config/load_config.md)
- [effective_bootstrap_albums_root_path](../../../functions/src/config/Config/effective_bootstrap_albums_root_path.md)