---
type: JavaScript Function
title: inferInitialCatalogCodeFromLeftRows
resource: src/review_ui/views/compare.js#L757-L788
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/normalizeCatalogCodeForUi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/set
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function inferInitialCatalogCodeFromLeftRows()`

# Calls

- [normalizeCatalogCodeForUi](../../../../../functions/src/review_ui/views/compare/normalizeCatalogCodeForUi.md)
- [set](../../../../../functions/src/album_globals/structs/GenericAttrs/set.md)
- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [getEffectiveDefaultCatalogCode](../../../../../functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode.md)