---
type: Rust Function
title: load_colnect_info_with_query
resource: src/colnect/db.rs#L44-L96
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/colnect/db/get_colnect_info
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/get_colnect_info_by_id
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn load_colnect_info_with_query( conn: &Connection, sql: &str, param: &str, ) -> Result<Option<(String, ColnectInfo)>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [get_colnect_info](../../../../functions/src/colnect/db/get_colnect_info.md)
- [get_colnect_info_by_id](../../../../functions/src/colnect/db/get_colnect_info_by_id.md)