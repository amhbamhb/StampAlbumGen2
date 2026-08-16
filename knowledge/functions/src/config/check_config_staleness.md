---
type: Rust Function
title: check_config_staleness
resource: src/config.rs#L1435-L1449
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/extract_keys
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn check_config_staleness(user_config_content: &str) -> Vec<String>`

# Calls

- [extract_keys](../../../functions/src/config/extract_keys.md)

# Called by

- [main](../../../functions/src/main.md)