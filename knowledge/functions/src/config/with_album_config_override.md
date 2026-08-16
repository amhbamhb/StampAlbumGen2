---
type: Rust Function
title: with_album_config_override
resource: src/config.rs#L2366-L2373
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/src/config/scoped_album_override_applies_stamp_visibility_keys_in_effective_config
    resolved_by: tree-sitter
    confidence: exact
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
---

# Signature

`pub fn with_album_config_override<R>(album_dir: &Path, f: impl FnOnce() -> R) -> R`

# Called by

- [scoped_album_override_applies_stamp_visibility_keys_in_effective_config](../../../functions/src/config/scoped_album_override_applies_stamp_visibility_keys_in_effective_config.md)
- [run_colnect_inbox_mode](../../../functions/src/modes/run_colnect_inbox_mode.md)
- [run_database_mode](../../../functions/src/modes/run_database_mode.md)
- [run_scraping_mode](../../../functions/src/modes/run_scraping_mode.md)
- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)
- [serve_review_requests](../../../functions/src/review_ui/http_handlers/serve_review_requests.md)