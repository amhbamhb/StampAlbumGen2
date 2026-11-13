---
type: Rust Function
title: pdf_valid
resource: src/external_album.rs#L1059-L1073
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/approve_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pdf_valid(path: &Path) -> Result<()>`

# Calls

- [no_links](../../../functions/src/external_album/no_links.md)
- [open](../../../functions/src/external_album/open.md)
- [err](../../../functions/src/external_album/err.md)

# Called by

- [render](../../../functions/src/external_album/render.md)
- [approve_album](../../../functions/src/external_album/approve_album.md)
- [edit_page](../../../functions/src/external_album/edit_page.md)