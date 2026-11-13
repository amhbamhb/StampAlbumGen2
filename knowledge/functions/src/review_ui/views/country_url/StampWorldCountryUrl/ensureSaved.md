---
type: JavaScript Method
title: ensureSaved
resource: src/review_ui/views/country_url.js#L66-L99
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/country_url/StampWorldCountryUrl/message
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/runStampWorldPull
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/runReviewStampWorldPull
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/country_url/StampWorldCountryUrl/constructor
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async ensureSaved(country)`

# Calls

- [json](../../../../../../functions/src/external_album/json.md)
- [message](../../../../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/message.md)

# Called by

- [runStampWorldPull](../../../../../../functions/src/review_ui/views/bootstrap/runStampWorldPull.md)
- [runReviewStampWorldPull](../../../../../../functions/src/review_ui/views/compare/runReviewStampWorldPull.md)
- [constructor](../../../../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/constructor.md)