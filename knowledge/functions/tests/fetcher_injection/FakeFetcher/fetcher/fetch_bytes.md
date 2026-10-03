---
type: Rust Method
title: fetch_bytes
resource: tests/fetcher_injection.rs#L17-L24
visibility: private
generated:
  by: okf-rs/0.4.0
---

# Signature

`async fn fetch_bytes( &self, _url: &str, ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error>`