---
type: Rust Function
title: get_colnect_info_by_id
resource: src/colnect/db.rs#L211-L221
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/colnect/db/load_colnect_info_with_query
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/hydrate_catalog_codes
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_colnect_info_by_id(conn: &Connection, colnect_id: &str) -> Result<Option<ColnectInfo>>`

# Calls

- [load_colnect_info_with_query](../../../../functions/src/colnect/db/load_colnect_info_with_query.md)
- [hydrate_catalog_codes](../../../../functions/src/colnect/db/hydrate_catalog_codes.md)

# Called by

- [fetch_and_filter_stamps](../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)