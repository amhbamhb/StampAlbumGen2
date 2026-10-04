---
type: Rust Module
title: album_year_filter
resource: tests/album_year_filter.rs#L1-L142
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-album-globals-generate-album-output-conn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-db-utils-init-db
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [FixtureStamp](../../classes/tests/album_year_filter/FixtureStamp.md)
- [insert_fixture_stamp](../../functions/tests/album_year_filter/insert_fixture_stamp.md)
- [populate_test_db](../../functions/tests/album_year_filter/populate_test_db.md)
- [test_generate_album_filters_by_year_exact](../../functions/tests/album_year_filter/test_generate_album_filters_by_year_exact.md)
- [test_generate_album_filters_by_year_open_ended](../../functions/tests/album_year_filter/test_generate_album_filters_by_year_open_ended.md)

# Imports

- `rusqlite::Connection`
- `stamp_album_gen2::album_globals::generate_album_output_conn`
- `stamp_album_gen2::db_utils::init_db`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)