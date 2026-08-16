---
type: Rust Function
title: validate_user_input
resource: src/validation.rs#L43-L76
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/runtime_defaults/default_end_year
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_user_input(cli: &crate::cli::Cli) -> Result<(), ValidationError>`

# Calls

- [default_end_year](../../../functions/src/runtime_defaults/default_end_year.md)

# Called by

- [main](../../../functions/src/main.md)