---
type: JavaScript Function
title: setConfigStatus
resource: src/review_ui/views/compare.js#L217-L221
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/setMainStatus
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/setConfigEditorStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/saveCatalogDefaultToConfig
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function setConfigStatus(message)`

# Calls

- [setMainStatus](../../../../../functions/src/review_ui/views/compare/setMainStatus.md)

# Called by

- [setConfigEditorStatus](../../../../../functions/src/review_ui/config_editor/setConfigEditorStatus.md)
- [saveCatalogDefaultToConfig](../../../../../functions/src/review_ui/views/compare/saveCatalogDefaultToConfig.md)