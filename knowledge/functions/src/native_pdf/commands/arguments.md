---
type: Rust Function
title: arguments
resource: src/native_pdf/commands/mod.rs#L6-L20
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_border/PageBorder/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_margins/Margins/from_arguments
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_size/PageSize/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn arguments<const N: usize>(name: &str, values: &[f64], positive: bool) -> Result<[f64; N]>`

# Called by

- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_border/PageBorder/parse.md)
- [from_arguments](../../../../functions/src/native_pdf/commands/page/album_pages_margins/Margins/from_arguments.md)
- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_size/PageSize/parse.md)
- [parse](../../../../functions/src/native_pdf/commands/page/album_pages_spacing/Spacing/parse.md)