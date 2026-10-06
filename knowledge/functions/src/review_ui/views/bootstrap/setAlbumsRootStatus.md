---
type: JavaScript Function
title: setAlbumsRootStatus
resource: src/review_ui/views/bootstrap.js#L269-L276
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/add
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/beginNativeDialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/applyAlbumsRootResponse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/browseAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/clearAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/refreshState
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function setAlbumsRootStatus(text, tone)`

# Calls

- [add](../../../../../functions/src/review_ui/views/external_album/add.md)

# Called by

- [beginNativeDialog](../../../../../functions/src/review_ui/views/bootstrap/beginNativeDialog.md)
- [applyAlbumsRootResponse](../../../../../functions/src/review_ui/views/bootstrap/applyAlbumsRootResponse.md)
- [browseAlbumsRootPath](../../../../../functions/src/review_ui/views/bootstrap/browseAlbumsRootPath.md)
- [setAlbumsRootPath](../../../../../functions/src/review_ui/views/bootstrap/setAlbumsRootPath.md)
- [clearAlbumsRootPath](../../../../../functions/src/review_ui/views/bootstrap/clearAlbumsRootPath.md)
- [refreshState](../../../../../functions/src/review_ui/views/bootstrap/refreshState.md)