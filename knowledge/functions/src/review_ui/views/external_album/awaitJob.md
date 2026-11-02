---
type: JavaScript Function
title: awaitJob
resource: src/review_ui/views/external_album.js#L143-L162
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/api
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/external_album/importAlbum
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/retry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function awaitJob(result, statusId)`

# Calls

- [api](../../../../../functions/src/review_ui/views/external_album/api.md)

# Called by

- [importAlbum](../../../../../functions/src/review_ui/views/external_album/importAlbum.md)
- [save](../../../../../functions/src/review_ui/views/external_album/save.md)
- [retry](../../../../../functions/src/review_ui/views/external_album/retry.md)