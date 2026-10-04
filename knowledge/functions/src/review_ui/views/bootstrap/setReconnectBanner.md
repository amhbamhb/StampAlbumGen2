---
type: JavaScript Function
title: setReconnectBanner
resource: src/review_ui/views/bootstrap.js#L52-L63
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/review_ui/views/bootstrap/checkBackendHealth
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/beginNativeDialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/endNativeDialog
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function setReconnectBanner(visible, title, text, recovered)`

# Called by

- [checkBackendHealth](../../../../../functions/src/review_ui/views/bootstrap/checkBackendHealth.md)
- [beginNativeDialog](../../../../../functions/src/review_ui/views/bootstrap/beginNativeDialog.md)
- [endNativeDialog](../../../../../functions/src/review_ui/views/bootstrap/endNativeDialog.md)