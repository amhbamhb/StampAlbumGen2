---
type: Rust Function
title: estimate_and_ensure_placeholders
resource: src/album_globals/generator_new/helpers.rs#L733-L922
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/image_utils/estimate_real_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/updates/update_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn estimate_and_ensure_placeholders( conn: &Connection, opts: &PlaceholderOptions<'_>, stamps: &mut [Stamp], ) -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [resolve_album_image_path](../../../../../functions/src/file_utils/resolve_album_image_path.md)
- [estimate_real_dimensions](../../../../../functions/src/image_utils/estimate_real_dimensions.md)
- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [update_image_dimensions](../../../../../functions/src/db_utils/updates/update_image_dimensions.md)
- [ensure_placeholder_image_for_stamp_with_hint](../../../../../functions/src/placeholder_image/ensure_placeholder_image_for_stamp_with_hint.md)