---
type: Rust Function
title: country_to_code
resource: src/flagdb.rs#L95-L128
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/load_flag_map_from_fs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/copy_flag_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_coa_for_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn country_to_code(country: &str) -> Option<String>`

# Calls

- [load_flag_map_from_fs](../../../functions/src/flagdb/load_flag_map_from_fs.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [copy_flag_for_album](../../../functions/src/flagdb/copy_flag_for_album.md)
- [copy_coa_for_album](../../../functions/src/flagdb/copy_coa_for_album.md)