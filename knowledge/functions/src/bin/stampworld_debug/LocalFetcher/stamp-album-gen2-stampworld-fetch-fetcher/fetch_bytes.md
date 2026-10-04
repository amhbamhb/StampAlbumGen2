---
type: Rust Method
title: fetch_bytes
resource: src/bin/stampworld_debug.rs#L43-L50
visibility: private
generated:
  by: okf-rs/0.4.0
---

# Signature

`async fn fetch_bytes( &self, url: &str, ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error>`