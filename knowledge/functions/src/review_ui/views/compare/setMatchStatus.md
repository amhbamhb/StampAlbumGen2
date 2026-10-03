---
type: JavaScript Function
title: setMatchStatus
resource: src/review_ui/views/compare.js#L210-L214
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/setMainStatus
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/refreshRightCandidatesRealtime
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/assignCurrent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/clearAssignment
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/applyMatches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/openAddDialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/deleteCurrentLeft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/clearCurrentLeftColnectLink
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/addCurrentColnectAsStamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/deleteCurrentCandidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/undoLastDelete
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function setMatchStatus(message)`

# Calls

- [setMainStatus](../../../../../functions/src/review_ui/views/compare/setMainStatus.md)

# Called by

- [refreshRightCandidatesRealtime](../../../../../functions/src/review_ui/views/compare/refreshRightCandidatesRealtime.md)
- [assignCurrent](../../../../../functions/src/review_ui/views/compare/assignCurrent.md)
- [clearAssignment](../../../../../functions/src/review_ui/views/compare/clearAssignment.md)
- [applyMatches](../../../../../functions/src/review_ui/views/compare/applyMatches.md)
- [openAddDialog](../../../../../functions/src/review_ui/views/compare/openAddDialog.md)
- [deleteCurrentLeft](../../../../../functions/src/review_ui/views/compare/deleteCurrentLeft.md)
- [clearCurrentLeftColnectLink](../../../../../functions/src/review_ui/views/compare/clearCurrentLeftColnectLink.md)
- [addCurrentColnectAsStamp](../../../../../functions/src/review_ui/views/compare/addCurrentColnectAsStamp.md)
- [deleteCurrentCandidate](../../../../../functions/src/review_ui/views/compare/deleteCurrentCandidate.md)
- [undoLastDelete](../../../../../functions/src/review_ui/views/compare/undoLastDelete.md)