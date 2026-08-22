---
type: Rust Function
title: is_likely_stamp_image
resource: src/colnect/image_fetcher.rs#L37-L52
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/colnect/image_fetcher/extract_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_likely_stamp_image(url: &str) -> bool`

# Called by

- [extract_colnect_image_url](../../../../functions/src/colnect/image_fetcher/extract_colnect_image_url.md)