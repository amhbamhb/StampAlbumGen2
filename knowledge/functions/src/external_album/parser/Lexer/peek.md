---
type: Rust Method
title: peek
resource: src/external_album/parser.rs#L245-L247
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/bump
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/command
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn peek(&self) -> Option<char>`

# Calls

- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [parse](../../../../../functions/src/external_album/parser/parse.md)
- [bump](../../../../../functions/src/external_album/parser/Lexer/bump.md)
- [skip](../../../../../functions/src/external_album/parser/Lexer/skip.md)
- [command](../../../../../functions/src/external_album/parser/Lexer/command.md)