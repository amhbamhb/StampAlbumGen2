---
type: Rust Function
title: set_attr_if_non_empty
resource: src/album_globals/generator_new/helpers.rs#L334-L338
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/set
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn set_attr_if_non_empty(attrs: &mut GenericAttrs, key: &str, value: &str)`

# Calls

- [set](../../../../../functions/src/album_globals/structs/GenericAttrs/set.md)

# Called by

- [fetch_and_filter_stamps](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)