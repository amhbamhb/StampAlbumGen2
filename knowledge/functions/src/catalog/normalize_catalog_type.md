---
type: Rust Function
title: normalize_catalog_type
resource: src/catalog.rs#L1-L14
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/wikipedia/models/ContentSource/as_str
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator/generate_album_output_conn
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/catalog_display_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/parse_catalog_codes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/preferred_catalog_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/infer_catalog_type_for_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/config_editor/update_default_catalog_code_for_album
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_left_stamp
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn normalize_catalog_type(code: &str) -> String`

# Calls

- [as_str](../../../functions/src/wikipedia/models/ContentSource/as_str.md)

# Called by

- [generate_album_output_conn](../../../functions/src/album_globals/generator/generate_album_output_conn.md)
- [resolve_stamp_heading_label](../../../functions/src/album_globals/generator_new/emit/resolve_stamp_heading_label.md)
- [fetch_and_filter_stamps](../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)
- [catalog_display_name](../../../functions/src/catalog/catalog_display_name.md)
- [parse_catalog_codes](../../../functions/src/catalog/parse_catalog_codes.md)
- [preferred_catalog_entry](../../../functions/src/catalog/preferred_catalog_entry.md)
- [infer_catalog_type_for_number](../../../functions/src/catalog/infer_catalog_type_for_number.md)
- [infer_and_fix_catalog_codes_for_colnect_rows](../../../functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows.md)
- [update_default_catalog_code_for_album](../../../functions/src/review_ui/config_editor/update_default_catalog_code_for_album.md)
- [add_left_stamp](../../../functions/src/review_ui/matcher_service/add_left_stamp.md)