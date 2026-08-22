---
type: Rust Function
title: clean_name
resource: src/colnect/parser.rs#L10-L20
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/colnect/parser/parse_csv
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn clean_name(name: &str) -> String`

# Called by

- [parse_csv](../../../../functions/src/colnect/parser/parse_csv.md)