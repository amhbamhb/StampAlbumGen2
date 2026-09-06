---
type: Rust Function
title: parse_stamp_lists
resource: src/us1847/collector.rs#L550-L558
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_stamp_lists( _conn: &Connection, _document: &Html, _series_id: i64, _series_title: &str, ) -> Result<usize, Box<dyn Error>>`

# Called by

- [process_1847us_page](../../../../functions/src/us1847/collector/process_1847us_page.md)