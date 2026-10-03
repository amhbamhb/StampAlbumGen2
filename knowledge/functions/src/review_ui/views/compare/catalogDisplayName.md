---
type: JavaScript Function
title: catalogDisplayName
resource: src/review_ui/views/compare.js#L738-L743
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/buildRightCatalogSelectOptions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/openAddDialog
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function catalogDisplayName(code)`

# Calls

- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [buildRightCatalogSelectOptions](../../../../../functions/src/review_ui/views/compare/buildRightCatalogSelectOptions.md)
- [render](../../../../../functions/src/review_ui/views/compare/render.md)
- [openAddDialog](../../../../../functions/src/review_ui/views/compare/openAddDialog.md)