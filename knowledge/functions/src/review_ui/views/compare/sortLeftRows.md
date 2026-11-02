---
type: JavaScript Function
title: sortLeftRows
resource: src/review_ui/views/compare.js#L1018-L1032
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/compareBySortMode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/updateLeftSearchMatches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/reloadLeftRowsFromServer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/upsertLeftRow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/removeLeftRow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/changeSortMode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/resetSortModeDefault
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function sortLeftRows(preferredRowid)`

# Calls

- [compareBySortMode](../../../../../functions/src/review_ui/views/compare/compareBySortMode.md)
- [updateLeftSearchMatches](../../../../../functions/src/review_ui/views/compare/updateLeftSearchMatches.md)

# Called by

- [reloadLeftRowsFromServer](../../../../../functions/src/review_ui/views/compare/reloadLeftRowsFromServer.md)
- [upsertLeftRow](../../../../../functions/src/review_ui/views/compare/upsertLeftRow.md)
- [removeLeftRow](../../../../../functions/src/review_ui/views/compare/removeLeftRow.md)
- [changeSortMode](../../../../../functions/src/review_ui/views/compare/changeSortMode.md)
- [resetSortModeDefault](../../../../../functions/src/review_ui/views/compare/resetSortModeDefault.md)