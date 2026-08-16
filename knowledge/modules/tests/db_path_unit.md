---
type: Rust Module
title: db_path_unit
resource: tests/db_path_unit.rs#L1-L81
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
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [FakeFetcherEmpty](../../classes/tests/db_path_unit/FakeFetcherEmpty.md)
- [fetch_text](../../functions/tests/db_path_unit/FakeFetcherEmpty/fetcher/fetch_text.md)
- [fetch_bytes](../../functions/tests/db_path_unit/FakeFetcherEmpty/fetcher/fetch_bytes.md)
- [collector_creates_db_at_expected_path](../../functions/tests/db_path_unit/collector_creates_db_at_expected_path.md)

# Imports

- `reqwest::StatusCode`
- `stamp_album_gen2::stampworld::fetch::Fetcher`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)