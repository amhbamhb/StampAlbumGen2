---
type: Rust Function
title: cover_image_reference
resource: src/album_utils/globals.rs#L6-L17
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn cover_image_reference(path: &Path) -> Option<String>`

# Called by

- [write_globals](../../../../functions/src/album_utils/globals/write_globals.md)