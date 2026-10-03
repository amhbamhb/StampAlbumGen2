---
type: Rust Function
title: extract_keys
resource: src/config.rs#L1626-L1631
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/config/check_config_staleness
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_keys(content: &str) -> std::collections::HashSet<String>`

# Called by

- [check_config_staleness](../../../functions/src/config/check_config_staleness.md)