---
type: Rust Function
title: build_image_client
resource: src/stampworld/fetch.rs#L129-L131
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/fetch/ReqwestFetcher/new_for_images
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/ReqwestFetcher/client
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/image_downloader/download_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_image_client() -> Client`

# Calls

- [new_for_images](../../../../functions/src/stampworld/fetch/ReqwestFetcher/new_for_images.md)
- [client](../../../../functions/src/stampworld/fetch/ReqwestFetcher/client.md)

# Called by

- [download_image](../../../../functions/src/image_downloader/download_image.md)