---
type: Rust Function
title: build_client
resource: src/stampworld/fetch.rs#L125-L127
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/fetch/ReqwestFetcher/client
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/colnect/image_fetcher/resolve_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn build_client() -> Client`

# Calls

- [client](../../../../functions/src/stampworld/fetch/ReqwestFetcher/client.md)

# Called by

- [resolve_colnect_image_url](../../../../functions/src/colnect/image_fetcher/resolve_colnect_image_url.md)
- [populate_missing_images_from_colnect_pages](../../../../functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages.md)