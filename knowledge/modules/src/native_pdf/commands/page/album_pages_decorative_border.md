---
type: Rust Module
title: album_pages_decorative_border
resource: src/native_pdf/commands/page/album_pages_decorative_border.rs#L1-L413
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-native-pdf-colour-namedcolour-document-bounds-units-mm-to-points-points-to-mm-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/oxidize-pdf-graphics-image
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs-io-cursor-path-path-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [DecorativeBorderPolicy](../../../../../classes/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderPolicy.md)
- [parse](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderPolicy/parse.md)
- [AlbumPagesDecorativeBorder](../../../../../classes/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder.md)
- [DecorativeImageLayout](../../../../../classes/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeImageLayout.md)
- [DecorativeLine](../../../../../classes/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeLine.md)
- [DecorativeBorderLayout](../../../../../classes/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderLayout.md)
- [footprints](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/DecorativeBorderLayout/footprints.md)
- [load](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/load.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/AlbumPagesDecorativeBorder/layout.md)
- [load_decorative_png](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/load_decorative_png.md)
- [image_dimensions_mm](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/image_dimensions_mm.md)
- [validate_image_rect](../../../../../functions/src/native_pdf/commands/page/album_pages_decorative_border/validate_image_rect.md)

# Imports

- `crate::native_pdf::{
    colour::NamedColour,
    document::Bounds,
    units::{mm_to_points, points_to_mm},
    Result,
}`
- `oxidize_pdf::graphics::Image`
- `std::{fs, io::Cursor, path::Path, sync::Arc}`

# Member of

- [stamp_album_gen2](../../../../../packages/stamp_album_gen2.md)