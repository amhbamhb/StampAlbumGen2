---
type: Rust Function
title: write_globals
resource: src/album_utils/globals.rs#L37-L721
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/date/DateFormat/format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/time_utils/formatted_now
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/write_page_start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/file_utils/resolve_album_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_flag_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/cover_image_reference
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/copy_coa_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
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

`pub async fn write_globals<W: Write>( writer: &mut W, album_name: &str, conn_opt: Option<&rusqlite::Connection>, start_year: Option<i32>, end_year: Option<i32>, ) -> usize`

# Calls

- [format](../../../../functions/src/native_pdf/date/DateFormat/format.md)
- [formatted_now](../../../../functions/src/time_utils/formatted_now.md)
- [args](../../../../functions/tests/native_pdf_page_settings/args.md)
- [load_effective_config](../../../../functions/src/config/load_effective_config.md)
- [write_page_start](../../../../functions/src/album_globals/generator_new/layout/write_page_start.md)
- [resolve_album_dir](../../../../functions/src/file_utils/resolve_album_dir.md)
- [copy_flag_for_album](../../../../functions/src/flagdb/copy_flag_for_album.md)
- [cover_image_reference](../../../../functions/src/album_utils/globals/cover_image_reference.md)
- [copy_coa_for_album](../../../../functions/src/flagdb/copy_coa_for_album.md)
- [contains](../../../../functions/src/native_pdf/stamp_image/contains.md)
- [as_str](../../../../functions/src/wikipedia/models/ContentSource/as_str.md)
- [write_page_vspace](../../../../functions/src/album_globals/generator_new/layout/write_page_vspace.md)
- [emit_image_only](../../../../functions/src/album_globals/generator_new/emit/emit_image_only.md)
- [add_wikipedia_section](../../../../functions/src/album_utils/globals/add_wikipedia_section.md)