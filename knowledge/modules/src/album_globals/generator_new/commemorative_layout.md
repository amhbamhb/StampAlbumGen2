---
type: Rust Module
title: commemorative_layout
resource: src/album_globals/generator_new/commemorative_layout.rs#L1-L297
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

- [clean_for_output](../../../../functions/src/album_globals/generator_new/commemorative_layout/clean_for_output.md)
- [is_commemorative](../../../../functions/src/album_globals/generator_new/commemorative_layout/is_commemorative.md)
- [emit_commemorative_page_start](../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page_start.md)
- [emit_stamp_grid_row](../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_stamp_grid_row.md)
- [emit_description_boxes](../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_description_boxes.md)
- [build_stamp_description](../../../../functions/src/album_globals/generator_new/commemorative_layout/build_stamp_description.md)
- [emit_commemorative_page_end](../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page_end.md)
- [emit_commemorative_page](../../../../functions/src/album_globals/generator_new/commemorative_layout/emit_commemorative_page.md)

# Imports

- `crate::album_globals::structs::Stamp`
- `std::io::Write`

# Member of

- [stamp_album_gen2](../../../../packages/stamp_album_gen2.md)