---
type: Rust Function
title: entry_relative
resource: src/external_album.rs#L266-L283
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/portable_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/inspect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/import_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn entry_relative(source: &Path, entry: &Path) -> Result<PathBuf>`

# Calls

- [relative](../../../functions/src/external_album/relative.md)
- [portable_path](../../../functions/src/external_album/portable_path.md)

# Called by

- [inspect](../../../functions/src/external_album/inspect.md)
- [import_album](../../../functions/src/external_album/import_album.md)