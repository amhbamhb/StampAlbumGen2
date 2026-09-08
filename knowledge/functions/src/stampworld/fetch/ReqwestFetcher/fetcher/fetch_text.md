---
type: Rust Method
title: fetch_text
resource: src/stampworld/fetch.rs#L108-L113
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

`fn fetch_text(&self, url: &str) -> Result<(StatusCode, String), reqwest::Error>`

# Calls

- [get](../../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)