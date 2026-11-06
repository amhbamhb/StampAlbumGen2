---
type: Rust Function
title: parse_stamp_from_title_and_description
resource: src/us1847/collector.rs#L684-L861
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/us1847/collector/parse_table_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_stamp_from_title_and_description( title: &str, description: &str, printer: &str, perforation: &str, printing_details: &str, stamp_type: &str, ) -> Option<StampInfo>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [parse_table_stamps](../../../../functions/src/us1847/collector/parse_table_stamps.md)