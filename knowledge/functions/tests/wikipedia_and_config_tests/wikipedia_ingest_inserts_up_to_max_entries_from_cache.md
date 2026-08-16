---
type: Rust Function
title: wikipedia_ingest_inserts_up_to_max_entries_from_cache
resource: tests/wikipedia_and_config_tests.rs#L62-L116
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/ingest/ingest_wikipedia_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn wikipedia_ingest_inserts_up_to_max_entries_from_cache()`

# Calls

- [ingest_wikipedia_to_db](../../../functions/src/wikipedia/ingest/ingest_wikipedia_to_db.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)