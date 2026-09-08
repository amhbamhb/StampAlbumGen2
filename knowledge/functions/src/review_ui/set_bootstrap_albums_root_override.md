---
type: Rust Function
title: set_bootstrap_albums_root_override
resource: src/review_ui.rs#L1975-L2021
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/ensure_runtime_startup_files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/save_runtime_bootstrap_albums_root_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/current_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/normalize_albums_root_candidate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/is_directory_empty
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/http_handlers/serve_bootstrap_requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn set_bootstrap_albums_root_override(path: &str) -> Result<Option<String>, String>`

# Calls

- [ensure_runtime_startup_files](../../../functions/src/review_ui/ensure_runtime_startup_files.md)
- [save_runtime_bootstrap_albums_root_path](../../../functions/src/review_ui/save_runtime_bootstrap_albums_root_path.md)
- [bootstrap_albums_root_override](../../../functions/src/review_ui/bootstrap_albums_root_override.md)
- [current_bootstrap_albums_root](../../../functions/src/review_ui/current_bootstrap_albums_root.md)
- [from](../../../functions/src/setup/SetupError/from-flexi-logger-flexiloggererror/from.md)
- [normalize_albums_root_candidate](../../../functions/src/review_ui/normalize_albums_root_candidate.md)
- [is_directory_empty](../../../functions/src/review_ui/is_directory_empty.md)

# Called by

- [serve_bootstrap_requests](../../../functions/src/review_ui/http_handlers/serve_bootstrap_requests.md)