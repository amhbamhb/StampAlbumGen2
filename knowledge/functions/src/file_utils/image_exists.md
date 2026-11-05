---
type: Rust Function
title: image_exists
resource: src/file_utils.rs#L83-L91
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/file_utils/get_image_path_or_placeholder
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn image_exists(album_name: &str, image_path: &str) -> bool`

# Calls

- [resolve_album_image_path](../../../functions/src/file_utils/resolve_album_image_path.md)

# Called by

- [get_image_path_or_placeholder](../../../functions/src/file_utils/get_image_path_or_placeholder.md)