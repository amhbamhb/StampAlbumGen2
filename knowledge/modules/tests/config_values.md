---
type: Rust Module
title: config_values
resource: tests/config_values.rs#L1-L113
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-album-globals-generate-album-output-conn
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

- [write_config](../../functions/tests/config_values/write_config.md)
- [make_test_db](../../functions/tests/config_values/make_test_db.md)
- [read_stamp_heading_size](../../functions/tests/config_values/read_stamp_heading_size.md)
- [test_stamp_heading_size_changes_output](../../functions/tests/config_values/test_stamp_heading_size_changes_output.md)

# Imports

- `stamp_album_gen2::album_globals::generate_album_output_conn`
- `stamp_album_gen2::file_utils`
- `std::env`
- `std::fs`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)