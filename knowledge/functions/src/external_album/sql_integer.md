---
type: Rust Function
title: sql_integer
resource: src/external_album.rs#L79-L81
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/persist_pages
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sql_integer(value: usize) -> Result<i64>`

# Calls

- [err](../../../functions/src/external_album/err.md)

# Called by

- [persist_pages](../../../functions/src/external_album/persist_pages.md)