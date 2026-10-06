---
type: Rust Function
title: extract_design_size
resource: src/us1847/collector.rs#L922-L973
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/us1847/collector/parse_liberty_issue_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_design_size(document: &Html) -> String`

# Calls

- [parse](../../../../functions/src/external_album/parser/parse.md)

# Called by

- [parse_liberty_issue_stamp](../../../../functions/src/us1847/collector/parse_liberty_issue_stamp.md)