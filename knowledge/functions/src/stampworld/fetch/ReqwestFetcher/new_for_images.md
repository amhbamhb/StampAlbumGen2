---
type: Rust Method
title: new_for_images
resource: src/stampworld/fetch.rs#L57-L78
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/bin/stampworld_debug/LocalFetcher/stamp-album-gen2-stampworld-fetch-fetcher/fetch_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/fetch/build_image_client
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn new_for_images() -> Self`

# Called by

- [fetch_bytes](../../../../../functions/src/bin/stampworld_debug/LocalFetcher/stamp-album-gen2-stampworld-fetch-fetcher/fetch_bytes.md)
- [build_image_client](../../../../../functions/src/stampworld/fetch/build_image_client.md)