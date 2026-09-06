---
type: Rust Method
title: effective_default_catalog_code
resource: src/config.rs#L1458-L1465
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/sync_missing_stamps_from_colnect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/apply_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_left_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn effective_default_catalog_code(&self) -> String`

# Called by

- [generate_album_output_conn](../../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [resolve_stamp_heading_label](../../../../functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label.md)
- [fetch_and_filter_stamps](../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)
- [effective_stamp_heading_label](../../../../functions/src/config/Config/effective_stamp_heading_label.md)
- [main](../../../../functions/src/main.md)
- [run_colnect_inbox_mode](../../../../functions/src/modes/run_colnect_inbox_mode.md)
- [sync_missing_stamps_from_colnect](../../../../functions/src/modes/sync_missing_stamps_from_colnect.md)
- [run_database_mode](../../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../../functions/src/modes/run_scraping_mode.md)
- [apply_matches](../../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [add_colnect_stamp_to_stamps](../../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)
- [add_left_stamp](../../../../functions/src/review_ui/matcher_service/add_left_stamp.md)