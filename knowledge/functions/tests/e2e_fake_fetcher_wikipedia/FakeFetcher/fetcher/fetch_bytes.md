---
type: Rust Method
title: fetch_bytes
resource: tests/e2e_fake_fetcher_wikipedia.rs#L24-L26
visibility: private
generated:
  by: okf-rs/0.4.0
---

# Signature

`fn fetch_bytes(&self, _url: &str) -> Result<(StatusCode, HeaderMap, Vec<u8>), reqwest::Error>`