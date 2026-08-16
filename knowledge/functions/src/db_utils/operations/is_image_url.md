---
type: Rust Function
title: is_image_url
resource: src/db_utils/operations.rs#L61-L71
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/db_utils/operations/is_stampworld_image_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/is_colnect_image_url
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_image_url(url: &str) -> bool`

# Called by

- [is_stampworld_image_url](../../../../functions/src/db_utils/operations/is_stampworld_image_url.md)
- [is_colnect_image_url](../../../../functions/src/db_utils/operations/is_colnect_image_url.md)