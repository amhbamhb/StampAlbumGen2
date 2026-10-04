---
type: JavaScript Function
title: beginNativeDialog
resource: src/review_ui/views/bootstrap.js#L126-L135
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/setReconnectBanner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setAlbumsRootStatus
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/browseAlbumsRootPath
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/browseAlbumeasyPathFromDialog
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function beginNativeDialog(message)`

# Calls

- [setReconnectBanner](../../../../../functions/src/review_ui/views/bootstrap/setReconnectBanner.md)
- [setAlbumsRootStatus](../../../../../functions/src/review_ui/views/bootstrap/setAlbumsRootStatus.md)

# Called by

- [browseAlbumsRootPath](../../../../../functions/src/review_ui/views/bootstrap/browseAlbumsRootPath.md)
- [browseAlbumeasyPathFromDialog](../../../../../functions/src/review_ui/views/bootstrap/browseAlbumeasyPathFromDialog.md)