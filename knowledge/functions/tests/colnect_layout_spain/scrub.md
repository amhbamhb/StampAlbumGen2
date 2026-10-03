---
type: Rust Function
title: scrub
resource: tests/colnect_layout_spain.rs#L28-L45
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/colnect_layout_spain/sanitized_config
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn scrub(value: &mut toml::Value)`

# Called by

- [sanitized_config](../../../functions/tests/colnect_layout_spain/sanitized_config.md)