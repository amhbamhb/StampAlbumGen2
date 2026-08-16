---
type: Rust Function
title: get_colnect_info
resource: src/colnect/db.rs#L114-L209
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/colnect/db/load_colnect_info_with_query
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/hydrate_catalog_codes
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_colnect_integration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_exact_match_returns_correct_stamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_nondgit_suffix_variant_is_accepted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_digit_suffix_variant_is_rejected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_only_genuine_variant_chosen_when_mixed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_star_suffix_variant_is_accepted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/colnect_integration/test_no_match_returns_none
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn get_colnect_info(conn: &Connection, sg_number: &str) -> Result<Option<ColnectInfo>>`

# Calls

- [load_colnect_info_with_query](../../../../functions/src/colnect/db/load_colnect_info_with_query.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)
- [hydrate_catalog_codes](../../../../functions/src/colnect/db/hydrate_catalog_codes.md)

# Called by

- [fetch_and_filter_stamps](../../../../functions/src/album_globals/generator_new/helpers/fetch_and_filter_stamps.md)
- [test_colnect_integration](../../../../functions/tests/colnect_integration/test_colnect_integration.md)
- [test_exact_match_returns_correct_stamp](../../../../functions/tests/colnect_integration/test_exact_match_returns_correct_stamp.md)
- [test_nondgit_suffix_variant_is_accepted](../../../../functions/tests/colnect_integration/test_nondgit_suffix_variant_is_accepted.md)
- [test_digit_suffix_variant_is_rejected](../../../../functions/tests/colnect_integration/test_digit_suffix_variant_is_rejected.md)
- [test_only_genuine_variant_chosen_when_mixed](../../../../functions/tests/colnect_integration/test_only_genuine_variant_chosen_when_mixed.md)
- [test_star_suffix_variant_is_accepted](../../../../functions/tests/colnect_integration/test_star_suffix_variant_is_accepted.md)
- [test_no_match_returns_none](../../../../functions/tests/colnect_integration/test_no_match_returns_none.md)