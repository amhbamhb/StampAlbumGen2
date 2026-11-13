---
type: Rust Function
title: count_import_files_rec
resource: src/review_ui.rs#L1843-L1861
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/count_colnect_csv_files_rec
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_import_files_rec(root: &Path, accepts: fn(&Path) -> bool) -> usize`

# Called by

- [count_colnect_csv_files_rec](../../../functions/src/review_ui/count_colnect_csv_files_rec.md)
- [scan_bootstrap_countries](../../../functions/src/review_ui/scan_bootstrap_countries.md)