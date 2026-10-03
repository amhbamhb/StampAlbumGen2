---
type: JavaScript Function
title: configEditorChangeList
resource: src/review_ui/config_editor.js#L306
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/collectConfigEditorValues
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/configEditorFieldMetaMap
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/configEditorChangeSummaryHtml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/configEditorSaveConfirmChoice
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function configEditorChangeList(values = collectConfigEditorValues())`

# Calls

- [collectConfigEditorValues](../../../../functions/src/review_ui/config_editor/collectConfigEditorValues.md)
- [configEditorFieldMetaMap](../../../../functions/src/review_ui/config_editor/configEditorFieldMetaMap.md)

# Called by

- [configEditorChangeSummaryHtml](../../../../functions/src/review_ui/config_editor/configEditorChangeSummaryHtml.md)
- [configEditorSaveConfirmChoice](../../../../functions/src/review_ui/config_editor/configEditorSaveConfirmChoice.md)