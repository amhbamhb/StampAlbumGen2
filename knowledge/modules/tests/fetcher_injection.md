---
type: Rust Module
title: fetcher_injection
resource: tests/fetcher_injection.rs#L1-L60
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/reqwest-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-stampworld-fetch-fetcher
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [FakeFetcher](../../classes/tests/fetcher_injection/FakeFetcher.md)
- [fetch_text](../../functions/tests/fetcher_injection/FakeFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../functions/tests/fetcher_injection/FakeFetcher/fetcher/fetch_bytes.md)
- [test_collect_with_fake_fetcher_returns_empty](../../functions/tests/fetcher_injection/test_collect_with_fake_fetcher_returns_empty.md)

# Imports

- `reqwest::StatusCode`
- `stamp_album_gen2::stampworld::fetch::Fetcher`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)