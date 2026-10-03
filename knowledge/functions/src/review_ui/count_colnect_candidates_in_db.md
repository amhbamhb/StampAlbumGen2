---
type: Rust Function
title: count_colnect_candidates_in_db
resource: src/review_ui.rs#L1839-L1860
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count_colnect_candidates_in_db(db_path: &Path) -> usize`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [scan_bootstrap_countries](../../../functions/src/review_ui/scan_bootstrap_countries.md)