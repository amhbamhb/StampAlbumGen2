---
type: Rust Module
title: page_break_overflow
resource: tests/page_break_overflow.rs#L1-L166
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/lazy-static-lazy-static
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-read
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-mutex
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

- [overflowing_row_moves_to_next_page_and_emits_row_start](../../functions/tests/page_break_overflow/overflowing_row_moves_to_next_page_and_emits_row_start.md)

# Imports

- `lazy_static::lazy_static`
- `std::fs`
- `std::io::Read`
- `std::sync::Mutex`
- `tempfile::tempdir`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)