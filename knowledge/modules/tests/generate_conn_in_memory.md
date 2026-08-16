---
type: Rust Module
title: generate_conn_in_memory
resource: tests/generate_conn_in_memory.rs#L1-L71
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/rusqlite-params
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

- [test_generate_conn_in_memory_clean](../../functions/tests/generate_conn_in_memory/test_generate_conn_in_memory_clean.md)

# Imports

- `rusqlite::params`
- `std::env`
- `std::fs`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)