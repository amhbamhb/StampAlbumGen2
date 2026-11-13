---
type: Rust Module
title: text
resource: src/native_pdf/text.rs#L1-L248
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/super-document-bounds-units-mm-to-points-points-to-mm-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/oxidize-pdf-text-fonts-get-standard-font-metrics-measure-text-font-textencoding
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

- [TextPosition](../../../classes/src/native_pdf/text/TextPosition.md)
- [parse](../../../functions/src/native_pdf/text/TextPosition/parse.md)
- [parse_for](../../../functions/src/native_pdf/text/TextPosition/parse_for.md)
- [line_left](../../../functions/src/native_pdf/text/TextPosition/line_left.md)
- [MeasuredLine](../../../classes/src/native_pdf/text/MeasuredLine.md)
- [PositionedTextLine](../../../classes/src/native_pdf/text/PositionedTextLine.md)
- [text_lines_overlap](../../../functions/src/native_pdf/text/text_lines_overlap.md)
- [MeasuredText](../../../classes/src/native_pdf/text/MeasuredText.md)
- [new](../../../functions/src/native_pdf/text/MeasuredText/new.md)
- [with_text](../../../functions/src/native_pdf/text/MeasuredText/with_text.md)
- [new_with_font](../../../functions/src/native_pdf/text/MeasuredText/new_with_font.md)
- [line_collisions_ignore_blank_lines_hints_and_touching_edges](../../../functions/src/native_pdf/text/line_collisions_ignore_blank_lines_hints_and_touching_edges.md)

# Imports

- `super::{
    document::Bounds,
    units::{mm_to_points, points_to_mm},
    Result,
}`
- `oxidize_pdf::text::{fonts::get_standard_font_metrics, measure_text, Font, TextEncoding}`
- `super::*`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)