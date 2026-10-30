---
type: JavaScript Function
title: rebuildLeftEditorCatalogTypeOptions
resource: src/review_ui/views/compare.js#L2746-L2792
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/populateLeftEditorCatalogSearchSuggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/catalogFamilyOptionText
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/normalizeCatalogCodeForUi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/toggleLeftEditorCatalogCodeCustom
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/applyLeftEditorCatalogSearchChoice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/tryApplyLeftEditorCatalogSearchChoice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/filterLeftEditorCatalogTypeOptions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/setLeftEditorCatalogCodeValue
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function rebuildLeftEditorCatalogTypeOptions(selectedCode)`

# Calls

- [leftEditorSuggestedCatalogCodes](../../../../../functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes.md)
- [populateLeftEditorCatalogSearchSuggestions](../../../../../functions/src/review_ui/views/compare/populateLeftEditorCatalogSearchSuggestions.md)
- [html](../../../../../functions/src/review_ui/views/compare/html.md)
- [catalogFamilyOptionText](../../../../../functions/src/review_ui/views/compare/catalogFamilyOptionText.md)
- [normalizeCatalogCodeForUi](../../../../../functions/src/review_ui/views/compare/normalizeCatalogCodeForUi.md)
- [getEffectiveDefaultCatalogCode](../../../../../functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode.md)
- [toggleLeftEditorCatalogCodeCustom](../../../../../functions/src/review_ui/views/compare/toggleLeftEditorCatalogCodeCustom.md)

# Called by

- [applyLeftEditorCatalogSearchChoice](../../../../../functions/src/review_ui/views/compare/applyLeftEditorCatalogSearchChoice.md)
- [tryApplyLeftEditorCatalogSearchChoice](../../../../../functions/src/review_ui/views/compare/tryApplyLeftEditorCatalogSearchChoice.md)
- [filterLeftEditorCatalogTypeOptions](../../../../../functions/src/review_ui/views/compare/filterLeftEditorCatalogTypeOptions.md)
- [setLeftEditorCatalogCodeValue](../../../../../functions/src/review_ui/views/compare/setLeftEditorCatalogCodeValue.md)