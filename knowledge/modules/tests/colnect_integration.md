---
type: Rust Module
title: colnect_integration
resource: tests/colnect_integration.rs#L1-L174
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-colnect-db-get-colnect-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-colnect-ingest-ingest-colnect-data
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-colnect-parser-parse-csv
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-db-utils-init-db
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [setup_colnect_db](../../functions/tests/colnect_integration/setup_colnect_db.md)
- [test_colnect_integration](../../functions/tests/colnect_integration/test_colnect_integration.md)
- [test_exact_match_returns_correct_stamp](../../functions/tests/colnect_integration/test_exact_match_returns_correct_stamp.md)
- [test_nondgit_suffix_variant_is_accepted](../../functions/tests/colnect_integration/test_nondgit_suffix_variant_is_accepted.md)
- [test_digit_suffix_variant_is_rejected](../../functions/tests/colnect_integration/test_digit_suffix_variant_is_rejected.md)
- [test_only_genuine_variant_chosen_when_mixed](../../functions/tests/colnect_integration/test_only_genuine_variant_chosen_when_mixed.md)
- [test_star_suffix_variant_is_accepted](../../functions/tests/colnect_integration/test_star_suffix_variant_is_accepted.md)
- [test_no_match_returns_none](../../functions/tests/colnect_integration/test_no_match_returns_none.md)

# Imports

- `rusqlite::Connection`
- `stamp_album_gen2::colnect::db::get_colnect_info`
- `stamp_album_gen2::colnect::ingest::ingest_colnect_data`
- `stamp_album_gen2::colnect::parser::parse_csv`
- `stamp_album_gen2::db_utils::init_db`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)