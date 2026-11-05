---
type: Rust Function
title: check_hash
resource: tests/albumeasy_baseline.rs#L35-L44
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/external_album/digest
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/albumeasy_baseline/check_frozen
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn check_hash(bytes: &[u8], expected: &str, label: &str) -> Result<()>`

# Calls

- [digest](../../../functions/src/external_album/digest.md)

# Called by

- [check_frozen](../../../functions/tests/albumeasy_baseline/check_frozen.md)