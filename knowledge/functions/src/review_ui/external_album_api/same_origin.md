---
type: Rust Function
title: same_origin
resource: src/review_ui/external_album_api.rs#L160-L180
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/handle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn same_origin(request: &tiny_http::Request) -> Result<(), String>`

# Calls

- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [handle](../../../../functions/src/review_ui/external_album_api/handle.md)