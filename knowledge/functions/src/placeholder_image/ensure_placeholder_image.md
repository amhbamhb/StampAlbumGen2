---
type: Rust Function
title: ensure_placeholder_image
resource: src/placeholder_image.rs#L814-L817
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/ensure_placeholder_image_for_album
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/placeholder_image/get_placeholder_image_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_placeholder_image() -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [ensure_placeholder_image_for_album](../../../functions/src/placeholder_image/ensure_placeholder_image_for_album.md)

# Called by

- [get_placeholder_image_path](../../../functions/src/placeholder_image/get_placeholder_image_path.md)