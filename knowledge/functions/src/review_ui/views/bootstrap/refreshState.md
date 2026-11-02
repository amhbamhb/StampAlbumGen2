---
type: JavaScript Function
title: refreshState
resource: src/review_ui/views/bootstrap.js#L1129-L1165
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/updateAlbumsRootHint
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setAlbumsRootStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/renderCountryPanel
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/updateCountrySearchMatches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/applyAlbumsRootResponse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/pollStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/pickAndCopyColnectFile
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/pickAndCopyStampworldPdfFile
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/submitCreateCountryOrTheme
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/importColnectFiles
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/external_album/pick
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function refreshState()`

# Calls

- [json](../../../../../functions/src/external_album/json.md)
- [updateAlbumsRootHint](../../../../../functions/src/review_ui/views/bootstrap/updateAlbumsRootHint.md)
- [setAlbumsRootStatus](../../../../../functions/src/review_ui/views/bootstrap/setAlbumsRootStatus.md)
- [renderCountryPanel](../../../../../functions/src/review_ui/views/bootstrap/renderCountryPanel.md)
- [updateCountrySearchMatches](../../../../../functions/src/review_ui/views/bootstrap/updateCountrySearchMatches.md)

# Called by

- [applyAlbumsRootResponse](../../../../../functions/src/review_ui/views/bootstrap/applyAlbumsRootResponse.md)
- [pollStampWorldStatus](../../../../../functions/src/review_ui/views/bootstrap/pollStampWorldStatus.md)
- [pickAndCopyColnectFile](../../../../../functions/src/review_ui/views/bootstrap/pickAndCopyColnectFile.md)
- [pickAndCopyStampworldPdfFile](../../../../../functions/src/review_ui/views/bootstrap/pickAndCopyStampworldPdfFile.md)
- [submitCreateCountryOrTheme](../../../../../functions/src/review_ui/views/bootstrap/submitCreateCountryOrTheme.md)
- [importColnectFiles](../../../../../functions/src/review_ui/views/bootstrap/importColnectFiles.md)
- [pick](../../../../../functions/src/review_ui/views/external_album/pick.md)