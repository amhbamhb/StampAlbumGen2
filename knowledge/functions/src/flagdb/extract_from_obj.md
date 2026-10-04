---
type: Rust Function
title: extract_from_obj
resource: src/flagdb.rs#L416-L430
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_from_obj(o: &serde_json::Value) -> Option<String>`

# Calls

- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [ensure_coat_of_arms_with_fetcher](../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)