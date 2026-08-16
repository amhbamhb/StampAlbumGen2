---
type: Rust Function
title: is_real_image_candidate
resource: src/db_utils/operations.rs#L56-L59
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/operations/is_placeholder_image_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db_utils/operations/insert_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_real_image_candidate(path: &str) -> bool`

# Calls

- [is_placeholder_image_path](../../../../functions/src/db_utils/operations/is_placeholder_image_path.md)

# Called by

- [insert_stamp](../../../../functions/src/db_utils/operations/insert_stamp.md)