---
type: Rust Function
title: build_matcher_payload
resource: src/review_ui.rs#L2631-L2699
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

`fn build_matcher_payload( conn: &Connection, db_path: &str, album_name: &str, opts: &MatcherPayloadOptions<'_>, show_imageless_left_rows: bool, ) -> Result<MatcherPayload, Box<dyn std::error::Error>>`

# Calls

- [load_left_stamps](../../../functions/src/review_ui/matcher_service/load_left_stamps.md)
- [load_right_candidates](../../../functions/src/review_ui/matcher_service/load_right_candidates.md)
- [album_dir_for_ui](../../../functions/src/review_ui/album_dir_for_ui.md)
- [count_colnect_csv_files_rec](../../../functions/src/review_ui/count_colnect_csv_files_rec.md)

# Called by

- [run_review_ui](../../../functions/src/review_ui/run_review_ui.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)