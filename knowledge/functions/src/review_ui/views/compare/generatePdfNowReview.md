---
type: JavaScript Function
title: generatePdfNowReview
resource: src/review_ui/views/compare.js#L3490-L3600
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/setDisabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/assertReady
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/add
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/getSelection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/persistPdfUiState
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/updatePdfPreviewControls
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function generatePdfNowReview()`

# Calls

- [setDisabled](../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/setDisabled.md)
- [assertReady](../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/assertReady.md)
- [add](../../../../../functions/src/review_ui/views/external_album/add.md)
- [getSelection](../../../../../functions/src/review_ui/views/pdf_filters/ReviewPdfFilters/getSelection.md)
- [json](../../../../../functions/src/external_album/json.md)
- [persistPdfUiState](../../../../../functions/src/review_ui/views/compare/persistPdfUiState.md)
- [updatePdfPreviewControls](../../../../../functions/src/review_ui/views/compare/updatePdfPreviewControls.md)