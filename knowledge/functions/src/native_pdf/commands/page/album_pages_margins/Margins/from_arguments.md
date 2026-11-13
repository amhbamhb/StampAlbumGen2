---
type: Rust Method
title: from_arguments
resource: src/native_pdf/commands/page/album_pages_margins.rs#L16-L24
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/commands/arguments
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/native_pdf/commands/page/album_pages_margins/Margins/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/commands/page/album_pages_marginse/EvenMargins/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn from_arguments(command: &str, arguments_mm: &[f64]) -> Result<Self>`

# Calls

- [arguments](../../../../../../../functions/src/native_pdf/commands/arguments.md)

# Called by

- [parse](../../../../../../../functions/src/native_pdf/commands/page/album_pages_margins/Margins/parse.md)
- [parse](../../../../../../../functions/src/native_pdf/commands/page/album_pages_marginse/EvenMargins/parse.md)