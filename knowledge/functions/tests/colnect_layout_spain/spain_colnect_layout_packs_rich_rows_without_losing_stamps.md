---
type: Rust Function
title: spain_colnect_layout_packs_rich_rows_without_losing_stamps
resource: tests/colnect_layout_spain.rs#L55-L193
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_layout_spain/copy_directory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_layout_spain/sanitized_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/with_album_config_override_async
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/Lexer/skip
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn spain_colnect_layout_packs_rich_rows_without_losing_stamps()`

# Calls

- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [copy_directory](../../../functions/tests/colnect_layout_spain/copy_directory.md)
- [sanitized_config](../../../functions/tests/colnect_layout_spain/sanitized_config.md)
- [with_album_config_override_async](../../../functions/src/config/with_album_config_override_async.md)
- [count](../../../functions/src/album_globals/selection/count.md)
- [skip](../../../functions/src/external_album/parser/Lexer/skip.md)
- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)