---
type: JavaScript Function
title: checkAlbumeasyStatusAfterPull
resource: src/review_ui/views/bootstrap.js#L907-L920
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/showAlbumeasyPathDialog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/saveAlbumeasyPath
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/pollStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/runStampWorldPull
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function checkAlbumeasyStatusAfterPull()`

# Calls

- [json](../../../../../functions/src/external_album/json.md)
- [showAlbumeasyPathDialog](../../../../../functions/src/review_ui/views/bootstrap/showAlbumeasyPathDialog.md)
- [saveAlbumeasyPath](../../../../../functions/src/review_ui/views/bootstrap/saveAlbumeasyPath.md)

# Called by

- [pollStampWorldStatus](../../../../../functions/src/review_ui/views/bootstrap/pollStampWorldStatus.md)
- [runStampWorldPull](../../../../../functions/src/review_ui/views/bootstrap/runStampWorldPull.md)