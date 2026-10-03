---
type: JavaScript Function
title: reloadLeftRowsFromServer
resource: src/review_ui/views/compare.js#L531-L555
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/logRuntimeCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/sortLeftRows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/refresh
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/pollReviewStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function reloadLeftRowsFromServer(preferredRowid = null)`

# Calls

- [logRuntimeCompare](../../../../../functions/src/review_ui/views/compare/logRuntimeCompare.md)
- [sortLeftRows](../../../../../functions/src/review_ui/views/compare/sortLeftRows.md)
- [refresh](../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/refresh.md)

# Called by

- [pollReviewStampWorldStatus](../../../../../functions/src/review_ui/views/compare/pollReviewStampWorldStatus.md)