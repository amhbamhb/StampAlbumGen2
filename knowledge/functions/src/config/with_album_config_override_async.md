---
type: Rust Function
title: with_album_config_override_async
resource: src/config.rs#L2603-L2614
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/modes/run_colnect_inbox_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_database_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/run_scraping_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/http_handlers/serve_review_requests
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn with_album_config_override_async<R, Fut>( album_dir: &Path, f: impl FnOnce() -> Fut, ) -> R where Fut: std::future::Future<Output = R>,`

# Called by

- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)
- [spain_colnect_layout_packs_rich_rows_without_losing_stamps](../../../functions/tests/colnect_layout_spain/spain_colnect_layout_packs_rich_rows_without_losing_stamps.md)