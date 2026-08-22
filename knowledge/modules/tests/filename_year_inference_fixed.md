---
type: Rust Module
title: filename_year_inference_fixed
resource: tests/filename_year_inference_fixed.rs#L1-L127
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/lazy-static-lazy-static
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-mutex
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [insert_minimal_stamp](../../functions/tests/filename_year_inference_fixed/insert_minimal_stamp.md)
- [filename_infers_db_min_max_when_none_fixed](../../functions/tests/filename_year_inference_fixed/filename_infers_db_min_max_when_none_fixed.md)
- [filename_uses_passed_start_and_db_max_when_end_missing_fixed](../../functions/tests/filename_year_inference_fixed/filename_uses_passed_start_and_db_max_when_end_missing_fixed.md)
- [filename_uses_db_min_and_passed_end_when_start_missing_fixed](../../functions/tests/filename_year_inference_fixed/filename_uses_db_min_and_passed_end_when_start_missing_fixed.md)

# Imports

- `lazy_static::lazy_static`
- `std::sync::Mutex`
- `tempfile`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)