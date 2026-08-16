---
type: Rust Function
title: write_startup_probe_snapshot
resource: src/setup.rs#L127-L167
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/write_startup_probe
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_startup_probe_snapshot( args: &[String], launched_without_args: bool, interactive_terminal: bool, )`

# Calls

- [write_startup_probe](../../../functions/src/setup/write_startup_probe.md)

# Called by

- [main](../../../functions/src/main.md)