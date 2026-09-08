---
type: Rust Function
title: parse_table_stamps
resource: src/us1847/collector.rs#L646-L698
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/us1847/collector/parse_stamp_from_title_and_description
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/us1847/collector/parse_html_tables
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_table_stamps( table_elem: &scraper::element_ref::ElementRef, printer: &str, perforation: &str, printing_details: &str, stamp_type: &str, ) -> Vec<StampInfo>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [parse_stamp_from_title_and_description](../../../../functions/src/us1847/collector/parse_stamp_from_title_and_description.md)

# Called by

- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)