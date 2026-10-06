---
type: JavaScript Function
title: runStampWorldPull
resource: src/review_ui/views/bootstrap.js#L720-L771
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/getEffectiveStampworldCountry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/updateStampWorldRunStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/country_url/StampWorldCountryUrl/ensureSaved
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/stampworld_start_error/showStampWorldStartError
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/pollStampWorldStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/checkAlbumeasyStatusAfterPull
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async function runStampWorldPull()`

# Calls

- [getEffectiveStampworldCountry](../../../../../functions/src/review_ui/views/bootstrap/getEffectiveStampworldCountry.md)
- [updateStampWorldRunStatus](../../../../../functions/src/review_ui/views/bootstrap/updateStampWorldRunStatus.md)
- [ensureSaved](../../../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/ensureSaved.md)
- [json](../../../../../functions/src/external_album/json.md)
- [showStampWorldStartError](../../../../../functions/src/review_ui/views/stampworld_start_error/showStampWorldStartError.md)
- [pollStampWorldStatus](../../../../../functions/src/review_ui/views/bootstrap/pollStampWorldStatus.md)
- [checkAlbumeasyStatusAfterPull](../../../../../functions/src/review_ui/views/bootstrap/checkAlbumeasyStatusAfterPull.md)