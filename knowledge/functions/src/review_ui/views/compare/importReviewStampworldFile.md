---
type: JavaScript Function
title: importReviewStampworldFile
resource: src/review_ui/views/compare.js#L1658-L1686
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/logRuntimeCompare
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/reloadLeftRowsFromServer
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/stampworld_pdf_import_ui/fixture
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function importReviewStampworldFile()`

# Calls

- [json](../../../../../functions/src/external_album/json.md)
- [logRuntimeCompare](../../../../../functions/src/review_ui/views/compare/logRuntimeCompare.md)
- [reloadLeftRowsFromServer](../../../../../functions/src/review_ui/views/compare/reloadLeftRowsFromServer.md)

# Called by

- [fixture](../../../../../functions/tests/stampworld_pdf_import_ui/fixture.md)