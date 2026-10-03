---
type: Rust Function
title: mark_colnect_candidate_processed_sets_flag_for_matching_candidate
resource: src/review_ui.rs#L3400-L3424
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/review_ui/matcher_service/mark_colnect_candidate_processed
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn mark_colnect_candidate_processed_sets_flag_for_matching_candidate()`

# Calls

- [mark_colnect_candidate_processed](../../../functions/src/review_ui/matcher_service/mark_colnect_candidate_processed.md)
- [get](../../../functions/src/album_globals/structs/GenericAttrs/get.md)