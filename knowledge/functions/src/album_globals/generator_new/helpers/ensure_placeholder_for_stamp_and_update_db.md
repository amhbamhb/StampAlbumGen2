---
type: Rust Function
title: ensure_placeholder_for_stamp_and_update_db
resource: src/album_globals/generator_new/helpers.rs#L887-L951
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
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

# Called by

- [generate_album_output_conn](../../../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [test_ensure_placeholder_for_stamp_and_update_db](../../../../../functions/tests/placeholder_helper_test/test_ensure_placeholder_for_stamp_and_update_db.md)