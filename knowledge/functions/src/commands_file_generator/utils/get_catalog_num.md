---
type: Rust Function
title: get_catalog_num
resource: src/commands_file_generator/utils.rs#L47-L49
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/commands_file_generator/album_creation/create_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_catalog_num(field: &str, _prefer_full: bool) -> String`

# Called by

- [create_album](../../../../functions/src/commands_file_generator/album_creation/create_album.md)