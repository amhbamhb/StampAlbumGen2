---
type: Rust Function
title: root
resource: src/review_ui/external_album_api.rs#L70-L73
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/resolve_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/handle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn root() -> Result<PathBuf, String>`

# Calls

- [resolve_bootstrap_albums_root](../../../../functions/src/review_ui/resolve_bootstrap_albums_root.md)

# Called by

- [route](../../../../functions/src/review_ui/external_album_api/route.md)
- [handle](../../../../functions/src/review_ui/external_album_api/handle.md)