---
type: Rust Method
title: new
resource: src/native_pdf/commands/page/album_pages_title.rs#L35-L62
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/units/mm_to_points
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn new( font: &str, size_pt: f64, text: &str, vspace_after_mm: f64, minimum_height_mm: f64, colour: NamedColour, ) -> Result<Self>`

# Calls

- [mm_to_points](../../../../../../../functions/src/native_pdf/units/mm_to_points.md)