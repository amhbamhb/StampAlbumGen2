---
type: Rust Function
title: test_nondgit_suffix_variant_is_accepted
resource: tests/colnect_integration.rs#L120-L128
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/colnect_integration/setup_colnect_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/colnect/db/get_colnect_info
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_nondgit_suffix_variant_is_accepted()`

# Calls

- [setup_colnect_db](../../../functions/tests/colnect_integration/setup_colnect_db.md)
- [get_colnect_info](../../../functions/src/colnect/db/get_colnect_info.md)