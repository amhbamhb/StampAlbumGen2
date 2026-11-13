---
type: Rust Function
title: normalize_catalog_number
resource: src/catalog.rs#L92-L112
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/preferred_catalog_number
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/catalog/infer_catalog_type_for_number
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

`pub fn normalize_catalog_number(raw: &str) -> String`

# Calls

- [contains](../../../functions/src/native_pdf/stamp_image/contains.md)

# Called by

- [fetch_and_filter_stamps_impl](../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps_impl.md)
- [preferred_catalog_number](../../../functions/src/catalog/preferred_catalog_number.md)
- [infer_catalog_type_for_number](../../../functions/src/catalog/infer_catalog_type_for_number.md)
- [apply_matches](../../../functions/src/review_ui/matcher_service/apply_matches.md)
- [add_colnect_stamp_to_stamps](../../../functions/src/review_ui/matcher_service/add_colnect_stamp_to_stamps.md)