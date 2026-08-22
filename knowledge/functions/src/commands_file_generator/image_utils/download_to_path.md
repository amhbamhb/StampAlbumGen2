---
type: Rust Function
title: download_to_path
resource: src/commands_file_generator/image_utils.rs#L20-L68
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn download_to_path( url: &str, dest: &std::path::Path, _no_ssl_check: bool, ) -> Result<(), String>`

# Calls

- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)