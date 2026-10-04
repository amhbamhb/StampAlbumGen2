---
type: Rust Function
title: ensure_flag_image_with_fetcher
resource: src/flagdb.rs#L150-L164
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/ensure_flag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/assets/flag_directories
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/flagdb/ensure_flag_image
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/flag_pipeline/public_pipeline_merges_overrides_honors_refresh_and_emits_local_cover_flag
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn ensure_flag_image_with_fetcher( alpha2: &str, size: &str, fetcher: &dyn Fetcher, ) -> Result<String, Box<dyn std::error::Error>>`

# Calls

- [load_effective_config](../../../functions/src/config/load_effective_config.md)
- [ensure_flag](../../../functions/src/flagdb/assets/ensure_flag.md)
- [flag_directories](../../../functions/src/flagdb/assets/flag_directories.md)

# Called by

- [ensure_flag_image](../../../functions/src/flagdb/ensure_flag_image.md)
- [public_pipeline_merges_overrides_honors_refresh_and_emits_local_cover_flag](../../../functions/tests/flag_pipeline/public_pipeline_merges_overrides_honors_refresh_and_emits_local_cover_flag.md)