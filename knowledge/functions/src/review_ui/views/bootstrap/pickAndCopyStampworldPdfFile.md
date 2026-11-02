---
type: JavaScript Function
title: pickAndCopyStampworldPdfFile
resource: src/review_ui/views/bootstrap.js#L1069-L1101
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/appendStampworldCopyLog
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/refreshState
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/stampworld_pdf_import_ui/fixture
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function pickAndCopyStampworldPdfFile()`

# Calls

- [appendStampworldCopyLog](../../../../../functions/src/review_ui/views/bootstrap/appendStampworldCopyLog.md)
- [json](../../../../../functions/src/external_album/json.md)
- [refreshState](../../../../../functions/src/review_ui/views/bootstrap/refreshState.md)

# Called by

- [fixture](../../../../../functions/tests/stampworld_pdf_import_ui/fixture.md)