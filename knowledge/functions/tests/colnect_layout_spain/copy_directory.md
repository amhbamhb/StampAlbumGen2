---
type: Rust Function
title: copy_directory
resource: tests/colnect_layout_spain.rs#L11-L25
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn copy_directory(source: &Path, target: &Path) -> std::io::Result<usize>`

# Called by

- [spain_colnect_layout_packs_rich_rows_without_losing_stamps](../../../functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps.md)