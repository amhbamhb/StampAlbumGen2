---
type: JavaScript Function
title: confirmDestructiveActionCompare
resource: src/review_ui/views/compare.js#L3051-L3085
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/deleteCurrentLeft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/deleteCurrentCandidate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function confirmDestructiveActionCompare(message, title = 'Permanent deletion warning', confirmLabel = 'Delete permanently')`

# Calls

- [contains](../../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [deleteCurrentLeft](../../../../../functions/src/review_ui/views/compare/deleteCurrentLeft.md)
- [deleteCurrentCandidate](../../../../../functions/src/review_ui/views/compare/deleteCurrentCandidate.md)