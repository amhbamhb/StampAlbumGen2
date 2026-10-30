---
type: Rust Function
title: validate_config
resource: src/review/config_api.rs#L11-L31
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_config(config: &HashMap<String, String>) -> Result<(), Vec<String>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)