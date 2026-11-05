---
type: Rust Function
title: route
resource: src/review_ui/external_album_api.rs#L182-L302
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/external_album_api/parameter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/body
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/start_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/retry_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/edit_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/load_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/asset_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/store_image
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/handle
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn route(request: &mut tiny_http::Request, action: &str) -> Result<Value, String>`

# Calls

- [parameter](../../../../functions/src/review_ui/external_album_api/parameter.md)
- [body](../../../../functions/src/review_ui/external_album_api/body.md)
- [start_job](../../../../functions/src/review_ui/external_album_api/start_job.md)
- [import_album](../../../../functions/src/external_album/import_album.md)
- [retry_album](../../../../functions/src/external_album/retry_album.md)
- [edit_page](../../../../functions/src/external_album/edit_page.md)
- [load_album](../../../../functions/src/external_album/load_album.md)
- [asset_directory](../../../../functions/src/external_album/asset_directory.md)
- [store_image](../../../../functions/src/review_ui/external_album_api/store_image.md)

# Called by

- [handle](../../../../functions/src/review_ui/external_album_api/handle.md)