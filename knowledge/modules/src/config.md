---
type: Rust Module
title: config
resource: src/config.rs#L1-L2846
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/log-debug-info
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-deserialize-serialize
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-cell-refcell
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-self-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-isterminal
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-env
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-process-command
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-os-unix-process-commandext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/rfd-filedialog-messagedialog-messagebuttons-messagelevel
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-collections-hashmap
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-detect-duplicate-keys-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-time-systemtime-unix-epoch
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [embedded_config_template](../../functions/src/config/embedded_config_template.md)
- [load_template_content_with_fallback](../../functions/src/config/load_template_content_with_fallback.md)
- [ensure_config_and_template_files](../../functions/src/config/ensure_config_and_template_files.md)
- [HeaderFormatting](../../classes/src/config/HeaderFormatting.md)
- [Config](../../classes/src/config/Config.md)
- [ColnectConfig](../../classes/src/config/ColnectConfig.md)
- [ColnectFieldToggles](../../classes/src/config/ColnectFieldToggles.md)
- [ReviewUiConfig](../../classes/src/config/ReviewUiConfig.md)
- [with_defaults](../../functions/src/config/Config/with_defaults.md)
- [colnect_details_enabled](../../functions/src/config/Config/colnect_details_enabled.md)
- [effective_colnect_detail_fields](../../functions/src/config/Config/effective_colnect_detail_fields.md)
- [effective_colnect_detail_field_set](../../functions/src/config/Config/effective_colnect_detail_field_set.md)
- [review_ui_odd_row_color](../../functions/src/config/Config/review_ui_odd_row_color.md)
- [review_ui_even_row_color](../../functions/src/config/Config/review_ui_even_row_color.md)
- [review_ui_default_sort_mode](../../functions/src/config/Config/review_ui_default_sort_mode.md)
- [review_ui_series_default_checked](../../functions/src/config/Config/review_ui_series_default_checked.md)
- [review_ui_description_default_checked](../../functions/src/config/Config/review_ui_description_default_checked.md)
- [review_ui_watermark_default_checked](../../functions/src/config/Config/review_ui_watermark_default_checked.md)
- [review_ui_design_default_checked](../../functions/src/config/Config/review_ui_design_default_checked.md)
- [review_ui_engraving_default_checked](../../functions/src/config/Config/review_ui_engraving_default_checked.md)
- [review_ui_perforation_default_checked](../../functions/src/config/Config/review_ui_perforation_default_checked.md)
- [review_ui_context_default_checked](../../functions/src/config/Config/review_ui_context_default_checked.md)
- [review_ui_colnect_links_default_checked](../../functions/src/config/Config/review_ui_colnect_links_default_checked.md)
- [review_ui_colnect_image_url_default_checked](../../functions/src/config/Config/review_ui_colnect_image_url_default_checked.md)
- [review_ui_colnect_metadata_default_checked](../../functions/src/config/Config/review_ui_colnect_metadata_default_checked.md)
- [review_ui_replace_sg_number_default_checked](../../functions/src/config/Config/review_ui_replace_sg_number_default_checked.md)
- [review_ui_right_unmatched_only_default_checked](../../functions/src/config/Config/review_ui_right_unmatched_only_default_checked.md)
- [review_ui_catalog_family_options](../../functions/src/config/Config/review_ui_catalog_family_options.md)
- [effective_default_catalog_code](../../functions/src/config/Config/effective_default_catalog_code.md)
- [wikipedia_ingest_enabled](../../functions/src/config/Config/wikipedia_ingest_enabled.md)
- [effective_wikipedia_ingest_max_entries](../../functions/src/config/Config/effective_wikipedia_ingest_max_entries.md)
- [effective_wikipedia_ingest_series_prefix](../../functions/src/config/Config/effective_wikipedia_ingest_series_prefix.md)
- [effective_stamp_heading_label](../../functions/src/config/Config/effective_stamp_heading_label.md)
- [effective_colnect_inbox_path](../../functions/src/config/Config/effective_colnect_inbox_path.md)
- [effective_albumeasy_cli_path](../../functions/src/config/Config/effective_albumeasy_cli_path.md)
- [effective_bootstrap_albums_root_path](../../functions/src/config/Config/effective_bootstrap_albums_root_path.md)
- [albumeasy_auto_pdf_enabled](../../functions/src/config/Config/albumeasy_auto_pdf_enabled.md)
- [pdf_dx_logging_enabled](../../functions/src/config/Config/pdf_dx_logging_enabled.md)
- [check_config_staleness](../../functions/src/config/check_config_staleness.md)
- [extract_keys](../../functions/src/config/extract_keys.md)
- [create_config_from_template](../../functions/src/config/create_config_from_template.md)
- [detect_duplicate_keys](../../functions/src/config/detect_duplicate_keys.md)
- [load_config](../../functions/src/config/load_config.md)
- [with_album_config_override](../../functions/src/config/with_album_config_override.md)
- [load_effective_config](../../functions/src/config/load_effective_config.md)
- [colnect_fields_boolean_toggles_are_parsed](../../functions/src/config/colnect_fields_boolean_toggles_are_parsed.md)
- [colnect_printing_type_alias_is_supported](../../functions/src/config/colnect_printing_type_alias_is_supported.md)
- [colnect_all_false_toggles_override_detail_fields_to_empty](../../functions/src/config/colnect_all_false_toggles_override_detail_fields_to_empty.md)
- [colnect_false_override_disables_global_true_field](../../functions/src/config/colnect_false_override_disables_global_true_field.md)
- [duplicate_detection_ignores_same_key_in_different_sections](../../functions/src/config/duplicate_detection_ignores_same_key_in_different_sections.md)
- [duplicate_detection_finds_duplicates_in_same_section](../../functions/src/config/duplicate_detection_finds_duplicates_in_same_section.md)
- [album_override_section_value_wins_over_root_for_stamp_visibility_keys](../../functions/src/config/album_override_section_value_wins_over_root_for_stamp_visibility_keys.md)
- [scoped_album_override_applies_stamp_visibility_keys_in_effective_config](../../functions/src/config/scoped_album_override_applies_stamp_visibility_keys_in_effective_config.md)
- [deep_merge_toml_values](../../functions/src/config/deep_merge_toml_values.md)
- [load_config_with_override](../../functions/src/config/load_config_with_override.md)
- [load_slug_overrides](../../functions/src/config/load_slug_overrides.md)

# Imports

- `log::{debug, info}`
- `serde::{Deserialize, Serialize}`
- `std::cell::RefCell`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `std::io::{self, Write}`
- `std::io::IsTerminal`
- `std::env`
- `std::process::Command`
- `std::os::unix::process::CommandExt`
- `rfd::{FileDialog, MessageDialog, MessageButtons, MessageLevel}`
- `std::collections::HashMap`
- `super::{detect_duplicate_keys, Config}`
- `std::time::{SystemTime, UNIX_EPOCH}`

# Member of

- [stamp_album_gen2](../../packages/stamp_album_gen2.md)