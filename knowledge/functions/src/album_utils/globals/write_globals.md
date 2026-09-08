---
type: Rust Function
title: write_globals
resource: src/album_utils/globals.rs#L7-L686
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_page_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_flag_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_coa_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_page_vspace
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/emit_image_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/add_wikipedia_section
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn write_globals<W: Write>( writer: &mut W, album_name: &str, conn_opt: Option<&rusqlite::Connection>, start_year: Option<i32>, end_year: Option<i32>, ) -> usize`

# Calls

- [formatted_now](../../../../functions/src/time_utils/formatted_now.md)
- [load_effective_config](../../../../functions/src/config/load_effective_config.md)
- [write_page_start](../../../../functions/src/album_globals/generator_new/layout/write_page_start.md)
- [copy_flag_for_album](../../../../functions/src/flagdb/copy_flag_for_album.md)
- [copy_coa_for_album](../../../../functions/src/flagdb/copy_coa_for_album.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [write_page_vspace](../../../../functions/src/album_globals/generator_new/layout/write_page_vspace.md)
- [emit_image_only](../../../../functions/src/album_globals/generator_new/emit/emit_image_only.md)
- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)