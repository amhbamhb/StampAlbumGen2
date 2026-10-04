---
type: JavaScript Function
title: selectedCatalogCodeForCurrent
resource: src/review_ui/views/compare.js#L1910-L1920
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/selectedCatalogTypeForCurrent
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

`function selectedCatalogCodeForCurrent(r)`

# Calls

- [getEffectiveDefaultCatalogCode](../../../../../functions/src/review_ui/views/compare/getEffectiveDefaultCatalogCode.md)

# Called by

- [selectedCatalogTypeForCurrent](../../../../../functions/src/review_ui/views/compare/selectedCatalogTypeForCurrent.md)
- [render](../../../../../functions/src/review_ui/views/compare/render.md)
- [openAddDialog](../../../../../functions/src/review_ui/views/compare/openAddDialog.md)