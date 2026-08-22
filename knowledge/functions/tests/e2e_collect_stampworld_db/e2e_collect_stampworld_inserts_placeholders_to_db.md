---
type: Rust Function
title: e2e_collect_stampworld_inserts_placeholders_to_db
resource: tests/e2e_collect_stampworld_db.rs#L30-L165
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn e2e_collect_stampworld_inserts_placeholders_to_db()`

# Calls

- [load_config](../../../functions/src/config/load_config.md)
- [collect_stampworld_data_with_fetcher](../../../functions/src/stampworld/data_collector/collect_stampworld_data_with_fetcher.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)