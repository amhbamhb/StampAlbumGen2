---
type: Rust Function
title: infer_catalog_type_for_number
resource: src/catalog.rs#L151-L163
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/catalog/normalize_catalog_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/parse_catalog_codes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/normalize_catalog_type
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/apply_matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn infer_catalog_type_for_number( raw_catalog_codes: &str, displayed_number: &str, fallback_catalog_code: &str, ) -> String`

# Calls

- [normalize_catalog_number](../../../functions/src/catalog/normalize_catalog_number.md)
- [parse_catalog_codes](../../../functions/src/catalog/parse_catalog_codes.md)
- [normalize_catalog_type](../../../functions/src/catalog/normalize_catalog_type.md)

# Called by

- [fetch_and_filter_stamps_impl](../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl.md)
- [infer_and_fix_catalog_codes_for_colnect_rows](../../../functions/src/modes/infer_and_fix_catalog_codes_for_colnect_rows.md)
- [apply_matches](../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [add_colnect_stamp_to_stamps](../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)