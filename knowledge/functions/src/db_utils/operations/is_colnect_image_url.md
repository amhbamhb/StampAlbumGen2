---
type: Rust Function
title: is_colnect_image_url
resource: src/db_utils/operations.rs#L84-L87
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/operations/is_image_url
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db_utils/operations/insert_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_colnect_image_url(url: &str) -> bool`

# Calls

- [is_image_url](../../../../functions/src/db_utils/operations/is_image_url.md)

# Called by

- [insert_stamp](../../../../functions/src/db_utils/operations/insert_stamp.md)