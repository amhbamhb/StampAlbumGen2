---
type: Rust Function
title: first_run_decision
resource: src/setup.rs#L927-L944
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/setup/ensure_first_run_installation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn first_run_decision( config_exists: bool, albums_exists: bool, exe_dir: &Path, default_dest: &Path, ) -> FirstRunDecision`

# Called by

- [ensure_first_run_installation](../../../functions/src/setup/ensure_first_run_installation.md)