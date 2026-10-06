---
type: Rust Function
title: parameter
resource: src/review_ui/external_album_api.rs#L146-L158
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/handle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parameter(url: &str, name: &str) -> Result<String, String>`

# Called by

- [route](../../../../functions/src/review_ui/external_album_api/route.md)
- [handle](../../../../functions/src/review_ui/external_album_api/handle.md)