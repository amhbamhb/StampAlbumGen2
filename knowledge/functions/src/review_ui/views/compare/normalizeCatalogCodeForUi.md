---
type: JavaScript Function
title: normalizeCatalogCodeForUi
resource: src/review_ui/views/compare.js#L744-L755
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/views/compare/inferInitialCatalogCodeFromLeftRows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/changeCountryDefaultCatalog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/saveCatalogDefaultToConfig
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/inferCatalogTypeForNumber
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/effectiveCatalogCodeForItem
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/rightCatalogFamiliesForCandidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/buildRightCatalogSelectOptions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/ordinalValueForCatalog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/parseCatalogTypesFromRaw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/rebuildLeftEditorCatalogTypeOptions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function normalizeCatalogCodeForUi(code)`

# Called by

- [inferInitialCatalogCodeFromLeftRows](../../../../../functions/src/review_ui/views/compare/inferInitialCatalogCodeFromLeftRows.md)
- [getEffectiveDefaultCatalogCode](../../../../../functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode.md)
- [changeCountryDefaultCatalog](../../../../../functions/src/review_ui/views/compare/changeCountryDefaultCatalog.md)
- [saveCatalogDefaultToConfig](../../../../../functions/src/review_ui/views/compare/saveCatalogDefaultToConfig.md)
- [inferCatalogTypeForNumber](../../../../../functions/src/review_ui/views/compare/inferCatalogTypeForNumber.md)
- [effectiveCatalogCodeForItem](../../../../../functions/src/review_ui/views/compare/effectiveCatalogCodeForItem.md)
- [rightCatalogFamiliesForCandidate](../../../../../functions/src/review_ui/views/compare/rightCatalogFamiliesForCandidate.md)
- [buildRightCatalogSelectOptions](../../../../../functions/src/review_ui/views/compare/buildRightCatalogSelectOptions.md)
- [ordinalValueForCatalog](../../../../../functions/src/review_ui/views/compare/ordinalValueForCatalog.md)
- [parseCatalogTypesFromRaw](../../../../../functions/src/review_ui/views/compare/parseCatalogTypesFromRaw.md)
- [leftEditorSuggestedCatalogCodes](../../../../../functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes.md)
- [rebuildLeftEditorCatalogTypeOptions](../../../../../functions/src/review_ui/views/compare/rebuildLeftEditorCatalogTypeOptions.md)