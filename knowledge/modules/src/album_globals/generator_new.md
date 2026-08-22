---
type: Rust Module
title: generator_new
resource: src/album_globals/generator_new/mod.rs#L1-L100
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/log-warn
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-connection
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rusqlite-result-as-sqlresult
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-db
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [generate_album_output_conn](../../../functions/src/album_globals/generator_new/generate_album_output_conn.md)
- [list_series](../../../functions/src/album_globals/generator_new/list_series.md)
- [for_each_series](../../../functions/src/album_globals/generator_new/for_each_series.md)
- [generate_album_output](../../../functions/src/album_globals/generator_new/generate_album_output.md)

# Imports

- `log::warn`
- `rusqlite::Connection`
- `rusqlite::Result as SqlResult`
- `stamp_album_db`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)