---
type: JavaScript Function
title: api
resource: src/review_ui/views/external_album.js#L31-L38
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/external_album/refresh
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/pick
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/inspect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/awaitJob
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/importAlbum
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/openAlbum
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/retry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/approve
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/upload
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function api(action, data)`

# Calls

- [json](../../../../../functions/src/external_album/json.md)

# Called by

- [refresh](../../../../../functions/src/review_ui/views/external_album/refresh.md)
- [pick](../../../../../functions/src/review_ui/views/external_album/pick.md)
- [inspect](../../../../../functions/src/review_ui/views/external_album/inspect.md)
- [awaitJob](../../../../../functions/src/review_ui/views/external_album/awaitJob.md)
- [importAlbum](../../../../../functions/src/review_ui/views/external_album/importAlbum.md)
- [openAlbum](../../../../../functions/src/review_ui/views/external_album/openAlbum.md)
- [save](../../../../../functions/src/review_ui/views/external_album/save.md)
- [retry](../../../../../functions/src/review_ui/views/external_album/retry.md)
- [approve](../../../../../functions/src/review_ui/views/external_album/approve.md)
- [upload](../../../../../functions/src/review_ui/views/external_album/upload.md)