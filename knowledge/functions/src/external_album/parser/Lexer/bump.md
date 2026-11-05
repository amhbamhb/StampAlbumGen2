---
type: Rust Method
title: bump
resource: src/external_album/parser.rs#L263-L270
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/parser/Lexer/peek
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/views/compare/inferInitialCatalogCodeFromLeftRows
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn bump(&mut self) -> Option<char>`

# Calls

- [peek](../../../../../functions/src/external_album/parser/Lexer/peek.md)

# Called by

- [parse](../../../../../functions/src/external_album/parser/parse.md)
- [skip](../../../../../functions/src/external_album/parser/Lexer/skip.md)
- [command](../../../../../functions/src/external_album/parser/Lexer/command.md)
- [inferInitialCatalogCodeFromLeftRows](../../../../../functions/src/review_ui/views/compare/inferInitialCatalogCodeFromLeftRows.md)