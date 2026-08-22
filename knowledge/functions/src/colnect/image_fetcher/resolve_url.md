---
type: Rust Function
title: resolve_url
resource: src/colnect/image_fetcher.rs#L54-L64
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

`fn resolve_url(base: &str, candidate: &str) -> Option<String>`

# Called by

- [extract_colnect_image_url](../../../../functions/src/colnect/image_fetcher/extract_colnect_image_url.md)