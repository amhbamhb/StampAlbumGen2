---
type: Rust Function
title: main
resource: src/bin/inspect_image_paths.rs#L18-L97
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/inspect_image_paths/resolve_image_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn main() -> Result<(), Box<dyn std::error::Error>>`

# Calls

- [args](../../../../functions/tests/native_pdf_page_settings/args.md)
- [skip](../../../../functions/src/external_album/parser/Lexer/skip.md)
- [from](../../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [open](../../../../functions/src/external_album/open.md)
- [resolve_image_path](../../../../functions/src/bin/inspect_image_paths/resolve_image_path.md)