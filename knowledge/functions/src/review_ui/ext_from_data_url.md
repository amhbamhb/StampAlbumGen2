---
type: Rust Function
title: ext_from_data_url
resource: src/review_ui.rs#L2768-L2787
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/store_uploaded_image_from_data_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ext_from_data_url(data_url: &str, fallback_name: &str) -> &'static str`

# Called by

- [store_uploaded_image_from_data_url](../../../functions/src/review_ui/store_uploaded_image_from_data_url.md)