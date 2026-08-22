---
type: Rust Module
title: e2e_fake_fetcher_wikipedia
resource: tests/e2e_fake_fetcher_wikipedia.rs#L1-L39
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/reqwest-header-headermap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/reqwest-statuscode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-stampworld-fetch-fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-wikipedia
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [FakeFetcher](../../classes/tests/e2e_fake_fetcher_wikipedia/FakeFetcher.md)
- [new](../../functions/tests/e2e_fake_fetcher_wikipedia/FakeFetcher/new.md)
- [fetch_text](../../functions/tests/e2e_fake_fetcher_wikipedia/FakeFetcher/fetcher/fetch_text.md)
- [fetch_bytes](../../functions/tests/e2e_fake_fetcher_wikipedia/FakeFetcher/fetcher/fetch_bytes.md)
- [e2e_wikipedia_fetcher_fake](../../functions/tests/e2e_fake_fetcher_wikipedia/e2e_wikipedia_fetcher_fake.md)

# Imports

- `reqwest::header::HeaderMap`
- `reqwest::StatusCode`
- `stamp_album_gen2::stampworld::fetch::Fetcher`
- `stamp_album_gen2::wikipedia`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)