---
type: Rust Module
title: album_pages_footer
resource: src/native_pdf/commands/page/album_pages_footer.rs#L1-L145
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-album-pages-footer-pad-footerpadding
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-native-pdf-colour-namedcolour-document-bounds-text-measuredtext-positionedtextline-textposition-units-mm-to-points-points-to-mm-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/oxidize-pdf-text-font
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [AlbumPagesFooter](../../../../../classes/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter.md)
- [FooterLayout](../../../../../classes/src/native_pdf/commands/page/album_pages_footer/FooterLayout.md)
- [new](../../../../../functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/new.md)
- [layout](../../../../../functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/layout.md)
- [with_text](../../../../../functions/src/native_pdf/commands/page/album_pages_footer/AlbumPagesFooter/with_text.md)

# Imports

- `super::album_pages_footer_pad::FooterPadding`
- `crate::native_pdf::{
    colour::NamedColour,
    document::Bounds,
    text::{MeasuredText, PositionedTextLine, TextPosition},
    units::{mm_to_points, points_to_mm},
    Result,
}`
- `oxidize_pdf::text::Font`

# Member of

- [stamp_album_gen2](../../../../../packages/stamp_album_gen2.md)