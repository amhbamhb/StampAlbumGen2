---
type: Rust Module
title: album_pages_title
resource: src/native_pdf/commands/page/album_pages_title.rs#L1-L125
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/crate-native-pdf-colour-namedcolour-document-bounds-text-measuredtext-units-mm-to-points-points-to-mm-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/oxidize-pdf-text-font
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-crate-native-pdf-text-positionedtextline-as-positionedtitleline
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [AlbumPagesTitle](../../../../../classes/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle.md)
- [TitleLayout](../../../../../classes/src/native_pdf/commands/page/album_pages_title/TitleLayout.md)
- [new](../../../../../functions/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle/new.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_title/AlbumPagesTitle/layout.md)

# Imports

- `crate::native_pdf::{
    colour::NamedColour,
    document::Bounds,
    text::MeasuredText,
    units::{mm_to_points, points_to_mm},
    Result,
}`
- `oxidize_pdf::text::Font`
- `pub use crate::native_pdf::text::PositionedTextLine as PositionedTitleLine`

# Member of

- [stamp_album_gen2](../../../../../packages/stamp_album_gen2.md)