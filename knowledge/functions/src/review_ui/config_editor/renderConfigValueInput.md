---
type: JavaScript Function
title: renderConfigValueInput
resource: src/review_ui/config_editor.js#L153-L184
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/configFieldInputId
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/configFieldType
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/configFieldKnownOptions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/cfgHtml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/stripTomlQuotes
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/renderConfigEditorSections
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function renderConfigValueInput(field)`

# Calls

- [configFieldInputId](../../../../functions/src/review_ui/config_editor/configFieldInputId.md)
- [configFieldType](../../../../functions/src/review_ui/config_editor/configFieldType.md)
- [configFieldKnownOptions](../../../../functions/src/review_ui/config_editor/configFieldKnownOptions.md)
- [cfgHtml](../../../../functions/src/review_ui/config_editor/cfgHtml.md)
- [stripTomlQuotes](../../../../functions/src/review_ui/config_editor/stripTomlQuotes.md)

# Called by

- [renderConfigEditorSections](../../../../functions/src/review_ui/config_editor/renderConfigEditorSections.md)