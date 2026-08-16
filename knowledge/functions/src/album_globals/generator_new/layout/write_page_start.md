---
type: Rust Function
title: write_page_start
resource: src/album_globals/generator_new/layout.rs#L4-L6
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/page_functions/new_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_page_start<W: Write>(out: &mut W) -> std::io::Result<()>`

# Called by

- [new_page](../../../../../functions/src/album_globals/page_functions/new_page.md)
- [write_globals](../../../../../functions/src/album_utils/globals/write_globals.md)
- [add_wikipedia_section](../../../../../functions/src/album_utils/globals/add_wikipedia_section.md)