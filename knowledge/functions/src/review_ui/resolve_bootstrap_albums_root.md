---
type: Rust Function
title: resolve_bootstrap_albums_root
resource: src/review_ui.rs#L2016-L2062
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/bootstrap_albums_root_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_bootstrap_albums_root_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/normalize_albums_root_candidate
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/review_ui/start_bootstrap_stampworld_job
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/scan_bootstrap_countries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/current_bootstrap_albums_root
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_bootstrap_albums_root() -> Option<PathBuf>`

# Calls

- [bootstrap_albums_root_override](../../../functions/src/review_ui/bootstrap_albums_root_override.md)
- [load_config](../../../functions/src/config/load_config.md)
- [effective_bootstrap_albums_root_path](../../../functions/src/config/Config/effective_bootstrap_albums_root_path.md)
- [normalize_albums_root_candidate](../../../functions/src/review_ui/normalize_albums_root_candidate.md)

# Called by

- [start_bootstrap_stampworld_job](../../../functions/src/review_ui/start_bootstrap_stampworld_job.md)
- [scan_bootstrap_countries](../../../functions/src/review_ui/scan_bootstrap_countries.md)
- [current_bootstrap_albums_root](../../../functions/src/review_ui/current_bootstrap_albums_root.md)