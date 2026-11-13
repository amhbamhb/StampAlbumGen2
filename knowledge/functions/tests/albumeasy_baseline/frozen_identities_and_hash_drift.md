---
type: Rust Function
title: frozen_identities_and_hash_drift
resource: tests/albumeasy_baseline.rs#L283-L290
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/tests/albumeasy_baseline/check_frozen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/albumeasy_baseline/expectations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn frozen_identities_and_hash_drift() -> Result<()>`

# Calls

- [check_frozen](../../../functions/tests/albumeasy_baseline/check_frozen.md)
- [expectations](../../../functions/tests/albumeasy_baseline/expectations.md)