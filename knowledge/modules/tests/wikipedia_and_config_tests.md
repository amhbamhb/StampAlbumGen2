---
type: Rust Module
title: wikipedia_and_config_tests
resource: tests/wikipedia_and_config_tests.rs#L1-L116
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-tempdir
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-params-connection
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [config_template_is_copied_to_config_toml_when_missing](../../functions/tests/wikipedia_and_config_tests/config_template_is_copied_to_config_toml_when_missing.md)
- [wikipedia_ingest_inserts_up_to_max_entries_from_cache](../../functions/tests/wikipedia_and_config_tests/wikipedia_ingest_inserts_up_to_max_entries_from_cache.md)

# Imports

- `std::fs`
- `tempfile::tempdir`
- `rusqlite::{params, Connection}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)