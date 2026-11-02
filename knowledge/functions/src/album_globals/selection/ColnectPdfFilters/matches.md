---
type: Rust Method
title: matches
resource: src/album_globals/selection.rs#L40-L50
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/count_albumeasy_text_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/page_text_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/selection/prepare_pdf_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/formatting/format_stamps_issued
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_stamp_img_integration_test/test_parameter_usage_consistency
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_stamp_img_integration_test/test_issue_resolution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_stamp_img_new_setting_usage/test_album_stamp_img_new_setting_parameter_usage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn matches(&self, stamp: &Stamp) -> bool`

# Calls

- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [count_albumeasy_text_lines](../../../../../functions/src/album_globals/generator/count_albumeasy_text_lines.md)
- [page_text_height](../../../../../functions/src/album_globals/generator_new/layout/page_text_height.md)
- [prepare_pdf_stamps](../../../../../functions/src/album_globals/selection/prepare_pdf_stamps.md)
- [format_stamps_issued](../../../../../functions/src/db_utils/formatting/format_stamps_issued.md)
- [test_parameter_usage_consistency](../../../../../functions/tests/album_stamp_img_integration_test/test_parameter_usage_consistency.md)
- [test_issue_resolution](../../../../../functions/tests/album_stamp_img_integration_test/test_issue_resolution.md)
- [test_album_stamp_img_new_setting_parameter_usage](../../../../../functions/tests/album_stamp_img_new_setting_usage/test_album_stamp_img_new_setting_parameter_usage.md)