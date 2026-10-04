---
type: Rust Module
title: fetch
resource: src/stampworld/fetch.rs#L1-L169
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-client-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-duration
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

# Imports

- `async_trait::async_trait`
- `reqwest::{Client, StatusCode}`
- `std::time::Duration`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)