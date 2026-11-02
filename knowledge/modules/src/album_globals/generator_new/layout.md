---
type: Rust Module
title: layout
resource: src/album_globals/generator_new/layout.rs#L1-L204
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-album-globals-generator-new-helpers-rowspacingmode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [write_page_start](../../../../functions/src/album_globals/generator_new/layout/write_page_start.md)
- [write_page_vspace](../../../../functions/src/album_globals/generator_new/layout/write_page_vspace.md)
- [write_page_vspace_marker](../../../../functions/src/album_globals/generator_new/layout/write_page_vspace_marker.md)
- [format_page_vspace_mm](../../../../functions/src/album_globals/generator_new/layout/format_page_vspace_mm.md)
- [format_page_vspace_marker](../../../../functions/src/album_globals/generator_new/layout/format_page_vspace_marker.md)
- [write_page_fit_diagnostic](../../../../functions/src/album_globals/generator_new/layout/write_page_fit_diagnostic.md)
- [write_stamp_page_header](../../../../functions/src/album_globals/generator_new/layout/write_stamp_page_header.md)
- [start_stamp_page](../../../../functions/src/album_globals/generator_new/layout/start_stamp_page.md)
- [emit_row_start](../../../../functions/src/album_globals/generator_new/layout/emit_row_start.md)
- [page_text_height](../../../../functions/src/album_globals/generator_new/layout/page_text_height.md)
- [write_header_block](../../../../functions/src/album_globals/generator_new/layout/write_header_block.md)
- [header_measurement_matches_font_spacing_and_explicit_breaks](../../../../functions/src/album_globals/generator_new/layout/header_measurement_matches_font_spacing_and_explicit_breaks.md)
- [header_spacing_is_numeric_even_with_a_legacy_marker](../../../../functions/src/album_globals/generator_new/layout/header_spacing_is_numeric_even_with_a_legacy_marker.md)

# Imports

- `std::io::Write`
- `crate::album_globals::generator_new::helpers::RowSpacingMode`
- `super::*`

# Member of

- [stamp_album_gen2](../../../../packages/stamp_album_gen2.md)