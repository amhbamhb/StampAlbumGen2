---
type: Rust Function
title: populate_missing_images_from_colnect_pages
resource: src/colnect/image_fetcher.rs#L152-L342
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/fetch/build_client
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/extract_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/sanitize_for_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/extract_file_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/download_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn populate_missing_images_from_colnect_pages( conn: &Connection, album_name: &str, ) -> Result<ColnectImageFetchStats, Box<dyn std::error::Error>>`

# Calls

- [build_client](../../../../functions/src/stampworld/fetch/build_client.md)
- [extract_colnect_image_url](../../../../functions/src/colnect/image_fetcher/extract_colnect_image_url.md)
- [sanitize_for_filename](../../../../functions/src/colnect/image_fetcher/sanitize_for_filename.md)
- [extract_file_extension](../../../../functions/src/colnect/image_fetcher/extract_file_extension.md)
- [download_image](../../../../functions/src/image_downloader/download_image.md)