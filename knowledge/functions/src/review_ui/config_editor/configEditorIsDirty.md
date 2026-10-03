---
type: JavaScript Function
title: configEditorIsDirty
resource: src/review_ui/config_editor.js#L263
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/collectConfigEditorValues
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/markConfigEditorDirty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/requestCloseConfigEditor
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function configEditorIsDirty()`

# Calls

- [collectConfigEditorValues](../../../../functions/src/review_ui/config_editor/collectConfigEditorValues.md)

# Called by

- [markConfigEditorDirty](../../../../functions/src/review_ui/config_editor/markConfigEditorDirty.md)
- [requestCloseConfigEditor](../../../../functions/src/review_ui/config_editor/requestCloseConfigEditor.md)