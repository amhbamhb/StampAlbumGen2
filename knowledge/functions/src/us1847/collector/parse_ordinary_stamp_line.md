---
type: Rust Function
title: parse_ordinary_stamp_line
resource: src/us1847/collector.rs#L863-L920
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
  called_by:
  - target: functions/src/us1847/collector/parse_html_tables
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_ordinary_stamp_line(line: &str) -> Option<StampInfo>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [parse_html_tables](../../../../functions/src/us1847/collector/parse_html_tables.md)