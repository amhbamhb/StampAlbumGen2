---
type: Rust Function
title: is_colnect_export_filename
resource: src/modes.rs#L130-L135
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/collect_colnect_csv_files_rec
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_colnect_export_filename(path: &std::path::Path) -> bool`

# Called by

- [collect_colnect_csv_files_rec](../../../functions/src/modes/collect_colnect_csv_files_rec.md)