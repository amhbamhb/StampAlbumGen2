---
type: JavaScript Function
title: refreshRightCandidatesRealtime
resource: src/review_ui/views/compare.js#L558-L578
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/pruneAssignmentsToExistingCandidates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/setMatchStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/refresh
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/deleteCurrentLeft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/clearCurrentLeftColnectLink
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/addCurrentColnectAsStamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/undoLastDelete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/importColnectFilesReview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/refreshColnectCandidatesReview
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function refreshRightCandidatesRealtime(restoredColnectId = null)`

# Calls

- [pruneAssignmentsToExistingCandidates](../../../../../functions/src/review_ui/views/compare/pruneAssignmentsToExistingCandidates.md)
- [setMatchStatus](../../../../../functions/src/review_ui/views/compare/setMatchStatus.md)
- [refresh](../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/refresh.md)

# Called by

- [deleteCurrentLeft](../../../../../functions/src/review_ui/views/compare/deleteCurrentLeft.md)
- [clearCurrentLeftColnectLink](../../../../../functions/src/review_ui/views/compare/clearCurrentLeftColnectLink.md)
- [addCurrentColnectAsStamp](../../../../../functions/src/review_ui/views/compare/addCurrentColnectAsStamp.md)
- [undoLastDelete](../../../../../functions/src/review_ui/views/compare/undoLastDelete.md)
- [importColnectFilesReview](../../../../../functions/src/review_ui/views/compare/importColnectFilesReview.md)
- [refreshColnectCandidatesReview](../../../../../functions/src/review_ui/views/compare/refreshColnectCandidatesReview.md)