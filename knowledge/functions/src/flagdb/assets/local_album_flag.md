---
type: Rust Function
title: local_album_flag
resource: src/flagdb/assets.rs#L201-L214
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/flagdb/assets/valid_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/is_missing
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/copy_flag_for_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn local_album_flag(dir: &Path) -> Option<PathBuf>`

# Calls

- [valid_file](../../../../functions/src/flagdb/assets/valid_file.md)
- [is_missing](../../../../functions/src/flagdb/assets/is_missing.md)

# Called by

- [copy_flag_for_album](../../../../functions/src/flagdb/copy_flag_for_album.md)