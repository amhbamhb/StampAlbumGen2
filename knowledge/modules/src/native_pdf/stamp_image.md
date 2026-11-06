---
type: Rust Module
title: stamp_image
resource: src/native_pdf/stamp_image.rs#L1-L403
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-colour-namedcolour-document-bounds-pagelayout-stamp-box-stampboxgeometry-text-measuredtext-positionedtextline-units-mm-to-points-points-to-mm-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/oxidize-pdf-image
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-native-pdf-commands-colour-colour-album-border-albumbordercolour-page-album-pages-border-pageborder-album-pages-margins-margins-album-pages-size-pagesize-album-pages-spacing-spacing-document-pagesettings
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [StampTextProofSettings](../../../classes/src/native_pdf/stamp_image/StampTextProofSettings.md)
- [new](../../../functions/src/native_pdf/stamp_image/StampTextProofSettings/new.md)
- [StampImageProofSettings](../../../classes/src/native_pdf/stamp_image/StampImageProofSettings.md)
- [with_stamp_box](../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/with_stamp_box.md)
- [validate_numbers](../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/validate_numbers.md)
- [prepare](../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)
- [PreparedStampImage](../../../classes/src/native_pdf/stamp_image/PreparedStampImage.md)
- [PreparedStampText](../../../classes/src/native_pdf/stamp_image/PreparedStampText.md)
- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)
- [position_text](../../../functions/src/native_pdf/stamp_image/position_text.md)
- [page_layout](../../../functions/src/native_pdf/stamp_image/page_layout.md)
- [natural_size_image_is_centered_in_padded_box_without_upscaling](../../../functions/src/native_pdf/stamp_image/natural_size_image_is_centered_in_padded_box_without_upscaling.md)
- [embedded_reference_scale_matches_great_britain_albumeasy_footprints](../../../functions/src/native_pdf/stamp_image/embedded_reference_scale_matches_great_britain_albumeasy_footprints.md)

# Imports

- `super::{
    colour::NamedColour,
    document::{Bounds, PageLayout},
    stamp_box::StampBoxGeometry,
    text::{MeasuredText, PositionedTextLine},
    units::{mm_to_points, points_to_mm},
    Result,
}`
- `oxidize_pdf::Image`
- `std::path::PathBuf`
- `super::*`
- `crate::native_pdf::{
        commands::{
            colour::colour_album_border::AlbumBorderColour,
            page::{
                album_pages_border::PageBorder, album_pages_margins::Margins,
                album_pages_size::PageSize, album_pages_spacing::Spacing,
            },
        },
        document::PageSettings,
    }`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)