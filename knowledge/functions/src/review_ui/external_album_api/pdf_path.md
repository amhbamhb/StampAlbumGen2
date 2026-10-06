---
type: Rust Function
title: pdf_path
resource: src/review_ui/external_album_api.rs#L369-L392
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/handle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pdf_path(root: &Path, name: &str, which: &str) -> Result<PathBuf, String>`

# Calls

- [load_album](../../../../functions/src/external_album/load_album.md)

# Called by

- [handle](../../../../functions/src/review_ui/external_album_api/handle.md)