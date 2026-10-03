---
type: JavaScript Function
title: pollReviewStampWorldStatus
resource: src/review_ui/views/compare.js#L1300-L1395
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/fetchJsonNoStoreWithTimeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/logRuntimeCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/updateReviewStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/reloadLeftRowsFromServer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/emitReviewClientLog
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/refreshRuntimeLogCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/runReviewStampWorldPull
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function pollReviewStampWorldStatus()`

# Calls

- [fetchJsonNoStoreWithTimeout](../../../../../functions/src/review_ui/views/compare/fetchJsonNoStoreWithTimeout.md)
- [logRuntimeCompare](../../../../../functions/src/review_ui/views/compare/logRuntimeCompare.md)
- [updateReviewStampWorldStatus](../../../../../functions/src/review_ui/views/compare/updateReviewStampWorldStatus.md)
- [reloadLeftRowsFromServer](../../../../../functions/src/review_ui/views/compare/reloadLeftRowsFromServer.md)
- [emitReviewClientLog](../../../../../functions/src/review_ui/views/compare/emitReviewClientLog.md)

# Called by

- [refreshRuntimeLogCompare](../../../../../functions/src/review_ui/views/compare/refreshRuntimeLogCompare.md)
- [runReviewStampWorldPull](../../../../../functions/src/review_ui/views/compare/runReviewStampWorldPull.md)