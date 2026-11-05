---
type: JavaScript Function
title: suggestedStampworldCommand
resource: src/review_ui/views/bootstrap.js#L603-L614
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/getEffectiveStampworldCountry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/stampworldPresetFlags
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/applyCountrySearchSelection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/renderCountryPanel
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function suggestedStampworldCommand(country)`

# Calls

- [getEffectiveStampworldCountry](../../../../../functions/src/review_ui/views/bootstrap/getEffectiveStampworldCountry.md)
- [stampworldPresetFlags](../../../../../functions/src/review_ui/views/bootstrap/stampworldPresetFlags.md)

# Called by

- [applyCountrySearchSelection](../../../../../functions/src/review_ui/views/bootstrap/applyCountrySearchSelection.md)
- [renderCountryPanel](../../../../../functions/src/review_ui/views/bootstrap/renderCountryPanel.md)