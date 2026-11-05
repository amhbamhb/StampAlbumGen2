---
type: Rust Function
title: ensure_first_run_installation
resource: src/setup.rs#L945-L1005
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/first_run_decision
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/document/PageSettings/set_title
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/native_pdf_page_settings/args
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn ensure_first_run_installation() -> std::io::Result<()>`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [first_run_decision](../../../functions/src/setup/first_run_decision.md)
- [set_title](../../../functions/src/native_pdf/document/PageSettings/set_title.md)
- [args](../../../functions/tests/native_pdf_page_settings/args.md)
- [skip](../../../functions/src/external_album/parser/Lexer/skip.md)

# Called by

- [main](../../../functions/src/main.md)