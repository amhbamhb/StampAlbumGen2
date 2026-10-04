---
type: Rust Function
title: best_effort_prepare_images_for_pdf
resource: src/review_ui/http_handlers.rs#L43-L141
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/http_handlers/local_image_file_exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/download_missing_stamp_images
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn best_effort_prepare_images_for_pdf(conn: &Connection, album_name: &str) -> (usize, usize)`

# Calls

- [local_image_file_exists](../../../../functions/src/review_ui/http_handlers/local_image_file_exists.md)
- [download_missing_stamp_images](../../../../functions/src/image_downloader/download_missing_stamp_images.md)

# Called by

- [serve_bootstrap_requests](../../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../../functions/src/review_ui/http_handlers/serve_review_requests.md)