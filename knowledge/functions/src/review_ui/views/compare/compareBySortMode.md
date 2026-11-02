---
type: JavaScript Function
title: compareBySortMode
resource: src/review_ui/views/compare.js#L954-L1016
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/normalizeSortMode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/sgSortText
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/compareIssueDateParts
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/sortLeftRows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/availableCandidatesForCurrentLeft
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function compareBySortMode(a, b, mode)`

# Calls

- [normalizeSortMode](../../../../../functions/src/review_ui/views/compare/normalizeSortMode.md)
- [sgSortText](../../../../../functions/src/review_ui/views/compare/sgSortText.md)
- [compareIssueDateParts](../../../../../functions/src/review_ui/views/compare/compareIssueDateParts.md)

# Called by

- [sortLeftRows](../../../../../functions/src/review_ui/views/compare/sortLeftRows.md)
- [availableCandidatesForCurrentLeft](../../../../../functions/src/review_ui/views/compare/availableCandidatesForCurrentLeft.md)