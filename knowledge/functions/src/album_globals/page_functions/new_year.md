---
type: Rust Function
title: new_year
resource: src/album_globals/page_functions.rs#L73-L81
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/generator_new/helpers/build_page_text_template
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/page_functions/new_row
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/commands_file_generator/album_creation/create_album
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn new_year<W: std::io::Write>(writer: &mut W, year: &str, config_opt: Option<&Config>)`

# Calls

- [build_page_text_template](../../../../functions/src/album_globals/generator_new/helpers/build_page_text_template.md)

# Called by

- [new_row](../../../../functions/src/album_globals/page_functions/new_row.md)
- [create_album](../../../../functions/src/commands_file_generator/album_creation/create_album.md)