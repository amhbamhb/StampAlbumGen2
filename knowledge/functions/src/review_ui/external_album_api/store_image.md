---
type: Rust Function
title: store_image
resource: src/review_ui/external_album_api.rs#L304-L367
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/log_runtime_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/external_album_api/route
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/external_album_api/external_album_image_upload_preserves_original_and_rejects_invalid_data
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn store_image(album: &Path, data_url: &str) -> Result<String, String>`

# Calls

- [open](../../../../functions/src/external_album/open.md)
- [log_runtime_event](../../../../functions/src/review_ui/log_runtime_event.md)

# Called by

- [route](../../../../functions/src/review_ui/external_album_api/route.md)
- [external_album_image_upload_preserves_original_and_rejects_invalid_data](../../../../functions/src/review_ui/external_album_api/external_album_image_upload_preserves_original_and_rejects_invalid_data.md)