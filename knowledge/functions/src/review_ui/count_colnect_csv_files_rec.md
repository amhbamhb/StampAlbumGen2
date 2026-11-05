---
type: Rust Function
title: count_colnect_csv_files_rec
resource: src/review_ui.rs#L1833-L1835
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/count_import_files_rec
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/build_matcher_payload
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_colnect_csv_files_rec(root: &Path) -> usize`

# Calls

- [count_import_files_rec](../../../functions/src/review_ui/count_import_files_rec.md)

# Called by

- [scan_bootstrap_countries](../../../functions/src/review_ui/scan_bootstrap_countries.md)
- [build_matcher_payload](../../../functions/src/review_ui/build_matcher_payload.md)