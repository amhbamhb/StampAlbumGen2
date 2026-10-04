---
type: Rust Function
title: process_1847us_page
resource: src/us1847/collector.rs#L43-L256
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_liberty_issue_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_html_tables
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_stamp_lists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/download_stamp_images
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/sync_us1847_to_stamps
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_html_tables
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn process_1847us_page( client: &Client, conn: &Connection, url: &str, opts: &Us1847PageOptions<'_>, ) -> Result<usize, Box<dyn Error>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [status](../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [parse_liberty_issue_stamp](../../../../functions/src/us1847/collector/parse_liberty_issue_stamp.md)
- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)
- [parse_stamp_lists](../../../../functions/src/us1847/collector/parse_stamp_lists.md)
- [download_stamp_images](../../../../functions/src/us1847/collector/download_stamp_images.md)
- [sync_us1847_to_stamps](../../../../functions/src/us1847/collector/sync_us1847_to_stamps.md)

# Called by

- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)