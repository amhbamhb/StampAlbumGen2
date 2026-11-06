---
type: Rust Method
title: border_line_colour
resource: src/native_pdf/document.rs#L213-L223
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/renderer/write_proof
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn border_line_colour( &self, line: super::commands::page::album_pages_border::BorderLine, ) -> AlbumBorderColour`

# Called by

- [write_proof](../../../../../functions/src/native_pdf/renderer/write_proof.md)