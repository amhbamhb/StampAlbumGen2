---
type: Rust Function
title: link_colnect_data
resource: src/colnect/ingest.rs#L169-L320
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/date/DateFormat/format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn link_colnect_data(conn: &Connection) -> Result<()>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [format](../../../../functions/src/native_pdf/date/DateFormat/format.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)