---
type: JavaScript Method
title: load
resource: src/review_ui/views/country_url.js#L25-L64
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/country_url/StampWorldCountryUrl/message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/json
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/pdf_dx_logging_enabled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/suggestedStampworldCommand
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/country_url/StampWorldCountryUrl/ensureSaved
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`load(country)`

# Calls

- [message](../../../../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/message.md)
- [json](../../../../../../functions/src/external_album/json.md)

# Called by

- [pdf_dx_logging_enabled](../../../../../../functions/src/album_globals/pdf_dx_logging_enabled.md)
- [extract_stampworld_pdf_text](../../../../../../functions/src/review_ui/pdf_parser/extraction/extract_stampworld_pdf_text.md)
- [suggestedStampworldCommand](../../../../../../functions/src/review_ui/views/bootstrap/suggestedStampworldCommand.md)
- [ensureSaved](../../../../../../functions/src/review_ui/views/country_url/StampWorldCountryUrl/ensureSaved.md)