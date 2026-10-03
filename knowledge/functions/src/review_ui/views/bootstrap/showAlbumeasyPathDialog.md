---
type: JavaScript Function
title: showAlbumeasyPathDialog
resource: src/review_ui/views/bootstrap.js#L830-L891
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/ensureAlbumeasyPathDialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/browseAlbumeasyPathFromDialog
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/checkAlbumeasyStatusAfterPull
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function showAlbumeasyPathDialog(statusPayload)`

# Calls

- [ensureAlbumeasyPathDialog](../../../../../functions/src/review_ui/views/bootstrap/ensureAlbumeasyPathDialog.md)
- [browseAlbumeasyPathFromDialog](../../../../../functions/src/review_ui/views/bootstrap/browseAlbumeasyPathFromDialog.md)

# Called by

- [checkAlbumeasyStatusAfterPull](../../../../../functions/src/review_ui/views/bootstrap/checkAlbumeasyStatusAfterPull.md)