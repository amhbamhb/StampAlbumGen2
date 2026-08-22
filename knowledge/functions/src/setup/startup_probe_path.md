---
type: Rust Function
title: startup_probe_path
resource: src/setup.rs#L106-L115
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/setup/write_startup_probe
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn startup_probe_path() -> PathBuf`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)

# Called by

- [write_startup_probe](../../../functions/src/setup/write_startup_probe.md)