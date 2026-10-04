---
type: JavaScript Function
title: saveConfigEditorChanges
resource: src/review_ui/config_editor.js#L323
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/collectConfigEditorValues
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/configEditorSaveConfirmChoice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/setConfigEditorStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/closeConfigEditorBackdrop
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/requestCloseConfigEditor
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function saveConfigEditorChanges(skipConfirm = false)`

# Calls

- [collectConfigEditorValues](../../../../functions/src/review_ui/config_editor/collectConfigEditorValues.md)
- [configEditorSaveConfirmChoice](../../../../functions/src/review_ui/config_editor/configEditorSaveConfirmChoice.md)
- [setConfigEditorStatus](../../../../functions/src/review_ui/config_editor/setConfigEditorStatus.md)
- [closeConfigEditorBackdrop](../../../../functions/src/review_ui/config_editor/closeConfigEditorBackdrop.md)

# Called by

- [requestCloseConfigEditor](../../../../functions/src/review_ui/config_editor/requestCloseConfigEditor.md)