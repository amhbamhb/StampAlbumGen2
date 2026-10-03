---
type: JavaScript Function
title: startReview
resource: src/review_ui/views/bootstrap.js#L1306-L1376
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/startLoadingPhases
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/waitForCompareAndRedirect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/stopLoadingPhases
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/openReviewAfterStampWorld
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function startReview()`

# Calls

- [startLoadingPhases](../../../../../functions/src/review_ui/views/bootstrap/startLoadingPhases.md)
- [waitForCompareAndRedirect](../../../../../functions/src/review_ui/views/bootstrap/waitForCompareAndRedirect.md)
- [stopLoadingPhases](../../../../../functions/src/review_ui/views/bootstrap/stopLoadingPhases.md)

# Called by

- [openReviewAfterStampWorld](../../../../../functions/src/review_ui/views/bootstrap/openReviewAfterStampWorld.md)