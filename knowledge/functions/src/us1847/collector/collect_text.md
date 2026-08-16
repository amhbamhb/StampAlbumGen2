---
type: Rust Function
title: collect_text
resource: src/us1847/collector.rs#L276-L284
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

`fn collect_text(node: &scraper::element_ref::ElementRef) -> String`

# Called by

- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)