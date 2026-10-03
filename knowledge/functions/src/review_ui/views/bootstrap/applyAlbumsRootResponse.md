---
type: JavaScript Function
title: applyAlbumsRootResponse
resource: src/review_ui/views/bootstrap.js#L276-L287
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/updateAlbumsRootHint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setAlbumsRootStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/refreshState
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/browseAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/clearAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function applyAlbumsRootResponse(j)`

# Calls

- [updateAlbumsRootHint](../../../../../functions/src/review_ui/views/bootstrap/updateAlbumsRootHint.md)
- [setAlbumsRootStatus](../../../../../functions/src/review_ui/views/bootstrap/setAlbumsRootStatus.md)
- [refreshState](../../../../../functions/src/review_ui/views/bootstrap/refreshState.md)

# Called by

- [browseAlbumsRootPath](../../../../../functions/src/review_ui/views/bootstrap/browseAlbumsRootPath.md)
- [setAlbumsRootPath](../../../../../functions/src/review_ui/views/bootstrap/setAlbumsRootPath.md)
- [clearAlbumsRootPath](../../../../../functions/src/review_ui/views/bootstrap/clearAlbumsRootPath.md)