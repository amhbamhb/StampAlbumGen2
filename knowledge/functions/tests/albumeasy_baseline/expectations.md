---
type: Rust Function
title: expectations
resource: tests/albumeasy_baseline.rs#L29-L33
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  called_by:
  - target: functions/tests/albumeasy_baseline/check_frozen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/frozen_identities_and_hash_drift
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/oxidize_reference_text_and_negative_controls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/reference_geometry_text_and_negative_controls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/candidate_matches_reference_strictly
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn expectations() -> Result<Expectations>`

# Called by

- [check_frozen](../../../functions/tests/albumeasy_baseline/check_frozen.md)
- [frozen_identities_and_hash_drift](../../../functions/tests/albumeasy_baseline/frozen_identities_and_hash_drift.md)
- [oxidize_reference_text_and_negative_controls](../../../functions/tests/albumeasy_baseline/oxidize_reference_text_and_negative_controls.md)
- [reference_geometry_text_and_negative_controls](../../../functions/tests/albumeasy_baseline/reference_geometry_text_and_negative_controls.md)
- [candidate_matches_reference_strictly](../../../functions/tests/albumeasy_baseline/candidate_matches_reference_strictly.md)