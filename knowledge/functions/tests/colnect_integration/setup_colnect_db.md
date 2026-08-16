---
type: Rust Function
title: setup_colnect_db
resource: tests/colnect_integration.rs#L9-L44
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
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

`fn setup_colnect_db(rows: &[(&str, &str)]) -> Connection`

# Called by

- [test_exact_match_returns_correct_stamp](../../../functions/tests/colnect_integration/test_exact_match_returns_correct_stamp.md)
- [test_nondgit_suffix_variant_is_accepted](../../../functions/tests/colnect_integration/test_nondgit_suffix_variant_is_accepted.md)
- [test_digit_suffix_variant_is_rejected](../../../functions/tests/colnect_integration/test_digit_suffix_variant_is_rejected.md)
- [test_only_genuine_variant_chosen_when_mixed](../../../functions/tests/colnect_integration/test_only_genuine_variant_chosen_when_mixed.md)
- [test_star_suffix_variant_is_accepted](../../../functions/tests/colnect_integration/test_star_suffix_variant_is_accepted.md)
- [test_no_match_returns_none](../../../functions/tests/colnect_integration/test_no_match_returns_none.md)