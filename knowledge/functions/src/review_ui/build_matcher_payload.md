---
type: Rust Function
title: build_matcher_payload
resource: src/review_ui.rs#L2199-L2263
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/matcher_service/load_left_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/load_right_candidates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/album_dir_for_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/count_colnect_csv_files_rec
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/run_review_ui
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_matcher_payload( conn: &Connection, db_path: &str, album_name: &str, default_catalog_code: &str, default_sort_mode: &str, odd_row_color: &str, even_row_color: &str, first_column_color: &str, second_column_color: &str, series_default_checked: bool, description_default_checked: bool, watermark_default_checked: bool, design_default_checked: bool, engraving_default_checked: bool, perforation_default_checked: bool, context_default_checked: bool, colnect_links_default_checked: bool, colnect_image_url_default_checked: bool, colnect_metadata_default_checked: bool, replace_sg_number_default_checked: bool, right_unmatched_only_default_checked: bool, albumeasy_auto_generate_pdf_default: bool, albumeasy_cli_configured: bool, catalog_family_options: &[String], show_imageless_left_rows: bool, ) -> Result<MatcherPayload, Box<dyn std::error::Error>>`

# Calls

- [load_left_stamps](../../../functions/src/review_ui/matcher_service/load_left_stamps.md)
- [load_right_candidates](../../../functions/src/review_ui/matcher_service/load_right_candidates.md)
- [album_dir_for_ui](../../../functions/src/review_ui/album_dir_for_ui.md)
- [count_colnect_csv_files_rec](../../../functions/src/review_ui/count_colnect_csv_files_rec.md)

# Called by

- [run_review_ui](../../../functions/src/review_ui/run_review_ui.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)