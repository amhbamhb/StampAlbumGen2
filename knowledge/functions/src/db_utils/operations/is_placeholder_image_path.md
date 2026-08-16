---
type: Rust Function
title: is_placeholder_image_path
resource: src/db_utils/operations.rs#L51-L54
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/db_utils/operations/is_real_image_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/operations/insert_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_placeholder_image_path(path: &str) -> bool`

# Called by

- [is_real_image_candidate](../../../../functions/src/db_utils/operations/is_real_image_candidate.md)
- [insert_stamp](../../../../functions/src/db_utils/operations/insert_stamp.md)