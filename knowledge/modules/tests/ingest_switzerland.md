---
type: Rust Module
title: ingest_switzerland
resource: tests/ingest_switzerland.rs#L1-L27
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
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

- [ingest_switzerland_1972](../../functions/tests/ingest_switzerland/ingest_switzerland_1972.md)

# Imports

- `stamp_album_gen2::colnect::ingest::ingest_colnect_data`
- `stamp_album_gen2::colnect::parser::parse_csv`
- `stamp_album_gen2::db_utils::init_db`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)