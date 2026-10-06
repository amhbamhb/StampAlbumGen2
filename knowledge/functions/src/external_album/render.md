---
type: Rust Function
title: render
resource: src/external_album.rs#L1059-L1154
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/no_links
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/err
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/pdf_valid
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn render(input: &Path, cli: &str, timeout: Duration) -> Result<()>`

# Calls

- [no_links](../../../functions/src/external_album/no_links.md)
- [err](../../../functions/src/external_album/err.md)
- [count](../../../functions/src/album_globals/selection/count.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [pdf_valid](../../../functions/src/external_album/pdf_valid.md)