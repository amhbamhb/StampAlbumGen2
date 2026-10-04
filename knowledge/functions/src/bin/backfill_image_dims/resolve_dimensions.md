---
type: Rust Function
title: resolve_dimensions
resource: src/bin/backfill_image_dims.rs#L13-L25
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/image_utils/get_image_dimensions
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/bin/backfill_image_dims/process_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_dimensions(images_dir: &Path, image_path: &str) -> Option<(u32, u32)>`

# Calls

- [get_image_dimensions](../../../../functions/src/image_utils/get_image_dimensions.md)

# Called by

- [process_db](../../../../functions/src/bin/backfill_image_dims/process_db.md)