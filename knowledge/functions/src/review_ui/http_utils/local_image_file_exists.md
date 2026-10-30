---
type: Rust Function
title: local_image_file_exists
resource: src/review_ui/http_utils.rs#L12-L15
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/resolve_album_image_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn local_image_file_exists(album_name: &str, image_path: &str) -> bool`

# Calls

- [resolve_album_image_path](../../../../functions/src/file_utils/resolve_album_image_path.md)