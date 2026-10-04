---
type: Rust Function
title: parse_extra_flags
resource: src/review_ui.rs#L534-L542
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/build_stampworld_args
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_extra_flags(extra_flags: &Option<String>) -> Vec<String>`

# Called by

- [build_stampworld_args](../../../functions/src/review_ui/build_stampworld_args.md)