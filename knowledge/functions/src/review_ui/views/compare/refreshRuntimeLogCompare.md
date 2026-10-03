---
type: JavaScript Function
title: refreshRuntimeLogCompare
resource: src/review_ui/views/compare.js#L395-L416
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/isRuntimeLogVisibleCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/pollReviewStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/startRuntimeLogPollingCompare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function refreshRuntimeLogCompare()`

# Calls

- [isRuntimeLogVisibleCompare](../../../../../functions/src/review_ui/views/compare/isRuntimeLogVisibleCompare.md)
- [pollReviewStampWorldStatus](../../../../../functions/src/review_ui/views/compare/pollReviewStampWorldStatus.md)

# Called by

- [startRuntimeLogPollingCompare](../../../../../functions/src/review_ui/views/compare/startRuntimeLogPollingCompare.md)