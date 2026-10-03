---
type: JavaScript Function
title: configEditorChangeSummaryHtml
resource: src/review_ui/config_editor.js#L307-L319
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/collectConfigEditorValues
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/configEditorChangeList
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/cfgHtml
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/configEditorSaveConfirmChoice
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function configEditorChangeSummaryHtml(values = collectConfigEditorValues())`

# Calls

- [collectConfigEditorValues](../../../../functions/src/review_ui/config_editor/collectConfigEditorValues.md)
- [configEditorChangeList](../../../../functions/src/review_ui/config_editor/configEditorChangeList.md)
- [cfgHtml](../../../../functions/src/review_ui/config_editor/cfgHtml.md)

# Called by

- [configEditorSaveConfirmChoice](../../../../functions/src/review_ui/config_editor/configEditorSaveConfirmChoice.md)