---
type: Rust Function
title: format_stamps_issued
resource: src/db_utils/formatting.rs#L7-L96
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/selection/ColnectPdfFilters/matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_utils/db_loader/load_stamps_from_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn format_stamps_issued(s: &str) -> String`

# Calls

- [matches](../../../../functions/src/album_globals/selection/ColnectPdfFilters/matches.md)
- [count](../../../../functions/src/album_globals/selection/count.md)

# Called by

- [load_stamps_from_db](../../../../functions/src/album_utils/db_loader/load_stamps_from_db.md)