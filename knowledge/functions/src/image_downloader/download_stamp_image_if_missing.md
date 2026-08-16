---
type: Rust Function
title: download_stamp_image_if_missing
resource: src/image_downloader.rs#L114-L149
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/download_image
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/image_downloader/download_missing_stamp_images
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn download_stamp_image_if_missing( image_path: &str, remote_url: &str, album_name: &str, sg: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [resolve_album_image_path](../../../functions/src/file_utils/resolve_album_image_path.md)
- [download_image](../../../functions/src/image_downloader/download_image.md)

# Called by

- [download_missing_stamp_images](../../../functions/src/image_downloader/download_missing_stamp_images.md)