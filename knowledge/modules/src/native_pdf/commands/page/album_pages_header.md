---
type: Rust Module
title: album_pages_header
resource: src/native_pdf/commands/page/album_pages_header.rs#L1-L142
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-album-pages-header-pad-headerpadding
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-native-pdf-colour-namedcolour-document-bounds-text-measuredtext-positionedtextline-units-mm-to-points-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/oxidize-pdf-text-font
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-crate-native-pdf-text-textposition-as-headerposition
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [AlbumPagesHeader](../../../../../classes/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader.md)
- [HeaderLayout](../../../../../classes/src/native_pdf/commands/page/album_pages_header/HeaderLayout.md)
- [with_text](../../../../../functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/with_text.md)
- [new](../../../../../functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/new.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_header/AlbumPagesHeader/layout.md)

# Imports

- `super::album_pages_header_pad::HeaderPadding`
- `crate::native_pdf::{
    colour::NamedColour,
    document::Bounds,
    text::{MeasuredText, PositionedTextLine},
    units::mm_to_points,
    Result,
}`
- `oxidize_pdf::text::Font`
- `pub use crate::native_pdf::text::TextPosition as HeaderPosition`

# Member of

- [stamp_album_gen2](../../../../../packages/stamp_album_gen2.md)