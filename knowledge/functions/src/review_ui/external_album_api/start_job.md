---
type: Rust Function
title: start_job
resource: src/review_ui/external_album_api.rs#L90-L144
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn start_job<F>(work: F) -> Result<Value, String> where F: FnOnce() -> Result<external_album::AlbumDetails, String> + Send + 'static,`

# Calls

- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [route](../../../../functions/src/review_ui/external_album_api/route.md)