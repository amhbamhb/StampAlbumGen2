---
type: Rust Function
title: backfill_image_dimensions_persists_to_db
resource: tests/backfill_image_dims.rs#L6-L80
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/get_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/update_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn backfill_image_dimensions_persists_to_db() -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [get_image_dimensions](../../../functions/src/image_utils/get_image_dimensions.md)
- [update_image_dimensions](../../../functions/src/db_utils/updates/update_image_dimensions.md)