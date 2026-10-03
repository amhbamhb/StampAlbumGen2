---
type: JavaScript Function
title: setConfigEditorStatus
resource: src/review_ui/config_editor.js#L26-L35
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/configEditorStatusBadge
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/setConfigStatus
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/onConfigSaveTargetChanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/markConfigEditorDirty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/openConfigEditor
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/toggleConfigEditorOverrideFilter
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/requestCloseConfigEditor
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/saveConfigEditorChanges
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/deleteAlbumConfigOverride
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function setConfigEditorStatus(message)`

# Calls

- [configEditorStatusBadge](../../../../functions/src/review_ui/config_editor/configEditorStatusBadge.md)
- [setConfigStatus](../../../../functions/src/review_ui/views/compare/setConfigStatus.md)

# Called by

- [onConfigSaveTargetChanged](../../../../functions/src/review_ui/config_editor/onConfigSaveTargetChanged.md)
- [markConfigEditorDirty](../../../../functions/src/review_ui/config_editor/markConfigEditorDirty.md)
- [openConfigEditor](../../../../functions/src/review_ui/config_editor/openConfigEditor.md)
- [toggleConfigEditorOverrideFilter](../../../../functions/src/review_ui/config_editor/toggleConfigEditorOverrideFilter.md)
- [requestCloseConfigEditor](../../../../functions/src/review_ui/config_editor/requestCloseConfigEditor.md)
- [saveConfigEditorChanges](../../../../functions/src/review_ui/config_editor/saveConfigEditorChanges.md)
- [deleteAlbumConfigOverride](../../../../functions/src/review_ui/config_editor/deleteAlbumConfigOverride.md)