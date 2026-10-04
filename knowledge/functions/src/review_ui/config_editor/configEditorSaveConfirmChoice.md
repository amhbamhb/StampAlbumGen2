---
type: JavaScript Function
title: configEditorSaveConfirmChoice
resource: src/review_ui/config_editor.js#L320
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
  - target: functions/src/review_ui/config_editor/configEditorSaveTargetLabel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/configEditorChangeSummaryHtml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/cfgHtml
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/saveConfigEditorChanges
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function configEditorSaveConfirmChoice(values = collectConfigEditorValues())`

# Calls

- [collectConfigEditorValues](../../../../functions/src/review_ui/config_editor/collectConfigEditorValues.md)
- [configEditorChangeList](../../../../functions/src/review_ui/config_editor/configEditorChangeList.md)
- [configEditorSaveTargetLabel](../../../../functions/src/review_ui/config_editor/configEditorSaveTargetLabel.md)
- [configEditorChangeSummaryHtml](../../../../functions/src/review_ui/config_editor/configEditorChangeSummaryHtml.md)
- [cfgHtml](../../../../functions/src/review_ui/config_editor/cfgHtml.md)

# Called by

- [saveConfigEditorChanges](../../../../functions/src/review_ui/config_editor/saveConfigEditorChanges.md)