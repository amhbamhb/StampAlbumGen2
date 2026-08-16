---
type: Rust Function
title: dedupe_details
resource: src/stampworld/parser.rs#L22-L36
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/data_collector/test_dedupe_details_preserves_order
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn dedupe_details(details: &[String]) -> Vec<String>`

# Called by

- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [test_dedupe_details_preserves_order](../../../../functions/tests/data_collector/test_dedupe_details_preserves_order.md)