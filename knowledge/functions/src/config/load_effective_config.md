---
type: Rust Function
title: load_effective_config
resource: src/config.rs#L2487-L2493
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_config_with_override
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/load_config
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generated_artifacts_from_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/page_functions/new_row
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_utils/globals/write_globals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/scoped_album_override_applies_stamp_visibility_keys_in_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_flag_image_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_effective_config(path: &str) -> Config`

# Calls

- [load_config_with_override](../../../functions/src/config/load_config_with_override.md)
- [load_config](../../../functions/src/config/load_config.md)

# Called by

- [generate_album_output_conn](../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [fetch_and_filter_stamps](../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)
- [generated_artifacts_from_path](../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [new_row](../../../functions/src/album_globals/page_functions/new_row.md)
- [write_globals](../../../functions/src/album_utils/globals/write_globals.md)
- [scoped_album_override_applies_stamp_visibility_keys_in_effective_config](../../../functions/src/config/scoped_album_override_applies_stamp_visibility_keys_in_effective_config.md)
- [ensure_flag_image_with_fetcher](../../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)
- [ensure_coat_of_arms_with_fetcher](../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)