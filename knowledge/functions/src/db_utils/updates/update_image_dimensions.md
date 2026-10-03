---
type: Rust Function
title: update_image_dimensions
resource: src/db_utils/updates.rs#L8-L18
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/backfill_image_dims/process_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/persist_image_dimensions_from_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/backfill_image_dims/backfill_image_dimensions_persists_to_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn update_image_dimensions( conn: &Connection, country: &str, series: &str, sg_number: &str, width: Option<i32>, height: Option<i32>, ) -> Result<usize>`

# Called by

- [estimate_and_ensure_placeholders](../../../../functions/src/album_globals/generator_new/helpers/estimate_and_ensure_placeholders.md)
- [ensure_placeholder_for_stamp_and_update_db](../../../../functions/src/album_globals/generator_new/helpers/ensure_placeholder_for_stamp_and_update_db.md)
- [process_db](../../../../functions/src/bin/backfill_image_dims/process_db.md)
- [persist_image_dimensions_from_file](../../../../functions/src/db_utils/updates/persist_image_dimensions_from_file.md)
- [backfill_image_dimensions_persists_to_db](../../../../functions/tests/backfill_image_dims/backfill_image_dimensions_persists_to_db.md)