---
type: Rust Function
title: ensure_placeholder_image_for_album
resource: src/placeholder_image.rs#L466-L501
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/generate_placeholder_image_with_options
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/placeholder_image/ensure_placeholder_image
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_placeholder_image_for_album( album_name: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [generate_placeholder_image_with_options](../../../functions/src/placeholder_image/generate_placeholder_image_with_options.md)

# Called by

- [ensure_placeholder_image](../../../functions/src/placeholder_image/ensure_placeholder_image.md)