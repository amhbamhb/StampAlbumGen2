---
type: JavaScript Function
title: effectiveCatalogCodeForItem
resource: src/review_ui/views/compare.js#L880-L889
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/normalizeCatalogCodeForUi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/inferCatalogTypeForNumber
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function effectiveCatalogCodeForItem(item)`

# Calls

- [normalizeCatalogCodeForUi](../../../../../functions/src/review_ui/views/compare/normalizeCatalogCodeForUi.md)
- [inferCatalogTypeForNumber](../../../../../functions/src/review_ui/views/compare/inferCatalogTypeForNumber.md)
- [getEffectiveDefaultCatalogCode](../../../../../functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode.md)

# Called by

- [render](../../../../../functions/src/review_ui/views/compare/render.md)