---
type: JavaScript Function
title: selectedCatalogTypeForCurrent
resource: src/review_ui/views/compare.js#L1960-L1974
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/selectedCatalogCodeForCurrent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/openAddDialog
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function selectedCatalogTypeForCurrent(r)`

# Calls

- [selectedCatalogCodeForCurrent](../../../../../functions/src/review_ui/views/compare/selectedCatalogCodeForCurrent.md)
- [getEffectiveDefaultCatalogCode](../../../../../functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode.md)

# Called by

- [render](../../../../../functions/src/review_ui/views/compare/render.md)
- [openAddDialog](../../../../../functions/src/review_ui/views/compare/openAddDialog.md)