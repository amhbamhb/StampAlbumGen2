---
type: Rust Method
title: get_stampworld_image
resource: src/commands_file_generator/generator.rs#L110-L113
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/data_sources/stampworld/collect_stampworld_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_stampworld_image(&mut self, _stamp: &Stamp) -> Option<String>`

# Called by

- [collect_stampworld_data](../../../../../functions/src/data_sources/stampworld/collect_stampworld_data.md)
- [collect_stampworld_data_with_fetcher](../../../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)