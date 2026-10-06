---
type: Rust Function
title: load_effective_config
resource: src/config.rs#L2616-L2622
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_config_with_override
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn_prepared
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl
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
  - target: functions/src/external_album/generate_artifacts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_flag_image_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/flagdb/ensure_coat_of_arms_with_fetcher
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/album_colnect_pdf_filters/verify_filtered_output
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn load_effective_config(path: &str) -> Config`

# Calls

- [load_config_with_override](../../../functions/src/config/load_config_with_override.md)

# Called by

- [generate_album_output_conn_prepared](../../../functions/src/album_globals/generator/generate_album_output_conn_prepared.md)
- [fetch_and_filter_stamps_impl](../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl.md)
- [generated_artifacts_from_path](../../../functions/src/album_globals/generated_artifacts_from_path.md)
- [new_row](../../../functions/src/album_globals/page_functions/new_row.md)
- [write_globals](../../../functions/src/album_utils/globals/write_globals.md)
- [scoped_album_override_applies_stamp_visibility_keys_in_effective_config](../../../functions/src/config/scoped_album_override_applies_stamp_visibility_keys_in_effective_config.md)
- [generate_artifacts](../../../functions/src/external_album/generate_artifacts.md)
- [ensure_flag_image_with_fetcher](../../../functions/src/flagdb/ensure_flag_image_with_fetcher.md)
- [ensure_coat_of_arms_with_fetcher](../../../functions/src/flagdb/ensure_coat_of_arms_with_fetcher.md)
- [verify_filtered_output](../../../functions/tests/album_colnect_pdf_filters/verify_filtered_output.md)