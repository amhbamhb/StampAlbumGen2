---
type: Rust Function
title: extract_colnect_image_url
resource: src/colnect/image_fetcher.rs#L66-L144
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/colnect/image_fetcher/resolve_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/is_likely_stamp_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/colnect/image_fetcher/resolve_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_colnect_image_url(page_url: &str, html: &str) -> Option<String>`

# Calls

- [resolve_url](../../../../functions/src/colnect/image_fetcher/resolve_url.md)
- [is_likely_stamp_image](../../../../functions/src/colnect/image_fetcher/is_likely_stamp_image.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [resolve_colnect_image_url](../../../../functions/src/colnect/image_fetcher/resolve_colnect_image_url.md)
- [populate_missing_images_from_colnect_pages](../../../../functions/src/colnect/image_fetcher/populate_missing_images_from_colnect_pages.md)