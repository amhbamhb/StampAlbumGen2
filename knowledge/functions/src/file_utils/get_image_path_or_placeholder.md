---
type: Rust Function
title: get_image_path_or_placeholder
resource: src/file_utils.rs#L103-L114
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/file_utils/image_exists
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_image_path_or_placeholder(album_name: &str, image_path: &str) -> String`

# Calls

- [image_exists](../../../functions/src/file_utils/image_exists.md)