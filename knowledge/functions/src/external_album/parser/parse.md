---
type: Rust Function
title: parse
resource: src/external_album/parser.rs#L209-L251
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/peek
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/bump
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/validate_document
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(super) fn parse(text: &str) -> (Vec<LayoutCommand>, Vec<LayoutPage>, Vec<String>)`

# Calls

- [skip](../../../../functions/src/external_album/parser/Lexer/skip.md)
- [peek](../../../../functions/src/external_album/parser/Lexer/peek.md)
- [bump](../../../../functions/src/external_album/parser/Lexer/bump.md)
- [validate_document](../../../../functions/src/external_album/parser/validate_document.md)