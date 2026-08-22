---
type: Rust Function
title: write_startup_probe
resource: src/setup.rs#L117-L125
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/startup_probe_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/write_startup_probe_snapshot
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_startup_probe(stage: &str, detail: &str)`

# Calls

- [formatted_now](../../../functions/src/time_utils/formatted_now.md)
- [startup_probe_path](../../../functions/src/setup/startup_probe_path.md)

# Called by

- [main](../../../functions/src/main.md)
- [write_startup_probe_snapshot](../../../functions/src/setup/write_startup_probe_snapshot.md)