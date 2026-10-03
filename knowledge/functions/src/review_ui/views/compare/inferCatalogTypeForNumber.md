---
type: JavaScript Function
title: inferCatalogTypeForNumber
resource: src/review_ui/views/compare.js#L870-L877
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/normalizeCatalogNumber
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/parseCatalogCodePairs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/normalizeCatalogCodeForUi
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/effectiveCatalogCodeForItem
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function inferCatalogTypeForNumber(rawCatalogCodes, displayedNumber, fallbackCatalogCode)`

# Calls

- [normalizeCatalogNumber](../../../../../functions/src/review_ui/views/compare/normalizeCatalogNumber.md)
- [parseCatalogCodePairs](../../../../../functions/src/review_ui/views/compare/parseCatalogCodePairs.md)
- [normalizeCatalogCodeForUi](../../../../../functions/src/review_ui/views/compare/normalizeCatalogCodeForUi.md)

# Called by

- [effectiveCatalogCodeForItem](../../../../../functions/src/review_ui/views/compare/effectiveCatalogCodeForItem.md)
- [render](../../../../../functions/src/review_ui/views/compare/render.md)