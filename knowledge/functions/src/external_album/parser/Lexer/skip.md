---
type: Rust Method
title: skip
resource: src/external_album/parser.rs#L271-L285
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
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/bin/inspect_image_paths/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ensure_config_and_template_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/parser/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/ensure_first_run_installation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn skip(&mut self) -> bool`

# Calls

- [peek](../../../../../functions/src/external_album/parser/Lexer/peek.md)
- [bump](../../../../../functions/src/external_album/parser/Lexer/bump.md)

# Called by

- [build_stamp_text](../../../../../functions/src/album_globals/generator/build_stamp_text.md)
- [main](../../../../../functions/src/bin/inspect_image_paths/main.md)
- [ensure_config_and_template_files](../../../../../functions/src/config/ensure_config_and_template_files.md)
- [parse](../../../../../functions/src/external_album/parser/parse.md)
- [ensure_first_run_installation](../../../../../functions/src/setup/ensure_first_run_installation.md)
- [spain_colnect_layout_packs_rich_rows_without_losing_stamps](../../../../../functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps.md)