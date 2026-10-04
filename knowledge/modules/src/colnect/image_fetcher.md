---
type: Rust Module
title: image_fetcher
resource: src/colnect/image_fetcher.rs#L1-L193
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/reqwest-client-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [Fetcher](../../../interfaces/src/colnect/image_fetcher/Fetcher.md)
- [ReqwestFetcher](../../../classes/src/colnect/image_fetcher/ReqwestFetcher.md)
- [default](../../../functions/src/colnect/image_fetcher/ReqwestFetcher/default/default.md)
- [new](../../../functions/src/colnect/image_fetcher/ReqwestFetcher/new.md)
- [new_for_images](../../../functions/src/colnect/image_fetcher/ReqwestFetcher/new_for_images.md)
- [new_for_api](../../../functions/src/colnect/image_fetcher/ReqwestFetcher/new_for_api.md)
- [client](../../../functions/src/colnect/image_fetcher/ReqwestFetcher/client.md)
- [fetch_text](../../../functions/src/colnect/image_fetcher/ReqwestFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../../functions/src/colnect/image_fetcher/ReqwestFetcher/fetcher/fetch_bytes.md)
- [build_client](../../../functions/src/colnect/image_fetcher/build_client.md)
- [build_image_client](../../../functions/src/colnect/image_fetcher/build_image_client.md)
- [fetch_text](../../../functions/src/colnect/image_fetcher/fetch_text.md)
- [fetch_bytes](../../../functions/src/colnect/image_fetcher/fetch_bytes.md)
- [resolve_colnect_image_url](../../../functions/src/colnect/image_fetcher/resolve_colnect_image_url.md)
- [test_fetch_text_invalid_url_err](../../../functions/src/colnect/image_fetcher/test_fetch_text_invalid_url_err.md)
- [test_fetch_bytes_invalid_url_err](../../../functions/src/colnect/image_fetcher/test_fetch_bytes_invalid_url_err.md)

# Imports

- `reqwest::{Client, StatusCode}`
- `std::time::Duration`
- `super::*`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)