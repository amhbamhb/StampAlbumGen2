---
type: Rust Method
title: command
resource: src/external_album/parser.rs#L263-L342
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
  called_by:
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn command(&mut self) -> Result<LayoutCommand>`

# Calls

- [peek](../../../../../functions/src/external_album/parser/Lexer/peek.md)
- [bump](../../../../../functions/src/external_album/parser/Lexer/bump.md)

# Called by

- [parse](../../../../../functions/src/external_album/parser/parse.md)
- [main](../../../../../functions/src/main.md)