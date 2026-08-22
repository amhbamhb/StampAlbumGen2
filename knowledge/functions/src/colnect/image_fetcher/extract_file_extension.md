---
type: Rust Function
title: extract_file_extension
resource: src/colnect/image_fetcher.rs#L15-L23
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_file_extension(url: &str) -> &'static str`

# Called by

- [populate_missing_images_from_colnect_pages](../../../../functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages.md)