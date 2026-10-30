---
type: Rust Function
title: ensure_placeholder_for_stamp_and_update_db
resource: src/album_globals/generator_new/helpers.rs#L948-L1012
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/update_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_placeholder_for_stamp_and_update_db( conn: &Connection, opts: &PlaceholderCallOptions, mut writer: Option<&mut dyn Write>, ) -> Option<(String, i32, i32)>`

# Calls

- [ensure_placeholder_image_for_stamp_with_hint](../../../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)
- [update_image_dimensions](../../../../../functions/src/db_utils/updates/update_image_dimensions.md)

# Called by

- [generate_album_output_conn_prepared](../../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [test_ensure_placeholder_for_stamp_and_update_db](../../../../../functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db.md)