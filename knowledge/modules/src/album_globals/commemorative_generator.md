---
type: Rust Module
title: commemorative_generator
resource: src/album_globals/commemorative_generator.rs#L1-L398
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-album-globals-structs-stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [get_commemorative_range](../../../functions/src/album_globals/commemorative_generator/get_commemorative_range.md)
- [is_commemorative_for_year](../../../functions/src/album_globals/commemorative_generator/is_commemorative_for_year.md)
- [is_commemorative_series](../../../functions/src/album_globals/commemorative_generator/is_commemorative_series.md)
- [is_special_stamp](../../../functions/src/album_globals/commemorative_generator/is_special_stamp.md)
- [clean_text](../../../functions/src/album_globals/commemorative_generator/clean_text.md)
- [emit_header_pages](../../../functions/src/album_globals/commemorative_generator/emit_header_pages.md)
- [emit_commemorative_pages](../../../functions/src/album_globals/commemorative_generator/emit_commemorative_pages.md)
- [emit_special_stamps_page](../../../functions/src/album_globals/commemorative_generator/emit_special_stamps_page.md)
- [emit_summary_page](../../../functions/src/album_globals/commemorative_generator/emit_summary_page.md)

# Imports

- `crate::album_globals::structs::Stamp`
- `std::io::Write`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)