---
type: JavaScript Function
title: ensureRightImageUrl
resource: src/review_ui/views/compare.js#L1208-L1236
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/compare/b
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/currentRightCandidate
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/compare/render
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function ensureRightImageUrl(r)`

# Calls

- [b](../../../../../functions/src/review_ui/views/compare/b.md)
- [json](../../../../../functions/src/external_album/json.md)
- [currentRightCandidate](../../../../../functions/src/review_ui/views/compare/currentRightCandidate.md)

# Called by

- [render](../../../../../functions/src/review_ui/views/compare/render.md)