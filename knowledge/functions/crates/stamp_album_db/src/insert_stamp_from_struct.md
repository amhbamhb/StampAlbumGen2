---
type: Rust Function
title: insert_stamp_from_struct
resource: crates/stamp_album_db/src/lib.rs#L9-L420
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_stampworld_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_colnect_page_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_real_image_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/cleanup_stampworld_identity_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/crates/stamp_album_db/src/is_placeholder_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn insert_stamp_from_struct(conn: &Connection, si: &StampInsert) -> Result<bool>`

# Calls

- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [is_stampworld_image_url](../../../../functions/crates/stamp_album_db/src/is_stampworld_image_url.md)
- [is_colnect_page_url](../../../../functions/crates/stamp_album_db/src/is_colnect_page_url.md)
- [is_colnect_image_url](../../../../functions/crates/stamp_album_db/src/is_colnect_image_url.md)
- [is_real_image_candidate](../../../../functions/crates/stamp_album_db/src/is_real_image_candidate.md)
- [cleanup_stampworld_identity_duplicates](../../../../functions/crates/stamp_album_db/src/cleanup_stampworld_identity_duplicates.md)
- [is_placeholder_image_path](../../../../functions/crates/stamp_album_db/src/is_placeholder_image_path.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [parse](../../../../functions/src/external_album/parser/parse.md)