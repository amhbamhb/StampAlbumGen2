---
type: Rust Function
title: validate_source
resource: src/validation.rs#L90-L169
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_source(cli: &crate::cli::Cli) -> Result<String, ValidationError>`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [main](../../../functions/src/main.md)