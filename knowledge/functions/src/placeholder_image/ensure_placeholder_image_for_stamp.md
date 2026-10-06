---
type: Rust Function
title: ensure_placeholder_image_for_stamp
resource: src/placeholder_image.rs#L505-L592
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/placeholder_image/draw_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_placeholder_image_for_stamp( album_name: &str, catalog_number: &str, year: &str, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [draw_text](../../../functions/src/placeholder_image/draw_text.md)
- [save](../../../functions/src/review_ui/views/external_album/save.md)