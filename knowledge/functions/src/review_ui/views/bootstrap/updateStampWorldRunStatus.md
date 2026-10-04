---
type: JavaScript Function
title: updateStampWorldRunStatus
resource: src/review_ui/views/bootstrap.js#L616-L624
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/views/bootstrap/pollStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/runStampWorldPull
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/renderCountryPanel
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function updateStampWorldRunStatus(text, isRunning, canOpenReview)`

# Called by

- [pollStampWorldStatus](../../../../../functions/src/review_ui/views/bootstrap/pollStampWorldStatus.md)
- [runStampWorldPull](../../../../../functions/src/review_ui/views/bootstrap/runStampWorldPull.md)
- [renderCountryPanel](../../../../../functions/src/review_ui/views/bootstrap/renderCountryPanel.md)