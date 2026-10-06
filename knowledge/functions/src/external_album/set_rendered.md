---
type: Rust Function
title: set_rendered
resource: src/external_album.rs#L1180-L1185
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn set_rendered(conn: &Connection, rendered: &Rendered) -> Result<()>`

# Calls

- [err](../../../functions/src/external_album/err.md)

# Called by

- [import_album](../../../functions/src/external_album/import_album.md)
- [retry_album](../../../functions/src/external_album/retry_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)