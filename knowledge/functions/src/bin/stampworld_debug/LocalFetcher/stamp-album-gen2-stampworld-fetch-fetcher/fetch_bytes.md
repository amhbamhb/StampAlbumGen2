---
type: Rust Method
title: fetch_bytes
resource: src/bin/stampworld_debug.rs#L42-L49
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/stampworld/fetch/ReqwestFetcher/new_for_images
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fetch_bytes( &self, url: &str, ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error>`

# Calls

- [new_for_images](../../../../../../functions/src/stampworld/fetch/ReqwestFetcher/new_for_images.md)