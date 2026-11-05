---
type: Rust Function
title: check_frozen
resource: tests/albumeasy_baseline.rs#L270-L280
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/albumeasy_baseline/expectations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/check_hash
    resolved_by: tree-sitter
    confidence: exact
  called_by:
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

`fn check_frozen() -> Result<()>`

# Calls

- [expectations](../../../functions/tests/albumeasy_baseline/expectations.md)
- [check_hash](../../../functions/tests/albumeasy_baseline/check_hash.md)

# Called by

- [frozen_identities_and_hash_drift](../../../functions/tests/albumeasy_baseline/frozen_identities_and_hash_drift.md)
- [oxidize_reference_text_and_negative_controls](../../../functions/tests/albumeasy_baseline/oxidize_reference_text_and_negative_controls.md)
- [reference_geometry_text_and_negative_controls](../../../functions/tests/albumeasy_baseline/reference_geometry_text_and_negative_controls.md)
- [candidate_matches_reference_strictly](../../../functions/tests/albumeasy_baseline/candidate_matches_reference_strictly.md)