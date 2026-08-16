---
type: Rust Function
title: validate_year_range
resource: src/validation.rs#L79-L87
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn validate_year_range(start_year: i32, end_year: i32) -> Result<(), ValidationError>`

# Called by

- [main](../../../functions/src/main.md)