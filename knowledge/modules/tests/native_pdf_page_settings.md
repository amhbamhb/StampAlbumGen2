---
type: Rust Module
title: native_pdf_page_settings
resource: tests/native_pdf_page_settings.rs#L1-L338
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/oxidize-pdf-parser-pdfdocument
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-commands-colour-colour-album-border-albumbordercolour-commands-page-album-pages-border-pageborder-album-pages-margins-margins-album-pages-marginse-evenmargins-album-pages-size-pagesize-album-pages-spacing-spacing-document-pagesettings-units-mm-to-points-points-to-mm
    resolved_by: tree-sitter
    confidence: exact
  - target: external/proof-write-page-geometry-proof
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-error-error-fs-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [fixture](../../functions/tests/native_pdf_page_settings/fixture.md)
- [args](../../functions/tests/native_pdf_page_settings/args.md)
- [settings](../../functions/tests/native_pdf_page_settings/settings.md)
- [near](../../functions/tests/native_pdf_page_settings/near.md)
- [four_page_sizes_render_and_read_back](../../functions/tests/native_pdf_page_settings/four_page_sizes_render_and_read_back.md)
- [margin_and_even_page_bounds_match_frozen_expectations](../../functions/tests/native_pdf_page_settings/margin_and_even_page_bounds_match_frozen_expectations.md)
- [spacing_state_matches_reference_content_anchors_and_gaps](../../functions/tests/native_pdf_page_settings/spacing_state_matches_reference_content_anchors_and_gaps.md)
- [invalid_commands_geometry_and_output_fail_explicitly](../../functions/tests/native_pdf_page_settings/invalid_commands_geometry_and_output_fail_explicitly.md)
- [missing_commands_and_arguments_use_frozen_negative_inputs](../../functions/tests/native_pdf_page_settings/missing_commands_and_arguments_use_frozen_negative_inputs.md)
- [attribute](../../functions/tests/native_pdf_page_settings/attribute.md)
- [rendered_paths_match_margin_and_spacing_fixtures](../../functions/tests/native_pdf_page_settings/rendered_paths_match_margin_and_spacing_fixtures.md)

# Imports

- `oxidize_pdf::parser::PdfDocument`
- `serde_json::Value`
- `stamp_album_gen2::native_pdf::{
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder, album_pages_margins::Margins,
        album_pages_marginse::EvenMargins, album_pages_size::PageSize,
        album_pages_spacing::Spacing,
    },
    document::PageSettings,
    units::{mm_to_points, points_to_mm},
}`
- `proof::write_page_geometry_proof`
- `std::{error::Error, fs, path::Path}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)