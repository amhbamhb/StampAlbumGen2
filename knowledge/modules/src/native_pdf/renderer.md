---
type: Rust Module
title: renderer
resource: src/native_pdf/renderer.rs#L1-L335
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-colour-namedcolour-document-pagesettings-proof-geometryproofsettings-stamp-box-stampboxgeometry-stamp-image-stampimageproofsettings-text-positionedtextline-units-mm-to-points-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/oxidize-pdf-color-document-page
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [write_page_geometry_proof](../../../functions/src/native_pdf/renderer/write_page_geometry_proof.md)
- [write_page_geometry_proof_with_date](../../../functions/src/native_pdf/renderer/write_page_geometry_proof_with_date.md)
- [write_stamp_image_proof](../../../functions/src/native_pdf/renderer/write_stamp_image_proof.md)
- [write_stamp_box_geometry_proof](../../../functions/src/native_pdf/renderer/write_stamp_box_geometry_proof.md)
- [write_proof](../../../functions/src/native_pdf/renderer/write_proof.md)
- [draw_text_lines](../../../functions/src/native_pdf/renderer/draw_text_lines.md)

# Imports

- `super::{
    colour::NamedColour, document::PageSettings, proof::GeometryProofSettings,
    stamp_box::StampBoxGeometry, stamp_image::StampImageProofSettings, text::PositionedTextLine,
    units::mm_to_points, Result,
}`
- `oxidize_pdf::{Color, Document, Page}`
- `std::path::Path`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)