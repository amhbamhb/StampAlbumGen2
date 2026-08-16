---
type: Rust Function
title: clean_perforation
resource: src/colnect/parser.rs#L24-L34
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

`fn clean_perforation(perf: &str) -> String`

# Called by

- [parse_csv](../../../../functions/src/colnect/parser/parse_csv.md)