---
type: JavaScript Function
title: updateCountrySearchMatches
resource: src/review_ui/views/bootstrap.js#L409-L444
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/views/bootstrap/refreshCountrySearchDatalist
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/setCountrySearchStatus
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/renderCountryPanel
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/views/bootstrap/onCountrySelectionChanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/applyCountrySearchSelection
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/bootstrap/refreshState
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`function updateCountrySearchMatches(preserveCurrentSelection)`

# Calls

- [refreshCountrySearchDatalist](../../../../../functions/src/review_ui/views/bootstrap/refreshCountrySearchDatalist.md)
- [setCountrySearchStatus](../../../../../functions/src/review_ui/views/bootstrap/setCountrySearchStatus.md)
- [renderCountryPanel](../../../../../functions/src/review_ui/views/bootstrap/renderCountryPanel.md)

# Called by

- [onCountrySelectionChanged](../../../../../functions/src/review_ui/views/bootstrap/onCountrySelectionChanged.md)
- [applyCountrySearchSelection](../../../../../functions/src/review_ui/views/bootstrap/applyCountrySearchSelection.md)
- [refreshState](../../../../../functions/src/review_ui/views/bootstrap/refreshState.md)