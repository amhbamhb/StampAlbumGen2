---
type: JavaScript Function
title: updateLeftSearchMatches
resource: src/review_ui/views/compare.js#L1123-L1161
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/setLeftSearchStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/leftSearchText
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/sortLeftRows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/onLeftSearchInput
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/stepLeftSearch
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function updateLeftSearchMatches(preserveCurrentSelection)`

# Calls

- [setLeftSearchStatus](../../../../../functions/src/review_ui/views/compare/setLeftSearchStatus.md)
- [leftSearchText](../../../../../functions/src/review_ui/views/compare/leftSearchText.md)

# Called by

- [sortLeftRows](../../../../../functions/src/review_ui/views/compare/sortLeftRows.md)
- [onLeftSearchInput](../../../../../functions/src/review_ui/views/compare/onLeftSearchInput.md)
- [stepLeftSearch](../../../../../functions/src/review_ui/views/compare/stepLeftSearch.md)
- [render](../../../../../functions/src/review_ui/views/compare/render.md)