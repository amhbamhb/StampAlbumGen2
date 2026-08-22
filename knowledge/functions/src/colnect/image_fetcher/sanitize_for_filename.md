---
type: Rust Function
title: sanitize_for_filename
resource: src/colnect/image_fetcher.rs#L25-L35
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

`fn sanitize_for_filename(v: &str) -> String`

# Called by

- [populate_missing_images_from_colnect_pages](../../../../functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages.md)