---
type: Rust Function
title: main
resource: src/bin/stampworld_debug.rs#L71-L134
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/bin/stampworld_debug/LocalFetcher/new_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn main()`

# Calls

- [new_from_path](../../../../functions/src/bin/stampworld_debug/LocalFetcher/new_from_path.md)
- [collect_stampworld_data_with_fetcher](../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)