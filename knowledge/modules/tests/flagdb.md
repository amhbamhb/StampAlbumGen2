---
type: Rust Module
title: flagdb
resource: tests/flagdb.rs#L1-L71
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
- [iso_names_codes_and_existing_aliases_resolve_without_historical_guessing](../../functions/tests/flagdb/iso_names_codes_and_existing_aliases_resolve_without_historical_guessing.md)
- [unmapped_country_can_reuse_a_valid_album_local_flag_without_network](../../functions/tests/flagdb/unmapped_country_can_reuse_a_valid_album_local_flag_without_network.md)

# Imports

- `stamp_album_gen2::flagdb::{country_to_code, default_flag_map}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)