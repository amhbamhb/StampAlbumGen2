---
type: Rust Function
title: download_image
resource: src/image_downloader.rs#L19-L103
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/fetch/build_image_client
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_downloader/download_stamp_image_if_missing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn download_image( url: &str, local_path: &Path, sg: &str, ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [build_image_client](../../../functions/src/stampworld/fetch/build_image_client.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [populate_missing_images_from_colnect_pages](../../../functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages.md)
- [download_stamp_image_if_missing](../../../functions/src/image_downloader/download_stamp_image_if_missing.md)