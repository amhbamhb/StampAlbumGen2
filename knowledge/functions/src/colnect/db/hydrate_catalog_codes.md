---
type: Rust Function
title: hydrate_catalog_codes
resource: src/colnect/db.rs#L98-L112
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

`fn hydrate_catalog_codes( conn: &Connection, colnect_id: String, mut info: ColnectInfo, ) -> Result<Option<ColnectInfo>>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [get_colnect_info](../../../../functions/src/colnect/db/get_colnect_info.md)
- [get_colnect_info_by_id](../../../../functions/src/colnect/db/get_colnect_info_by_id.md)