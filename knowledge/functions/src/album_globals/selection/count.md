---
type: Rust Function
title: count
resource: src/album_globals/selection.rs#L162-L164
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/count_albumeasy_text_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/count_nonempty_albumeasy_text_lines
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/layout/page_text_height
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/postprocess/center_page
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db_utils/formatting/format_stamps_issued
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/render
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/external_album/reflow/reflow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/score_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/report_generation/generate_pdf_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/wikipedia/utils/estimate_text_height
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
  - target: functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/config_template_key_usage_audit/count_occurrences
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn count(prepared: &PreparedPdfStamps) -> usize`

# Called by

- [count_albumeasy_text_lines](../../../../functions/src/album_globals/generator/count_albumeasy_text_lines.md)
- [count_nonempty_albumeasy_text_lines](../../../../functions/src/album_globals/generator/count_nonempty_albumeasy_text_lines.md)
- [generate_album_output_conn_prepared](../../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [page_text_height](../../../../functions/src/album_globals/generator_new/layout/page_text_height.md)
- [center_page](../../../../functions/src/album_globals/generator_new/postprocess/center_page.md)
- [format_stamps_issued](../../../../functions/src/db_utils/formatting/format_stamps_issued.md)
- [render](../../../../functions/src/external_album/render.md)
- [reflow](../../../../functions/src/external_album/reflow/reflow.md)
- [score_candidate](../../../../functions/src/modes/score_candidate.md)
- [generate_pdf_report](../../../../functions/src/review_ui/report_generation/generate_pdf_report.md)
- [estimate_text_height](../../../../functions/src/wikipedia/utils/estimate_text_height.md)
- [test_parameter_usage_consistency](../../../../functions/tests/album_stamp_img_integration_test/test_parameter_usage_consistency.md)
- [test_issue_resolution](../../../../functions/tests/album_stamp_img_integration_test/test_issue_resolution.md)
- [test_album_stamp_img_new_setting_parameter_usage](../../../../functions/tests/album_stamp_img_new_setting_usage/test_album_stamp_img_new_setting_parameter_usage.md)
- [spain_colnect_layout_packs_rich_rows_without_losing_stamps](../../../../functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps.md)
- [count_occurrences](../../../../functions/tests/config_template_key_usage_audit/count_occurrences.md)
- [invalid_edits_roll_back_and_render_failure_saves_retryable_draft](../../../../functions/tests/external_album_backend/invalid_edits_roll_back_and_render_failure_saves_retryable_draft.md)
- [database_write_failure_rolls_back_layout_and_render_artifacts](../../../../functions/tests/external_album_backend/database_write_failure_rolls_back_layout_and_render_artifacts.md)