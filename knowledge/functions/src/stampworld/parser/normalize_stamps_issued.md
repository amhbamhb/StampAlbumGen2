---
type: Rust Function
title: normalize_stamps_issued
resource: src/stampworld/parser.rs#L9-L21
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn normalize_stamps_issued(s: &str) -> String`

# Calls

- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)