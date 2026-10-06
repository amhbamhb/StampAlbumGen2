---
type: Rust Function
title: generate_transparent_spacer
resource: src/placeholder_image.rs#L188-L212
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn generate_transparent_spacer( album_name: &str, page: usize, row: usize, px_w: u32, px_h: u32, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [save](../../../functions/src/review_ui/views/external_album/save.md)