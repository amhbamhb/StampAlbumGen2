---
type: Rust Function
title: download_missing_stamp_images
resource: src/image_downloader.rs#L159-L207
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/download_stamp_image_if_missing
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/best_effort_prepare_images_for_pdf
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn download_missing_stamp_images( stamps: &[crate::db_utils::StampStub], album_name: &str, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [resolve_album_image_path](../../../functions/src/file_utils/resolve_album_image_path.md)
- [download_stamp_image_if_missing](../../../functions/src/image_downloader/download_stamp_image_if_missing.md)

# Called by

- [best_effort_prepare_images_for_pdf](../../../functions/src/review_ui/http_handlers/best_effort_prepare_images_for_pdf.md)