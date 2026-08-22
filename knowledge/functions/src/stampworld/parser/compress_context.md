---
type: Rust Function
title: compress_context
resource: src/stampworld/parser.rs#L148-L176
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn compress_context(s: &str, max_len: usize) -> String`

# Called by

- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)