---
type: Rust Function
title: insert_stamp_if_not_exists
resource: src/us1847/collector.rs#L1308-L1444
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/us1847/collector/parse_html_tables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_liberty_issue_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn insert_stamp_if_not_exists( conn: &Connection, series_id: i64, _series_title: &str, stamp: &StampInfo, ) -> Result<(), Box<dyn Error>>`

# Called by

- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)
- [parse_liberty_issue_stamp](../../../../functions/src/us1847/collector/parse_liberty_issue_stamp.md)