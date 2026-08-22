---
type: Rust Method
title: fetch_bytes
resource: src/stampworld/fetch.rs#L113-L119
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fetch_bytes(&self, url: &str) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error>`

# Calls

- [get](../../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)