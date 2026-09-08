---
type: Rust Function
title: file_sha256
resource: src/modes.rs#L160-L179
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn file_sha256(path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>>`

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)