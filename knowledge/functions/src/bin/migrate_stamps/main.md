---
type: Rust Function
title: main
resource: src/bin/migrate_stamps.rs#L5-L78
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn main() -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [formatted_now](../../../../functions/src/time_utils/formatted_now.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)