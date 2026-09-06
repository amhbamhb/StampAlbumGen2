---
type: Rust Module
title: config_editor
resource: src/review_ui/config_editor.rs#L1-L1264
generated:
  by: okf-rs/0.4.0
relationships:
  imports:
  - target: external/std-collections-btreemap-hashmap-hashset
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-fs
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-configeditorfield-configeditorsection
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/stamp_album_gen2
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [update_default_catalog_code_for_album](../../../functions/src/review_ui/config_editor/update_default_catalog_code_for_album.md)
- [config_field_id](../../../functions/src/review_ui/config_editor/config_field_id.md)
- [friendly_config_section_title](../../../functions/src/review_ui/config_editor/friendly_config_section_title.md)
- [parse_config_key_value_line](../../../functions/src/review_ui/config_editor/parse_config_key_value_line.md)
- [load_example_config_help_map](../../../functions/src/review_ui/config_editor/load_example_config_help_map.md)
- [enrich_field_help](../../../functions/src/review_ui/config_editor/enrich_field_help.md)
- [template_literal_kind](../../../functions/src/review_ui/config_editor/template_literal_kind.md)
- [quote_toml_basic_string](../../../functions/src/review_ui/config_editor/quote_toml_basic_string.md)
- [normalize_value_for_template](../../../functions/src/review_ui/config_editor/normalize_value_for_template.md)
- [ensure_config_section](../../../functions/src/review_ui/config_editor/ensure_config_section.md)
- [parse_template_config_sections](../../../functions/src/review_ui/config_editor/parse_template_config_sections.md)
- [flatten_toml_value_into_map](../../../functions/src/review_ui/config_editor/flatten_toml_value_into_map.md)
- [load_current_config_values](../../../functions/src/review_ui/config_editor/load_current_config_values.md)
- [build_config_editor_data](../../../functions/src/review_ui/config_editor/build_config_editor_data.md)
- [render_config_from_template_values](../../../functions/src/review_ui/config_editor/render_config_from_template_values.md)
- [is_multiline_array_start](../../../functions/src/review_ui/config_editor/is_multiline_array_start.md)
- [save_full_config_values](../../../functions/src/review_ui/config_editor/save_full_config_values.md)
- [build_config_editor_data_for_album](../../../functions/src/review_ui/config_editor/build_config_editor_data_for_album.md)
- [save_album_config_diff](../../../functions/src/review_ui/config_editor/save_album_config_diff.md)
- [render_config_editor_assets](../../../functions/src/review_ui/config_editor/render_config_editor_assets.md)

# Imports

- `std::collections::{BTreeMap, HashMap, HashSet}`
- `std::fs`
- `crate::config`
- `super::{ConfigEditorField, ConfigEditorSection}`

# Member of

- [stamp_album_gen2](../../../packages/stamp_album_gen2.md)