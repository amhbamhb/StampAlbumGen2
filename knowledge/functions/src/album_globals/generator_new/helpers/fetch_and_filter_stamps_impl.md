---
type: Rust Function
title: fetch_and_filter_stamps_impl
resource: src/album_globals/generator_new/helpers.rs#L418-L748
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/config/load_effective_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/effective_default_catalog_code
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/leading_year_from_series
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/set
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/load_stored_colnect_meta
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/set_attr_if_non_empty
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/get_colnect_info_by_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/get_colnect_info
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/set_attr_if_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/preferred_catalog_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/infer_catalog_type_for_number
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_for_selection
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fetch_and_filter_stamps_impl( conn: &Connection, series: &str, start_year: Option<i32>, end_year: Option<i32>, strict_colnect: bool, ) -> SqlResult<Vec<Stamp>>`

# Calls

- [load_effective_config](../../../../../functions/src/config/load_effective_config.md)
- [effective_default_catalog_code](../../../../../functions/src/config/Config/effective_default_catalog_code.md)
- [leading_year_from_series](../../../../../functions/src/album_globals/generator_new/helpers/leading_year_from_series.md)
- [set](../../../../../functions/src/album_globals/structs/GenericAttrs/set.md)
- [load_stored_colnect_meta](../../../../../functions/src/album_globals/generator_new/helpers/load_stored_colnect_meta.md)
- [set_attr_if_non_empty](../../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_non_empty.md)
- [get_colnect_info_by_id](../../../../../functions/src/colnect/db/get_colnect_info_by_id.md)
- [get_colnect_info](../../../../../functions/src/colnect/db/get_colnect_info.md)
- [set_attr_if_missing](../../../../../functions/src/album_globals/generator_new/helpers/set_attr_if_missing.md)
- [get](../../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [preferred_catalog_entry](../../../../../functions/src/catalog/preferred_catalog_entry.md)
- [normalize_catalog_number](../../../../../functions/src/catalog/normalize_catalog_number.md)
- [normalize_catalog_type](../../../../../functions/src/catalog/normalize_catalog_type.md)
- [infer_catalog_type_for_number](../../../../../functions/src/catalog/infer_catalog_type_for_number.md)

# Called by

- [fetch_and_filter_stamps](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)
- [fetch_and_filter_stamps_for_selection](../../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_for_selection.md)