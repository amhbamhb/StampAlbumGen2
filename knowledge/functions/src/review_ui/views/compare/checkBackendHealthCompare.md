---
type: JavaScript Function
title: checkBackendHealthCompare
resource: src/review_ui/views/compare.js#L263-L299
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/fetchBackendHealthCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/setReconnectBannerCompare
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/startBackendHealthPollingCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/retryBackendHealthCheckCompare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function checkBackendHealthCompare()`

# Calls

- [fetchBackendHealthCompare](../../../../../functions/src/review_ui/views/compare/fetchBackendHealthCompare.md)
- [setReconnectBannerCompare](../../../../../functions/src/review_ui/views/compare/setReconnectBannerCompare.md)

# Called by

- [startBackendHealthPollingCompare](../../../../../functions/src/review_ui/views/compare/startBackendHealthPollingCompare.md)
- [retryBackendHealthCheckCompare](../../../../../functions/src/review_ui/views/compare/retryBackendHealthCheckCompare.md)