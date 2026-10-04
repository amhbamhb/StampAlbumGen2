---
type: JavaScript Function
title: buildRightCatalogSelectOptions
resource: src/review_ui/views/compare.js#L1987-L2027
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/rightCatalogFamiliesForCandidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/normalizeCatalogCodeForUi
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/catalogDisplayName
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/populateRightCatalogSearchSuggestions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/catalogFamilyOptionText
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function buildRightCatalogSelectOptions(r, selectedCode, selectedType)`

# Calls

- [rightCatalogFamiliesForCandidate](../../../../../functions/src/review_ui/views/compare/rightCatalogFamiliesForCandidate.md)
- [normalizeCatalogCodeForUi](../../../../../functions/src/review_ui/views/compare/normalizeCatalogCodeForUi.md)
- [catalogDisplayName](../../../../../functions/src/review_ui/views/compare/catalogDisplayName.md)
- [populateRightCatalogSearchSuggestions](../../../../../functions/src/review_ui/views/compare/populateRightCatalogSearchSuggestions.md)
- [html](../../../../../functions/src/review_ui/views/compare/html.md)
- [catalogFamilyOptionText](../../../../../functions/src/review_ui/views/compare/catalogFamilyOptionText.md)

# Called by

- [render](../../../../../functions/src/review_ui/views/compare/render.md)