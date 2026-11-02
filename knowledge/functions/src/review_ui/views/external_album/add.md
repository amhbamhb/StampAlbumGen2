---
type: JavaScript Function
title: add
resource: src/review_ui/views/external_album.js#L277-L292
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/renderCommands
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/hideConfigEditorLoading
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/filterConfigEditorFields
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setAlbumsRootStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/restorePdfUiState
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/previewLastPdfReview
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/navigateWithFade
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/usedColnectIds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/rightCatalogFamiliesForCandidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/parseCatalogTypesFromRaw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/generatePdfNowReview
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function add(name)`

# Calls

- [renderCommands](../../../../../functions/src/review_ui/views/external_album/renderCommands.md)

# Called by

- [hideConfigEditorLoading](../../../../../functions/src/review_ui/config_editor/hideConfigEditorLoading.md)
- [filterConfigEditorFields](../../../../../functions/src/review_ui/config_editor/filterConfigEditorFields.md)
- [setAlbumsRootStatus](../../../../../functions/src/review_ui/views/bootstrap/setAlbumsRootStatus.md)
- [restorePdfUiState](../../../../../functions/src/review_ui/views/compare/restorePdfUiState.md)
- [previewLastPdfReview](../../../../../functions/src/review_ui/views/compare/previewLastPdfReview.md)
- [navigateWithFade](../../../../../functions/src/review_ui/views/compare/navigateWithFade.md)
- [usedColnectIds](../../../../../functions/src/review_ui/views/compare/usedColnectIds.md)
- [rightCatalogFamiliesForCandidate](../../../../../functions/src/review_ui/views/compare/rightCatalogFamiliesForCandidate.md)
- [parseCatalogTypesFromRaw](../../../../../functions/src/review_ui/views/compare/parseCatalogTypesFromRaw.md)
- [leftEditorSuggestedCatalogCodes](../../../../../functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes.md)
- [generatePdfNowReview](../../../../../functions/src/review_ui/views/compare/generatePdfNowReview.md)