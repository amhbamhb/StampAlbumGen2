---
type: Rust Function
title: parse_html_tables
resource: src/us1847/collector.rs#L258-L532
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/us1847/collector/collect_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_table_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_single_image_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/insert_stamp_if_not_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_ordinary_stamp_line
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_printing_section
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_commemorative_line
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_html_tables( client: &Client, conn: &Connection, document: &Html, series_id: i64, series_title: &str, opts: &Us1847PageOptions<'_>, ) -> Result<usize, Box<dyn Error>>`

# Calls

- [collect_text](../../../../functions/src/us1847/collector/collect_text.md)
- [parse_table_stamps](../../../../functions/src/us1847/collector/parse_table_stamps.md)
- [parse_single_image_stamp](../../../../functions/src/us1847/collector/parse_single_image_stamp.md)
- [insert_stamp_if_not_exists](../../../../functions/src/us1847/collector/insert_stamp_if_not_exists.md)
- [process_1847us_page](../../../../functions/src/us1847/collector/process_1847us_page.md)
- [parse_ordinary_stamp_line](../../../../functions/src/us1847/collector/parse_ordinary_stamp_line.md)
- [parse_printing_section](../../../../functions/src/us1847/collector/parse_printing_section.md)
- [parse_commemorative_line](../../../../functions/src/us1847/collector/parse_commemorative_line.md)

# Called by

- [process_1847us_page](../../../../functions/src/us1847/collector/process_1847us_page.md)