---
type: Rust Function
title: enforce_runtime_schema_compatibility
resource: src/db_utils/migrations.rs#L360-L404
visibility: private
generated:
  by: okf-rs/0.4.0
relationships:
  calls:
  - target: functions/src/db_utils/migrations/ensure_columns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/album_globals/structs/GenericAttrs/get
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/db_utils/migrations/run_pending_schema_migrations
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn enforce_runtime_schema_compatibility(conn: &Connection) -> Result<()>`

# Calls

- [ensure_columns](../../../../functions/src/db_utils/migrations/ensure_columns.md)
- [get](../../../../functions/src/album_globals/structs/GenericAttrs/get.md)

# Called by

- [run_pending_schema_migrations](../../../../functions/src/db_utils/migrations/run_pending_schema_migrations.md)