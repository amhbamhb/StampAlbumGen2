---
type: Rust Module
title: flagdb
resource: tests/flagdb.rs#L1-L21
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/stamp-album-gen2-flagdb-country-to-code-default-flag-map
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [test_load_builtin_map](../../functions/tests/flagdb/test_load_builtin_map.md)
- [test_country_to_code_normalization](../../functions/tests/flagdb/test_country_to_code_normalization.md)
- [test_missing_country_returns_none](../../functions/tests/flagdb/test_missing_country_returns_none.md)

# Imports

- `stamp_album_gen2::flagdb::{country_to_code, default_flag_map}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)