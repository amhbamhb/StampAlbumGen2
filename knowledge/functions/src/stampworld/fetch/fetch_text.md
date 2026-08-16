---
type: Rust Function
title: fetch_text
resource: src/stampworld/fetch.rs#L131-L136
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn fetch_text(client: &Client, url: &str) -> Result<(StatusCode, String), reqwest::Error>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)