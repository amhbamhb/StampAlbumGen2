---
type: Rust Function
title: process_db
resource: src/bin/backfill_image_dims.rs#L27-L82
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/resolve_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/update_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/bin/backfill_image_dims/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn process_db(db_path: &Path, apply: bool) -> Result<usize, Box<dyn std::error::Error>>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [resolve_dimensions](../../../../functions/src/bin/backfill_image_dims/resolve_dimensions.md)
- [update_image_dimensions](../../../../functions/src/db_utils/updates/update_image_dimensions.md)

# Called by

- [main](../../../../functions/src/bin/backfill_image_dims/main.md)