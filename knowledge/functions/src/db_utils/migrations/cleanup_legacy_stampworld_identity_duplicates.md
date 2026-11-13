---
type: Rust Function
title: cleanup_legacy_stampworld_identity_duplicates
resource: src/db_utils/migrations.rs#L157-L208
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db_utils/migrations/migration_002_stamps_uniq_key_and_indexes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn cleanup_legacy_stampworld_identity_duplicates(conn: &Connection) -> Result<usize>`

# Calls

- [prepare](../../../../functions/src/native_pdf/stamp_image/StampImageProofSettings/prepare.md)

# Called by

- [migration_002_stamps_uniq_key_and_indexes](../../../../functions/src/db_utils/migrations/migration_002_stamps_uniq_key_and_indexes.md)