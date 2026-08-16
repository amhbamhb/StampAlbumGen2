---
type: Rust Method
title: clear
resource: src/progress.rs#L100-L110
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/parser/parse_csv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/progress/ProgressIndicator/drop/drop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/load_example_config_help_map
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/parse_template_config_sections
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn clear(&self)`

# Called by

- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [parse_csv](../../../../functions/src/colnect/parser/parse_csv.md)
- [drop](../../../../functions/src/progress/ProgressIndicator/drop/drop.md)
- [load_example_config_help_map](../../../../functions/src/review_ui/config_editor/load_example_config_help_map.md)
- [parse_template_config_sections](../../../../functions/src/review_ui/config_editor/parse_template_config_sections.md)