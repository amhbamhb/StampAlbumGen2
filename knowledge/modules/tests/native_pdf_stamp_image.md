---
type: Rust Module
title: native_pdf_stamp_image
resource: tests/native_pdf_stamp_image.rs#L1-L215
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/oxidize-pdf-parser-pdfdocument
    resolved_by: tree-sitter
    confidence: exact
  - target: external/stamp-album-gen2-native-pdf-colour-namedcolour-commands-colour-colour-album-border-albumbordercolour-page-album-pages-border-pageborder-album-pages-margins-margins-album-pages-size-pagesize-album-pages-spacing-spacing-document-bounds-pagesettings-proof-geometryproofsettings-renderer-write-stamp-box-geometry-proof-write-stamp-image-proof-stamp-box-stampboxgeometry-stamp-image-stampimageproofsettings-stamptextproofsettings-units-mm-to-points-points-to-mm
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

- [settings](../../functions/tests/native_pdf_stamp_image/settings.md)
- [proof](../../functions/tests/native_pdf_stamp_image/proof.md)
- [make_stamp_image](../../functions/tests/native_pdf_stamp_image/make_stamp_image.md)
- [stamp_image_heading_caption_and_page_border_render_with_physical_geometry](../../functions/tests/native_pdf_stamp_image/stamp_image_heading_caption_and_page_border_render_with_physical_geometry.md)
- [invalid_image_does_not_create_or_truncate_the_output](../../functions/tests/native_pdf_stamp_image/invalid_image_does_not_create_or_truncate_the_output.md)
- [illustration_scale_changes_pdf_image_but_not_known_size_or_frame](../../functions/tests/native_pdf_stamp_image/illustration_scale_changes_pdf_image_but_not_known_size_or_frame.md)

# Imports

- `oxidize_pdf::parser::PdfDocument`
- `stamp_album_gen2::native_pdf::{
    colour::NamedColour,
    commands::{
        colour::colour_album_border::AlbumBorderColour,
        page::{
            album_pages_border::PageBorder, album_pages_margins::Margins,
            album_pages_size::PageSize, album_pages_spacing::Spacing,
        },
    },
    document::{Bounds, PageSettings},
    proof::GeometryProofSettings,
    renderer::{write_stamp_box_geometry_proof, write_stamp_image_proof},
    stamp_box::StampBoxGeometry,
    stamp_image::{StampImageProofSettings, StampTextProofSettings},
    units::{mm_to_points, points_to_mm},
}`
- `std::{error::Error, fs, path::Path}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)