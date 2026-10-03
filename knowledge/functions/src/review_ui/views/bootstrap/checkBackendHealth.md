---
type: JavaScript Function
title: checkBackendHealth
resource: src/review_ui/views/bootstrap.js#L77-L107
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/fetchBackendHealth
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setReconnectBanner
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/startBackendHealthPolling
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/retryBackendHealthCheck
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function checkBackendHealth()`

# Calls

- [fetchBackendHealth](../../../../../functions/src/review_ui/views/bootstrap/fetchBackendHealth.md)
- [setReconnectBanner](../../../../../functions/src/review_ui/views/bootstrap/setReconnectBanner.md)

# Called by

- [startBackendHealthPolling](../../../../../functions/src/review_ui/views/bootstrap/startBackendHealthPolling.md)
- [retryBackendHealthCheck](../../../../../functions/src/review_ui/views/bootstrap/retryBackendHealthCheck.md)