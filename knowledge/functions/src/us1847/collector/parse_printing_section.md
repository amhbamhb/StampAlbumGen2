---
type: Rust Function
title: parse_printing_section
resource: src/us1847/collector.rs#L544-L563
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/us1847/collector/parse_html_tables
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_printing_section(title: &str) -> (String, String, String)`

# Called by

- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)