---
type: JavaScript Function
title: parseCatalogTypesFromRaw
resource: src/review_ui/views/compare.js#L2658-L2672
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/normalizeCatalogCodeForUi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/add
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function parseCatalogTypesFromRaw(raw)`

# Calls

- [normalizeCatalogCodeForUi](../../../../../functions/src/review_ui/views/compare/normalizeCatalogCodeForUi.md)
- [add](../../../../../functions/src/review_ui/views/external_album/add.md)

# Called by

- [leftEditorSuggestedCatalogCodes](../../../../../functions/src/review_ui/views/compare/leftEditorSuggestedCatalogCodes.md)