---
type: Rust Method
title: command
resource: src/external_album/parser.rs#L286-L367
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/Lexer/peek
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/bump
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn command(&mut self) -> Result<LayoutCommand>`

# Calls

- [peek](../../../../../functions/src/external_album/parser/Lexer/peek.md)
- [bump](../../../../../functions/src/external_album/parser/Lexer/bump.md)