---
type: Rust Method
title: fetch_bytes
resource: tests/db_path_unit.rs#L14-L23
visibility: private
generated:
  by: okf-rs/0.4.0
---

# Signature

`async fn fetch_bytes( &self, _url: &str, ) -> Result<(StatusCode, reqwest::header::HeaderMap, Vec<u8>), reqwest::Error>`