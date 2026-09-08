---
type: Rust Module
title: fetch
resource: src/stampworld/fetch.rs#L1-L168
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/reqwest-blocking-client
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-header-headermap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-statuscode
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

- [Fetcher](../../../interfaces/src/stampworld/fetch/Fetcher.md)
- [ReqwestFetcher](../../../classes/src/stampworld/fetch/ReqwestFetcher.md)
- [default](../../../functions/src/stampworld/fetch/ReqwestFetcher/default/default.md)
- [new](../../../functions/src/stampworld/fetch/ReqwestFetcher/new.md)
- [new_for_images](../../../functions/src/stampworld/fetch/ReqwestFetcher/new_for_images.md)
- [new_for_api](../../../functions/src/stampworld/fetch/ReqwestFetcher/new_for_api.md)
- [client](../../../functions/src/stampworld/fetch/ReqwestFetcher/client.md)
- [fetch_text](../../../functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../../functions/src/stampworld/fetch/ReqwestFetcher/fetcher/fetch_bytes.md)
- [build_client](../../../functions/src/stampworld/fetch/build_client.md)
- [build_image_client](../../../functions/src/stampworld/fetch/build_image_client.md)
- [fetch_text](../../../functions/src/stampworld/fetch/fetch_text.md)
- [fetch_bytes](../../../functions/src/stampworld/fetch/fetch_bytes.md)
- [test_fetch_text_invalid_url_err](../../../functions/src/stampworld/fetch/test_fetch_text_invalid_url_err.md)
- [test_fetch_bytes_invalid_url_err](../../../functions/src/stampworld/fetch/test_fetch_bytes_invalid_url_err.md)

# Imports

- `reqwest::blocking::Client`
- `reqwest::header::HeaderMap`
- `reqwest::StatusCode`
- `super::*`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)