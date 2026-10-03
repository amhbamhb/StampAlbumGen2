---
type: Rust Method
title: fetch_bytes
resource: tests/e2e_fake_fetcher_wikipedia.rs#L26-L31
visibility: private
generated:
  by: okf-rs/0.4.0
---

# Signature

`async fn fetch_bytes( &self, _url: &str, ) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error>`