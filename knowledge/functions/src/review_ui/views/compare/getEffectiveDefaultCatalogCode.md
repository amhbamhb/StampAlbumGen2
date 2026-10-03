---
type: JavaScript Function
title: getEffectiveDefaultCatalogCode
resource: src/review_ui/views/compare.js#L791-L796
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/normalizeCatalogCodeForUi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/inferInitialCatalogCodeFromLeftRows
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/changeCountryDefaultCatalog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/effectiveCatalogCodeForItem
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/selectedCatalogCodeForCurrent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/selectedCatalogTypeForCurrent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/openAddDialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/rebuildLeftEditorCatalogTypeOptions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/openLeftEditor
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function getEffectiveDefaultCatalogCode()`

# Calls

- [normalizeCatalogCodeForUi](../../../../../functions/src/review_ui/views/compare/normalizeCatalogCodeForUi.md)
- [inferInitialCatalogCodeFromLeftRows](../../../../../functions/src/review_ui/views/compare/inferInitialCatalogCodeFromLeftRows.md)

# Called by

- [changeCountryDefaultCatalog](../../../../../functions/src/review_ui/views/compare/changeCountryDefaultCatalog.md)
- [effectiveCatalogCodeForItem](../../../../../functions/src/review_ui/views/compare/effectiveCatalogCodeForItem.md)
- [selectedCatalogCodeForCurrent](../../../../../functions/src/review_ui/views/compare/selectedCatalogCodeForCurrent.md)
- [selectedCatalogTypeForCurrent](../../../../../functions/src/review_ui/views/compare/selectedCatalogTypeForCurrent.md)
- [render](../../../../../functions/src/review_ui/views/compare/render.md)
- [openAddDialog](../../../../../functions/src/review_ui/views/compare/openAddDialog.md)
- [leftEditorSuggestedCatalogCodes](../../../../../functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes.md)
- [rebuildLeftEditorCatalogTypeOptions](../../../../../functions/src/review_ui/views/compare/rebuildLeftEditorCatalogTypeOptions.md)
- [openLeftEditor](../../../../../functions/src/review_ui/views/compare/openLeftEditor.md)