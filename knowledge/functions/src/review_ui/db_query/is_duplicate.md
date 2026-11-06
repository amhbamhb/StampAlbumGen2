---
type: Rust Function
title: is_duplicate
resource: src/review_ui/db_query.rs#L13-L22
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_duplicate(conn: &Connection, record: &PdfStampRecord) -> rusqlite::Result<bool>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)