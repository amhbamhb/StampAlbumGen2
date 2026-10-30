---
type: Rust Method
title: clear
resource: src/progress.rs#L100-L110
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/build_stamp_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/center_tmp_pages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/full_pages_covers_and_disabled_centering_get_no_extra_space
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
  - target: functions/src/review_ui/pdf_parser/extraction/LayoutText/outputdev/end_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn clear(&self)`

# Called by

- [build_stamp_text](../../../../functions/src/album_globals/generator/build_stamp_text.md)
- [center_tmp_pages](../../../../functions/src/album_globals/generator_new/postprocess/center_tmp_pages.md)
- [full_pages_covers_and_disabled_centering_get_no_extra_space](../../../../functions/src/album_globals/generator_new/postprocess/full_pages_covers_and_disabled_centering_get_no_extra_space.md)
- [parse_csv](../../../../functions/src/colnect/parser/parse_csv.md)
- [drop](../../../../functions/src/progress/ProgressIndicator/drop/drop.md)
- [load_example_config_help_map](../../../../functions/src/review_ui/config_editor/load_example_config_help_map.md)
- [parse_template_config_sections](../../../../functions/src/review_ui/config_editor/parse_template_config_sections.md)
- [end_page](../../../../functions/src/review_ui/pdf_parser/extraction/LayoutText/outputdev/end_page.md)
- [parse_stampworld_text](../../../../functions/src/review_ui/pdf_parser/extraction/parse_stampworld_text.md)