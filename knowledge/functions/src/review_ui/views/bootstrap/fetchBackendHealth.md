---
type: JavaScript Function
title: fetchBackendHealth
resource: src/review_ui/views/bootstrap.js#L65-L77
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/checkBackendHealth
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function fetchBackendHealth()`

# Calls

- [json](../../../../../functions/src/external_album/json.md)

# Called by

- [checkBackendHealth](../../../../../functions/src/review_ui/views/bootstrap/checkBackendHealth.md)