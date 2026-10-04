---
type: JavaScript Function
title: configFieldType
resource: src/review_ui/config_editor.js#L132-L140
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/config_editor/isBooleanValue
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/isFontField
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/configFieldKnownOptions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/isNumericValue
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/config_editor/configFieldHelpHtml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/renderConfigValueInput
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/renderConfigEditorSections
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function configFieldType(field)`

# Calls

- [isBooleanValue](../../../../functions/src/review_ui/config_editor/isBooleanValue.md)
- [isFontField](../../../../functions/src/review_ui/config_editor/isFontField.md)
- [configFieldKnownOptions](../../../../functions/src/review_ui/config_editor/configFieldKnownOptions.md)
- [isNumericValue](../../../../functions/src/review_ui/config_editor/isNumericValue.md)

# Called by

- [configFieldHelpHtml](../../../../functions/src/review_ui/config_editor/configFieldHelpHtml.md)
- [renderConfigValueInput](../../../../functions/src/review_ui/config_editor/renderConfigValueInput.md)
- [renderConfigEditorSections](../../../../functions/src/review_ui/config_editor/renderConfigEditorSections.md)