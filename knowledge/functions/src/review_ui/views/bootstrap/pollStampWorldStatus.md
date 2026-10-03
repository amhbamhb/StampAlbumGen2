---
type: JavaScript Function
title: pollStampWorldStatus
resource: src/review_ui/views/bootstrap.js#L623-L715
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/updateStampWorldRunStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/renderCountryPanel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/checkAlbumeasyStatusAfterPull
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/refreshState
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/runStampWorldPull
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function pollStampWorldStatus()`

# Calls

- [updateStampWorldRunStatus](../../../../../functions/src/review_ui/views/bootstrap/updateStampWorldRunStatus.md)
- [from](../../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [renderCountryPanel](../../../../../functions/src/review_ui/views/bootstrap/renderCountryPanel.md)
- [checkAlbumeasyStatusAfterPull](../../../../../functions/src/review_ui/views/bootstrap/checkAlbumeasyStatusAfterPull.md)
- [refreshState](../../../../../functions/src/review_ui/views/bootstrap/refreshState.md)

# Called by

- [runStampWorldPull](../../../../../functions/src/review_ui/views/bootstrap/runStampWorldPull.md)