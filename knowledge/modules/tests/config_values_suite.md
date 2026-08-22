---
type: Rust Module
title: config_values_suite
resource: tests/config_values_suite.rs#L1-L260
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-album-globals-generate-album-output-conn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-db-utils
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-file-utils
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-env
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
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

- [write_config](../../functions/tests/config_values_suite/write_config.md)
- [make_test_db_multi](../../functions/tests/config_values_suite/make_test_db_multi.md)
- [config_values_suite_iterates_and_changes_output](../../functions/tests/config_values_suite/config_values_suite_iterates_and_changes_output.md)

# Imports

- `stamp_album_gen2::album_globals::generate_album_output_conn`
- `stamp_album_gen2::db_utils`
- `stamp_album_gen2::file_utils`
- `std::env`
- `std::fs`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)