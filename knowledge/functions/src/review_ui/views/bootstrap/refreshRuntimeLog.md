---
type: JavaScript Function
title: refreshRuntimeLog
resource: src/review_ui/views/bootstrap.js#L224-L237
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/isRuntimeLogVisible
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/startRuntimeLogPolling
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function refreshRuntimeLog()`

# Calls

- [isRuntimeLogVisible](../../../../../functions/src/review_ui/views/bootstrap/isRuntimeLogVisible.md)

# Called by

- [startRuntimeLogPolling](../../../../../functions/src/review_ui/views/bootstrap/startRuntimeLogPolling.md)