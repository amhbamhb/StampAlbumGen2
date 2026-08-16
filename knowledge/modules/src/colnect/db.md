---
type: Rust Module
title: db
resource: src/colnect/db.rs#L1-L221
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/rusqlite-connection-optionalextension-result
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ColnectInfo](../../../classes/src/colnect/db/ColnectInfo.md)
- [load_colnect_info_with_query](../../../functions/src/colnect/db/load_colnect_info_with_query.md)
- [hydrate_catalog_codes](../../../functions/src/colnect/db/hydrate_catalog_codes.md)
- [get_colnect_info](../../../functions/src/colnect/db/get_colnect_info.md)
- [get_colnect_info_by_id](../../../functions/src/colnect/db/get_colnect_info_by_id.md)

# Imports

- `rusqlite::{Connection, OptionalExtension, Result}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)