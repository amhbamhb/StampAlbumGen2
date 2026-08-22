---
type: Rust Function
title: migration_002_stamps_uniq_key_and_indexes
resource: src/db_utils/migrations.rs#L292-L337
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/cleanup_legacy_stampworld_identity_duplicates
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn migration_002_stamps_uniq_key_and_indexes(conn: &Connection) -> Result<()>`

# Calls

- [cleanup_legacy_stampworld_identity_duplicates](../../../../functions/src/db_utils/migrations/cleanup_legacy_stampworld_identity_duplicates.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)