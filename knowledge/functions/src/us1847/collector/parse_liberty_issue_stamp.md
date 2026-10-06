---
type: Rust Function
title: parse_liberty_issue_stamp
resource: src/us1847/collector.rs#L975-L1229
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/us1847/collector/extract_design_size
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/insert_stamp_if_not_exists
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/us1847/collector/process_1847us_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_liberty_issue_stamp( document: &Html, series_id: i64, series_title: &str, conn: &Connection, _base_url: &str, ) -> Result<usize, Box<dyn Error>>`

# Calls

- [extract_design_size](../../../../functions/src/us1847/collector/extract_design_size.md)
- [parse](../../../../functions/src/external_album/parser/parse.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [insert_stamp_if_not_exists](../../../../functions/src/us1847/collector/insert_stamp_if_not_exists.md)

# Called by

- [process_1847us_page](../../../../functions/src/us1847/collector/process_1847us_page.md)