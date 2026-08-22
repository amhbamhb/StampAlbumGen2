---
type: Rust Function
title: local_image_file_exists
resource: src/review_ui/http_handlers.rs#L23-L25
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/best_effort_prepare_images_for_pdf
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn local_image_file_exists(album_name: &str, image_path: &str) -> bool`

# Calls

- [resolve_album_image_path](../../../../functions/src/file_utils/resolve_album_image_path.md)

# Called by

- [best_effort_prepare_images_for_pdf](../../../../functions/src/review_ui/http_handlers/best_effort_prepare_images_for_pdf.md)