---
type: Rust Function
title: safe_filename_token
resource: src/review_ui.rs#L2750-L2766
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

`fn safe_filename_token(raw: &str) -> String`

# Called by

- [store_uploaded_image_from_data_url](../../../functions/src/review_ui/store_uploaded_image_from_data_url.md)