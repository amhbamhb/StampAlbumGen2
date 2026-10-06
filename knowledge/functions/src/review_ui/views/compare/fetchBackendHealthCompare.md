---
type: JavaScript Function
title: fetchBackendHealthCompare
resource: src/review_ui/views/compare.js#L249-L261
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/checkBackendHealthCompare
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function fetchBackendHealthCompare()`

# Calls

- [json](../../../../../functions/src/external_album/json.md)

# Called by

- [checkBackendHealthCompare](../../../../../functions/src/review_ui/views/compare/checkBackendHealthCompare.md)