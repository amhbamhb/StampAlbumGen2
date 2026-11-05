---
type: Rust Module
title: native_pdf_page_border
resource: tests/native_pdf_page_border.rs#L1-L272
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/sha2-digest-sha256
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-commands-colour-colour-album-border-albumbordercolour-commands-page-album-pages-border-pageborder-album-pages-margins-margins-album-pages-size-pagesize-album-pages-spacing-spacing-document-bounds-pagesettings-units-points-to-mm
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

- [root](../../functions/tests/native_pdf_page_border/root.md)
- [manifest](../../functions/tests/native_pdf_page_border/manifest.md)
- [border](../../functions/tests/native_pdf_page_border/border.md)
- [settings](../../functions/tests/native_pdf_page_border/settings.md)
- [near](../../functions/tests/native_pdf_page_border/near.md)
- [numbers](../../functions/tests/native_pdf_page_border/numbers.md)
- [frozen_reference_identities_and_text](../../functions/tests/native_pdf_page_border/frozen_reference_identities_and_text.md)
- [border_geometry_and_content_occupancy_match_reference](../../functions/tests/native_pdf_page_border/border_geometry_and_content_occupancy_match_reference.md)
- [invalid_border_arguments_and_unusable_strokes_fail_before_output](../../functions/tests/native_pdf_page_border/invalid_border_arguments_and_unusable_strokes_fail_before_output.md)
- [attribute](../../functions/tests/native_pdf_page_border/attribute.md)
- [trace](../../functions/tests/native_pdf_page_border/trace.md)
- [path_bounds](../../functions/tests/native_pdf_page_border/path_bounds.md)
- [rendered_border_paths_match_frozen_references](../../functions/tests/native_pdf_page_border/rendered_border_paths_match_frozen_references.md)

# Imports

- `serde_json::Value`
- `sha2::{Digest, Sha256}`
- `stamp_album_gen2::native_pdf::{
    commands::colour::colour_album_border::AlbumBorderColour,
    commands::page::{
        album_pages_border::PageBorder, album_pages_margins::Margins, album_pages_size::PageSize,
        album_pages_spacing::Spacing,
    },
    document::{Bounds, PageSettings},
    units::points_to_mm,
}`
- `proof::write_page_geometry_proof`
- `std::{error::Error, fs, path::Path}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)