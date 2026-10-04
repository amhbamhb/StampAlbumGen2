---
type: Rust Method
title: fetch_bytes
resource: tests/flag_pipeline.rs#L36-L42
visibility: private
generated:
  by: okf-rs/0.4.0
---

# Signature

`async fn fetch_bytes( &self, url: &str, ) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error>`