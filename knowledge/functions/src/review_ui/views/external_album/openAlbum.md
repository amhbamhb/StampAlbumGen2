---
type: JavaScript Function
title: openAlbum
resource: src/review_ui/views/external_album.js#L180-L187
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/external_album/api
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/renderAlbum
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/external_album/refresh
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function openAlbum(name)`

# Calls

- [api](../../../../../functions/src/review_ui/views/external_album/api.md)
- [renderAlbum](../../../../../functions/src/review_ui/views/external_album/renderAlbum.md)

# Called by

- [refresh](../../../../../functions/src/review_ui/views/external_album/refresh.md)