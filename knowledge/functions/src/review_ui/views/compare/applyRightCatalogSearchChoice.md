---
type: JavaScript Function
title: applyRightCatalogSearchChoice
resource: src/review_ui/views/compare.js#L1997-L2015
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/parseCatalogCodeFromSearchValue
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/rememberCatalogCode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/tryApplyRightCatalogSearchChoice
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function applyRightCatalogSearchChoice()`

# Calls

- [parseCatalogCodeFromSearchValue](../../../../../functions/src/review_ui/views/compare/parseCatalogCodeFromSearchValue.md)
- [from](../../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [rememberCatalogCode](../../../../../functions/src/review_ui/views/compare/rememberCatalogCode.md)

# Called by

- [tryApplyRightCatalogSearchChoice](../../../../../functions/src/review_ui/views/compare/tryApplyRightCatalogSearchChoice.md)