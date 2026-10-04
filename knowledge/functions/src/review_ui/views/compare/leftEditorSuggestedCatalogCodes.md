---
type: JavaScript Function
title: leftEditorSuggestedCatalogCodes
resource: src/review_ui/views/compare.js#L2637-L2650
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/normalizeCatalogCodeForUi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/parseCatalogTypesFromRaw
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/rebuildLeftEditorCatalogTypeOptions
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function leftEditorSuggestedCatalogCodes(base)`

# Calls

- [normalizeCatalogCodeForUi](../../../../../functions/src/review_ui/views/compare/normalizeCatalogCodeForUi.md)
- [parseCatalogTypesFromRaw](../../../../../functions/src/review_ui/views/compare/parseCatalogTypesFromRaw.md)
- [getEffectiveDefaultCatalogCode](../../../../../functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode.md)

# Called by

- [rebuildLeftEditorCatalogTypeOptions](../../../../../functions/src/review_ui/views/compare/rebuildLeftEditorCatalogTypeOptions.md)