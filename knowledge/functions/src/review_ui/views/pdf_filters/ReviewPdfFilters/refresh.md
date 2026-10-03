---
type: JavaScript Method
title: refresh
resource: src/review_ui/views/pdf_filters.js#L36-L68
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/normalize
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/reloadLeftRowsFromServer
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/refreshRightCandidatesRealtime
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/submitLeftEditor
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/generatePdfNowReview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/init
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async refresh()`

# Calls

- [status](../../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/status.md)
- [normalize](../../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/normalize.md)

# Called by

- [reloadLeftRowsFromServer](../../../../../../functions/src/review_ui/views/compare/reloadLeftRowsFromServer.md)
- [refreshRightCandidatesRealtime](../../../../../../functions/src/review_ui/views/compare/refreshRightCandidatesRealtime.md)
- [submitLeftEditor](../../../../../../functions/src/review_ui/views/compare/submitLeftEditor.md)
- [generatePdfNowReview](../../../../../../functions/src/review_ui/views/compare/generatePdfNowReview.md)
- [init](../../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/init.md)