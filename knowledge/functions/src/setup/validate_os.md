---
type: Rust Function
title: validate_os
resource: src/setup.rs#L170-L176
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/get_operating_system_type
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_os() -> Result<OSType, SetupError>`

# Calls

- [get_operating_system_type](../../../functions/src/setup/get_operating_system_type.md)

# Called by

- [main](../../../functions/src/main.md)